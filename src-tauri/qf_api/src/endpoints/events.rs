use std::{
    sync::{Arc, Mutex, OnceLock, Weak},
    time::Duration,
};

use reqwest::Method;
use tokio::runtime::Runtime;
use tokio::sync::Mutex as AsyncMutex;
use utils::{LoggerOptions, error, info, warning};

use crate::{client::Client, enums::ResponseFormat, errors::ApiError, types::*};

const EVENT_BATCH_SIZE: usize = 50;
const EVENT_FLUSH_INTERVAL: Duration = Duration::from_secs(10);

fn log_options() -> LoggerOptions {
    LoggerOptions::default().set_file("events.log")
}

fn flush_runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .thread_name("qf-events-flush")
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("Failed to build events flush runtime")
    })
}

#[derive(Debug)]
pub struct EventsRoute {
    client: Weak<Client>,
    queue: Mutex<Vec<CreateEventDto>>,
    flush_lock: AsyncMutex<()>,
}

impl EventsRoute {
    pub fn new(client: Arc<Client>) -> Arc<Self> {
        let route = Arc::new(Self {
            client: Arc::downgrade(&client),
            queue: Mutex::new(Vec::new()),
            flush_lock: AsyncMutex::new(()),
        });

        Self::start_flush_task(&route);

        route
    }

    /// Queues an event for background sending.
    ///
    /// This method does not perform any network I/O and is safe to call
    /// from any thread, including outside of a tokio runtime context.
    pub fn track_event(&self, dto: CreateEventDto) {
        self.queue.lock().unwrap().push(dto);
    }

    async fn flush(&self) -> Result<(), ApiError> {
        let _flush_lock = self.flush_lock.lock().await;

        let mut events = {
            let mut queue = self.queue.lock().unwrap();

            if queue.is_empty() {
                return Ok(());
            }

            std::mem::take(&mut *queue)
        };

        info(
            "Events:Flush",
            format!("Flushing {} queued events", events.len()),
            &log_options(),
        );

        let client = match self.client.upgrade() {
            Some(client) => client,
            None => {
                warning(
                    "Events:Flush",
                    "Client dropped, requeueing events",
                    &log_options(),
                );
                let mut queue = self.queue.lock().unwrap();
                queue.append(&mut events);

                return Ok(());
            }
        };

        loop {
            let (batch, rest) = events.split_at(events.len().min(EVENT_BATCH_SIZE));

            let body = serde_json::json!({
                "events": batch,
            });

            match client
                .call_api::<String>(
                    Method::POST,
                    "/events/track",
                    Some(body),
                    None,
                    ResponseFormat::String,
                )
                .await
            {
                Ok(_) => {
                    info(
                        "Events:Flush",
                        format!("Sent batch of {} events", batch.len()),
                        &log_options(),
                    );
                    events = rest.to_vec();
                    if events.is_empty() {
                        return Ok(());
                    }
                }
                Err(e) => {
                    error(
                        "Events:Flush",
                        format!("Failed to send batch of {} events: {:?}", batch.len(), e),
                        &log_options(),
                    );
                    let mut queue = self.queue.lock().unwrap();
                    queue.extend(batch.iter().cloned());
                    queue.extend(rest.iter().cloned());

                    return Err(e);
                }
            }
        }
    }

    fn start_flush_task(route: &Arc<Self>) {
        let route = Arc::clone(route);

        info("Events:Flush", "Flush task started", &log_options());

        flush_runtime().spawn(async move {
            let mut interval = tokio::time::interval(EVENT_FLUSH_INTERVAL);

            loop {
                interval.tick().await;

                if let Err(e) = route.flush().await {
                    error(
                        "Events:Flush",
                        format!("Flush failed: {:?}", e),
                        &log_options(),
                    );
                }
            }
        });
    }

    pub fn from_existing(_old: &EventsRoute, client: Arc<Client>) -> Arc<Self> {
        Self::new(client)
    }
}

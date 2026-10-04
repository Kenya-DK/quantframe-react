use std::sync::{Arc, Mutex};

use qf_api::{types::CreateEventDto, Client as QFClient};

pub struct Analytics {
    _stop: Arc<Mutex<bool>>,
    client: Mutex<QFClient>,
}

impl Analytics {
    pub fn new(client: QFClient) -> Arc<Self> {
        Arc::new(Self {
            // Analytics is disabled until the user is authenticated and not banned.
            _stop: Arc::new(Mutex::new(true)),
            client: Mutex::new(client),
        })
    }

    pub fn set_client(&self, client: QFClient) {
        let mut current = self.client.lock().unwrap();
        *current = client;
    }

    fn is_stopped(&self) -> bool {
        *self._stop.lock().unwrap()
    }

    // --------------------------------------------------
    // Events
    // --------------------------------------------------

    pub fn track_event<I, K, V>(
        &self,
        event_type: qf_api::enums::app_events::ApplicationEvent,
        properties: I,
    ) where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        // Do not queue events while logged out or banned.
        if self.is_stopped() {
            return;
        }

        let event = CreateEventDto::from_pairs(event_type, properties);

        self.client.lock().unwrap().events().track_event(event);
    }

    // --------------------------------------------------
    // Lifecycle
    // --------------------------------------------------

    pub fn start(&self) {
        let mut stop = self._stop.lock().unwrap();
        *stop = false;
    }

    pub fn stop(&self) {
        let mut stop = self._stop.lock().unwrap();
        *stop = true;
    }

    /// Force-flushes any queued events. Used on shutdown so the `app_exit`
    /// event is sent before the process terminates.
    pub fn flush(&self) {
        let events = match self.client.lock() {
            Ok(client) => client.events(),
            Err(_) => return,
        };

        if let Err(e) = tauri::async_runtime::block_on(events.flush_now()) {
            utils::warning(
                "Analytics:Flush",
                format!("Failed to flush events on shutdown: {e:?}"),
                &utils::LoggerOptions::default(),
            );
        }
    }
}

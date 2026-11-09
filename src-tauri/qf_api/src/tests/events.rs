//! End-to-end tests for `EventsRoute::flush` against a scripted mock server.
//!
//! The client targets `http://localhost:6969` in development mode, so the mock
//! binds that port on both IPv4 and IPv6 loopback. All scenarios run inside a
//! single test so they don't race for the port.

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{Client, enums::ApplicationEvent, types::CreateEventDto};

#[derive(Default)]
struct MockState {
    /// (status, body) responses handed out in order.
    responses: VecDeque<(u16, String)>,
    /// Number of events in each received `/events/track` request.
    received: Vec<usize>,
}

type Shared = Arc<Mutex<MockState>>;

fn reason(status: u16) -> &'static str {
    match status {
        201 => "Created",
        400 => "Bad Request",
        401 => "Unauthorized",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        _ => "Status",
    }
}

async fn handle(mut stream: tokio::net::TcpStream, state: Shared) {
    let mut buf = Vec::new();
    let mut tmp = [0u8; 8192];
    // Read until headers + full body have arrived.
    loop {
        let n = stream.read(&mut tmp).await.unwrap_or(0);
        if n == 0 {
            return;
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            let head = String::from_utf8_lossy(&buf[..pos]).to_lowercase();
            let len = head
                .lines()
                .find_map(|l| l.strip_prefix("content-length:"))
                .map(|v| v.trim().parse::<usize>().unwrap())
                .unwrap_or(0);
            if buf.len() >= pos + 4 + len {
                let body: serde_json::Value =
                    serde_json::from_slice(&buf[pos + 4..pos + 4 + len]).unwrap();
                let count = body["events"].as_array().map(|a| a.len()).unwrap_or(0);
                let (status, resp_body) = {
                    let mut s = state.lock().unwrap();
                    s.received.push(count);
                    s.responses
                        .pop_front()
                        .expect("mock server received an unexpected request")
                };
                let resp = format!(
                    "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    status,
                    reason(status),
                    resp_body.len(),
                    resp_body
                );
                let _ = stream.write_all(resp.as_bytes()).await;
                let _ = stream.shutdown().await;
                return;
            }
        }
    }
}

async fn serve(listener: TcpListener, state: Shared) {
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle(stream, state.clone()));
        }
    }
}

fn script(state: &Shared, responses: &[(u16, &str)]) {
    let mut s = state.lock().unwrap();
    s.received.clear();
    s.responses = responses
        .iter()
        .map(|(c, b)| (*c, b.to_string()))
        .collect();
}

fn received(state: &Shared) -> Vec<usize> {
    state.lock().unwrap().received.clone()
}

fn queue_events(client: &Client, n: usize) {
    for _ in 0..n {
        client
            .events()
            .track_event(CreateEventDto::new(ApplicationEvent::AppStart, Default::default()));
    }
}

const CREATED: &str = r#"{"ok":true}"#;
const NEST_400: &str = r#"{"message":["events.0.event must be one of the following values: app_start"],"error":"Bad Request","statusCode":400}"#;
const ERR_BODY: &str = r#"{"message":"nope","error":"Error","statusCode":0}"#;

#[tokio::test(flavor = "multi_thread")]
async fn flush_drops_rejected_batches_and_requeues_transient_failures() {
    let state: Shared = Arc::new(Mutex::new(MockState::default()));
    let v4 = TcpListener::bind("127.0.0.1:6969")
        .await
        .expect("port 6969 must be free (is the local API running?)");
    tokio::spawn(serve(v4, state.clone()));
    if let Ok(v6) = TcpListener::bind("[::1]:6969").await {
        tokio::spawn(serve(v6, state.clone()));
    }

    let client = Client::new(
        "", "test", "windows", "test", true, "Quantframe", "0.0.0", "pc", "tester", "id",
        false,
    );
    let events = client.events();

    // 1. A 400 on the first batch drops it and the second batch is still sent.
    queue_events(&client, 60);
    script(&state, &[(400, NEST_400), (201, CREATED)]);
    events.flush_now().await.expect("flush should succeed after dropping");
    assert_eq!(received(&state), vec![50, 10], "both batches attempted once");

    // Nothing left in the queue: a further flush sends nothing.
    script(&state, &[]);
    events.flush_now().await.unwrap();
    assert!(received(&state).is_empty(), "rejected batch must not be retried");

    // 2. A 400 with an unparsable body (ParsingError) is also dropped.
    queue_events(&client, 2);
    script(&state, &[(400, "not json")]);
    events.flush_now().await.unwrap();
    script(&state, &[]);
    events.flush_now().await.unwrap();
    assert!(received(&state).is_empty());

    // 3. Transient failures keep the events and they go out on the next flush.
    for status in [500u16, 429, 401] {
        queue_events(&client, 3);
        script(&state, &[(status, ERR_BODY)]);
        assert!(
            events.flush_now().await.is_err(),
            "status {status} should surface an error"
        );
        script(&state, &[(201, CREATED)]);
        events.flush_now().await.unwrap();
        assert_eq!(received(&state), vec![3], "status {status} should requeue");
    }

    // 4. The happy path still works.
    queue_events(&client, 5);
    script(&state, &[(201, CREATED)]);
    events.flush_now().await.unwrap();
    assert_eq!(received(&state), vec![5]);
}

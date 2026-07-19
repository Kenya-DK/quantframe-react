use chrono::Utc;
use serde_json::json;
use std::sync::{
    mpsc::{self, Sender},
    Mutex, OnceLock,
};

static LOKI_STATE: OnceLock<Mutex<LokiState>> = OnceLock::new();

struct LokiState {
    sender: Option<Sender<LokiEntry>>,
    url: String,
    extra_labels: Vec<(String, String)>,
}

struct LokiEntry {
    level: String,
    component: String,
    message: String,
}

fn get_or_init_state() -> &'static Mutex<LokiState> {
    LOKI_STATE.get_or_init(|| {
        Mutex::new(LokiState {
            sender: None,
            url: String::new(),
            extra_labels: Vec::new(),
        })
    })
}

/// Initialise the Loki logger. Call once at app startup.
/// `url` is the full Loki push endpoint (e.g. `http://localhost:3100/loki/api/v1/push`).
/// `extra_labels` are additional key=value labels attached to every log stream.
pub fn init(url: impl Into<String>, extra_labels: Vec<(String, String)>) {
    let url_str = url.into();
    let labels = extra_labels.clone();
    let url_for_thread = url_str.clone();
    let (tx, rx) = mpsc::channel::<LokiEntry>();

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok();

    std::thread::spawn(move || {
        let client = match client {
            Some(c) => c,
            None => return,
        };

        let mut batch: Vec<LokiEntry> = Vec::new();
        let mut last_flush = std::time::Instant::now();
        let max_batch = 100;
        let flush_interval = std::time::Duration::from_secs(2);

        loop {
            let deadline = last_flush + flush_interval;
            let remaining = deadline
                .checked_duration_since(std::time::Instant::now())
                .unwrap_or_default();

            if let Ok(entry) = rx.recv_timeout(remaining) {
                batch.push(entry);
                if batch.len() >= max_batch {
                    flush(&client, &url_for_thread, &labels, &mut batch);
                    last_flush = std::time::Instant::now();
                }
            } else {
                if !batch.is_empty() {
                    flush(&client, &url_for_thread, &labels, &mut batch);
                }
                last_flush = std::time::Instant::now();
            }
        }
    });

    if let Ok(mut state) = get_or_init_state().lock() {
        state.sender = Some(tx);
        state.url = url_str;
        state.extra_labels = extra_labels;
    }
}

fn flush(
    client: &reqwest::blocking::Client,
    url: &str,
    extra_labels: &[(String, String)],
    batch: &mut Vec<LokiEntry>,
) {
    if batch.is_empty() {
        return;
    }

    let entries = std::mem::take(batch);
    let mut streams: Vec<serde_json::Value> = Vec::new();
    let mut stream_map: std::collections::BTreeMap<(String, String), Vec<String>> =
        std::collections::BTreeMap::new();

    for entry in entries {
        let key = (entry.component.clone(), entry.level.clone());
        let ts = Utc::now()
            .timestamp_nanos_opt()
            .unwrap_or(0)
            .to_string();
        stream_map
            .entry(key.clone())
            .or_default()
            .push(ts);
        stream_map
            .entry(key)
            .or_default()
            .push(entry.message);
    }

    for ((component, level), values) in stream_map {
        let mut labels = serde_json::Map::new();
        labels.insert("component".into(), json!(component));
        labels.insert("level".into(), json!(level));
        for (k, v) in extra_labels {
            labels.insert(k.clone(), json!(v));
        }

        let paired: Vec<Vec<String>> = values.chunks(2).map(|c| c.to_vec()).collect();

        streams.push(json!({
            "stream": labels,
            "values": paired,
        }));
    }

    if streams.is_empty() {
        return;
    }

    let body = json!({ "streams": streams });

    let _ = client.post(url).json(&body).send();
}

/// Push a single log entry to Loki (called from dolog).
pub fn push(level: &str, component: &str, message: &str) {
    if let Ok(state) = get_or_init_state().lock() {
        if let Some(ref sender) = state.sender {
            let _ = sender.send(LokiEntry {
                level: level.to_string(),
                component: component.to_string(),
                message: message.to_string(),
            });
        }
    }
}

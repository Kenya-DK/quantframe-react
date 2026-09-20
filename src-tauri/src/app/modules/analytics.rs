use std::sync::{Arc, Mutex};

use qf_api::{types::CreateEventDto, Client as QFClient};

pub struct Analytics {
    _stop: Arc<Mutex<bool>>,
    client: Mutex<QFClient>,
}

impl Analytics {
    pub fn new(client: QFClient) -> Arc<Self> {
        Arc::new(Self {
            _stop: Arc::new(Mutex::new(false)),
            client: Mutex::new(client),
        })
    }

    pub fn set_client(&self, client: QFClient) {
        let mut current = self.client.lock().unwrap();
        *current = client;
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
}

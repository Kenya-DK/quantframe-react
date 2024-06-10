



use crate::{
    qf_client::{client::QFClient},
};

#[derive(Clone, Debug)]
pub struct AnalyticsModule {
    pub client: QFClient,
    component: String,
}

impl AnalyticsModule {
    pub fn new(client: QFClient) -> Self {
      AnalyticsModule {
            client,
            component: "Analytics".to_string(),
        }
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
  }

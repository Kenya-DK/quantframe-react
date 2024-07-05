use crate::qf_client::client::QFClient;

#[derive(Clone, Debug)]
pub struct AnalyticsModule {
    pub client: QFClient,
}

impl AnalyticsModule {
    pub fn new(client: QFClient) -> Self {
        AnalyticsModule { client }
    }
}





#[derive(PartialEq, Debug, Clone)]
pub enum AnalyticsEvent {
    Initialize,
    InitializeError,
    Install,
}
impl AnalyticsEvent {
    pub fn as_str(&self) -> &str {
        match *self {
            AnalyticsEvent::Initialize => "Initialize",
            AnalyticsEvent::InitializeError => "InitializeError",
            AnalyticsEvent::Install => "Install",
        }
    }
}

use eyre::eyre;

use crate::{
    helper,
    log_parser::client::LogParser,
    utils::modules::{
        error::AppError,
        logger::{self, LoggerOptions},
        states,
    },
};

#[derive(Clone, Debug)]
pub struct OnMissionEvent {
    pub client: LogParser,
    component: String,
    regex: Vec<String>,
    logs: Vec<String>,
    getting_mission_multiline: bool,
}

impl OnMissionEvent {
    pub fn new(client: LogParser) -> Self {
        OnMissionEvent {
            client,
            component: "OnMissionEvent".to_string(),
            logs: vec![],
            getting_mission_multiline: false,
            regex: vec![
                r"Client loaded (.*?) with MissionInfo:".to_string(),
                r"Host loading (.*?) with MissionInfo:".to_string(),
            ],
        }
    }
    fn update_state(&self) {
        self.client.update_mission_event(self.clone());
    }
    fn get_component(&self, component: &str) -> String {
        format!("{}:{}:{}", self.client.component, self.component, component)
    }
    pub fn process_line(&mut self, line: &str, _pos: u64) -> Result<bool, AppError> {
        return Ok(false);
        let component = self.get_component("ProcessLine");
        let settings = states::settings()?;
        let notify = states::notify_client()?;

        while self.getting_mission_multiline {
            let machs: Vec<String> = vec![
                "[Info]".to_string(),
                "[Error]".to_string(),
                "[Warning]".to_string(),
            ];
            if machs.iter().any(|mach| line.contains(mach)) {
                let info = self.logs.join("\n").replace("info=", " ");
                println!("Full Mission Detected: {}", info);
                self.getting_mission_multiline = false;
                self.update_state();
                return Ok(true);
            } else {
                self.logs.push(line.to_string());
                self.update_state();
                return Ok(false);
            }
        }

        // 575.016 Script [Info]: EndOfMatch.lua: Mission Succeeded
        // 575.016 Script [Info]: EndOfMatch.lua: EndOfMatch.lua:

        if line.contains("[Info]: EndOfMatch.lua: Initialize") {
            println!("Mission Done Detected: {}", line);
        }
        if !line.contains(" with MissionInfo:") {
            return Ok(false);
        }
        let (found, captures) = helper::match_pattern(line, self.regex.clone())
            .map_err(|e| AppError::new("OnNewConversationEvent", eyre!(e)))?;
        if found {
            let info = captures.get(0).unwrap().clone().unwrap();
            self.getting_mission_multiline = true;
            self.update_state();
            println!("Mission Info Detected: {}", info);
        }
        logger::info(
            &self.get_component(&component),
            &format!("Mission Info: {}", line),
            LoggerOptions::default(),
        );
        Ok(false)
    }
}

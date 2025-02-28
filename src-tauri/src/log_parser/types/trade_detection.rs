use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, PartialEq)]
pub enum DetectionStatus {
    None,
    Line,
    CombinedWithSpace,
    CombinedWithOutSpace,
}

impl DetectionStatus {
    pub fn is_found(&self) -> bool {
        match self {
            DetectionStatus::None => false,
            _ => true,
        }
    }
    pub fn is_combined(&self) -> bool {
        match self {
            DetectionStatus::CombinedWithOutSpace | DetectionStatus::CombinedWithSpace => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TradeDetection {
    pub start: String,
    pub confirmation_line: String,
    pub failed_line: String,
    pub cancelled_line: String,
    pub receive_line_first_part: String,
    pub receive_line_second_part: String,
    pub platinum_name: String,
}

impl TradeDetection {
    pub fn new(
        start: String,
        confirmation_line: String,
        failed_line: String,
        cancelled_line: String,
        receive_line_first_part: String,
        receive_line_second_part: String,
        platinum_name: String,
    ) -> Self {
        TradeDetection {
            start,
            confirmation_line,
            failed_line,
            cancelled_line,
            receive_line_first_part,
            receive_line_second_part,
            platinum_name,
        }
    }

    pub fn was_trade_failed(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, is_previous);

        if is_dialog.is_found() && line.contains(&self.failed_line) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) = self.mach_combined(&line, next_line, &self.failed_line, is_previous);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
    pub fn was_trade_cancelled(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, is_previous);

        if is_dialog.is_found() && line.contains(&self.cancelled_line) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) = self.mach_combined(&line, next_line, &self.cancelled_line, is_previous);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
    pub fn was_trade_successful(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, is_previous);

        if is_dialog.is_found() && line.contains(&self.confirmation_line) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) =
            self.mach_combined(&line, next_line, &self.confirmation_line, is_previous);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
    pub fn is_first_part(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        if line.contains(&self.receive_line_first_part) {
            return (line.to_string(), DetectionStatus::Line);
        }

        if ignore_combined {
            return ("".to_string(), DetectionStatus::None);
        }

        self.mach_combined(&line, next_line, &self.receive_line_first_part, is_previous)
    }
    pub fn is_second_part(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        if line.contains(&self.receive_line_second_part) {
            return (line.to_string(), DetectionStatus::Line);
        }

        if ignore_combined {
            return ("".to_string(), DetectionStatus::None);
        }

        self.mach_combined(
            &line,
            next_line,
            &self.receive_line_second_part,
            is_previous,
        )
    }
    pub fn is_platinum(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        if line.contains(&self.platinum_name) && next_line.trim().starts_with("x") {
            return (
                format!("{} {}", self.platinum_name, next_line.trim()),
                DetectionStatus::CombinedWithOutSpace,
            );
        }

        if line.contains(&self.platinum_name) {
            return (line.to_string(), DetectionStatus::Line);
        }

        if ignore_combined {
            return (line.to_string().to_string(), DetectionStatus::None);
        }

        self.mach_combined(&line, next_line, &self.platinum_name, is_previous)
    }
    pub fn is_offer_line(&self, line: &str, next_line: &str) -> (String, DetectionStatus) {
        let (_, first_status) = self.is_first_part(line, next_line, false, false);
        let (mut second_part, second_status) = self.is_second_part(line, next_line, false, false);

        if first_status.is_found() && second_status.is_found() {
            if first_status == DetectionStatus::Line && second_status.is_combined() {
                second_part = second_part.replace(
                    &self.receive_line_first_part.trim(),
                    &self.receive_line_first_part,
                );
            }
            let status = if first_status.is_combined() || second_status.is_combined() {
                DetectionStatus::CombinedWithSpace
            } else {
                DetectionStatus::Line
            };

            // println!("First part: {:?}, Line:{}", first_status, first_part);
            // println!("Second part: {:?}, Line:{}", second_status, second_part);
            // println!("Status: {:?}", status);
            return (second_part, status);
        }
        ("".to_string(), DetectionStatus::None)
    }
    pub fn is_end_of_trade(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let machs: Vec<String> = vec![
            "[Info]".to_string(),
            "[Error]".to_string(),
            "[Warning]".to_string(),
        ];
        if machs.iter().any(|mach| line.contains(mach)) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        for mach in machs.iter() {
            let (_, status) = self.mach_combined(&line, next_line, mach, is_previous);
            if status.is_found() {
                return status;
            }
        }
        DetectionStatus::None
    }
    pub fn is_irrelevant_trade_line(
        &self,
        line: &str,
        next_line: &str,
    ) -> (bool, String, DetectionStatus) {
        if line == "\n" || line == "" {
            return (false, "NotIrrelevant".to_string(), DetectionStatus::None);
        }
        let is_beginning = self.is_beginning_of_trade(line, next_line, false, false);
        if is_beginning.is_found() {
            return (false, "BeginningOfTrade".to_string(), is_beginning);
        }

        let was_successful = self.was_trade_successful(line, next_line, false, false);
        if was_successful.is_found() {
            return (false, "TradeSuccessful".to_string(), was_successful);
        }

        let was_failed = self.was_trade_failed(line, next_line, false, false);
        if was_failed.is_found() {
            return (false, "TradeFailed".to_string(), was_failed);
        }
        (true, "Irrelevant".to_string(), DetectionStatus::None)
    }

    pub fn is_dialog_line(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
    ) -> DetectionStatus {
        let machs: Vec<String> = vec![
            "[Info]: Dialog.lua: Dialog::CreateOkCancel(description=".to_string(),
            "[Info]: Dialog.lua: Dialog::CreateOk(description=".to_string(),
        ];
        if machs.iter().any(|mach| line.contains(mach)) {
            return DetectionStatus::Line;
        }
        for mach in machs.iter() {
            let (_, status) = self.mach_combined(&line, next_line, mach, is_previous);
            if status.is_found() {
                return status;
            }
        }
        DetectionStatus::None
    }

    fn mach_combined(
        &self,
        line: &str,
        next_line: &str,
        mach_to_find: &str,
        is_previous: bool,
    ) -> (String, DetectionStatus) {
        if !is_previous && next_line == "" {
            return (line.to_string(), DetectionStatus::None);
        } else if is_previous && line == "" {
            return (next_line.to_string(), DetectionStatus::None);
        }

        let trimmed_combined = if is_previous {
            next_line.trim().to_owned() + line.trim()
        } else {
            line.trim().to_owned() + next_line.trim()
        };
        if trimmed_combined.contains(mach_to_find) {
            return (trimmed_combined, DetectionStatus::CombinedWithOutSpace);
        }
        let trimmed_combined = if is_previous {
            next_line.trim().to_owned() + " " + line.trim()
        } else {
            line.trim().to_owned() + " " + next_line.trim()
        };
        if trimmed_combined.contains(mach_to_find) {
            return (trimmed_combined, DetectionStatus::CombinedWithSpace);
        }
        if !is_previous {
            return (line.to_string(), DetectionStatus::None);
        } else {
            return (next_line.to_string(), DetectionStatus::None);
        }
    }

    pub fn is_beginning_of_trade(
        &self,
        line: &str,
        next_line: &str,
        is_previous: bool,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, is_previous);

        if is_dialog.is_found() && line.contains(&self.start) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) = self.mach_combined(&line, next_line, &self.start, is_previous);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
}

pub static DETECTIONS: OnceLock<HashMap<String, TradeDetection>> = OnceLock::new();

pub fn init_detections() {
    DETECTIONS.get_or_init(|| {
        let mut detections = HashMap::new();
        detections.insert(
            "en".to_string(),
            TradeDetection::new(
                "description=Are you sure you want to accept this trade? You are offering"
                    .to_string(),
                "description=The trade was successful!, leftItem=/Menu/Confirm_Item_Ok".to_string(),
                "description=The trade failed., leftItem=/Menu/Confirm_Item_Ok".to_string(),
                "description=The trade was cancelled".to_string(),
                "and will receive from ".to_string(),
                " the following:".to_string(),
                "Platinum".to_string(),
            ),
        );
        detections
    });
}

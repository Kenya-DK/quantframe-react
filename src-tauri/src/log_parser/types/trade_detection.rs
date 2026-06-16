use std::{collections::HashMap, sync::OnceLock};

use regex::Regex;
use utils::{combine_and_detect_match, DetectionStatus};

use crate::{enums::TradeItemType, log_parser::TradeResult};

#[derive(Clone, Debug)]
pub struct TradeDetection {
    pub start: String,
    pub offer_line: Regex,
    pub confirmation_line: String,
    pub failed_line: String,
    pub cancelled_line: String,
    pub receive_line_first_part: String,
    pub receive_line_second_part: String,
    pub platinum_name: String,
    pub credits_name: String,
    pub imprint_name: String,
}

impl TradeDetection {
    pub fn new(
        start: impl Into<String>,
        confirmation_line: impl Into<String>,
        failed_line: impl Into<String>,
        cancelled_line: impl Into<String>,
        receive_line_first_part: impl Into<String>,
        receive_line_second_part: impl Into<String>,
        platinum_name: impl Into<String>,
        credits_name: impl Into<String>,
        imprint_name: impl Into<String>,
    ) -> Self {
        TradeDetection {
            start: start.into(),
            confirmation_line: confirmation_line.into(),
            failed_line: failed_line.into(),
            cancelled_line: cancelled_line.into(),
            receive_line_first_part: receive_line_first_part.into(),
            receive_line_second_part: receive_line_second_part.into(),
            platinum_name: platinum_name.into(),
            credits_name: credits_name.into(),
            imprint_name: imprint_name.into(),
        }
    }

    fn detect_trade_state(
        &self,
        line: &str,
        prev_line: &str,
        target: &str,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, prev_line, ignore_combined);

        if !is_dialog.is_found() {
            return DetectionStatus::None;
        }
        let (_, status) =
            combine_and_detect_match(&line, next_line, &self.failed_line, ignore_combined, false);

        let (_, status) = combine_and_detect_match(line, prev_line, target, ignore_combined, false);

        if status.is_found() {
            status
        } else {
            DetectionStatus::None
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) = combine_and_detect_match(
            &line,
            next_line,
            &self.cancelled_line,
            ignore_combined,
            false,
        );

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }

    pub fn get_trade_result(
        &self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> (DetectionStatus, TradeResult) {
        let checks: [(DetectionStatus, TradeResult); 3] = [
            (
                self.detect_trade_state(line, prev_line, &self.confirmation_line, ignore_combined),
                TradeResult::Success,
            ),
            (
                self.detect_trade_state(line, prev_line, &self.failed_line, ignore_combined),
                TradeResult::Failed,
            ),
            (
                self.detect_trade_state(line, prev_line, &self.cancelled_line, ignore_combined),
                TradeResult::Cancelled,
            ),
        ];

        for (status, result) in checks {
            if status.is_found() {
                return (status, result);
            }
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) = combine_and_detect_match(
            &line,
            next_line,
            &self.confirmation_line,
            ignore_combined,
            false,
        );

        (DetectionStatus::None, TradeResult::Unknown)
    }

    fn detect_trade_state(
        &self,
        line: &str,
        next_line: &str,
        target: &str,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, ignore_combined);

        if !is_dialog.is_found() {
            return DetectionStatus::None;
        }

        let (_, status) = combine_and_detect_match(line, next_line, target, ignore_combined, false);

        if status.is_found() {
            status
        } else {
            DetectionStatus::None
        }
    }

    pub fn get_trade_result(
        &self,
        line: &str,
        next_line: &str,
        ignore_combined: bool,
    ) -> (DetectionStatus, TradeResult) {
        let checks: [(DetectionStatus, TradeResult); 3] = [
            (
                self.detect_trade_state(line, next_line, &self.confirmation_line, ignore_combined),
                TradeResult::Success,
            ),
            (
                self.detect_trade_state(line, next_line, &self.failed_line, ignore_combined),
                TradeResult::Failed,
            ),
            (
                self.detect_trade_state(line, next_line, &self.cancelled_line, ignore_combined),
                TradeResult::Cancelled,
            ),
        ];

        for (status, result) in checks {
            if status.is_found() {
                return (status, result);
            }
        }

        (DetectionStatus::None, TradeResult::Unknown)
    }

    pub fn is_first_part(
        &self,
        line: &str,
        next_line: &str,
        _is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        combine_and_detect_match(
            &line,
            prev_line,
            &self.receive_line_first_part,
            ignore_combined,
            false,
        )
    }
    pub fn is_second_part(
        &self,
        line: &str,
        next_line: &str,
        _is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        combine_and_detect_match(
            &line,
            prev_line,
            &self.receive_line_second_part,
            ignore_combined,
            false,
        )
    }

    pub fn is_currency(
        &self,
        line: &str,
        next_line: &str,
        _is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus, TradeItemType) {
        let detect = |match_text: &str, ty, suffix: &str| {
            let (full_text, status) = combine_and_detect_match(
                &line,
                prev_line,
                &format!("{}{}", match_text, suffix),
                ignore_combined,
                false,
            );
            (full_text, status, ty)
        };

        for (name, ty, suffix) in [
            (&self.platinum_name, TradeItemType::Platinum, " x"),
            (&self.credits_name, TradeItemType::Credits, ""),
        ] {
            let (combined, status) =
                combine_and_detect_match(line, next_line, name, ignore_combined, false);
            if status.is_found() {
                return (full_text, status, ty);
            }
        }
        return (
            line.to_string(),
            DetectionStatus::None,
            TradeItemType::Unknown,
        );
    }
    pub fn is_offer_line(
        &self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        let (first_part, first_status) = self.is_first_part(line, prev_line, ignore_combined);

        if !first_status.is_found() {
            return (line.to_string(), DetectionStatus::None);
        }

        let (second_part, second_status) = self.is_second_part(line, prev_line, ignore_combined);

        if !second_status.is_found() {
            return (line.to_string(), DetectionStatus::None);
        }

        println!(
            "First Part: '{}' | Status: {:?} | Second Part: '{}' | Status: {:?}",
            first_part, first_status, second_part, second_status
        );

        let text = if first_status == second_status {
            first_part.clone()
        } else {
            line.to_string()
        };

        let status = if first_status == second_status {
            first_status
        } else {
            DetectionStatus::Line
        };
        return (text, status);
    }
    pub fn is_end_of_trade(
        &self,
        line: &str,
        next_line: &str,
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
            let (_, status) =
                combine_and_detect_match(&line, next_line, mach, ignore_combined, false);
            if status.is_found() {
                return status;
            }
        }
        DetectionStatus::None
    }
    pub fn is_irrelevant_trade_line(&self, line: &str, next_line: &str) -> (bool, DetectionStatus) {
        if line == "\n" || line == "" {
            return (false, DetectionStatus::None);
        }
        let is_beginning = self.is_beginning_of_trade(line, next_line, false);
        if is_beginning.is_found() {
            return (false, is_beginning);
        }

    pub fn is_dialog_line(
        &self,
        line: &str,
        next_line: &str,
        ignore_combined: bool,
    ) -> DetectionStatus {
        const MATCHES: [&str; 2] = [
            "[Info]: Dialog.lua: Dialog::CreateOkCancel(description=",
            "[Info]: Dialog.lua: Dialog::CreateOk(description=",
        ];

        for pattern in MATCHES {
            let (_, status) =
                combine_and_detect_match(line, next_line, pattern, ignore_combined, false);

            if status.is_found() {
                return status;
            }
        }

        DetectionStatus::None
    }
    pub fn is_beginning_of_trade(
        &self,
        line: &str,
        next_line: &str,
        ignore_combined: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, next_line, ignore_combined);

        if is_dialog.is_found() && line.contains(&self.start) {
            return DetectionStatus::Line;
        }
        if ignore_combined {
            return DetectionStatus::None;
        }
        let (_, status) =
            combine_and_detect_match(&line, next_line, &self.start, ignore_combined, false);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
    pub fn is_last_item(
        &self,
        line: &str,
        next_line: &str,
        _is_previous: bool,
        ignore_combined: bool,
    ) -> (String, DetectionStatus) {
        let last_item_mach = ", leftItem=/Menu/Confirm_Item_Ok";
        // let last_item_mach = ", title= leftItem=/Menu/Confirm_Item_Ok";
        if next_line.contains(last_item_mach) {
            return (next_line.to_string(), DetectionStatus::NextLine);
        }
        if line.contains(match_pattern) {
            return (line.to_string(), DetectionStatus::Line);
        }

        if ignore_combined {
            return (line.to_owned(), DetectionStatus::None);
        }

        combine_and_detect_match(&line, next_line, last_item_mach, ignore_combined, false)
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
                "description=The trade was successful!",
                "description=The trade failed.",
                "description=The trade was cancelled",
                "and will receive from ",
                " the following:",
                "Platinum",
                "Credits",
                "imprint of",
            ),
        );
        detections.insert(
            "ru".to_string(),
            TradeDetection::new(
                "description=Вы хотите принять условия сделки? Вы предлагаете",
                "description=Обмен успешно завершён!",
                "description=Обмен не удался.",
                "description=Обмен был отменён",
                "и получите от ",
                " следующее:",
                "Платина",
                "Кредиты",
                "оттиск от",
            ),
        );
        detections
    });
}

use std::{collections::HashMap, sync::OnceLock};

use regex::Regex;
use utils::{combine_and_detect_match, DetectionStatus};

use crate::{enums::TradeItemType, log_parser::TradeResult};

#[derive(Clone, Debug)]
pub struct TradeDetection {
    pub start: String,
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
        ignored_combinations: &[DetectionStatus],
        skip_dialog_check: bool,
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, prev_line, ignored_combinations);

        if !is_dialog.is_found() && !skip_dialog_check {
            return DetectionStatus::None;
        }

        let (_, status) =
            combine_and_detect_match(line, prev_line, target, ignored_combinations, false);

        if status.is_found() {
            status
        } else {
            DetectionStatus::None
        }
    }

    pub fn get_trade_result(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> (DetectionStatus, TradeResult) {
        let checks: [(DetectionStatus, TradeResult); 4] = [
            (
                self.detect_trade_state(
                    line,
                    prev_line,
                    &self.confirmation_line,
                    ignored_combinations,
                    false,
                ),
                TradeResult::Success,
            ),
            (
                self.detect_trade_state(
                    line,
                    prev_line,
                    &self.failed_line,
                    ignored_combinations,
                    false,
                ),
                TradeResult::Failed,
            ),
            (
                self.detect_trade_state(
                    line,
                    prev_line,
                    &self.cancelled_line,
                    ignored_combinations,
                    false,
                ),
                TradeResult::Cancelled,
            ),
            (
                self.detect_trade_state(
                    line,
                    prev_line,
                    "[Info]: OnTradeAccepted failed",
                    ignored_combinations,
                    true,
                ),
                TradeResult::OnTradeAcceptedFailed,
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
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> (String, DetectionStatus) {
        combine_and_detect_match(
            &line,
            prev_line,
            &self.receive_line_first_part,
            ignored_combinations,
            false,
        )
    }
    pub fn is_second_part(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> (String, DetectionStatus) {
        combine_and_detect_match(
            &line,
            prev_line,
            &self.receive_line_second_part,
            ignored_combinations,
            false,
        )
    }

    pub fn is_currency(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> (String, DetectionStatus, TradeItemType) {
        let detect = |match_text: &str, ty, suffix: &str| {
            let (full_text, status) = combine_and_detect_match(
                &line,
                prev_line,
                &format!("{}{}", match_text, suffix),
                ignored_combinations,
                false,
            );
            (full_text, status, ty)
        };

        for (name, ty, suffix) in [
            (&self.platinum_name, TradeItemType::Platinum, " x"),
            (&self.credits_name, TradeItemType::Credits, ""),
        ] {
            let (full_text, status, ty) = detect(name, ty, suffix);
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
        ignored_combinations: &[DetectionStatus],
    ) -> (String, DetectionStatus) {
        let (first_part, first_status) = self.is_first_part(line, prev_line, ignored_combinations);

        if !first_status.is_found() {
            return (line.to_string(), DetectionStatus::None);
        }

        let (second_part, second_status) =
            self.is_second_part(line, prev_line, ignored_combinations);

        if !second_status.is_found() {
            return (line.to_string(), DetectionStatus::None);
        }

        // if first_status.is_found() || second_status.is_found() {
        //     println!(
        //         "First Path: {:?}, First Status: {:?}",
        //         first_part, first_status
        //     );
        //     println!(
        //         "Second Path: {:?}, Second Status: {:?}",
        //         second_part, second_status
        //     );
        // }

        let (text, status) = match (&first_status, &second_status) {
            (a, b) if a == b => (first_part.clone(), first_status.clone()),
            (DetectionStatus::LineThenPreviousLine, DetectionStatus::Line) => {
                (first_part.clone(), DetectionStatus::Combined)
            }
            (DetectionStatus::LineThenPreviousLine, DetectionStatus::PreviousLine) => {
                (first_part.clone(), DetectionStatus::Combined)
            }
            (DetectionStatus::Line, DetectionStatus::PreviousLine) => (
                format!("{}{}", first_part, second_part),
                DetectionStatus::Combined,
            ),
            (DetectionStatus::Line, DetectionStatus::LineThenPreviousLine) => {
                (second_part.clone(), DetectionStatus::Combined)
            }
            _ => (line.to_string(), DetectionStatus::Line),
        };

        let player_name = text
            .strip_prefix(&self.receive_line_first_part)
            .and_then(|s| s.strip_suffix(&self.receive_line_second_part))
            .unwrap_or("Unknown")
            .replace('\u{e000}', "")
            .replace('\u{e001}', "")
            .replace('', "");

        (player_name.trim().to_string(), status)
    }
    pub fn is_end_of_trade(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> DetectionStatus {
        let machs: Vec<String> = vec![
            "[Info]".to_string(),
            "[Error]".to_string(),
            "[Warning]".to_string(),
        ];
        for mach in machs.iter() {
            let (_, status) =
                combine_and_detect_match(&line, prev_line, mach, ignored_combinations, false);
            if status.is_found() {
                return status;
            }
        }
        DetectionStatus::None
    }

    pub fn is_dialog_line(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> DetectionStatus {
        const MATCHES: [&str; 2] = [
            "[Info]: Dialog.lua: Dialog::CreateOkCancel(description=",
            "[Info]: Dialog.lua: Dialog::CreateOk(description=",
        ];

        for pattern in MATCHES {
            let (_, status) =
                combine_and_detect_match(line, prev_line, pattern, ignored_combinations, false);

            if status.is_found() {
                return status;
            }
        }

        DetectionStatus::None
    }
    pub fn is_beginning_of_trade(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> DetectionStatus {
        let is_dialog = self.is_dialog_line(line, prev_line, ignored_combinations);

        if is_dialog.is_found() && line.contains(&self.start) {
            return DetectionStatus::Line;
        }
        if ignored_combinations.contains(&DetectionStatus::Combined) {
            return DetectionStatus::None;
        }
        let (_, status) =
            combine_and_detect_match(&line, prev_line, &self.start, ignored_combinations, false);

        if is_dialog.is_found() && status.is_found() {
            return status;
        }
        DetectionStatus::None
    }
    pub fn is_last_item(
        &self,
        line: &str,
        prev_line: &str,
        ignored_combinations: &[DetectionStatus],
    ) -> (String, DetectionStatus) {
        let matches = vec![
            ", title= leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)",
            ", leftItem=/Menu/Confirm_Item_Ok, rightItem=/Menu/Confirm_Item_Cancel)",
        ];
        for mach in matches.iter() {
            let (full_text, status) =
                combine_and_detect_match(&line, prev_line, mach, ignored_combinations, false);
            if status.is_found() {
                // Remove all text after the match to get the full item name
                let full_text = full_text.split(mach).next().unwrap_or("").to_string();
                return (full_text, status);
            }
        }
        (line.to_string(), DetectionStatus::None)
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

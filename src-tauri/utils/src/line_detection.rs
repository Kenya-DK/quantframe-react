use regex::Regex;

use crate::Error;

#[derive(Debug, PartialEq)]
pub enum DetectionStatus {
    None,
    Line,
    PreviousLine,
    Combined,
    LineThenPreviousLine,
    PreviousLineThenLine,
}

impl DetectionStatus {
    pub fn is_found(&self) -> bool {
        !matches!(self, DetectionStatus::None)
    }

    pub fn is_combined(&self) -> bool {
        matches!(self, DetectionStatus::Combined)
    }
    pub fn is_previous_line(&self) -> bool {
        matches!(
            self,
            DetectionStatus::PreviousLine | DetectionStatus::PreviousLineThenLine
        )
    }
    pub fn is_line(&self) -> bool {
        matches!(
            self,
            DetectionStatus::Line | DetectionStatus::LineThenPreviousLine
        )
    }
}

/// Checks if `line` matches **all** of the given patterns.
fn contains_any_match(line: &str, match_patterns: &[&str], is_exact_match: bool) -> bool {
    match_patterns
        .iter()
        .all(|&pattern| contains_match(line, pattern, is_exact_match))
}

/// Checks if `line` matches the given `match_pattern`.
fn contains_match(line: &str, match_pattern: &str, is_exact_match: bool) -> bool {
    if is_exact_match {
        line == match_pattern
    } else {
        line.contains(match_pattern)
    }
}
/// Checks if `line` matches the given `match_pattern`.
pub fn strip_prefix(
    prefix: impl Into<String>,
    line: &str,
    prev_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    let prefix = prefix.into();
    if let Some(part) = line.strip_prefix(&prefix) {
        let name_part = part.trim();
        return (name_part.to_string(), DetectionStatus::Line);
    }

    let combined = if use_previous_line {
        prev_line.to_owned() + line
    } else {
        line.to_owned() + prev_line
    };

    if let Some(part) = combined.strip_prefix(&prefix) {
        let name_part = part.trim();
        return (name_part.to_string(), DetectionStatus::Combined);
    }

    if use_previous_line {
        (prev_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

/// Detects if a line or a combined line contains Unicode characters.
pub fn contains_unicode(
    line: &str,
    prev_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    if line.len() != line.chars().count() {
        return (line.to_string(), DetectionStatus::Line);
    }

    let combined = if use_previous_line {
        prev_line.to_owned() + line
    } else {
        line.to_owned() + prev_line
    };

    if combined.len() != combined.chars().count() {
        return (combined, DetectionStatus::Combined);
    }

    if use_previous_line {
        (prev_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

/// Combines two lines and detects if the result matches a single pattern.
pub fn combine_and_detect_match(
    line: &str,
    prev_line: &str,
    match_pattern: &str,
    ignore_combined: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if contains_match(line, match_pattern, is_exact_match) {
        return (line.to_string(), DetectionStatus::Line);
    }

    if contains_match(prev_line, match_pattern, is_exact_match) {
        return (prev_line.to_string(), DetectionStatus::PreviousLine);
    }

    if !ignore_combined {
        let line_then_next = format!("{line}{prev_line}");
        if contains_match(&line_then_next, match_pattern, is_exact_match) {
            return (line_then_next, DetectionStatus::LineThenPreviousLine);
        }

        let previous_then_line = format!("{prev_line}{line}");
        if contains_match(&previous_then_line, match_pattern, is_exact_match) {
            return (previous_then_line, DetectionStatus::PreviousLineThenLine);
        }
    }

    (line.to_string(), DetectionStatus::None)
}

/// Combines two lines and detects if the result matches **all** given patterns.
pub fn combine_and_detect_multiple_matches(
    line: &str,
    prev_line: &str,
    match_patterns: &[&str],
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && prev_line.is_empty() {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line.is_empty() {
        return (prev_line.to_string(), DetectionStatus::None);
    }

    let combined = if use_previous_line {
        prev_line.to_owned() + line
    } else {
        line.to_owned() + prev_line
    };

    if contains_any_match(&combined, match_patterns, is_exact_match) {
        return (combined, DetectionStatus::Combined);
    }

    if use_previous_line {
        (prev_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

pub fn is_start_of_log(line: impl Into<String>) -> bool {
    let re = Regex::new(r"^\d+\.\d+\s").unwrap();
    if let Some(_) = re.captures(line.into().as_str()) {
        return true;
    } else {
        return false;
    }
}

pub fn contains_at_least(haystack: &str, needles: &str, count: usize, exact: bool) -> bool {
    let found = haystack.chars().filter(|c| needles.contains(*c)).count();
    if exact {
        found == count
    } else {
        found >= count
    }
}

pub fn remove_special_characters(input: &str) -> String {
    // Define the pattern for special characters except _ , space , - , .
    let pattern = Regex::new("[^a-zA-Z0-9_ \\-\\.]").unwrap();

    // Replace special characters with empty string
    let result = pattern.replace_all(input, "");

    result.into_owned()
}

pub fn is_match(
    input: impl Into<String>,
    to_match: impl Into<String>,
    ignore_case: bool,
    remove_string: Option<String>,
) -> bool {
    let mut input = input.into();
    if let Some(remove_string) = remove_string {
        input = input.replace(&remove_string, "");
    }
    let to_match = to_match.into();
    if ignore_case {
        input.to_lowercase() == to_match.to_lowercase()
    } else {
        input == to_match
    }
}

pub fn detect_enclosed_text(
    current_line: &str,
    next_line: &str,
    opening_delimiter: &str,
    closing_delimiter: &str,
) -> Option<(String, DetectionStatus)> {
    let has_open = contains_at_least(&current_line, opening_delimiter, 1, true);
    let has_close = contains_at_least(&current_line, closing_delimiter, 1, true);

    if has_open && has_close {
        return Some((current_line.to_string(), DetectionStatus::Line));
    }

    let (merged_text, detection_status) = combine_and_detect_multiple_matches(
        current_line,
        next_line,
        &[opening_delimiter, closing_delimiter],
        false,
        false,
    );

    if detection_status.is_found()
        && contains_at_least(&merged_text, opening_delimiter, 1, true)
        && contains_at_least(&merged_text, closing_delimiter, 1, true)
    {
        Some((merged_text, detection_status))
    } else {
        None
    }
}

pub fn split_base_name_and_enclosed_value(
    text: &str,
    opening_delimiter: char,
    closing_delimiter: char,
) -> (String, String) {
    let start_index = text.find(opening_delimiter).unwrap_or(0);

    let enclosed_value =
        text[start_index..].replace(&[opening_delimiter, closing_delimiter][..], "");

    let base_name = text[..start_index].trim_end();

    (base_name.to_string(), enclosed_value)
}

pub fn extract_item_variant(
    current_line: &str,
    next_line: &str,
    opening_delimiter: char,
    closing_delimiter: char,
) -> Result<(DetectionStatus, String, String), Error> {
    // Example: "Serration (RIVEN RANK 0)"
    if let Some((detected_text, detection_status)) = detect_enclosed_text(
        current_line,
        next_line,
        &opening_delimiter.to_string(),
        &closing_delimiter.to_string(),
    ) {
        let (item_name, enclosed_value) = split_base_name_and_enclosed_value(
            &detected_text,
            opening_delimiter,
            closing_delimiter,
        );

        return Ok((detection_status, item_name, enclosed_value));
    }

    Ok((DetectionStatus::None, String::new(), String::new()))
}

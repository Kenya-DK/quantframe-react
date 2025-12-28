use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum DetectionStatus {
    None,
    Line,
    NextLine,
    Combined,
}

impl DetectionStatus {
    pub fn is_found(&self) -> bool {
        !matches!(self, DetectionStatus::None)
    }

    pub fn is_combined(&self) -> bool {
        matches!(self, DetectionStatus::Combined)
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
    next_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    let prefix = prefix.into();
    if let Some(part) = line.strip_prefix(&prefix) {
        let name_part = part.trim();
        return (name_part.to_string(), DetectionStatus::Line);
    }

    let combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if let Some(part) = combined.strip_prefix(&prefix) {
        let name_part = part.trim();
        return (name_part.to_string(), DetectionStatus::Combined);
    }

    if use_previous_line {
        (next_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

/// Detects if a line or a combined line contains Unicode characters.
pub fn contains_unicode(
    line: &str,
    next_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    if line.len() != line.chars().count() {
        return (line.to_string(), DetectionStatus::Line);
    }

    let combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if combined.len() != combined.chars().count() {
        return (combined, DetectionStatus::Combined);
    }

    if use_previous_line {
        (next_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

/// Combines two lines and detects if the result matches a single pattern.
pub fn combine_and_detect_match(
    line: &str,
    next_line: &str,
    match_pattern: &str,
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line.is_empty() {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line.is_empty() {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if contains_match(&combined, match_pattern, is_exact_match) {
        return (combined, DetectionStatus::Combined);
    }

    if use_previous_line {
        (next_line.to_string(), DetectionStatus::None)
    } else {
        (line.to_string(), DetectionStatus::None)
    }
}

/// Combines two lines and detects if the result matches **all** given patterns.
pub fn combine_and_detect_multiple_matches(
    line: &str,
    next_line: &str,
    match_patterns: &[&str],
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line.is_empty() {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line.is_empty() {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let combined = if use_previous_line {
        next_line.to_owned() + line
    } else {
        line.to_owned() + next_line
    };

    if contains_any_match(&combined, match_patterns, is_exact_match) {
        return (combined, DetectionStatus::Combined);
    }

    if use_previous_line {
        (next_line.to_string(), DetectionStatus::None)
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
    // Define the pattern for special characters except _ and space
    let pattern = Regex::new("[^a-zA-Z0-9_ ]").unwrap();

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

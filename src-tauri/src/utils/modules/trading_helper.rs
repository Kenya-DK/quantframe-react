use crate::log_parser::types::trade_detection::DetectionStatus;

fn contains_any_match(line: &str, match_patterns: &[&str], is_exact_match: bool) -> bool {
    match_patterns
        .iter()
        .all(|&pattern| contains_match(line, pattern, is_exact_match))
}
fn contains_match(line: &str, match_pattern: &str, is_exact_match: bool) -> bool {
    if is_exact_match {
        line == match_pattern
    } else {
        line.contains(match_pattern)
    }
}

pub fn contains_unicode(
    line: &str,
    next_line: &str,
    use_previous_line: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line == "" {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line == "" {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let trimmed_combined = if use_previous_line {
        next_line.trim().to_owned() + line.trim()
    } else {
        line.trim().to_owned() + next_line.trim()
    };

    if trimmed_combined.len() != trimmed_combined.chars().count() {
        return (trimmed_combined, DetectionStatus::CombinedWithOutSpace);
    }

    let trimmed_combined = if use_previous_line {
        next_line.trim().to_owned() + " " + line.trim()
    } else {
        line.trim().to_owned() + " " + next_line.trim()
    };

    if trimmed_combined.len() != trimmed_combined.chars().count() {
        return (trimmed_combined, DetectionStatus::CombinedWithSpace);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}

pub fn combine_and_detect_match(
    line: &str,
    next_line: &str,
    match_pattern: &str,
    use_previous_line: bool,
    is_exact_match: bool,
) -> (String, DetectionStatus) {
    if !use_previous_line && next_line == "" {
        return (line.to_string(), DetectionStatus::None);
    } else if use_previous_line && line == "" {
        return (next_line.to_string(), DetectionStatus::None);
    }

    let trimmed_combined = if use_previous_line {
        next_line.trim().to_owned() + line.trim()
    } else {
        line.trim().to_owned() + next_line.trim()
    };

    if contains_match(&trimmed_combined, match_pattern, is_exact_match) {
        return (trimmed_combined, DetectionStatus::CombinedWithOutSpace);
    }

    let trimmed_combined = if use_previous_line {
        next_line.trim().to_owned() + " " + line.trim()
    } else {
        line.trim().to_owned() + " " + next_line.trim()
    };

    if contains_match(&trimmed_combined, match_pattern, is_exact_match) {
        return (trimmed_combined, DetectionStatus::CombinedWithSpace);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}
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

    let concatenated_string = if use_previous_line {
        next_line.trim().to_owned() + line.trim()
    } else {
        line.trim().to_owned() + next_line.trim()
    };

    if contains_any_match(&concatenated_string, match_patterns, is_exact_match) {
        return (concatenated_string, DetectionStatus::CombinedWithOutSpace);
    }

    let concatenated_with_space = if use_previous_line {
        next_line.trim().to_owned() + " " + line.trim()
    } else {
        line.trim().to_owned() + " " + next_line.trim()
    };

    if contains_any_match(&concatenated_with_space, match_patterns, is_exact_match) {
        return (concatenated_with_space, DetectionStatus::CombinedWithSpace);
    }

    if !use_previous_line {
        return (line.to_string(), DetectionStatus::None);
    } else {
        return (next_line.to_string(), DetectionStatus::None);
    }
}

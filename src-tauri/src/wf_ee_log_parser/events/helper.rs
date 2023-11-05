use crate::error::AppError;
use eyre::eyre;
use regex::Regex;

pub fn match_pattern(
    input: &str,
    regexs: Vec<String>,
) -> Result<(bool, Vec<Option<String>>), regex::Error> {
    // Loop through the regexs and try to match them
    for regex in regexs {
        let re: Regex = Regex::new(&regex)?;
        if let Some(captures) = re.captures(input) {
            let mut result: Vec<Option<String>> = vec![];
            for i in 1..captures.len() {
                let group = captures.get(i).map(|m| m.as_str().to_string());
                let group: Option<String> =
                    group.map(|s| s.chars().filter(|c| c.is_ascii()).collect());
                result.push(group);
            }
            return Ok((true, result));
        }
    }
    Ok((false, vec![]))
}

pub fn get_range_of_lines(
    file_path: &str,
    start_line: i32,
    min_line: i32,
    max_line: i32,
) -> Result<Vec<String>, AppError> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| AppError::new("get_range_of_lines", eyre!(e.to_string())))?;
    let reader = std::io::BufReader::new(file);
    let umax_line = (start_line + max_line) as usize; 
    let umin_line = (start_line - min_line).max(0) as usize;

    let mut lines = Vec::new();
    for (index, line) in std::io::BufRead::lines(reader).enumerate() {
        let line_number = index + 1;
        if line_number >= umin_line && line_number <= umax_line {
            if let Ok(line) = line {
                lines.push(line);
            }
        } else if line_number > umax_line {
            break; // Stop reading lines if we've passed the specified range
        }
    }

    Ok(lines)
}

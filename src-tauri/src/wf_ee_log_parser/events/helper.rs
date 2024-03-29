use regex::Regex;

pub fn match_pattern(
    input: &str,
    patterns: Vec<String>,
) -> Result<(bool, Vec<Option<String>>), regex::Error> {
    // Loop through the patterns and try to match them
    for regex in patterns {
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
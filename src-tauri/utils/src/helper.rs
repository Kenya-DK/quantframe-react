use std::{fs::File, io, path::PathBuf};

use serde_json::{Map, Value, json};
use uuid::Uuid;

use crate::{Error, get_location};

pub fn format_text(text: impl Into<String>, color: &str, bold: bool, use_color: bool) -> String {
    if !use_color {
        return text.into();
    }
    let color_code = match color {
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        "orange" => "38;5;208",
        _ => "0",
    };

    if bold {
        format!("\x1b[1;{}m{}\x1b[0m", color_code, text.into())
    } else {
        format!("\x1b[{}m{}\x1b[0m", color_code, text.into())
    }
}

pub fn format_square_bracket(msg: impl Into<String>, use_color: bool) -> String {
    format!(
        "{}{}{}",
        format_text("[", "cyan", false, use_color),
        msg.into(),
        format_text("]", "cyan", false, use_color)
    )
}

pub fn remove_ansi_codes(s: impl Into<String>) -> String {
    let re = regex::Regex::new(r"\x1B\[[0-9;]*[mK]").unwrap();
    re.replace_all(&s.into(), "").to_string()
}

pub fn mask_sensitive_data(data: &mut Map<String, Value>, properties: &[&str]) {
    for (key, value) in data.iter_mut() {
        match value {
            Value::Object(sub_object) => {
                mask_sensitive_data(sub_object, properties);
            }
            _ => {
                if properties.contains(&key.as_str()) {
                    *value = json!("*********");
                }
            }
        }
    }
}

pub fn read_json_file_optional<T: serde::de::DeserializeOwned + Default>(
    path: &PathBuf,
) -> Result<T, Error> {
    // Return default value if file doesn't exist
    if !path.exists() {
        eprintln!(
            "[WARNING] File does not exist, returning default: {}",
            path.display()
        );
        return Ok(T::default());
    }

    read_json_file(path)
}

pub fn read_json_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, Error> {
    // Check if the file exists
    if !path.exists() {
        return Err(Error::from_io(
            "Helper:ReadJsonFile",
            path,
            "File does not exist",
            std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
            get_location!(),
        ));
    }

    let file = File::open(path).map_err(|e| {
        Error::from_io(
            "Helper:ReadJsonFile",
            path,
            "Failed to open file",
            e,
            get_location!(),
        )
    })?;
    let reader = io::BufReader::new(file);
    let data: Value = serde_json::from_reader(reader).map_err(|e| {
        Error::from_json(
            "Helper:ReadJsonFile",
            &path,
            "N/A",
            "Failed to parse JSON file",
            e,
            get_location!(),
        )
    })?;
    match serde_json::from_value(data.clone()) {
        Ok(payload) => Ok(payload),
        Err(e) => Err(Error::from_json(
            "Helper:ReadJsonFile",
            &path,
            &data.to_string(),
            format!("Failed to deserialize JSON from file {}", path.display()),
            e,
            get_location!(),
        )),
    }
}
/// Find an object in a Vec<T> by multiple criteria using a predicate function
///
/// # Arguments
/// * `vec` - The vector to search in
/// * `predicate` - A function that returns true if the item matches all criteria
///
/// # Returns
/// `Option<&T>` - Reference to the found item, or None if not found
///
/// # Example
/// ```rust
/// #[derive(Debug)]
/// struct User {
///     id: u32,
///     name: String,
///     age: u32,
///     role: String,
/// }
///
/// let users = vec![
///     User { id: 1, name: "Alice".to_string(), age: 25, role: "admin".to_string() },
///     User { id: 2, name: "Bob".to_string(), age: 20, role: "user".to_string() },
///     User { id: 3, name: "Carol".to_string(), age: 20, role: "admin".to_string() },
/// ];
///
/// // Find user where id = 2 AND age = 20
/// if let Some(user) = find_by(&users, |u| u.id == 2 && u.age == 20) {
///     println!("Found user: {}", user.name);
/// }
///
/// // Find user where age = 20 AND role = "admin"
/// if let Some(user) = find_by(&users, |u| u.age == 20 && u.role == "admin") {
///     println!("Found admin user aged 20: {}", user.name);
/// }
/// ```
pub fn find_by<T>(vec: &[T], predicate: impl Fn(&T) -> bool) -> Option<&T> {
    vec.iter().find(|item| predicate(item))
}

/// Filters a vector by a predicate function
/// # Arguments
/// * `vec` - The vector to filter
/// * `predicate` - A function that returns true for items to keep
/// # Returns
/// `Vec<&T>` - A new vector containing references to items that match the predicate
/// # Example
/// ```rust
/// #[derive(Debug)]
/// struct User {
///    id: u32,
///   name: String,
///   age: u32,
///  role: String,
/// }
/// let users = vec![
///     User { id: 1, name: "Alice".to_string(), age: 25, role: "admin".to_string() },
///     User { id: 2, name: "Bob".to_string(), age: 20, role: "user".to_string() },
///     User { id: 3, name: "Carol".to_string(), age: 20, role: "admin".to_string() },
/// ];
/// // // Find all users aged 20
/// let young_users: Vec<&User> = filters_by(&users, |u| u
/// .age == 20);
/// for user in young_users {
///     println!("Found young user: {}", user.name);
/// }       
/// /// // Find all admin users
/// let admin_users: Vec<&User> = filters_by(&users, |u| u
/// .role == "admin");
/// for user in admin_users {
///     println!("Found admin user: {}", user.name);
/// }
/// ```
pub fn filters_by<T>(vec: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T>
where
    T: Clone,
{
    vec.iter().filter(|item| predicate(item)).cloned().collect()
}

/// Truncate a string if it exceeds the maximum length, adding a truncation indicator
///
/// # Arguments
/// * `text` - The text to potentially truncate
/// * `max_length` - Maximum allowed length before truncation
/// * `truncation_buffer` - Number of characters to reserve for the truncation indicator (default: 50)
///
/// # Returns
/// `(String, bool)` - The processed text and whether it was truncated
///
/// # Example
/// ```rust
/// let long_text = "x".repeat(5000);
/// let (result, was_truncated) = truncate_with_indicator(&long_text, 2048, 50);
///
/// if was_truncated {
///     println!("Text was truncated: {}", result);
///     // Output: "xxxx... [TRUNCATED - 5000 total chars]"
/// }
/// ```
pub fn truncate_with_indicator(
    text: &str,
    max_length: usize,
    truncation_buffer: Option<usize>,
) -> (String, bool) {
    let buffer = truncation_buffer.unwrap_or(50);

    if text.len() <= max_length {
        (text.to_string(), false)
    } else {
        let truncated_text = format!(
            "{}... [TRUNCATED - {} total chars]",
            &text[..max_length.saturating_sub(buffer)],
            text.len()
        );
        (truncated_text, true)
    }
}

/// Smart text processing for dual output (console and file) with different truncation rules
///
/// # Arguments
/// * `text` - The text to process
/// * `console_max_length` - Maximum length for console output
/// * `file_max_length` - Maximum length for file output (None = no limit)
/// * `label` - Label to prefix the text (e.g., "Context", "Stack")
///
/// # Returns
/// `(Option<String>, Option<String>)` - (console_text, file_text)
/// Returns None for either if the text should be skipped for that output
///
/// # Example
/// ```rust
/// let large_context = "x".repeat(5000);
/// let (console_text, file_text) = smart_text_processing(
///     &large_context,
///     2048,      // Console limit
///     Some(8192), // File limit
///     "Context"
/// );
///
/// if let Some(console) = console_text {
///     println!("Console: {}", console);
/// }
/// if let Some(file) = file_text {
///     println!("File: {}", file);
/// }
/// ```
pub fn smart_text_processing(
    text: &str,
    console_max_length: usize,
    file_max_length: Option<usize>,
    label: &str,
) -> (Option<String>, Option<String>) {
    if text.is_empty() {
        return (None, None);
    }

    // Process console text
    let console_text = if text.len() > console_max_length {
        let (truncated, _) = truncate_with_indicator(text, console_max_length, Some(50));
        Some(format!(" | {}: {}", label, truncated))
    } else {
        Some(format!(" | {}: {}", label, text))
    };

    // Process file text
    let file_text = if let Some(file_max) = file_max_length {
        if text.len() > file_max {
            let (truncated, _) = truncate_with_indicator(text, file_max, Some(100));
            Some(format!(" | {}: {}", label, truncated))
        } else {
            Some(format!(" | {}: {}", label, text))
        }
    } else {
        // No file limit
        Some(format!(" | {}: {}", label, text))
    };

    (console_text, file_text)
}

/// Calculates the average of the lowest prices, with a limit and threshold filter.
///
/// # Arguments
///
/// * `prices` - A list of prices sorted in ascending order by buyout price.
/// * `limit_to` - The maximum number of lowest prices to consider.
/// * `threshold_percentage` - A percentage filter relative to the minimum price.
///   Prices above `min_price * (1.0 + threshold_percentage)` are discarded.
///
/// # Returns
///
/// * The average of the filtered prices.
/// * Returns `-1` if no valid prices remain after filtering.
///
/// # Example
///
/// ```
/// let prices = vec![100, 102, 105, 110, 150, 200];
/// let avg = average_filtered_lowest_prices(prices, 5, 0.10);
/// assert_eq!(avg, 104); // (100 + 102 + 105 + 110) / 4
/// ```
pub fn average_filtered_lowest_prices(
    prices: Vec<i64>,          // List of ascending prices
    limit_to: i64,             // Limit of auctions to consider
    threshold_percentage: f64, // Percentage threshold for filtering
) -> i64 {
    if prices.is_empty() {
        return -1;
    }

    // Take only the lowest `limit_to` prices
    let mut top_price: Vec<i64> = prices.into_iter().take(limit_to as usize).collect();

    if top_price.is_empty() {
        return -1;
    }

    // The minimum price is always the first element (since input is sorted)
    let min_price = *top_price.first().unwrap_or(&0);

    // Threshold = min_price * (1 + threshold_percentage)
    let threshold = min_price as f64 * (1.0 + threshold_percentage);

    // Keep only prices within the threshold
    top_price.retain(|&price| price <= threshold as i64);

    if top_price.is_empty() {
        return -1;
    }

    // Return the average
    top_price.iter().sum::<i64>() / top_price.len() as i64
}

/*
    Validates a JSON object against a set of required properties.
    - If a required property is missing, it is added to the modified JSON.
    - Returns a tuple containing the modified JSON and a list of missing properties.
*/
pub fn validate_json(json: &Value, required: &Value, path: &str) -> (Value, Vec<String>) {
    let mut modified_json = json.clone();
    let mut missing_properties = Vec::new();

    if let Some(required_obj) = required.as_object() {
        // If json isn't an object, replace it with an empty one
        let mut json_obj = json.as_object().cloned().unwrap_or_default();

        for (key, value) in required_obj {
            let full_path = if path.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", path, key)
            };

            match json_obj.get(key) {
                None => {
                    // Key missing → add default
                    missing_properties.push(full_path.clone());
                    json_obj.insert(key.clone(), value.clone());
                }
                Some(existing_val) if value.is_object() => {
                    // Both sides are objects → recurse
                    let (modified_sub_json, sub_missing) =
                        validate_json(existing_val, value, &full_path);

                    if !sub_missing.is_empty() {
                        json_obj.insert(key.clone(), modified_sub_json);
                        missing_properties.extend(sub_missing);
                    }
                }
                Some(existing_val) => {
                    // Existing type mismatch with required (object vs non-object)
                    if value.is_object() && !existing_val.is_object() {
                        missing_properties.push(full_path.clone());
                        json_obj.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        modified_json = Value::Object(json_obj);
    }

    (modified_json, missing_properties)
}

/// Generate a UUID v5 from an object with optional prefix and suffix
///
/// This function creates a deterministic UUID based on the content of a JSON object.
/// The same object will always generate the same UUID, making it useful for
/// creating consistent identifiers.
///
/// # Arguments
/// * `prefix` - String to prepend to the object data
/// * `object` - Optional JSON Value object to generate UUID from
/// * `suffix` - String to append to the object data
///
/// # Returns
/// A tuple containing:
/// * The generated UUID as a string
/// * The key string used to generate the UUID
///
/// # Examples
/// ```
/// use serde_json::{json, Value};
/// use utils::generate_uuid_from_object;
///
/// let obj = Some(json!({"name": "John", "age": 30}));
/// let (uuid, key) = generate_uuid_from_object("user_", &obj, "_v1");
/// println!("UUID: {}, Key: {}", uuid, key);
/// ```
pub fn generate_uuid_from_object(
    prefix: impl Into<String>,
    object: &Option<Value>,
    suffix: impl Into<String>,
) -> (String, String) {
    let mut key = prefix.into();

    if let Some(obj) = object {
        let mut object_values: Vec<String> = Vec::new();
        // Sort the keys to ensure consistent ordering
        if let Some(map) = obj.as_object() {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for k in keys {
                if let Some(v) = map.get(k) {
                    // Remove quotes from string values
                    let value_str = match v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string().trim_matches('"').to_string(),
                    };
                    object_values.push(format!("{}:{}", k, value_str));
                }
            }
        }
        key.push_str(&object_values.join(";"));
    }
    key.push_str(&suffix.into());
    (
        Uuid::new_v5(&Uuid::NAMESPACE_OID, key.as_bytes()).to_string(),
        key,
    )
}

/// Trait for inserting text at a specific line and column position in a String
pub trait InsertAt {
    /// Insert text at the specified line and column position
    ///
    /// # Arguments
    /// * `line` - 1-based line number
    /// * `column` - 1-based column number
    /// * `text` - Text to insert
    ///
    /// # Examples
    /// ```
    /// use utils::InsertAt;
    ///
    /// let mut content = String::from("Hello\nWorld");
    /// content.insert_at(1, 6, " there");
    /// assert_eq!(content, "Hello there\nWorld");
    /// ```
    fn insert_at(&mut self, line: usize, column: usize, text: &str);
}

impl InsertAt for String {
    fn insert_at(&mut self, line: usize, column: usize, text: &str) {
        // Split into lines
        let mut lines: Vec<String> = self.lines().map(|s| s.to_string()).collect();

        // Convert to zero-based indices
        let line_idx = line.saturating_sub(1);
        let col_idx = column.saturating_sub(1);

        if let Some(target_line) = lines.get_mut(line_idx) {
            // Make sure we don't go out of bounds
            if col_idx <= target_line.chars().count() {
                // Handle UTF-8 safely using char indices
                let byte_pos = target_line
                    .char_indices()
                    .nth(col_idx)
                    .map(|(i, _)| i)
                    .unwrap_or_else(|| target_line.len());
                target_line.insert_str(byte_pos, text);
            } else {
                eprintln!("Column index out of range for line {}", line);
            }
        } else {
            eprintln!("Line {} does not exist", line);
        }

        *self = lines.join("\n");
    }
}

/// Merges two serde_json::Value objects recursively
///# Arguments
/// * `target` - The target JSON Value to merge into
/// * `source` - The source JSON Value to merge from
pub fn merge_json(target: &mut Value, source: &Value) {
    match (target, source) {
        (Value::Object(target_map), Value::Object(source_map)) => {
            for (key, source_value) in source_map {
                match target_map.get_mut(key) {
                    Some(target_value) => merge_json(target_value, source_value),
                    None => {
                        target_map.insert(key.clone(), source_value.clone());
                    }
                }
            }
        }
        (target_value, source_value) => {
            *target_value = source_value.clone();
        }
    }
}

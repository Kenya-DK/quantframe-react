use std::num::NonZeroU32;

use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use serde_json::{Map, Value, json};

/**
INTERNAL: Build the rate limiter for throttling outgoing requests to max allowed speeds
*/
pub(super) fn build_limiter(rps: NonZeroU32) -> RateLimiter<NotKeyed, InMemoryState, DefaultClock> {
    RateLimiter::direct(Quota::per_second(rps))
}

pub(super) fn mask_sensitive_data(data: &mut Map<String, Value>, properties: &[&str]) {
    // Iterate over each key-value pair in the JSON object
    for (key, value) in data.iter_mut() {
        // Perform actions based on the property key or type
        match value {
            Value::Object(sub_object) => {
                // If the value is another object, recursively loop through its properties
                mask_sensitive_data(sub_object, properties);
            }
            _ => {
                if properties.contains(&key.as_str()) {
                    *value = json!("***");
                }
            }
        }
    }
}

/**
 * INTERNAL: Write a JSON file to the specified path.
 * This function serializes the provided data into JSON format and writes it to a file.
 *
 * # Arguments
 * - `path`: The file path where the JSON data should be written.
 * - `data`: The data to be serialized and written to the file.
 *
 * # Returns
 * - `Ok(())` if the operation was successful.
 * - An `std::io::Error` if there was an issue creating or writing to the file.
 */
pub fn write_json_file<T: serde::Serialize>(path: &str, data: &T) -> std::io::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer(file, data)?;
    Ok(())
}

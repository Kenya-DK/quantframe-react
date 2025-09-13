use std::path::PathBuf;
use utils::*;

fn main() {
    println!("=== JSON Error Line/Column Demo ===\n");

    // Example 1: Valid JSON (no error)
    test_json_parsing("Valid JSON", r#"{"name": "John", "age": 30}"#);

    // Example 2: Missing closing brace
    test_json_parsing("Missing closing brace", r#"{"name": "John", "age": 30"#);

    // Example 3: Missing quote
    test_json_parsing("Missing quote", r#"{"name: "John", "age": 30}"#);

    // Example 4: Trailing comma
    test_json_parsing("Trailing comma", r#"{"name": "John", "age": 30,}"#);

    // Example 5: Invalid number
    test_json_parsing("Invalid number", r#"{"name": "John", "age": 30.5.2}"#);

    // Example 6: Multi-line JSON with error
    test_json_parsing(
        "Multi-line JSON error",
        r#"{
    "name": "John",
    "age": 30,
    "address": {
        "street": "123 Main St",
        "city": "Invalid syntax here: ][
    }
}"#,
    );

    // Example 7: Array with error
    test_json_parsing(
        "Array syntax error",
        r#"[1, 2, 3, 4, 5, "invalid syntax: }}]"#,
    );

    println!("=== Demo Complete ===");
}

fn test_json_parsing(test_name: &str, json_content: &str) {
    println!("{}:", test_name);
    println!(
        "  JSON content: {}",
        if json_content.len() > 50 {
            format!("{}...", &json_content[..50])
        } else {
            json_content.to_string()
        }
    );

    match serde_json::from_str::<serde_json::Value>(json_content) {
        Ok(_) => {
            println!("  ✅ JSON parsed successfully");
        }
        Err(err) => {
            // Create error using our enhanced from_json method
            let path = PathBuf::from("test.json");
            let error = Error::from_json(
                "JSONParser",
                &path,
                json_content,
                "Failed to parse JSON",
                err,
                "main.rs:test_json_parsing",
            );

            println!("  ❌ JSON parsing failed:");
            println!("    Message: {}", error.message);
            println!("    Cause: {}", error.cause);

            if let Some(context) = &error.context {
                if let Some(line) = context.get("line") {
                    println!("    Line: {}", line);
                }
                if let Some(column) = context.get("column") {
                    println!("    Column: {}", column);
                }
                if let Some(error_type) = context.get("error_type") {
                    println!("    Error Type: {}", error_type);
                }
            }

            // Show location info
            if let Some(location) = &error.location {
                println!("    Location: {}", location);
            }

            // Log the error to see the full formatted output
            println!("  📝 Full error log:");
            error.log("json_errors.log");
        }
    }
    println!();
}

fn demonstrate_error_context_analysis() {
    println!("=== Error Context Analysis ===");

    let problematic_json = r#"{
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"},
        {"id": 3, "name": "Carol", "invalid": }
    ],
    "metadata": {
        "total": 3,
        "page": 1
    }
}"#;

    println!("Analyzing JSON with syntax error:");
    match serde_json::from_str::<serde_json::Value>(problematic_json) {
        Err(err) => {
            let path = PathBuf::from("users.json");
            let error = Error::from_json(
                "UserService",
                &path,
                problematic_json,
                "Failed to parse user data",
                err,
                "user_service.rs:load_users:45",
            );

            println!("Error Analysis:");
            println!("  Component: {}", error.component);
            println!("  Message: {}", error.message);
            println!("  Cause: {}", error.cause);

            if let Some(context) = &error.context {
                println!("  Context Details:");
                for (key, value) in context.as_object().unwrap() {
                    match key.as_str() {
                        "line" | "column" => println!("    {}: {}", key, value),
                        "error_type" => println!("    {}: {}", key, value),
                        "path" => println!("    {}: {}", key, value),
                        "content" => println!(
                            "    {}: <JSON content {} chars>",
                            key,
                            value.as_str().map_or(0, |s| s.len())
                        ),
                        _ => println!("    {}: {}", key, value),
                    }
                }
            }

            // Show how to extract the problematic line
            if let Some(context) = &error.context {
                if let (Some(line_num), Some(content)) = (
                    context.get("line").and_then(|v| v.as_u64()),
                    context.get("content").and_then(|v| v.as_str()),
                ) {
                    println!("\n  Problematic line content:");
                    let lines: Vec<&str> = content.lines().collect();
                    if let Some(problematic_line) = lines.get((line_num - 1) as usize) {
                        println!("    Line {}: {}", line_num, problematic_line.trim());
                    }
                }
            }
        }
        Ok(_) => println!("JSON parsed successfully (unexpected)"),
    }
}

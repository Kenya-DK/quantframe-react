use serde_json::json;
use utils::*;

fn main() -> Result<(), Error> {
    println!("=== JSON Formatting Demo ===\n");

    let sample_data = json!({
        "level": "INFO",
        "component": "FormatDemo",
        "message": "Testing JSON formatting options",
        "timestamp": chrono::Local::now().to_rfc3339(),
        "metadata": {
            "version": "1.0.0",
            "environment": "development",
            "features": ["logging", "json", "formatting"]
        },
        "performance": {
            "cpu_usage": 45.2,
            "memory_mb": 256,
            "response_time_ms": 12.5
        }
    });

    // Pretty formatted JSON (default behavior)
    println!("📝 Writing pretty formatted JSON to 'formatted_pretty.json'");
    log_json(sample_data.clone(), "formatted_pretty.json")?;

    // Compact JSON
    println!("📝 Writing compact JSON to 'formatted_compact.json'");
    log_json_formatted(sample_data, "formatted_compact.json", false)?;

    println!("\n✅ Both files created!");
    println!("Compare the formatting by opening both files:");
    println!("- logs/formatted_pretty.json (readable with indentation)");
    println!("- logs/formatted_compact.json (compact single line)");

    Ok(())
}

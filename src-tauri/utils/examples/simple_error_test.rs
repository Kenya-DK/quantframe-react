use serde_json::json;
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("=== Simple Error Context Test ===\n");

    // Test with small context (should display in full)
    println!("1. Testing small context:");
    let small_error = Error::new("TestComponent", "Small context test", get_location!())
        .with_context(json!({"key": "value", "number": 42}));
    small_error.log(Some("test_errors.log"));

    // Test with large context (should be truncated for console)
    println!("\n2. Testing large context:");
    let large_text = "x".repeat(3000); // Create a 3000 character string
    let large_error = Error::new("TestComponent", "Large context test", get_location!())
        .with_context(json!({"large_data": large_text, "metadata": "some info"}));
    large_error.log(Some("test_errors.log"));

    // Test without context
    println!("\n3. Testing without context:");
    let no_context_error = Error::new("TestComponent", "No context test", get_location!())
        .with_cause("Just a simple error");
    no_context_error.log(Some("test_errors.log"));

    println!("\nDone! Check logs/[date]/test_errors.log for full context data.");

    Ok(())
}

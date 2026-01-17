use utils::helper::*;

fn main() {
    println!("=== Text Truncation Helper Examples ===\n");

    // Example 1: Basic truncation
    example_basic_truncation();

    // Example 2: Smart dual-output processing
    example_smart_processing();

    // Example 3: Practical usage scenarios
    example_practical_usage();

    // Example 4: Error integration
    demonstrate_error_integration();

    println!("=== Examples Complete ===");
}

fn example_basic_truncation() {
    println!("1. Basic Text Truncation:");

    // Short text - no truncation needed
    let short_text = "This is a short message";
    let (result, was_truncated) = truncate_with_indicator(&short_text, 100, None);
    println!("  Short text: '{}' (truncated: {})", result, was_truncated);

    // Long text - will be truncated
    let long_text = "This is a very long message that exceeds the maximum length limit and should be truncated with an indicator showing the total character count for debugging purposes.".repeat(10);
    let (result, was_truncated) = truncate_with_indicator(&long_text, 200, Some(60));
    println!("  Long text: '{}' (truncated: {})", result, was_truncated);

    // Custom truncation buffer
    let medium_text = "x".repeat(500);
    let (result, was_truncated) = truncate_with_indicator(&medium_text, 100, Some(30));
    println!("  Medium text: '{}' (truncated: {})", result, was_truncated);

    println!();
}

fn example_smart_processing() {
    println!("2. Smart Dual-Output Processing:");

    // Test with various text sizes
    let medium_text = "x".repeat(1000);
    let large_text = "y".repeat(3000);
    let huge_text = "z".repeat(10000);

    let test_cases = vec![
        ("Small", "Small context data"),
        ("Medium", medium_text.as_str()),
        ("Large", large_text.as_str()),
        ("Huge", huge_text.as_str()),
    ];

    for (name, text) in test_cases {
        println!("  {} text ({} chars):", name, text.len());

        let (console_text, file_text) = smart_text_processing(
            &text,
            2048,       // Console limit
            Some(5000), // File limit
            "Context",
        );

        if let Some(console) = console_text {
            let display_console = if console.len() > 100 {
                format!("{}...", &console[..100])
            } else {
                console
            };
            println!("    Console: {}", display_console);
        }

        if let Some(file) = file_text {
            let display_file = if file.len() > 100 {
                format!("{}...", &file[..100])
            } else {
                file
            };
            println!("    File: {}", display_file);
        }
        println!();
    }
}

fn example_practical_usage() {
    println!("3. Practical Usage Scenarios:");

    // Scenario 1: Error context processing
    println!("  Scenario 1: Error Context Processing");
    let error_context = format!(
        "{{\"error_details\": \"{}\", \"stack_trace\": \"{}\", \"request_data\": \"{}\"}}",
        "Database connection failed".repeat(50),
        "line 1\nline 2\nline 3".repeat(100),
        "user_data".repeat(200)
    );

    let (console_ctx, file_ctx) = smart_text_processing(
        &error_context,
        2048, // Console limit
        None, // No file limit
        "ErrorContext",
    );

    println!("    Error context length: {}", error_context.len());
    if let Some(console) = console_ctx {
        println!(
            "    Console output: {}...",
            &console[..std::cmp::min(80, console.len())]
        );
    }
    if let Some(file) = file_ctx {
        println!("    File output length: {}", file.len());
    }

    // Scenario 2: Log message processing
    println!("\n  Scenario 2: Log Message Processing");
    let large_payload = format!("Large payload: {}", "data".repeat(1000));
    let debug_info = format!("Massive debug info: {}", "debug".repeat(2000));

    let log_messages = vec![
        "User login successful",
        large_payload.as_str(),
        debug_info.as_str(),
    ];

    for (i, message) in log_messages.iter().enumerate() {
        println!("    Message {}: {} chars", i + 1, message.len());

        let (console_msg, file_msg) = smart_text_processing(
            message,
            500,        // Console limit
            Some(2000), // File limit
            "LogData",
        );

        match (console_msg, file_msg) {
            (Some(console), Some(file)) => {
                let console_preview = if console.len() > 60 {
                    format!("{}...", &console[..60])
                } else {
                    console
                };
                println!("      Console: {}", console_preview);
                println!("      File: {} chars", file.len());
            }
            _ => println!("      No output generated"),
        }
    }

    // Scenario 3: Stack trace processing
    println!("\n  Scenario 3: Stack Trace Processing");
    let stack_trace = (0..50)
        .map(|i| format!("  at function_{}() in file_{}.rs:line_{}", i, i % 5, i * 10))
        .collect::<Vec<_>>()
        .join("\n");

    let (console_stack, file_stack) = smart_text_processing(
        &stack_trace,
        1024,       // Console limit for stack traces
        Some(4096), // File limit for stack traces
        "StackTrace",
    );

    println!("    Stack trace length: {}", stack_trace.len());
    if let Some(console) = console_stack {
        println!(
            "    Console preview: {}...",
            &console[..std::cmp::min(100, console.len())]
        );
    }
    if let Some(file) = file_stack {
        println!("    File output length: {}", file.len());
    }

    println!();
}

// Helper function to demonstrate integration with existing error handling
fn demonstrate_error_integration() {
    println!("4. Integration with Error Handling:");

    // Simulate large context data
    let large_context = format!(
        "{{\"request_id\": \"{}\", \"user_data\": \"{}\", \"debug_info\": \"{}\"}}",
        "req_123456789".repeat(10),
        "user_info".repeat(100),
        "debug_data".repeat(200)
    );

    // Process for dual output
    let (console_context, file_context) = smart_text_processing(
        &large_context,
        2048, // MAX_CONTEXT_LENGTH for console
        None, // No limit for file
        "Context",
    );

    // Simulate the error logging pattern
    let base_message = "Database operation failed";

    if let Some(console_msg) = console_context {
        let console_full_message = format!("{}{}", base_message, console_msg);
        println!(
            "  Console message: {}...",
            &console_full_message[..std::cmp::min(100, console_full_message.len())]
        );
    }

    if let Some(file_msg) = file_context {
        let file_full_message = format!("{}{}", base_message, file_msg);
        println!("  File message length: {}", file_full_message.len());
    }
}

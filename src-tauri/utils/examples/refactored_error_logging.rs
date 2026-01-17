use serde_json::json;
use utils::helper::*;
use utils::*;

fn main() {
    println!("=== Refactored Error Logging Example ===\n");

    // Example 1: Simple error with small context
    example_simple_error();

    // Example 2: Error with large context
    example_large_context_error();

    // Example 3: Error with both large context and stack trace
    example_complex_error();

    println!("=== Refactoring Complete ===");
}

fn example_simple_error() {
    println!("1. Simple Error with Small Context:");

    let error = Error::new("Database", "Connection failed", get_location!())
        .with_cause("Network timeout")
        .with_context(json!({
            "host": "localhost",
            "port": 5432,
            "timeout_ms": 5000
        }));

    // Show how the helper could be used
    if let Some(context) = &error.context {
        let context_str = context.to_string();
        let (console_text, file_text) = smart_text_processing(
            &context_str,
            2048, // MAX_CONTEXT_LENGTH
            None, // No file limit
            "Context",
        );

        println!("  Context length: {}", context_str.len());
        if let Some(console) = console_text {
            println!("  Console context: {}", console);
        }
        if let Some(file) = file_text {
            println!("  File context: {}", file);
        }
    }

    println!();
}

fn example_large_context_error() {
    println!("2. Error with Large Context:");

    // Create a large context that exceeds the limit
    let large_context = json!({
        "request_data": "x".repeat(3000),
        "user_info": {
            "id": 12345,
            "session": "sess_".to_string() + &"y".repeat(1000),
            "permissions": vec!["read", "write", "admin"]
        },
        "debug_info": {
            "trace_id": "trace_".to_string() + &"z".repeat(2000),
            "performance_data": "perf_".to_string() + &"a".repeat(1500)
        }
    });

    let error = Error::new("API", "Request processing failed", get_location!())
        .with_cause("Validation error")
        .with_context(large_context);

    // Demonstrate the helper usage
    if let Some(context) = &error.context {
        let context_str = context.to_string();
        let (console_text, file_text) = smart_text_processing(
            &context_str,
            2048, // MAX_CONTEXT_LENGTH
            None, // No file limit for files
            "Context",
        );

        println!("  Original context length: {}", context_str.len());

        let console_len = console_text.as_ref().map_or(0, |c| c.len());
        let file_len = file_text.as_ref().map_or(0, |f| f.len());

        if let Some(console) = console_text {
            println!("  Console output length: {}", console.len());
            println!(
                "  Console preview: {}...",
                &console[..std::cmp::min(100, console.len())]
            );
        }

        if let Some(file) = file_text {
            println!("  File output length: {}", file.len());
            println!("  File preserved full context: {}", file_len > console_len);
        }
    }

    println!();
}

fn example_complex_error() {
    println!("3. Complex Error with Context and Stack Trace:");

    // Create error with both large context and stack trace
    let complex_context = json!({
        "sql_query": "SELECT * FROM users WHERE active = true AND created_at > '2025-01-01' AND role IN ('admin', 'moderator', 'user') AND last_login > NOW() - INTERVAL '30 days'".repeat(50),
        "parameters": {
            "limit": 1000,
            "offset": 0,
            "filters": "filter_data".repeat(500)
        },
        "execution_plan": "plan_".repeat(1000),
        "performance_metrics": {
            "query_time_ms": 5000,
            "rows_scanned": 1000000,
            "index_usage": "index_info".repeat(300)
        }
    });

    let error = Error::new("Database", "Query execution failed", get_location!())
        .with_cause("Query timeout after 30 seconds")
        .with_context(complex_context);

    // Process context
    if let Some(context) = &error.context {
        let context_str = context.to_string();
        let (console_context, file_context) = smart_text_processing(
            &context_str,
            2048, // Console limit
            None, // No file limit
            "Context",
        );

        println!("  Context processing:");
        println!("    Original length: {} chars", context_str.len());

        if let Some(console) = console_context {
            println!("    Console length: {} chars", console.len());
        }

        if let Some(file) = file_context {
            println!("    File length: {} chars", file.len());
        }
    }

    // Process stack trace (simulate)
    let simulated_stack = (0..30)
        .map(|i| format!("  at function_{}() in database.rs:line_{}", i, i * 15))
        .collect::<Vec<_>>()
        .join("\n");

    let (console_stack, file_stack) = smart_text_processing(
        &simulated_stack,
        1024, // Stack trace console limit
        None, // No file limit
        "Stack",
    );

    println!("  Stack trace processing:");
    println!("    Original length: {} chars", simulated_stack.len());

    if let Some(console) = console_stack {
        println!("    Console length: {} chars", console.len());
    }

    if let Some(file) = file_stack {
        println!("    File length: {} chars", file.len());
    }

    // Show how this would work in practice
    println!("\n  Practical usage:");
    demonstrate_dual_logging(&error, &simulated_stack);

    println!();
}

fn demonstrate_dual_logging(error: &Error, stack_trace: &str) {
    let mut base_message = format!("{}", error.message);

    if !error.cause.is_empty() {
        base_message.push_str(&format!(" | Cause: {}", error.cause));
    }

    // Process stack trace
    let (console_stack, file_stack) = smart_text_processing(
        stack_trace,
        1024, // MAX_STACK_LENGTH
        None, // No file limit
        "Stack",
    );

    // Process context
    let (console_context, file_context) = if let Some(context) = &error.context {
        let context_str = context.to_string();
        smart_text_processing(
            &context_str,
            2048, // MAX_CONTEXT_LENGTH
            None, // No file limit
            "Context",
        )
    } else {
        (None, None)
    };

    // Build console message
    let mut console_message = base_message.clone();
    if let Some(stack) = console_stack {
        console_message.push_str(&stack);
    }
    if let Some(context) = console_context {
        console_message.push_str(&context);
    }

    // Build file message
    let mut file_message = base_message;
    if let Some(stack) = file_stack {
        file_message.push_str(&stack);
    }
    if let Some(context) = file_context {
        file_message.push_str(&context);
    }

    println!("    Console message length: {}", console_message.len());
    println!("    File message length: {}", file_message.len());
    println!(
        "    Space saved for console: {} chars",
        file_message.len() - console_message.len()
    );
}

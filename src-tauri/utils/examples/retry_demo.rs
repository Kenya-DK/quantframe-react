use std::sync::atomic::{AtomicU32, Ordering};
use utils::{BackoffStrategy, RetryConfig, retry, retry_with_config};

fn main() {
    let counter = AtomicU32::new(0);

    let result = retry(|| {
        let attempt = counter.fetch_add(1, Ordering::SeqCst) + 1;
        println!("  -> Attempt {}", attempt);

        if attempt < 3 {
            Err(format!("Attempt {} failed", attempt))
        } else {
            Ok("Success on attempt 3".to_string())
        }
    });

    match result {
        Ok(msg) => println!("Result: {}\n", msg),
        Err(e) => println!("Failed: {}\n", e),
    }

    println!("--- Exponential backoff example ---");
    let counter2 = AtomicU32::new(0);

    let config = RetryConfig::new(5, 200, BackoffStrategy::Exponential);
    let result = retry_with_config(
        || {
            let attempt = counter2.fetch_add(1, Ordering::SeqCst) + 1;
            println!("  -> Attempt {}", attempt);

            if attempt < 5 {
                Err(format!("Attempt {} failed", attempt))
            } else {
                Ok("Success on attempt 5".to_string())
            }
        },
        &config,
    );

    match result {
        Ok(msg) => println!("Result: {}\n", msg),
        Err(e) => println!("Failed: {}\n", e),
    }

    println!("--- All attempts fail ---");
    let counter3 = AtomicU32::new(0);

    let result: Result<String, String> = retry_with_config(
        || {
            let attempt = counter3.fetch_add(1, Ordering::SeqCst) + 1;
            println!("  -> Attempt {}", attempt);
            return Err(format!("Permanent failure on attempt {}", attempt));
        },
        &RetryConfig::new(2, 100, BackoffStrategy::Fixed),
    );

    match result {
        Ok(msg) => println!("Result: {}", msg),
        Err(e) => println!("Failed after all retries: {}", e),
    }
}

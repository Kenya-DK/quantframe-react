use serde_json::json;
use utils::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sensitive Data Masking Demo ===");

    // Initialize logger
    init_logger();

    // Demo 1: Basic sensitive data masking
    println!("\n=== Basic Sensitive Data Masking ===");
    let mut error = Error::new("Authentication", "Login attempt failed", get_location!())
        .with_context(json!({
            "user_id": 12345,
            "email": "john.doe@example.com",
            "password": "mySecretPassword123",
            "ip_address": "192.168.1.100",
            "session_token": "abc123def456ghi789",
            "timestamp": "2025-07-26T16:30:00Z"
        }));

    println!("Before masking:");
    println!("{}", serde_json::to_string_pretty(&error)?);

    // Mask sensitive fields
    error.mask_sensitive_data(&["password", "session_token"]);

    println!("\nAfter masking password and session_token:");
    println!("{}", serde_json::to_string_pretty(&error)?);

    // Demo 2: Payment processing example
    println!("\n=== Payment Processing Example ===");
    let mut payment_error = Error::new("Payment", "Credit card processing failed", get_location!())
        .set_log_level(LogLevel::Error)
        .with_context(json!({
            "transaction_id": "txn_123456789",
            "amount": 99.99,
            "currency": "USD",
            "credit_card_number": "4532-1234-5678-9012",
            "cvv": "123",
            "cardholder_name": "John Doe",
            "billing_address": {
                "street": "123 Main St",
                "city": "Anytown",
                "zip": "12345"
            },
            "merchant_id": "merchant_abc123"
        }));

    println!("Payment error before masking:");
    println!("{}", serde_json::to_string_pretty(&payment_error)?);

    // Mask financial sensitive data
    payment_error.mask_sensitive_data(&["credit_card_number", "cvv"]);

    println!("\nPayment error after masking:");
    println!("{}", serde_json::to_string_pretty(&payment_error)?);

    // Demo 3: API request with multiple sensitive fields
    println!("\n=== API Request Example ===");
    let mut api_error = Error::new("API", "External service call failed", get_location!())
        .set_log_level(LogLevel::Warning)
        .with_context(json!({
            "endpoint": "https://api.example.com/users",
            "method": "POST",
            "api_key": "sk_live_abcdef123456789",
            "request_body": {
                "username": "johndoe",
                "email": "john@example.com",
                "password": "newPassword456",
                "phone": "+1-555-123-4567",
                "ssn": "123-45-6789"
            },
            "response_code": 400,
            "error_message": "Validation failed"
        }));

    println!("API error before masking:");
    println!("{}", serde_json::to_string_pretty(&api_error)?);

    // Mask API keys and sensitive user data
    api_error.mask_sensitive_data(&["api_key", "password", "ssn"]);

    println!("\nAPI error after masking:");
    println!("{}", serde_json::to_string_pretty(&api_error)?);

    // Demo 4: Database connection example
    println!("\n=== Database Connection Example ===");
    let mut db_error = Error::new("Database", "Connection failed", get_location!())
        .set_log_level(LogLevel::Critical)
        .with_context(json!({
            "host": "db.example.com",
            "port": 5432,
            "database": "production_db",
            "username": "db_user",
            "password": "db_secret_password",
            "connection_string": "postgresql://db_user:db_secret_password@db.example.com:5432/production_db",
            "ssl_enabled": true,
            "timeout_ms": 5000
        }));

    println!("Database error before masking:");
    println!("{}", serde_json::to_string_pretty(&db_error)?);

    // Mask database credentials
    db_error.mask_sensitive_data(&["password", "connection_string"]);

    println!("\nDatabase error after masking:");
    println!("{}", serde_json::to_string_pretty(&db_error)?);

    // Demo 5: Log the masked errors
    println!("\n=== Logging Masked Errors ===");
    error.log("masked_errors.log".to_string());
    payment_error.log("masked_errors.log".to_string());
    api_error.log("masked_errors.log".to_string());
    db_error.log("masked_errors.log".to_string());

    println!("All masked errors have been logged to 'masked_errors.log'");

    // Demo 6: Chaining with other methods
    println!("\n=== Chaining with Other Methods ===");
    let mut chained_error = Error::new("Security", "Unauthorized access attempt", get_location!())
        .set_log_level(LogLevel::Critical)
        .with_cause("Invalid credentials provided")
        .with_context(json!({
            "user_agent": "Mozilla/5.0...",
            "ip_address": "203.0.113.42",
            "attempted_username": "admin",
            "attempted_password": "password123",
            "api_token": "bearer_token_xyz789",
            "session_id": "sess_abcdef123456"
        }));

    // Chain masking with other operations
    chained_error.mask_sensitive_data(&["attempted_password", "api_token", "session_id"]);
    chained_error.log("security.log".to_string());

    println!("Chained error with masking logged to 'security.log'");

    println!("\n=== Demo Completed ===");
    println!("Sensitive data masking provides:");
    println!("✅ Automatic masking of specified sensitive fields");
    println!("✅ Preserves non-sensitive context information");
    println!("✅ Works with nested JSON objects");
    println!("✅ Integrates seamlessly with logging");
    println!("✅ Helps comply with data privacy regulations");
    println!("✅ Prevents accidental exposure of secrets in logs");

    Ok(())
}

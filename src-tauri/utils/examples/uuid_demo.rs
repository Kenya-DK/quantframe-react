use serde_json::{Value, json};
use utils::generate_uuid_from_object;

fn main() {
    println!("=== UUID Generation Demo ===\n");

    // Example 1: Simple object
    let obj1 = Some(json!({
        "test": "test"
    }));

    let (uuid1, key1) = generate_uuid_from_object("wfm_id:asd;", &obj1, "");
    println!("Example 1 - Simple object:");
    println!("Object: {}", obj1.as_ref().unwrap());
    println!("Key: {}", key1);
    println!("UUID: {}", uuid1);
    println!();

    // Example 2: Same object should generate same UUID
    let obj2 = Some(json!({
        "age": 30,
        "name": "John Doe",  // Different order
        "city": "New York"
    }));

    let (uuid2, key2) = generate_uuid_from_object("user_", &obj2, "_v1");
    println!("Example 2 - Same object, different key order:");
    println!("Object: {}", obj2.as_ref().unwrap());
    println!("Key: {}", key2);
    println!("UUID: {}", uuid2);
    println!("Same UUID as example 1? {}", uuid1 == uuid2);
    println!();

    // Example 3: Empty object
    let obj3 = Some(json!({}));
    let (uuid3, key3) = generate_uuid_from_object("empty_", &obj3, "_test");
    println!("Example 3 - Empty object:");
    println!("Object: {}", obj3.as_ref().unwrap());
    println!("Key: {}", key3);
    println!("UUID: {}", uuid3);
    println!();

    // Example 4: None object
    let obj4: Option<Value> = None;
    let (uuid4, key4) = generate_uuid_from_object("none_", &obj4, "_end");
    println!("Example 4 - None object:");
    println!("Key: {}", key4);
    println!("UUID: {}", uuid4);
    println!();

    // Example 5: Complex nested object
    let obj5 = Some(json!({
        "user": {
            "profile": {
                "name": "Alice",
                "preferences": ["dark_mode", "notifications"]
            }
        },
        "timestamp": "2025-10-17T00:00:00Z",
        "version": 1.2
    }));

    let (uuid5, key5) = generate_uuid_from_object("complex_", &obj5, "_final");
    println!("Example 5 - Complex object:");
    println!("Object: {}", obj5.as_ref().unwrap());
    println!("Key: {}", key5);
    println!("UUID: {}", uuid5);

    println!("\n=== Demo completed ===");
}

use std::time::Instant;
use utils::multi_key_map::MultiKeyMap;

#[derive(Debug, Clone)]
struct UserData {
    name: String,
    age: u32,
}

fn main() {
    println!("=== MultiKeyMap Performance Demo ===\n");

    // Create a new MultiKeyMap
    let mut map = MultiKeyMap::new();

    // Example 1: Basic usage
    println!("Example 1: Basic Usage");
    println!("----------------------");

    let user1 = UserData {
        name: "Alice Smith".to_string(),
        age: 30,
    };

    // Insert a value with multiple keys
    let id1 = map.insert_value(user1.clone(), vec!["alice", "alice_smith", "user_001"]);
    println!("Inserted user with ID: {}", id1);

    let user2 = UserData {
        name: "Bob Johnson".to_string(),
        age: 25,
    };

    let id2 = map.insert_value(user2.clone(), vec!["bob", "bobby", "user_002"]);
    println!("Inserted user with ID: {}", id2);

    // Retrieve by different keys
    println!("\nRetrieving by different keys:");
    if let Some(user) = map.get("alice") {
        println!("  Found by 'alice': {} (age {})", user.name, user.age);
    }
    if let Some(user) = map.get("alice_smith") {
        println!("  Found by 'alice_smith': {} (age {})", user.name, user.age);
    }
    if let Some(user) = map.get("user_001") {
        println!("  Found by 'user_001': {} (age {})", user.name, user.age);
    }
    if let Some(user) = map.get("bobby") {
        println!("  Found by 'bobby': {} (age {})", user.name, user.age);
    }

    // Example 2: Adding more keys later
    println!("\n\nExample 2: Adding Additional Keys");
    println!("----------------------------------");
    map.add_keys(id1, vec!["alice.smith", "a.smith"]);
    println!("Added new keys for Alice");

    if let Some(user) = map.get("alice.smith") {
        println!("  Found by 'alice.smith': {}", user.name);
    }
    if let Some(user) = map.get("a.smith") {
        println!("  Found by 'a.smith': {}", user.name);
    }

    // Example 3: Mutable access
    println!("\n\nExample 3: Mutable Access");
    println!("-------------------------");
    if let Some(user) = map.get_mut("bob") {
        println!("  Before: {} is {} years old", user.name, user.age);
        user.age = 26;
        println!("  After: {} is {} years old", user.name, user.age);
    }

    // Example 4: Performance test with large dataset
    println!("\n\nExample 4: Performance Test");
    println!("---------------------------");

    let mut perf_map = MultiKeyMap::new();
    let num_entries = 1_000_000;

    // Measure insertion time
    let start = Instant::now();
    for i in 0..num_entries {
        let user = UserData {
            name: format!("User {}", i),
            age: 20 + (i % 50) as u32,
        };

        // Each user has 3 different keys
        perf_map.insert_value(
            user,
            vec![
                format!("user_{}", i),
                format!("id_{}", i),
                format!("email_{}", i),
            ],
        );
    }
    let insert_duration = start.elapsed();
    println!("  Inserted {} entries with 3 keys each", num_entries);
    println!("  Time: {:?}", insert_duration);
    println!("  Avg per entry: {:?}", insert_duration / num_entries);

    // Measure lookup time
    let start = Instant::now();
    let mut found_count = 0;
    for i in 0..num_entries {
        if perf_map.get(&format!("user_{}", i)).is_some() {
            found_count += 1;
        }
    }
    let lookup_duration = start.elapsed();
    println!("\n  Performed {} lookups", num_entries);
    println!("  Found: {}", found_count);
    println!("  Time: {:?}", lookup_duration);
    println!("  Avg per lookup: {:?}", lookup_duration / num_entries);

    // Measure lookup by different key types
    let start = Instant::now();
    for i in 0..num_entries {
        perf_map.get(&format!("id_{}", i));
    }
    let lookup2_duration = start.elapsed();
    println!("\n  Lookups by alternate keys: {:?}", lookup2_duration);

    // Test clear and size
    println!("\n\nExample 5: Clear and Size");
    println!("-------------------------");
    println!("  Map size before clear: {}", perf_map.len());
    perf_map.clear();
    println!("  Map size after clear: {}", perf_map.len());

    // Example 6: Real-world scenario - Session management
    println!("\n\nExample 6: Real-World Scenario - Session Management");
    println!("---------------------------------------------------");

    #[derive(Debug, Clone)]
    struct Session {
        user_id: u64,
        token: String,
    }

    let mut session_map = MultiKeyMap::new();

    // Insert sessions with multiple identifiers
    let session1 = Session {
        user_id: 1001,
        token: "abc123def456".to_string(),
    };

    session_map.insert_value(
        session1,
        vec![
            "token:abc123def456",  // Lookup by token
            "user:1001",           // Lookup by user ID
            "session:session_001", // Lookup by session ID
        ],
    );

    let session2 = Session {
        user_id: 1002,
        token: "xyz789ghi012".to_string(),
    };

    session_map.insert_value(
        session2,
        vec!["token:xyz789ghi012", "user:1002", "session:session_002"],
    );

    println!("  Sessions created: {}", session_map.len());

    // Look up session by token
    if let Some(session) = session_map.get("token:abc123def456") {
        println!("  Found session by token: user_id={}", session.user_id);
    }

    // Look up session by user ID
    if let Some(session) = session_map.get("user:1001") {
        println!("  Found session by user_id: token={}", session.token);
    }

    // Look up session by session ID
    if let Some(session) = session_map.get("session:session_002") {
        println!("  Found session by session_id: user_id={}", session.user_id);
    }

    println!("\n=== Demo Complete ===");
}

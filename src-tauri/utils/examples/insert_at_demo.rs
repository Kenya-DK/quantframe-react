use utils::InsertAt;

fn main() {
    println!("=== InsertAt Trait Demo ===\n");

    // Example 1: Simple single-line insertion
    let mut text1 = String::from("Hello World!");
    println!("Original: '{}'", text1);
    text1.insert_at(1, 7, "Beautiful ");
    println!(
        "After inserting 'Beautiful ' at line 1, column 7: '{}'",
        text1
    );

    println!();

    // Example 2: Multi-line text insertion
    let mut text2 = String::from("Line 1: Start\nLine 2: Middle\nLine 3: End");
    println!("Original multi-line text:");
    println!("{}", text2);

    text2.insert_at(2, 9, "Important ");
    println!("\nAfter inserting 'Important ' at line 2, column 9:");
    println!("{}", text2);

    println!();

    // Example 3: Insert at the beginning of a line
    let mut text3 = String::from("First line\nSecond line\nThird line");
    println!("Original:");
    println!("{}", text3);

    text3.insert_at(2, 1, ">>> ");
    println!("\nAfter inserting '>>> ' at line 2, column 1:");
    println!("{}", text3);

    println!();

    // Example 4: Insert at the end of a line
    let mut text4 = String::from("Task 1\nTask 2\nTask 3");
    println!("Original:");
    println!("{}", text4);

    text4.insert_at(1, 7, " - Completed");
    text4.insert_at(3, 7, " - Pending");
    println!("\nAfter adding status to tasks:");
    println!("{}", text4);

    println!();

    // Example 5: Error handling - out of bounds
    let mut text5 = String::from("Short");
    println!("Original: '{}'", text5);
    println!("Attempting to insert at line 5 (non-existent):");
    text5.insert_at(5, 1, "This won't work");
    println!("Result: '{}'", text5);

    println!("\nAttempting to insert at column 50 (out of bounds):");
    text5.insert_at(1, 50, "This won't work either");
    println!("Result: '{}'", text5);

    println!("\n=== Demo completed ===");
}

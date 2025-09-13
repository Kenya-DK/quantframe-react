use std::thread;

use std::time::Duration;
use utils::*;

#[derive(Clone, Debug)]
pub struct OnConversationEvent {}

impl OnConversationEvent {
    pub fn new() -> Self {
        OnConversationEvent {}
    }
}

impl LineHandler for OnConversationEvent {
    fn process_line(
        &mut self,
        line: &str,
        prev_line: &str,
        ignore_combined: bool,
    ) -> Result<(bool, bool), Error> {
        Ok((false, false)) // no match → process normally
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("Starting FileWatcher...");

    let watcher = FileWatcher::new(
        "C:/Users/Kenya/Desktop/Andet/Coding/Warframe/warframe-data/_cache/WFLogSimulation/EE.log",
    );
    // Add multiple dynamic handlers
    watcher.add_handler(Box::new(OnConversationEvent::new()));
    println!(
        "Watching file: C:/Users/Kenya/Desktop/Andet/Coding/Warframe/warframe-data/_cache/WFLogSimulation/TradeInfo/EE.log"
    );
    println!("FileWatcher will print any new lines added to the file...");
    thread::spawn(move || {
        println!("File watcher thread started!");
        match watcher.watch() {
            Ok(_) => println!("File watcher finished successfully"),
            Err(e) => println!("File watcher error: {}", e),
        }
    });
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

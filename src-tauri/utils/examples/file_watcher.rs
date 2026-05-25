use std::thread;

use std::time::Duration;
use utils::*;

pub struct OnTradeEvent {}

impl LineHandler for OnTradeEvent {
    fn process_line(&mut self, _entry: &LineEntry) -> Result<(bool, bool), Error> {
        let com = _entry.line.contains("TradeEvent");
        println!("Trade Entry: {} | Contains 'TradeEvent': {}", _entry, com);
        Ok((com, com)) // no match → process normally
    }
}
pub struct OnConversationEvent {}

impl LineHandler for OnConversationEvent {
    fn process_line(&mut self, _entry: &LineEntry) -> Result<(bool, bool), Error> {
        let com = _entry.line.contains("ConversationEvent");
        println!(
            "Conversation Entry: {} | Contains 'ConversationEvent': {}",
            _entry, com
        );
        Ok((com, com)) // no match → process normally
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    init_logger();

    println!("Starting FileWatcher...");
    let paths = vec![
        "C:/Users/Kenya/AppData/Local/Warframe/EE.log",
        "C:/Users/Kenya/Desktop/Andet/Coding/Warframe/warframe-data/_cache/WFLogSimulation/EE.log",
    ];
    let index = 1;
    // C:\Users\Kenya\Desktop\Andet\Coding\Warframe\warframe-data\_cache\WFLogSimulation\EE.log
    // C:\Users\Kenya\AppData\Local\Warframe\EE.log
    let watcher = FileWatcher::new(paths[index]);
    // Add multiple dynamic handlers
    watcher.add_handler(Box::new(OnTradeEvent {}));
    watcher.add_handler(Box::new(OnConversationEvent {}));
    println!("Watching file: {}", paths[index]);
    println!("FileWatcher will print any new lines added to the file...");
    match watcher.watch() {
        Ok(_) => println!("File watcher finished successfully"),
        Err(e) => println!("File watcher error: {}", e),
    }
    // loop {
    //     watcher.set_path(paths[1]);
    // }
    thread::sleep(Duration::from_secs(6000));
    Ok(())
}

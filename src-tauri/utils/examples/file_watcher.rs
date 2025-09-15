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
    let paths = vec![
        "C:/Users/Kenya/AppData/Local/Warframe/EE.log",
        "C:/Users/Kenya/Desktop/Andet/Coding/Warframe/warframe-data/_cache/WFLogSimulation/EE.log",
    ];
    let index = 0;
    // C:\Users\Kenya\Desktop\Andet\Coding\Warframe\warframe-data\_cache\WFLogSimulation\EE.log
    // C:\Users\Kenya\AppData\Local\Warframe\EE.log
    let mut watcher = FileWatcher::new(paths[index]);
    // Add multiple dynamic handlers
    watcher.add_handler(Box::new(OnConversationEvent::new()));
    println!("Watching file: {}", paths[index]);
    println!("FileWatcher will print any new lines added to the file...");
    match watcher.watch() {
        Ok(_) => println!("File watcher finished successfully"),
        Err(e) => println!("File watcher error: {}", e),
    }
    loop {
        thread::sleep(Duration::from_secs(60));
        watcher.set_path(paths[1]);
    }
}

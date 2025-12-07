use chrono::{DateTime, Duration, Utc};

#[derive(Debug)]
pub struct StopWatch {
    start_time: Option<DateTime<Utc>>,
    accumulated: Duration,
    paused: bool,
}

impl StopWatch {
    pub fn new() -> Self {
        Self {
            start_time: None,
            accumulated: Duration::zero(),
            paused: true,
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Utc::now());
            self.paused = false;
        }
    }

    pub fn pause(&mut self) {
        if !self.paused {
            if let Some(start) = self.start_time {
                self.accumulated = self.accumulated + (Utc::now() - start);
            }
            self.start_time = None;
            self.paused = true;
        }
    }

    pub fn resume(&mut self) {
        if self.paused {
            self.start_time = Some(Utc::now());
            self.paused = false;
        }
    }

    pub fn stop(&mut self) {
        self.pause(); // finalize any running time
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(start) = self.start_time {
            // running
            self.accumulated + (Utc::now() - start)
        } else {
            // paused/stopped
            self.accumulated
        }
    }

    pub fn elapsed_hms(&self) -> (i64, i64, i64) {
        let secs = self.elapsed().num_seconds();
        (secs / 3600, (secs % 3600) / 60, secs % 60)
    }
}

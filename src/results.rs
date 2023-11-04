use std::time::Duration;


#[derive(Debug)]
pub struct Results {
    pub typed : u32,
    pub time : Duration,
    pub wpm: u64,
}

impl Results {
    pub fn new() -> Self {
        Self {
            typed: 0,
            time: Duration::new(0, 0),
            wpm: 0,
        }
    }

    pub fn set_wpm(&mut self) {
        self.wpm = ((self.typed / 5) as u64) * (60 / self.time.as_secs());
    }
}

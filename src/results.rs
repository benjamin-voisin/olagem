use std::time::Duration;


#[derive(Debug)]
pub struct Results {
    pub typed : usize,
    pub time : Duration,
}

impl Results {
    pub fn new() -> Self {
        Self {
            typed: 0,
            time: Duration::new(0, 0),
        }
    }
}

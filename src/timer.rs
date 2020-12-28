pub struct Timer {
    last_tick: chrono::DateTime<chrono::Local>,
    interval: f64,
}

impl Timer {
    pub fn new(interval: f64) -> Self {
        Self {
            last_tick: chrono::Local::now(),
            interval,
        }
    }

    pub fn tick(&mut self) -> bool {
        let now = chrono::Local::now();
        let elapsed_time = (now - self.last_tick).num_nanoseconds().unwrap() as f64 * 1e-9;
        if elapsed_time >= self.interval {
            self.last_tick = now;
            true
        } else {
            false
        }
    }
}

struct Timer {
    pub enabled: bool,
    pub counter: u16,
    pub insterval: u16,
}

impl Timer {
    pub fn new(interval: u16) -> Self {
        Timer {
            counter: 0,
            interval,
            enabled: false,
        }
    }

    pub fn start(&mut self) {
        self.enabled = true;
        self.counter = 0;
    }

    pub fn stop(&mut self) {
        self.enabled = false;
    }

    pub fn tick(&mut self) -> bool {
        if !self.enabled {return false;}
        self.counter += 1;
        if self.counter >= self.interval {
            self.counter = 0;
            return true;
        }
        false
    }
}

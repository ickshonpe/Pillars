#[derive(Copy, Clone, PartialEq)]
pub struct Timer {
    time_left: f64,
    time_limit: f64,
}


impl Timer {
    pub fn new(time_limit: f64) -> Timer {        
        Timer {
            time_left: time_limit,
            time_limit: time_limit,
        }
    }

    pub fn update_and_check(&mut self, time_delta: f64) -> bool {        
        self.time_left -= time_delta;
        self.time_left < 0.
    }

    pub fn remaining(&self) -> f64 {
        self.time_left
    }

    pub fn elapsed(&self) -> f64 {
        self.time_limit - self.time_left
    }

    pub fn elapsed_as_fraction(&self) -> f64 {
        self.elapsed() / self.time_limit
    }

    pub fn reset(&mut self) {
        self.time_left = self.time_limit
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn timer_test() {
        let mut timer = ::timer::Timer::new(100.);
        assert!(!timer.update_and_check(30.));
        assert!(!timer.update_and_check(30.));
        assert!(!timer.update_and_check(30.));
        assert!(timer.update_and_check(30.));
        timer.reset();
        assert!(!timer.update_and_check(90.));
    }
}

use std::time;

pub struct Timer {
    tl_miliseconds: u64,
    now: time::Instant,
}

impl Timer {
    pub fn new(tl_miliseconds: u64) -> Timer {
        Timer {
            tl_miliseconds,
            now: time::Instant::now(),
        }
    }

    pub fn timer_loop(&self) -> bool {
        let elapsed = self.now.elapsed().as_millis() as u64;
        if elapsed >= self.tl_miliseconds {
            return false;
        }
        true
    }

    pub fn get_time(&self) -> u64 {
        self.now.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_timer1() {
        let timer = Timer::new(500);
        sleep(time::Duration::from_millis(200));
        assert_eq!(timer.timer_loop(), true);
        sleep(time::Duration::from_millis(300));
        assert_eq!(timer.timer_loop(), false);
    }
}

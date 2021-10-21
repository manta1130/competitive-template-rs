pub struct XorShift {
    seed: u64,
}

impl XorShift {
    pub fn init(seed: u64) -> XorShift {
        XorShift { seed }
    }
}

impl Default for XorShift {
    fn default() -> XorShift {
        XorShift {
            seed: 88172645463325252,
        }
    }
}

impl XorShift {
    pub fn next_u64(&mut self) -> u64 {
        self.seed = self.seed ^ (self.seed << 7);
        self.seed = self.seed ^ (self.seed >> 9);
        self.seed
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    pub fn range_u64(&mut self, f: u64, t: u64) -> u64 {
        f + self.next_u64() % (t - f)
    }
}

use crate::SmallRng;
use std::ops::RangeBounds;
use std::time::{SystemTime, UNIX_EPOCH};

const PCG64_DEFAULT_MULTIPLIER: u128 = 6364136223846793005;
const PCG64_DEFAULT_INCREMENT: u128 = 1442695040888963407;
const PCG64_INIT_STATE: u128 = 0x4d595df4d0f33173;

pub struct PCG64 {
    seed: u128,
    state: u128,
    increment: u128,
    multiplier: u128,
}

impl Default for PCG64 {
    fn default() -> PCG64 {
        PCG64 {
            seed: 88172645463325252,
            state: PCG64_INIT_STATE.wrapping_add(88172645463325252),
            increment: PCG64_DEFAULT_INCREMENT,
            multiplier: PCG64_DEFAULT_MULTIPLIER,
        }
    }
}

impl PCG64 {
    pub fn init(seed: u128) -> PCG64 {
        PCG64 {
            seed,
            state: PCG64_INIT_STATE.wrapping_add(seed),
            increment: PCG64_DEFAULT_INCREMENT,
            multiplier: PCG64_DEFAULT_MULTIPLIER,
        }
    }

    pub fn init_unix_epoch() -> PCG64 {
        let mills = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_millis() as u128;

        let micros = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_millis() as u128;

        PCG64::init(mills << 32 | micros)
    }

    pub fn get_seed(&self) -> u128 {
        self.seed
    }
}

impl SmallRng for PCG64 {
    fn next_u64(&mut self) -> u64 {
        let x = self.state;
        let count = x >> 122;
        let x64 = (x ^ (x >> 64)) as u64;
        self.state = (x.wrapping_mul(self.multiplier)).wrapping_add(self.increment);
        (x64).rotate_right(count as u32)
    }

    fn next_u8(&mut self) -> u8 {
        self.next_u64() as u8
    }

    fn next_i8(&mut self) -> i8 {
        self.next_u64() as i8
    }

    fn next_u16(&mut self) -> u16 {
        self.next_u64() as u16
    }

    fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }

    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_i32(&mut self) -> i32 {
        self.next_u64() as i32
    }

    fn next_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }

    fn next_u128(&mut self) -> u128 {
        ((self.next_u64() as u128) << 64) | (self.next_u64() as u128)
    }

    fn next_i128(&mut self) -> i128 {
        self.next_u128() as i128
    }

    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    fn next_isize(&mut self) -> isize {
        self.next_u64() as isize
    }

    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    fn range_u64<R>(&mut self, r: R) -> u64
    where
        R: RangeBounds<u64>,
    {
        let start = match r.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match r.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => u64::MAX,
        };

        start + self.next_u64() % (end - start)
    }

    fn range_i64<R>(&mut self, r: R) -> i64
    where
        R: RangeBounds<i64>,
    {
        let start = match r.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match r.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => i64::MAX,
        };

        start + self.next_i64() % (end - start)
    }

    fn range_usize<R>(&mut self, r: R) -> usize
    where
        R: RangeBounds<usize>,
    {
        let start = match r.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match r.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => usize::MAX,
        };

        start + self.next_usize() % (end - start)
    }

    fn range_isize<R>(&mut self, r: R) -> isize
    where
        R: RangeBounds<isize>,
    {
        let start = match r.start_bound() {
            std::ops::Bound::Included(&s) => s,
            std::ops::Bound::Excluded(&s) => s + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match r.end_bound() {
            std::ops::Bound::Included(&e) => e + 1,
            std::ops::Bound::Excluded(&e) => e,
            std::ops::Bound::Unbounded => isize::MAX,
        };

        start + self.next_isize() % (end - start)
    }
}

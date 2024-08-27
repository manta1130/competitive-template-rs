use std::ops::RangeBounds;

pub trait SmallRng {
    fn next_u8(&mut self) -> u8;
    fn next_i8(&mut self) -> i8;
    fn next_u16(&mut self) -> u16;
    fn next_i16(&mut self) -> i16;
    fn next_u32(&mut self) -> u32;
    fn next_i32(&mut self) -> i32;
    fn next_i64(&mut self) -> i64;
    fn next_u64(&mut self) -> u64;
    fn next_u128(&mut self) -> u128;
    fn next_i128(&mut self) -> i128;
    fn next_usize(&mut self) -> usize;
    fn next_isize(&mut self) -> isize;
    fn next_bool(&mut self) -> bool;
    fn range_u64<R>(&mut self, r: R) -> u64
    where
        R: RangeBounds<u64>;
    fn range_i64<R>(&mut self, r: R) -> i64
    where
        R: RangeBounds<i64>;
    fn range_usize<R>(&mut self, r: R) -> usize
    where
        R: RangeBounds<usize>;
    fn range_isize<R>(&mut self, r: R) -> isize
    where
        R: RangeBounds<isize>;
}

pub trait Shuffle<T, R>
where
    R: SmallRng,
{
    fn shuffle(&mut self, rng: &mut R);
}

impl<T, R> Shuffle<T, R> for Vec<T>
where
    R: SmallRng,
{
    fn shuffle(&mut self, rng: &mut R) {
        if self.is_empty() {
            return;
        }
        for idx1 in 0..self.len() - 1 {
            let idx2 = rng.range_usize(idx1..self.len());
            self.swap(idx1, idx2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XorShift;
    use crate::PCG64;

    #[test]
    fn xorshift_works() {
        let mut rng = XorShift::default();
        for i in [0, 10, 1000, 10000, 998244353] {
            assert_eq!(i, rng.range_i64(i..=i));
            assert_eq!(i, rng.range_i64(i..i + 1));

            assert_eq!(i as u64, rng.range_u64(i as u64..=i as u64));
            assert_eq!(i as u64, rng.range_u64(i as u64..i as u64 + 1));

            assert_eq!(i as usize, rng.range_usize(i as usize..=i as usize));
            assert_eq!(i as usize, rng.range_usize(i as usize..i as usize + 1));

            assert_eq!(i as isize, rng.range_isize(i as isize..=i as isize));
            assert_eq!(i as isize, rng.range_isize(i as isize..i as isize + 1));
        }
    }

    #[test]
    fn pcg64_works() {
        let mut rng = PCG64::default();
        for i in [0, 10, 1000, 10000, 998244353] {
            assert_eq!(i, rng.range_i64(i..=i));
            assert_eq!(i, rng.range_i64(i..i + 1));

            assert_eq!(i as u64, rng.range_u64(i as u64..=i as u64));
            assert_eq!(i as u64, rng.range_u64(i as u64..i as u64 + 1));

            assert_eq!(i as usize, rng.range_usize(i as usize..=i as usize));
            assert_eq!(i as usize, rng.range_usize(i as usize..i as usize + 1));

            assert_eq!(i as isize, rng.range_isize(i as isize..=i as isize));
            assert_eq!(i as isize, rng.range_isize(i as isize..i as isize + 1));
        }
    }

    #[test]
    fn shuffle_works() {
        let mut rng = XorShift::init_unix_epoch();
        let mut v: Vec<usize> = Vec::new();
        v.shuffle(&mut rng);
    }
}

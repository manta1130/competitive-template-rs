use std::iter::Iterator;

type ValueType = u64;

pub trait GetDivisor {
    fn get_divisor(&self) -> Divisor;
}

macro_rules! GetDivisor_macro{
    ($($t:ty),*) => {
        $(
        impl GetDivisor for $t {
            fn get_divisor(&self) -> Divisor {
                Divisor::calc(*self as ValueType)
            }
        })*
    };

}

GetDivisor_macro!(u32, u64, u128, usize, i32, i64, i128, isize);

pub trait GetPrimeFactorization {
    fn prime_factorization(&self) -> PrimeFactorization;
}

macro_rules! PrimeFactorization_macro{
    ($($t:ty),*) => {
        $(
        impl GetPrimeFactorization for $t {
            fn prime_factorization(&self) -> PrimeFactorization {
                PrimeFactorization::calc(*self as ValueType)
            }
        })*
    };
}

PrimeFactorization_macro!(u32, u64, u128, usize, i32, i64, i128, isize);
pub struct Divisor {
    n: ValueType,
    cur: ValueType,
    flag: bool,
}

impl Divisor {
    pub fn calc(n: ValueType) -> Divisor {
        Divisor {
            n,
            cur: 1,
            flag: false,
        }
    }
}
impl Iterator for Divisor {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur * self.cur > self.n {
            None
        } else if self.flag {
            if self.cur * self.cur == self.n {
                return None;
            }
            self.flag = false;
            self.cur += 1;
            Some(self.n / (self.cur - 1))
        } else {
            while self.n % self.cur != 0 {
                self.cur += 1;
                if self.cur * self.cur > self.n {
                    return None;
                }
            }
            self.flag = true;
            Some(self.cur)
        }
    }
}

pub struct PrimeFactorization<'a> {
    n: ValueType,
    cur: ValueType,
    p_list: Option<&'a [ValueType]>,
    idx: usize,
}

///素因数分解
impl<'a> PrimeFactorization<'a> {
    ///素因数を計算するイテレータを返す。
    pub fn calc(n: ValueType) -> PrimeFactorization<'a> {
        PrimeFactorization {
            n,
            cur: 1,
            p_list: None,
            idx: 0,
        }
    }
    pub fn calc_fast(n: ValueType, p_list: &'a [ValueType]) -> PrimeFactorization<'a> {
        PrimeFactorization {
            n,
            cur: 1,
            p_list: Some(p_list),
            idx: 0,
        }
    }
}

impl<'a> Iterator for PrimeFactorization<'a> {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur == 0 || self.cur > self.n {
                return None;
            }

            if self.p_list.is_some() {
                if self.idx >= self.p_list.unwrap().len() {
                    return None;
                }
                self.cur = self.p_list.unwrap()[self.idx];
                self.idx += 1;
            } else {
                self.cur += 1;
            }

            if self.cur * self.cur > self.n {
                if self.n != 1 {
                    self.cur = 0;
                    return Some(self.n);
                }
                return None;
            }
            if self.n % self.cur == 0 {
                self.n /= self.cur;
                if self.p_list.is_some() {
                    self.idx -= 1;
                }
                self.cur -= 1;
                return Some(self.cur + 1);
            }
        }
    }
}

pub fn get_primelist(u: ValueType) -> Vec<ValueType> {
    let mut v = vec![true; u as usize + 1];
    let mut r = vec![];
    for i in 2..=u as usize {
        if v[i] {
            r.push(i as ValueType);
            let mut j = i * i;
            while j <= u as usize {
                v[j] = false;
                j += i;
            }
        }
    }
    r
}

pub fn get_mobius(n: ValueType) -> Vec<isize> {
    let mut r = vec![0, 1];
    let p = get_primelist(n);
    for i in 2..=n {
        let mut f = PrimeFactorization::calc_fast(i as u64, &p).collect::<Vec<_>>();
        let count = f.len();
        f.dedup();
        if f.len() != count {
            r.push(0);
        } else {
            r.push(if f.len() % 2 == 0 { 1 } else { -1 });
        }
    }
    r
}

fn modpow_128bit(mut s: u128, mut n: u128, p: u128) -> u128 {
    if p == 0 {
        return 1;
    }
    let mut t = s;
    s = 1;
    while n > 0 {
        if n & 1 != 0 {
            s *= t;
            s %= p;
        }
        n >>= 1;
        t *= t;
        t %= p;
    }
    s
}

fn modpow_64bit(mut s: u64, mut n: u64, p: u64) -> u64 {
    if p == 0 {
        return 1;
    }
    let mut t = s;
    s = 1;
    while n > 0 {
        if n & 1 != 0 {
            s *= t;
            s %= p;
        }
        n >>= 1;
        t *= t;
        t %= p;
    }
    s
}

pub fn miller_rabin(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 || n % 2 == 0 {
        return false;
    }

    let (mut s, mut t) = (0, n - 1);

    while t % 2 == 0 {
        s += 1;
        t >>= 1;
    }

    let arr = if n < 4_759_123_141 {
        vec![2, 7, 61]
    } else if n < 341_550_071_728_321 {
        vec![2, 3, 5, 7, 11, 13, 17]
    } else if n < 3_825_123_056_546_413_051 {
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23]
    } else {
        vec![2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022]
    }
    .iter()
    .filter(|&&q| q < n)
    .cloned()
    .collect::<Vec<_>>();

    let millor_rabin_inner = |a| {
        if modpow_128bit(a as u128, t as u128, n as u128) == 1 {
            return true;
        }

        for i in 0..s {
            if modpow_128bit(a as u128, 2_u128.pow(i) * t as u128, n as u128) as u64 == n - 1 {
                return true;
            }
        }
        false
    };

    let millor_rabin_inner_small = |a| {
        if modpow_64bit(a, t, n) == 1 {
            return true;
        }

        for i in 0..s {
            if modpow_64bit(a, 2_u64.pow(i) * t, n) == n - 1 {
                return true;
            }
        }
        false
    };

    if n < 1_000_000_000 {
        for a in arr {
            if !millor_rabin_inner_small(a) {
                return false;
            }
        }
    } else {
        for a in arr {
            if !millor_rabin_inner(a) {
                return false;
            }
        }
    }

    true
}

fn gcd_u64(a: u64, b: u64) -> u64
where
{
    if b + b == b {
        return a;
    }
    gcd_u64(b, a % b)
}

pub struct PollardRho {
    arr: Vec<u64>,
}

impl PollardRho {
    pub fn calc(n: u64) -> PollardRho {
        PollardRho { arr: vec![n] }
    }
}

impl Iterator for PollardRho {
    type Item = ValueType;

    #[allow(clippy::many_single_char_names)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.arr.is_empty() {
            return None;
        }
        let n = self.arr.pop().unwrap();
        if n == 1 {
            return None;
        }
        if miller_rabin(n) {
            let r = n;
            return Some(r);
        }
        if n % 2 == 0 {
            self.arr.push(n / 2);
            return Some(2);
        }

        let f = |x, seed| ((x as u128 * x as u128 + seed as u128) % n as u128) as u64;
        let f_small = |x, seed| ((x * x + seed) % n);

        for s in 1.. {
            let (mut x, mut y, mut d) = (2, 2, 1);

            while d == 1 {
                if n <= 1_000_000_000 {
                    x = f_small(x, s);
                    y = f_small(f_small(y, s), s);
                } else {
                    x = f(x, s);
                    y = f(f(y, s), s);
                }
                d = gcd_u64(std::cmp::max(x, y) - std::cmp::min(x, y), n)
            }
            if d != n {
                self.arr.push(n / d);
                self.arr.push(d);
                return self.next();
            }
        }
        panic![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XorShift;

    #[test]
    fn miller_rabin_works() {
        let mut prime_list = get_primelist(1000000);
        prime_list.append(&mut vec![
            2_147_483_647,
            67_280_421_310_721,
            9_007_199_254_740_997,
            123_456_789_012_345_671,
        ]);
        let not_prime_list = vec![
            4,
            4_759_123_141,
            1_565_912_117_761,
            8_635_844_967_113_809,
            406_248_370_438_173_883,
            9_223_372_036_854_775_807,
        ];

        assert!(prime_list.iter().map(|&q| miller_rabin(q)).all(|q| q));
        assert!(not_prime_list.iter().map(|&q| miller_rabin(q)).all(|q| !q));
    }

    #[test]
    fn pollard_rho_works() {
        let mut rng = XorShift::default();
        for _ in 0..100 {
            let num = rng.range_u64(0, 1_000_000_000);
            let mut slow = PrimeFactorization::calc(num).collect::<Vec<_>>();
            let mut fast = PollardRho::calc(num).collect::<Vec<_>>();
            slow.sort_unstable();
            fast.sort_unstable();
            assert_eq!(slow, fast);
        }
    }
}

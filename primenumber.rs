use std::iter::Iterator;

type ValueType = usize;

pub struct PrimeFactorization {
    n: ValueType,
    cur: ValueType,
}

///素因数分解
impl PrimeFactorization {
    ///素因数を計算するイテレータを返す。
    pub fn calc(n: ValueType) -> PrimeFactorization {
        PrimeFactorization { n, cur: 1 }
    }
}

impl Iterator for PrimeFactorization {
    type Item = ValueType;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur == 0 || self.cur > self.n {
                return None;
            }

            self.cur += 1;

            if self.cur * self.cur > self.n {
                if self.n != 1 {
                    self.cur = 0;
                    return Some(self.n);
                }
                return None;
            }
            if self.n % self.cur == 0 {
                self.n /= self.cur;
                self.cur -= 1;
                return Some(self.cur + 1);
            }
        }
    }
}

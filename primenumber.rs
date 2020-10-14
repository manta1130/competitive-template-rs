use std::iter::Iterator;

type ValueType = usize;

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

pub fn get_primelist(u: usize) -> Vec<usize> {
    let mut v = vec![true; u + 1];
    let mut r = vec![];
    for i in 2..=u {
        if v[i] {
            r.push(i);
            let mut j = i * 2;
            while j <= u {
                v[j] = false;
                j += i;
            }
        }
    }
    r
}

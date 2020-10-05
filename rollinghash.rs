use super::internal_math::{safe_mod, Barrett};
use super::modint::*;
pub struct RollingHash {
    m: u32,
    v: Vec<u32>,
    pow: Vec<u32>,
}

///ローリングハッシュ
impl RollingHash {
    ///ローリングハッシュを計算する。
    ///
    /// base:基数
    pub fn calc(input_str: &str, base: u32, m: u32) -> RollingHash {
        ModInt::set_modulus(m);
        let mut v = vec![0];
        let mut pow = vec![1];
        let mut pow_buf = ModInt::new(base);
        let base = ModInt::new(base);
        let mut hash = ModInt::new(0);

        for _ in 0..input_str.len() {
            pow.push(pow_buf.val());
            pow_buf *= base;
        }

        for c in input_str.as_bytes() {
            hash *= base;
            hash += ModInt::new(*c);
            v.push(hash.val());
        }
        RollingHash { m, v, pow }
    }

    ///[from:to)のローリングハッシュを求める。
    pub fn get(&self, from: usize, to: usize) -> i64 {
        let bt = Barrett::new(self.m);
        let buf = bt.mul(self.v[from], self.pow[to - from]);
        safe_mod((self.v[to] + self.m - buf) as i64, self.m as i64)
    }
}

const MOD: u128 = (1 << 61) - 1;

pub struct RollingHash {
    v: Vec<u128>,
    pow: Vec<u128>,
}

/// ローリングハッシュ
impl RollingHash {
    /// ローリングハッシュを計算する。
    ///
    /// base:基数
    pub fn calc(input_str: &str, base: u128) -> RollingHash {
        let mut v = vec![0];
        let mut pow = vec![1];
        let mut pow_buf = base;

        let mut hash = 0;

        for _ in 0..input_str.len() {
            pow.push(pow_buf);
            pow_buf = mulmod(pow_buf, base);
        }

        for c in input_str.as_bytes() {
            hash = mulmod(hash, base);
            hash = addmod(hash, *c as u128);
            v.push(hash);
        }
        RollingHash { v, pow }
    }

    /// [from:to)のローリングハッシュを求める。
    pub fn get(&self, from: usize, to: usize) -> u128 {
        submod(self.v[to], mulmod(self.v[from], self.pow[to - from]))
    }
}

fn mulmod(a: u128, b: u128) -> u128 {
    let t = a as u128 * b as u128;
    calcmod(t)
}

fn addmod(a: u128, b: u128) -> u128 {
    let t = a as u128 + b as u128;
    calcmod(t)
}

fn submod(a: u128, b: u128) -> u128 {
    let t = a as u128 + MOD - b as u128;
    calcmod(t)
}

fn calcmod(mut t: u128) -> u128 {
    t = (t >> 61) + (t & MOD);
    if t >= MOD {
        t - MOD
    } else {
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        let test_str = "hogehogepiyopiyoabcabc";
        let hash = RollingHash::calc(test_str, 1234);
        for i in 0..test_str.len() {
            for j in i..test_str.len() {
                for k in 0..test_str.len() {
                    for l in k..test_str.len() {
                        assert_eq!(
                            test_str[i..j] == test_str[k..l],
                            hash.get(i, j) == hash.get(k, l)
                        );
                    }
                }
            }
        }
    }
}

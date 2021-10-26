use crate::math::inv_mod;
use std::collections::HashMap;

fn modpow(mut s: i64, mut n: i64, p: i64) -> i64 {
    if n == 0 {
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

fn gcd(a: i64, b: i64) -> i64 {
    if b + b == b {
        return a;
    }
    gcd(b, a % b)
}

/// Returns the smallest number k that satisfies x^k=y mod m
#[allow(clippy::many_single_char_names)]
pub fn dlp(x: i64, y: i64, m: i64, allow_zero: bool) -> Option<i64> {
    if m == 1 && allow_zero {
        return Some(0);
    }

    if x == 0 {
        if y == 1 && allow_zero {
            return Some(0);
        } else if y == 0 {
            return Some(1);
        } else {
            return None;
        }
    }

    if y == 1 && allow_zero {
        return Some(0);
    }

    let g = gcd(x, m);
    if g != 1 {
        if y % g != 0 {
            return None;
        }
        let (xd, yd, md) = (x / g, y / g, m / g);
        let xd_inv = inv_mod(xd, md);
        let r = dlp(x % md, (yd * xd_inv) % md, md, true);
        if let Some(r) = r {
            return Some(r + 1);
        }
        return None;
    }

    let block = (m as f64).sqrt() as i64 + 1;

    let mut buf = if allow_zero { 1 } else { x % m };
    let mut map = HashMap::new();

    for j in if allow_zero { 0..block } else { 1..block + 1 } {
        map.entry(buf).or_insert(j);
        buf = (buf * x) % m;
    }

    let g = inv_mod(x, m);
    let g = modpow(g, block, m);
    let mut buf = y;

    for i in 0..block {
        if map.contains_key(&buf) {
            return Some(i * block + map[&buf]);
        }
        buf = (buf * g) % m;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dlp_naive(x: i64, y: i64, m: i64, allow_zero: bool) -> Option<i64> {
        if x == 0 && y == 0 && m == 1 && allow_zero {
            return Some(0);
        }
        for r in if allow_zero { 0 } else { 1 }..=m {
            if modpow(x, r, m) == y {
                return Some(r);
            }
        }
        None
    }

    #[test]
    fn dlp_works() {
        for m in 1..100 {
            for x in 0..m {
                for y in 0..m {
                    assert_eq!(dlp(x, y, m, true), dlp_naive(x, y, m, true));
                    assert_eq!(dlp(x, y, m, false), dlp_naive(x, y, m, false));
                }
            }
        }
    }
}

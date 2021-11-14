pub struct EratosthenesSieve {
    sieve: Vec<usize>,
}

impl EratosthenesSieve {
    /// Constructs a new instance of EratosthenesSieve [0,n].
    pub fn new(n: usize) -> EratosthenesSieve {
        let mut sieve = Vec::with_capacity(n + 1);
        (0..=n).for_each(|i| sieve.push(i));

        for i in 2.. {
            if i * i > n {
                break;
            }

            if sieve[i] != i {
                continue;
            }

            let mut j = i + i;
            while j <= n {
                if sieve[j] == j {
                    sieve[j] = i;
                }
                j += i;
            }
        }

        EratosthenesSieve { sieve }
    }

    pub fn get_factor(&self, n: usize) -> usize {
        self.sieve[n]
    }

    /// Returns true if the n is prime number.
    pub fn is_prime(&self, n: usize) -> bool {
        assert!(n < self.sieve.len());
        n > 1 && self.sieve[n] == n
    }

    pub fn prime_factorization(&self, n: usize) -> PrimeFactorizationResult {
        PrimeFactorizationResult { n, sieve: self }
    }
}

pub struct PrimeFactorizationResult<'a> {
    n: usize,
    sieve: &'a EratosthenesSieve,
}

impl Iterator for PrimeFactorizationResult<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.n <= 1 {
            return None;
        }
        let ret = self.sieve.get_factor(self.n);
        self.n /= ret;
        Some(ret)
    }
}

pub struct EratosthenesSieveRange {
    pub sieve_small: Vec<u64>,
    pub sieve_big: Vec<Vec<u64>>,
    offset: u64,
}

impl EratosthenesSieveRange {
    /// Constructs a new instance of EratosthenesSieve [f,t].
    pub fn new(f: u64, t: u64) -> EratosthenesSieveRange {
        let mut sieve_small = Vec::with_capacity(((t + 1) as f64).sqrt() as usize + 1);
        let mut sieve_big = Vec::with_capacity((t - f + 2) as usize);
        let offset = f;

        for i in 0.. {
            if i * i > t {
                break;
            }
            sieve_small.push(i);
        }

        for _ in f..=t {
            sieve_big.push(vec![]);
        }

        for i in 2.. {
            if i * i > t {
                break;
            }

            if sieve_small[i as usize] != i {
                continue;
            }

            let mut j = i + i;
            while j < sieve_small.len() as u64 {
                if sieve_small[j as usize] == j {
                    sieve_small[j as usize] = i;
                }
                j += i;
            }
            let mut j = ((f + i - 1) / i) * i;
            while j <= t {
                sieve_big[(j - offset) as usize].push(i);
                j += i;
            }
        }

        EratosthenesSieveRange {
            sieve_small,
            sieve_big,
            offset: f,
        }
    }

    /// Returns true if the n is prime number.
    pub fn is_prime(&self, n: u64) -> bool {
        if n >= self.sieve_small.len() as u64 {
            self.sieve_big[(n - self.offset) as usize].is_empty()
        } else {
            self.sieve_small[n as usize] == n
        }
    }

    pub fn prime_factorization(&mut self, n: u64) -> PrimeFactorizationRangeResult {
        PrimeFactorizationRangeResult {
            n,
            sieve: self,
            buf: Vec::new(),
        }
    }
}

pub struct PrimeFactorizationRangeResult<'a> {
    n: u64,
    buf: Vec<u64>,
    sieve: &'a mut EratosthenesSieveRange,
}

impl Iterator for PrimeFactorizationRangeResult<'_> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.n <= 1 {
            return None;
        }
        if !self.buf.is_empty() {
            let ret = self.buf.pop().unwrap();
            self.n /= ret;
            if self.n % ret == 0 {
                self.buf.push(ret);
            }
            Some(ret)
        } else if self.n < self.sieve.sieve_small.len() as u64 {
            let ret = self.sieve.sieve_small[self.n as usize];
            self.n /= ret;
            Some(ret)
        } else if self.n >= self.sieve.offset {
            self.buf
                .append(&mut self.sieve.sieve_big[(self.n - self.sieve.offset) as usize].clone());
            if self.buf.is_empty() {
                let ret = self.n;
                self.n = 1;
                Some(ret)
            } else {
                self.next()
            }
        } else {
            let ret = self.n;
            self.n = 1;
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primenumber::*;
    use crate::xorshift::*;

    #[test]
    fn test_is_prime() {
        let limit = 1_000_000;
        let sieve = EratosthenesSieve::new(limit);
        let mut rng = XorShift::default();

        for n in 0..10 {
            assert_eq!(sieve.is_prime(n), miller_rabin(n as u64));
        }
        for n in limit - 10..=limit {
            assert_eq!(sieve.is_prime(n), miller_rabin(n as u64));
        }

        for _ in 0..1000 {
            let n = rng.range_usize(0, limit);
            assert_eq!(sieve.is_prime(n), miller_rabin(n as u64));
        }
    }

    #[test]
    fn test_prime_factorization() {
        let limit = 1_000_000;
        let sieve = EratosthenesSieve::new(limit);
        let mut rng = XorShift::default();

        for n in 0..10 {
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64)
                .map(|q| q as usize)
                .collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }
        for n in limit - 10..=limit {
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64)
                .map(|q| q as usize)
                .collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }

        for _ in 0..1000 {
            let n = rng.range_usize(0, limit);
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64)
                .map(|q| q as usize)
                .collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }
    }

    #[test]
    fn test_prime_factorization_range() {
        let limit = 1_000_000_000_000;
        let offset = 1_000_000;
        let mut sieve = EratosthenesSieveRange::new(limit - offset, limit);
        let mut rng = XorShift::default();

        for n in 0..10 {
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64).collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }

        for n in limit - 10..=limit {
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64).collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }

        for _ in 0..100 {
            let n = rng.range_u64(limit - offset, limit);
            let mut result = sieve.prime_factorization(n).collect::<Vec<_>>();
            let mut result_pollardrho = PollardRho::calc(n as u64).collect::<Vec<_>>();

            result.sort_unstable();
            result_pollardrho.sort_unstable();

            assert_eq!(result, result_pollardrho);
        }
    }

    #[test]
    #[should_panic]
    fn test_is_prime_panic_1() {
        let sieve = EratosthenesSieve::new(100);
        sieve.is_prime(101);
    }
}

use crate::fenwicktree::*;

type ValueType = i64;

pub struct RaqBit {
    n: usize,
    bit0: FenwickTree<ValueType>,
    bit1: FenwickTree<ValueType>,
}

impl RaqBit {
    pub fn new(n: usize) -> RaqBit {
        RaqBit {
            n,
            bit0: FenwickTree::new(n, 0),
            bit1: FenwickTree::new(n, 0),
        }
    }

    pub fn accum(&self, idx: usize) -> ValueType {
        assert!(idx <= self.n);
        self.bit0.accum(idx) + self.bit1.accum(idx) * idx as ValueType
    }

    pub fn add(&mut self, l: usize, r: usize, val: ValueType) {
        assert!(r <= self.n && l <= r);
        self.bit0.add(l, -val * l as ValueType);
        self.bit0.add(r, val * r as ValueType);
        self.bit1.add(l, val);
        self.bit1.add(r, -val);
    }

    /// Returns data[l] + ... + data[r - 1].
    pub fn sum(&self, l: usize, r: usize) -> ValueType {
        assert!(r <= self.n && l <= r);
        self.accum(r) - self.accum(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xorshift::*;
    struct RaqNaive {
        data: Vec<ValueType>,
    }
    impl RaqNaive {
        pub fn new(n: usize) -> RaqNaive {
            RaqNaive { data: vec![0; n] }
        }

        pub fn add(&mut self, l: usize, r: usize, val: i64) {
            for i in l..r {
                self.data[i] += val;
            }
        }

        pub fn sum(&self, l: usize, r: usize) -> i64 {
            (l..r).map(|i| self.data[i]).sum()
        }
    }

    #[test]
    fn bit_raq_works_small() {
        let mut raq = RaqBit::new(10);
        let mut raq_naive = RaqNaive::new(10);

        let instruction = [(0, 4, -4_i64), (9, 10, 4), (2, 10, -3), (5, 7, 3)];
        for (l, r, val) in instruction {
            raq.add(l, r, val);
            raq_naive.add(l, r, val);
            for i in 0..10 {
                for j in i..10 {
                    dbg!(l, r, val, i, j);
                    assert_eq!(raq.sum(i, j), raq_naive.sum(i, j));
                }
            }
        }
    }

    #[test]
    fn bit_raq_works_large() {
        let size = 100;

        let mut raq = RaqBit::new(size);
        let mut raq_naive = RaqNaive::new(size);

        let mut rng = XorShift::default();

        for _ in 0..50 {
            let l = rng.range_usize(0, size);
            let r = rng.range_usize(l, size);
            let val = rng.range_i64(-1_000_000, 1_000_000);

            raq.add(l, r, val);
            raq_naive.add(l, r, val);
            for i in 0..size {
                for j in i..size {
                    dbg!(l, r, val, i, j);
                    assert_eq!(raq.sum(i, j), raq_naive.sum(i, j));
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bit_raq_works_panic1() {
        let mut raq = RaqBit::new(10);
        raq.add(9, 11, 1);
    }

    #[test]
    #[should_panic]
    fn bit_raq_works_panic2() {
        let raq = RaqBit::new(10);
        raq.accum(11);
    }

    #[test]
    #[should_panic]
    fn bit_raq_works_panic3() {
        let raq = RaqBit::new(10);
        raq.sum(0, 11);
    }
}

use std::collections::BTreeSet;

type ValueType = i64;

pub struct Mex {
    set: BTreeSet<(ValueType, ValueType)>,
}

impl Default for Mex {
    fn default() -> Self {
        Self::new()
    }
}

impl Mex {
    pub fn new() -> Self {
        let mut set = BTreeSet::new();
        set.insert((i64::min_value(), i64::min_value()));
        set.insert((i64::max_value(), i64::max_value()));
        Mex { set }
    }

    pub fn insert(&mut self, x: ValueType) {
        let back = *self
            .set
            .range(..(x, ValueType::max_value()))
            .next_back()
            .unwrap();
        let next = *self
            .set
            .range((x + 1, ValueType::min_value())..)
            .next()
            .unwrap();

        if back.1 == x && x + 1 == next.0 {
            self.set.remove(&back);
            self.set.remove(&next);
            self.set.insert((back.0, next.1));
        } else if back.1 == x {
            self.set.remove(&back);
            self.set.insert((back.0, x + 1));
        } else if x + 1 == next.0 {
            self.set.remove(&next);
            self.set.insert((x, next.1));
        } else {
            self.set.insert((x, x + 1));
        }
    }

    pub fn remove(&mut self, x: ValueType) {
        let back = *self
            .set
            .range(..(x, ValueType::max_value()))
            .next_back()
            .unwrap();

        if back.1 > x {
            self.set.remove(&back);
            self.set.insert((back.0, x));
            self.set.insert((x + 1, back.1));
        }
    }

    ///Returns mex beetween [a,b)
    pub fn mex(&mut self, f: ValueType, t: ValueType) -> Option<ValueType> {
        if f >= t {
            return None;
        }

        let r = self
            .set
            .range(..(f, ValueType::max_value()))
            .next_back()
            .unwrap()
            .1;

        if r >= t {
            None
        } else {
            Some(std::cmp::max(r, f))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MexDebug {
        set: BTreeSet<ValueType>,
    }

    impl MexDebug {
        fn new() -> Self {
            let set = BTreeSet::new();
            MexDebug { set }
        }

        fn insert(&mut self, x: ValueType) {
            self.set.insert(x);
        }

        fn remove(&mut self, x: ValueType) {
            self.set.remove(&x);
        }

        fn mex(&mut self, f: ValueType, t: ValueType) -> Option<ValueType> {
            for i in f..t {
                if !self.set.contains(&i) {
                    return Some(i);
                }
            }
            None
        }
    }

    #[test]
    fn mex_works() {
        let mut v = Mex::new();
        let mut v_dbg = MexDebug::new();
        for i in -5..0 {
            v.insert(i);
            v_dbg.insert(i);
        }
        for i in 5..10 {
            v.insert(i);
            v_dbg.insert(i);
        }
        for i in 15..20 {
            v.insert(i);
            v_dbg.insert(i);
        }

        for f in -10..30 {
            for t in -10..30 {
                assert_eq!(v.mex(f, t), v_dbg.mex(f, t));
            }
        }
        let remove_list = [-5, -4, -3, -2, -1, 6, 8, 9, 15, 20, 30];
        for remove in remove_list {
            v.remove(remove);
            v_dbg.remove(remove);
            for f in -10..30 {
                for t in -10..30 {
                    assert_eq!(v.mex(f, t), v_dbg.mex(f, t));
                }
            }
        }
    }
}

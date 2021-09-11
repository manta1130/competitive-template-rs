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
        set.insert((ValueType::min_value(), ValueType::min_value()));
        set.insert((ValueType::max_value(), ValueType::max_value()));
        Mex { set }
    }

    pub fn insert(&mut self, x: ValueType) {
        if self.mex(x, x + 1).is_none() {
            return;
        }

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

    pub fn insert_range(&mut self, f: i64, t: i64) {
        if f + 1 == t {
            self.insert(f);
            return;
        }

        let left = self.mex(f - 1, f).is_none();
        let right = self.mex(t, t + 1).is_none();

        self.remove(f - 1);
        self.remove(t);

        self.remove_range(f, t);
        self.set.insert((f, t));

        if left {
            self.insert(f - 1);
        }
        if right {
            self.insert(t);
        }
    }

    pub fn remove_range(&mut self, f: i64, t: i64) {
        if f + 1 == t {
            self.remove(f);
            return;
        }
        let left = self.mex(f - 1, f).is_none();
        let right = self.mex(t, t + 1).is_none();

        self.insert(f);
        self.insert(t - 1);

        self.remove(f - 1);
        self.remove(t);

        let remove_list = self
            .set
            .range((f, f + 1)..=(t - 1, t))
            .cloned()
            .collect::<Vec<_>>();

        for k in remove_list {
            self.set.remove(&k);
        }
        if left {
            self.insert(f - 1);
        }
        if right {
            self.insert(t);
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

    ///Returns mex beetween [f,t)
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

        fn insert_range(&mut self, f: ValueType, t: ValueType) {
            for k in f..t {
                self.set.insert(k);
            }
        }
        fn remove_range(&mut self, f: ValueType, t: ValueType) {
            for k in f..t {
                self.set.remove(&k);
            }
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
        for i in 18..22 {
            v.insert(i);
            v_dbg.insert(i);
        }

        for f in -10..30 {
            for t in -10..30 {
                assert_eq!(v.mex(f, t), v_dbg.mex(f, t));
            }
        }
        let remove_list = [-5, -4, -3, -2, -1, 6, 8, 9, 15, 20, 30, -4, -2];
        for &remove in &remove_list {
            v.remove(remove);
            v_dbg.remove(remove);
            for f in -10..30 {
                for t in -10..30 {
                    assert_eq!(v.mex(f, t), v_dbg.mex(f, t));
                }
            }
        }
    }

    #[test]
    fn mex_range_works() {
        let mut v = Mex::new();
        let mut v_dbg = MexDebug::new();

        let list = vec![
            (true, 1, 6),
            (true, 4, 9),
            (true, 1, 10),
            (false, 4, 8),
            (true, 5, 6),
            (false, 5, 9),
            (true, -1, 5),
            (true, -10, 10),
            (false, -10, 10),
        ];

        for (ty, f, t) in list {
            if ty {
                v.insert_range(f, t);
                v_dbg.insert_range(f, t);
            } else {
                v.remove_range(f, t);
                v_dbg.remove_range(f, t);
            }
            for f in -30..30 {
                for t in -30..30 {
                    if f == 8 && t == 10 {
                        dbg!(f, t, &v.set);
                    }
                    assert_eq!(v.mex(f, t), v_dbg.mex(f, t));
                }
            }
        }
    }
}

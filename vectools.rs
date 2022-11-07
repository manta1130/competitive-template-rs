pub trait BSearch<T>
where
    T: PartialOrd,
{
    fn lower_bound(&self, k: T) -> usize;
    fn upper_bound(&self, k: T) -> usize;
}

impl<T> BSearch<T> for Vec<T>
where
    T: PartialOrd,
{
    fn lower_bound(&self, k: T) -> usize {
        let mut from = 0;
        let mut to = self.len();
        while to - from > 1 {
            let mid = (from + to) / 2;
            if self[mid] < k {
                from = mid;
            } else {
                to = mid;
            }
        }
        if self[from] < k {
            to
        } else {
            from
        }
    }
    fn upper_bound(&self, k: T) -> usize {
        let mut from = 0;
        let mut to = self.len();
        while to - from > 1 {
            let mid = (from + to) / 2;
            if self[mid] <= k {
                from = mid;
            } else {
                to = mid;
            }
        }
        if self[from] > k {
            from
        } else {
            to
        }
    }
}

pub fn next_permutation<T>(v: &mut [T]) -> bool
where
    T: PartialOrd,
{
    let mut pivot = v.len() - 1;
    while pivot > 0 {
        if v[pivot] > v[pivot - 1] {
            break;
        }
        pivot -= 1;
    }

    if pivot == 0 {
        return false;
    }

    pivot -= 1;

    let mut pivot_swap = v.len() - 1;
    while v[pivot] >= v[pivot_swap] {
        pivot_swap -= 1;
    }
    v.swap(pivot, pivot_swap);
    v[pivot + 1..].reverse();

    true
}

pub fn prev_permutation<T>(v: &mut [T]) -> bool
where
    T: PartialOrd,
{
    let mut pivot = v.len() - 1;
    while pivot > 0 {
        if v[pivot] < v[pivot - 1] {
            break;
        }
        pivot -= 1;
    }

    if pivot == 0 {
        return false;
    }

    pivot -= 1;

    let mut pivot_swap = v.len() - 1;
    while v[pivot] <= v[pivot_swap] {
        pivot_swap -= 1;
    }
    v.swap(pivot, pivot_swap);
    v[pivot + 1..].reverse();

    true
}

pub fn seq_compress<T>(v: &mut [T], start: T, step: T)
where
    T: Ord + Copy + std::ops::AddAssign,
{
    let mut buf_v = v.iter().cloned().enumerate().collect::<Vec<_>>();
    buf_v.sort_unstable_by_key(|q| q.1);
    let mut prev_val = buf_v[0].1;
    let mut new_val = start;
    for (_i, x) in buf_v.iter_mut() {
        if prev_val != *x {
            new_val += step;
            prev_val = *x;
        }
        *x = new_val;
    }
    buf_v.sort_unstable();
    for (i, x) in v.iter_mut().enumerate() {
        *x = buf_v[i].1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn next_permutation_works() {
        let check = |mut v: Vec<usize>| {
            let mut prev_v = vec![0; v.len()];
            let mut set = HashSet::new();
            loop {
                assert!(!set.contains(&v));
                set.insert(v.clone());
                assert!(prev_v < v);
                prev_v = v.clone();
                if !next_permutation(&mut v) {
                    break;
                }
            }
            assert!(set.contains(&v));
            set.len()
        };

        check(vec![1]);
        check(vec![1, 2, 3, 4, 5]);
        check(vec![1, 1, 5, 5, 9]);
        check(vec![1, 1, 1, 1, 1]);

        let mut u = vec![2, 1, 3];
        next_permutation(&mut u);
        assert_eq!(u, vec![2, 3, 1]);

        assert_eq!(check(vec![1, 2, 3, 4, 5]), (1..=5).product());
    }

    #[test]
    fn prev_permutation_works() {
        let check = |mut v: Vec<usize>| {
            let mut prev_v = vec![std::usize::MAX; v.len()];
            let mut set = HashSet::new();
            loop {
                assert!(!set.contains(&v));
                set.insert(v.clone());
                assert!(prev_v > v);
                prev_v = v.clone();
                if !prev_permutation(&mut v) {
                    break;
                }
            }
            assert!(set.contains(&v));
            set.len()
        };

        check(vec![1]);
        check(vec![5, 4, 3, 2, 1]);
        check(vec![5, 5, 2, 2, 1]);
        check(vec![1, 1, 1, 1, 1]);

        let mut u = vec![3, 1, 2];
        prev_permutation(&mut u);
        assert_eq!(u, vec![2, 3, 1]);

        assert_eq!(check(vec![5, 4, 3, 2, 1]), (1..=5).product());
    }

    #[test]
    fn bsearch_works() {
        let v = vec![1, 2, 3, 4, 4, 7, 13, 19];

        assert_eq!(v.lower_bound(0), 0);
        assert_eq!(v.upper_bound(0), 0);
        assert_eq!(v.lower_bound(1), 0);
        assert_eq!(v.upper_bound(1), 1);
        assert_eq!(v.lower_bound(4), 3);
        assert_eq!(v.upper_bound(4), 5);
        assert_eq!(v.lower_bound(8), 6);
        assert_eq!(v.upper_bound(8), 6);
        assert_eq!(v.lower_bound(20), 8);
        assert_eq!(v.upper_bound(20), 8);
    }

    #[test]
    fn seq_compress_works() {
        let mut v = vec![4, 9, -2, 3, 5];
        seq_compress(&mut v, 0, 1);
        assert_eq!(v, vec![2, 4, 0, 1, 3]);

        let mut v = vec![4, 9, -2, 3, 5];
        seq_compress(&mut v, 0, 2);
        assert_eq!(v, vec![4, 8, 0, 2, 6]);

        let mut v = vec![4, 9, -2, 3, 5];
        seq_compress(&mut v, 1, 2);
        assert_eq!(v, vec![5, 9, 1, 3, 7]);
    }
}

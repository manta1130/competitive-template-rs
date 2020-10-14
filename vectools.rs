pub fn upper_bound<T>(v: &[T], k: T) -> usize
where
    T: PartialOrd,
{
    let mut from = 0;
    let mut to = v.len();

    while to - from > 1 {
        let mid = (from + to) / 2;
        if v[mid] <= k {
            from = mid;
        } else {
            to = mid;
        }
    }
    if v[from] > k {
        from
    } else {
        to
    }
}

pub fn lower_bound<T>(v: &[T], k: T) -> usize
where
    T: PartialOrd,
{
    let mut from = 0;
    let mut to = v.len();

    while to - from > 1 {
        let mid = (from + to) / 2;
        if v[mid] < k {
            from = mid;
        } else {
            to = mid;
        }
    }

    if v[from] < k {
        to
    } else {
        from
    }
}

pub fn next_permutation<T>(v: &mut Vec<T>) -> bool
where
    T: PartialOrd,
{
    let mut pivot = v.len() - 1;
    loop {
        if pivot == 0 {
            return false;
        }
        if v[pivot] > v[pivot - 1] {
            pivot -= 1;
            break;
        }
        pivot -= 1;
    }
    let mut pivot_swap = v.len() - 1;
    while v[pivot] >= v[pivot_swap] {
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
    let mut buf_v = v.iter().copied().enumerate().collect::<Vec<_>>();
    buf_v.sort_unstable_by_key(|q| q.1);
    let mut prev_val = buf_v[0].1;
    let mut new_val = start;
    for (_i, x) in buf_v.iter_mut() {
        if prev_val == *x {
            *x = new_val;
        } else {
            new_val += step;
            prev_val = *x;
            *x = new_val;
        }
    }
    buf_v.sort_unstable();
    for (i, x) in v.iter_mut().enumerate() {
        *x = buf_v[i].1;
    }
}

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

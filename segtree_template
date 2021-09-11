use super::lazysegtree::*;
use super::segtree::*;

pub struct MaxAdd;
impl MapMonoid for MaxAdd {
    type M = Max<i64>;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &i64, &x: &i64) -> i64 {
        f + x
    }

    fn composition(&f: &i64, &g: &i64) -> i64 {
        f + g
    }
}

pub struct MinAdd;
impl MapMonoid for MinAdd {
    type M = Max<i64>;
    type F = i64;

    fn identity_map() -> Self::F {
        0
    }

    fn mapping(&f: &i64, &x: &i64) -> i64 {
        f + x
    }

    fn composition(&f: &i64, &g: &i64) -> i64 {
        f + g
    }
}

const INF: i64 = i64::max_value();

pub struct MaxForLazy;
impl Monoid for MaxForLazy {
    type S = i64;
    fn identity() -> Self::S {
        INF
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if *a == INF {
            *b
        } else if *b == INF {
            *a
        } else {
            std::cmp::max(*a, *b)
        }
    }
}

pub struct MinForLazy;
impl Monoid for MinForLazy {
    type S = i64;
    fn identity() -> Self::S {
        INF
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if *a == INF {
            *b
        } else if *b == INF {
            *a
        } else {
            std::cmp::min(*a, *b)
        }
    }
}

pub struct MaxUpdate;
impl MapMonoid for MaxUpdate {
    type M = MaxForLazy;
    type F = i64;

    fn identity_map() -> Self::F {
        INF
    }

    fn mapping(&f: &i64, &x: &i64) -> i64 {
        if f == INF {
            x
        } else {
            f
        }
    }

    fn composition(&f: &i64, &g: &i64) -> i64 {
        if f == INF {
            g
        } else {
            f
        }
    }
}

pub struct MinUpdate;
impl MapMonoid for MinUpdate {
    type M = MinForLazy;
    type F = i64;

    fn identity_map() -> Self::F {
        INF
    }

    fn mapping(&f: &i64, &x: &i64) -> i64 {
        if f == INF {
            x
        } else {
            f
        }
    }

    fn composition(&f: &i64, &g: &i64) -> i64 {
        if f == INF {
            g
        } else {
            f
        }
    }
}

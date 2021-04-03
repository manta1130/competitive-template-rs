use std::collections::VecDeque;
use std::default::Default;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;
    fn add(self, other: Point<T>) -> Point<T> {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Point<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;
    fn sub(self, other: Point<T>) -> Point<T> {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Point<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineStatus {
    Horizontal,
    Vertical,
    Normal,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line<T> {
    a: T,
    b: T,
    status: LineStatus,
}

impl<T> Line<T>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + PartialEq
        + Neg
        + Default,
{
    pub fn new(a: T, b: T) -> Line<T> {
        Line {
            a,
            b,
            status: LineStatus::Normal,
        }
    }

    #[allow(clippy::eq_op)]
    pub fn new_horizontal(y: T) -> Line<T> {
        Line {
            a: T::default(),
            b: y,
            status: LineStatus::Horizontal,
        }
    }

    #[allow(clippy::eq_op)]
    pub fn new_vertical(x: T) -> Line<T> {
        Line {
            a: T::default(),
            b: x,
            status: LineStatus::Vertical,
        }
    }

    pub fn from(l1: Point<T>, l2: Point<T>) -> Option<Line<T>> {
        if l1 == l2 {
            return None;
        }

        let (a, b, status);
        if l1.x == l2.x {
            a = T::default();
            b = l1.x;
            status = LineStatus::Vertical;
        } else if l1.y == l2.y {
            a = T::default();
            b = l1.y;
            status = LineStatus::Horizontal;
        } else {
            a = (l1.y - l2.y) / (l1.x - l2.x);
            b = l1.y - a * l1.x;
            status = LineStatus::Normal;
        }
        Some(Line { a, b, status })
    }

    pub fn from_point_slope(p: Point<T>, a: T, s: LineStatus) -> Line<T> {
        if s == LineStatus::Horizontal {
            Line::new_horizontal(p.y)
        } else if s == LineStatus::Vertical {
            Line::new_vertical(p.x)
        } else {
            Line::new(a, p.y - a * p.x)
        }
    }

    pub fn get_intersection(&self, other: Line<T>) -> Option<Point<T>> {
        if self.get_status() == LineStatus::Normal && other.get_status() == LineStatus::Normal {
            let x = (other.get_intercept() - self.get_intercept())
                / (self.get_slope() - other.get_slope());
            Some(Point {
                x,
                y: x * self.get_slope() + self.get_intercept(),
            })
        } else if self.status == other.status {
            None
        } else if self.is_horizontal() && other.is_vertical() {
            Some(Point {
                x: other.get_intercept(),
                y: self.get_intercept(),
            })
        } else if self.is_vertical() && other.is_horizontal() {
            Some(Point {
                y: other.get_intercept(),
                x: self.get_intercept(),
            })
        } else if self.is_horizontal() || self.is_vertical() {
            if self.is_horizontal() {
                let y = self.get_intercept();
                Some(Point {
                    x: other.substitution_y(y).unwrap(),
                    y,
                })
            } else {
                let x = self.get_intercept();
                Some(Point {
                    x,
                    y: other.substitution_x(x).unwrap(),
                })
            }
        } else if other.is_horizontal() {
            let y = other.get_intercept();
            Some(Point {
                x: self.substitution_y(y).unwrap(),
                y,
            })
        } else {
            let x = other.get_intercept();
            Some(Point {
                x,
                y: self.substitution_x(x).unwrap(),
            })
        }
    }

    pub fn get_slope(&self) -> T {
        self.a
    }

    pub fn get_intercept(&self) -> T {
        self.b
    }

    pub fn get_data(&self) -> (T, T) {
        (self.a, self.b)
    }

    pub fn is_horizontal(&self) -> bool {
        self.status == LineStatus::Horizontal
    }

    pub fn is_vertical(&self) -> bool {
        self.status == LineStatus::Vertical
    }

    pub fn get_status(&self) -> LineStatus {
        self.status
    }

    pub fn substitution_x(&self, x: T) -> Option<T> {
        if self.is_horizontal() {
            Some(self.get_intercept())
        } else if self.is_vertical() {
            None
        } else {
            Some(self.get_slope() * x + self.get_intercept())
        }
    }

    pub fn substitution_y(&self, y: T) -> Option<T> {
        if self.is_horizontal() {
            None
        } else if self.is_vertical() {
            Some(self.get_intercept())
        } else {
            Some((y - self.get_intercept()) / self.get_slope())
        }
    }
}

impl<T> Line<T>
where
    T: Copy
        + Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + PartialEq
        + Neg
        + Default
        + Into<f64>,
{
    pub fn get_perpendicular(&self, p: Point<T>) -> Line<f64> {
        Line::from_point_slope(
            Point::new(p.x.into(), p.y.into()),
            -(1.0 / self.get_slope().into()),
            match self.get_status() {
                LineStatus::Horizontal => LineStatus::Vertical,
                LineStatus::Vertical => LineStatus::Horizontal,
                LineStatus::Normal => LineStatus::Normal,
            },
        )
    }
}

pub fn cross<T>(p1: Point<T>, p2: Point<T>) -> T
where
    T: Sub<Output = T> + Mul<Output = T>,
{
    p1.x * p2.y - p1.y * p2.x
}

pub fn dot<T>(p1: Point<T>, p2: Point<T>) -> T
where
    T: Add<Output = T> + Mul<Output = T>,
{
    p1.x * p2.x + p1.y * p2.y
}

pub fn norm<T>(p: Point<T>) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    p.x * p.x + p.y * p.y
}

pub fn ccw<T>(a: Point<T>, mut b: Point<T>, mut c: Point<T>) -> isize
where
    T: SubAssign
        + Sub<Output = T>
        + Mul<Output = T>
        + Copy
        + PartialOrd
        + Add<Output = T>
        + Default,
{
    let zero = T::default();
    b -= a;
    c -= a;
    if cross(b, c) > zero {
        1
    } else if cross(b, c) < zero {
        -1
    } else if dot(b, c) < zero {
        2
    } else if norm(b) < norm(c) {
        -2
    } else {
        0
    }
}

pub fn is_parallel<T>(p11: Point<T>, p12: Point<T>, p21: Point<T>, p22: Point<T>) -> bool
where
    T: Sub<Output = T> + Mul<Output = T> + PartialEq,
{
    (p12.y - p11.y) * (p22.x - p21.x) == (p22.y - p21.y) * (p12.x - p11.x)
}

pub fn is_orthogonal<T>(p11: Point<T>, p12: Point<T>, p21: Point<T>, p22: Point<T>) -> bool
where
    T: Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + PartialEq,
{
    (p12.y - p11.y) * (p22.y - p21.y) == -(p22.x - p21.x) * (p12.x - p11.x)
}

pub fn arg_sort<T>(v: &mut [Point<T>], origin: Point<T>, start: Point<T>)
where
    T: Add<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + PartialOrd
        + Copy
        + Default,
{
    arg_sort_internal(v, origin, start, 0, v.len(), true);
}

#[allow(clippy::many_single_char_names)]
fn arg_sort_internal<T>(
    v: &mut [Point<T>],
    origin: Point<T>,
    pivot: Point<T>,
    f: usize,
    t: usize,
    flag: bool,
) where
    T: Add<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + PartialOrd
        + Copy
        + Default,
{
    if f == t {
        return;
    }
    let zero = T::default();

    let mut plus = vec![];
    let mut minus = vec![];
    let mut inv = vec![];
    let mut r = vec![];
    let mut origin_count = 0_usize;

    for &it in v.iter().take(t).skip(f) {
        let p = pivot - origin;
        let c = it - origin;
        let cx = cross(p, c);
        if it == origin {
            origin_count += 1;
        } else if zero < cx {
            plus.push(it);
        } else if zero > cx {
            minus.push(it);
        } else if dot(pivot - origin, it - origin) < zero {
            inv.push(it);
        } else {
            r.push(it);
        }
    }

    for _ in 0..origin_count {
        r.insert(0, origin);
    }

    let mut po = f;
    if flag {
        for &i in &r {
            v[po] = i;
            po += 1;
        }
        for &i in &plus {
            v[po] = i;
            po += 1;
        }
        for &i in &inv {
            v[po] = i;
            po += 1;
        }
        for &i in &minus {
            v[po] = i;
            po += 1;
        }
        let ps = f + r.len();
        let pe = f + r.len() + plus.len();
        let ms = f + r.len() + plus.len() + inv.len();
        let me = f + r.len() + plus.len() + inv.len() + minus.len();
        if ps != pe {
            arg_sort_internal(v, origin, v[ps], ps, pe, false);
        }
        if ms != me {
            arg_sort_internal(v, origin, v[ms], ms, me, false);
        }
    } else {
        for &i in &minus {
            v[po] = i;
            po += 1;
        }
        for &i in &r {
            v[po] = i;
            po += 1;
        }
        for &i in &plus {
            v[po] = i;
            po += 1;
        }
        for &i in &inv {
            v[po] = i;
            po += 1;
        }
        let ms = f;
        let me = f + minus.len();
        let ps = f + minus.len() + r.len();
        let pe = f + minus.len() + r.len() + plus.len();

        if ps != pe {
            arg_sort_internal(v, origin, v[ps], ps, pe, false);
        }
        if ms != me {
            arg_sort_internal(v, origin, v[ms], ms, me, false);
        }
    }
}

pub fn graham_scan<T>(list: &mut [Point<T>]) -> Vec<Point<T>>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + Ord
        + Copy
        + Default
        + From<i8>
        + std::fmt::Debug,
{
    if list.len() <= 2 {
        return Vec::new();
    }
    list.sort_by_key(|p| (p.y, p.x));
    let pivot = Point::new(list[0].x + 1_i8.into(), list[0].y);
    arg_sort(list, list[0], pivot);

    let mut list_dedup = vec![list[0], list[1]];
    let origin = list_dedup[0];
    for &p in list.iter().skip(2) {
        let top = *list_dedup.last().unwrap();
        if cross(top - origin, p - origin) != T::default() {
            list_dedup.push(p);
        } else if norm(top - origin) < norm(p - origin) {
            list_dedup.pop();
            list_dedup.push(p);
        }
    }

    let list = list_dedup;
    let mut stack = VecDeque::new();

    stack.push_front(list[0]);
    stack.push_front(list[1]);
    stack.push_front(list[2]);

    for &p in list.iter().skip(3) {
        while ccw(stack[1], stack[0], p) != 1 {
            stack.pop_front();
        }
        stack.push_front(p);
    }

    stack.iter().rev().cloned().collect::<Vec<_>>()
}

#[cfg(test)]
#[allow(clippy::clippy::vec_init_then_push)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_point() {
        let p1 = Point::new(3, 4);
        let p2 = p1;
        assert_eq!(p1, p2);
        let p2 = Point::new(3, 2);
        assert_ne!(p1, p2);
        let p2 = Point::new(1, 4);
        assert_ne!(p1, p2);

        let p1 = Point::new(3, -2);
        let p2 = Point::new(1, 1);
        let mut p3 = p1;
        let mut p4 = p1;
        p3 += p2;
        p4 -= p2;

        assert_eq!(p1 + p2, Point::new(4, -1));
        assert_eq!(p1 - p2, Point::new(2, -3));
        assert_eq!(p3, p1 + p2);
        assert_eq!(p4, p1 - p2);

        let p1 = Point::new(3.0, 4.0_f64);
        let p2 = p1;
        assert_eq!(p1, p2);
        let p2 = Point::new(3.0, 2.0);
        assert_ne!(p1, p2);
        let p2 = Point::new(1.0, 4.0);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_geometry_ccw() {
        assert_eq!(
            1,
            ccw(Point::new(1, 1), Point::new(2, 53), Point::new(-2, 21))
        );
        assert_eq!(
            -1,
            ccw(Point::new(1, 1), Point::new(-2, 21), Point::new(29, 2214))
        );
        assert_eq!(
            -2,
            ccw(
                Point::new(46, 46),
                Point::new(50, 50),
                Point::new(4002, 4002)
            )
        );
        assert_eq!(
            2,
            ccw(
                Point::new(46, 46),
                Point::new(-4423, -4423),
                Point::new(4002, 4002)
            )
        );
        assert_eq!(
            0,
            ccw(Point::new(46, 46), Point::new(46, 46), Point::new(46, 46))
        );
    }

    #[test]
    fn test_geometry_line() {
        assert_eq!(
            Line::new(1, 0),
            Line::from(Point::new(0, 0), Point::new(-1, -1)).unwrap()
        );
        assert_eq!(Line::new(2, 2), Line::new(2, 2));

        let a = Line::new(1_isize, 0);
        let b = Line::new_horizontal(3);
        let c = a.get_intersection(b).unwrap();
        assert_eq!(c, Point::new(3, 3));

        let a = Line::new_horizontal(7);
        let b = Line::new_horizontal(3);
        let c = a.get_intersection(b);
        assert_eq!(c, None);
    }

    #[test]
    fn test_geometry_parallel() {
        assert!(is_parallel(
            Point::new(0_i32, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3)
        ));
        assert!(!is_parallel(
            Point::new(0_i32, 8),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3)
        ));
    }

    #[test]
    fn test_geometry_orthogonal() {
        assert!(is_orthogonal(
            Point::new(0_i32, 0),
            Point::new(1, 1),
            Point::new(10, -10),
            Point::new(-3, 3)
        ));
        assert!(!is_orthogonal(
            Point::new(0_i32, 4),
            Point::new(1, 1),
            Point::new(10, -10),
            Point::new(-3, 3)
        ));
    }

    #[test]
    fn test_geometry_arg_sort() {
        let mut v = vec![];
        v.push(Point::new(-3, 63));
        v.push(Point::new(-3, 4));
        v.push(Point::new(1, 9128));
        v.push(Point::new(3, 1));
        v.push(Point::new(1000, 1));
        v.push(Point::new(3, 0));
        v.push(Point::new(0, -5));
        v.push(Point::new(0, 0));
        arg_sort(&mut v, Point::new(0, 0), Point::new(10, 0));

        let mut va = vec![];
        va.push(Point::new(0, 0));
        va.push(Point::new(3, 0));
        va.push(Point::new(1000, 1));
        va.push(Point::new(3, 1));
        va.push(Point::new(1, 9128));
        va.push(Point::new(-3, 63));
        va.push(Point::new(-3, 4));
        va.push(Point::new(0, -5));

        assert_eq!(v, va);
    }

    #[test]
    fn test_geometry_graham_scan() {
        let mut v = vec![];
        v.push(Point::new(-2_i32, 3));
        v.push(Point::new(-1, -2));
        v.push(Point::new(1, 1));
        v.push(Point::new(2, 5));
        v.push(Point::new(3, 2));
        v.push(Point::new(0, 3));
        v.push(Point::new(3, -3));
        v.push(Point::new(5, 3));
        let res = graham_scan(&mut v);

        let mut va = vec![];
        va.push(Point::new(3, -3));
        va.push(Point::new(5, 3));
        va.push(Point::new(2, 5));
        va.push(Point::new(-2, 3));
        va.push(Point::new(-1, -2));

        assert_eq!(va, res);
    }
}

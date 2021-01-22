use std::fmt;
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix<T> {
    v: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn new(h: usize, w: usize, init: T) -> Matrix<T> {
        Matrix {
            v: vec![vec![init; w]; h],
        }
    }

    pub fn from(v: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { v }
    }

    pub fn h(&self) -> usize {
        self.v.len()
    }

    pub fn w(&self) -> usize {
        self.v[0].len()
    }

    pub fn chrow(&mut self, a: usize, b: usize) {
        self.v.swap(a, b);
    }

    pub fn chcol(&mut self, a: usize, b: usize) {
        for i in 0..self.v.len() {
            self.v[i].swap(a, b);
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for l in &self.v {
            if !first {
                writeln!(f).unwrap();
            }
            first = false;
            for k in l {
                write!(f, "{:?} ", k).unwrap();
            }
        }
        write!(f, "")
    }
}

impl<T> Matrix<T>
where
    T: Copy + AddAssign + Sub<Output = T>,
{
    #[allow(clippy::eq_op)]
    pub fn t(&self) -> Matrix<T> {
        let mut r = Matrix::new(self.w(), self.h(), self[0][0] - self[0][0]);
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[j][i] = self[i][j];
            }
        }
        r
    }
}

impl<T> Matrix<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + MulAssign
        + Mul<Output = T>
        + AddAssign
        + Div<Output = T>,
{
    #[allow(clippy::eq_op)]
    pub fn pow(&self, mut p: u64, init: T) -> Matrix<T> {
        let mut r = Matrix::new(self.h(), self.w(), self[0][0] - self[0][0]);
        for i in 0..r.len() {
            r[i][i] = init;
        }
        let mut buf = self.clone();
        while p > 0 {
            if p & 1 == 1 {
                r = &r * &buf;
            }
            buf = &buf * &buf;
            p >>= 1;
        }
        r
    }
}

#[allow(clippy::eq_op)]
impl<T> Add for &Matrix<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + MulAssign,
{
    type Output = Matrix<T>;
    fn add(self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.h(), other.h());
        assert_eq!(self.w(), other.w());
        let mut r = Matrix::new(self.h(), self.w(), self.v[0][0] - self.v[0][0]);
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[i][j] = self[i][j] + other[i][j];
            }
        }
        r
    }
}

impl<T> Add<T> for &Matrix<T>
where
    T: Copy + AddAssign,
{
    type Output = Matrix<T>;
    fn add(self, other: T) -> Matrix<T> {
        let mut r = self.clone();
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[i][j] += other;
            }
        }
        r
    }
}

impl<T> AddAssign<T> for Matrix<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: T) {
        for i in 0..self.h() {
            for j in 0..self.w() {
                self[i][j] += other;
            }
        }
    }
}

#[allow(clippy::eq_op)]
impl<T> Sub for &Matrix<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Matrix<T>;
    fn sub(self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.h(), other.h());
        assert_eq!(self.w(), other.w());
        let mut r = Matrix::new(self.h(), self.w(), self.v[0][0] - self.v[0][0]);
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[i][j] = self[i][j] - other[i][j];
            }
        }
        r
    }
}

impl<T> Sub<T> for &Matrix<T>
where
    T: Copy + SubAssign,
{
    type Output = Matrix<T>;
    fn sub(self, other: T) -> Matrix<T> {
        let mut r = self.clone();
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[i][j] -= other;
            }
        }
        r
    }
}

impl<T> SubAssign<T> for Matrix<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: T) {
        for i in 0..self.h() {
            for j in 0..self.w() {
                self[i][j] -= other;
            }
        }
    }
}

#[allow(clippy::eq_op)]
impl<T> Mul for &Matrix<T>
where
    T: Copy + Sub<Output = T> + AddAssign + Mul<Output = T>,
{
    type Output = Matrix<T>;
    fn mul(self, other: &Matrix<T>) -> Matrix<T> {
        assert_eq!(self.w(), other.h());
        let mut r = Matrix::new(self.h(), other.w(), self.v[0][0] - self.v[0][0]);
        for i in 0..r.h() {
            for j in 0..r.w() {
                for q in 0..self.w() {
                    r[i][j] += self[i][q] * other[q][j];
                }
            }
        }
        r
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Copy + MulAssign,
{
    type Output = Matrix<T>;
    fn mul(self, other: T) -> Matrix<T> {
        let mut r = self.clone();
        for i in 0..self.h() {
            for j in 0..self.w() {
                r[i][j] *= other;
            }
        }
        r
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: Copy + MulAssign,
{
    fn mul_assign(&mut self, other: T) {
        for i in 0..self.h() {
            for j in 0..self.w() {
                self[i][j] *= other;
            }
        }
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Vec<Vec<T>> {
        &self.v
    }
}
impl<T> DerefMut for Matrix<T> {
    fn deref_mut(&mut self) -> &mut Vec<Vec<T>> {
        &mut self.v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_basic() {
        let m1 = Matrix::new(3, 2, 1);
        let mut m2 = Matrix::from(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        let m3 = Matrix::from(vec![vec![2, 3], vec![4, 5], vec![6, 7]]);
        assert_eq!(&m1 + &m2, m3);
        assert_eq!(&m2 + 1, m3);
        m2 += 1;
        assert_eq!(m2, m3);
        m2 -= 1;
        assert_eq!(&m2 + 1, m3);

        let m1 = Matrix::from(vec![vec![1, 2], vec![1, 2]]);
        let m2 = Matrix::from(vec![vec![1, -2], vec![1, 4]]);
        let m3 = Matrix::from(vec![vec![3, 6], vec![3, 6]]);
        assert_eq!(&m1 * &m2, m3);

        let m1 = Matrix::from(vec![vec![1, 2, 3], vec![4, -5, 6]]);
        let m2 = Matrix::from(vec![vec![1, 4], vec![2, -5], vec![3, 6]]);
        assert_eq!(m1.t(), m2);

        let mut m1 = Matrix::from(vec![vec![1, 2, 3], vec![4, -5, 6], vec![7, 8, 9]]);
        let m2 = Matrix::from(vec![vec![4, -5, 6], vec![1, 2, 3], vec![7, 8, 9]]);
        m1.chrow(0, 1);
        assert_eq!(m1, m2);

        let mut m1 = Matrix::from(vec![vec![1, 2, 3], vec![4, -5, 6], vec![7, 8, 9]]);
        let m2 = Matrix::from(vec![vec![2, 1, 3], vec![-5, 4, 6], vec![8, 7, 9]]);
        m1.chcol(0, 1);
        assert_eq!(m1, m2);
    }
}

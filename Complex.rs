use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, PartialEq, Debug, Eq, Copy, Default)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Neg<Output = T>,
{
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }

    pub fn conjugate(&self) -> Complex<T> {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn abs2(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T> Complex<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Neg<Output = T> + Into<f64>,
{
    pub fn abs(&self) -> f64 {
        self.abs2().into().sqrt()
    }

    pub fn arg(&self) -> f64 {
        (self.im.into()).atan2(self.re.into())
    }
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Debug + fmt::Display + Default + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < T::default() {
            write!(f, "{} - {}i", self.re, self.im)
        } else {
            write!(f, "{} + {}i", self.re, self.im)
        }
    }
}

impl<T> Add for Complex<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    type Output = Complex<T>;
    fn add(self, other: Complex<T>) -> Complex<T> {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl<T> AddAssign for Complex<T>
where
    T: Copy + AddAssign,
{
    fn add_assign(&mut self, other: Complex<T>) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl<T> Sub for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    type Output = Complex<T>;
    fn sub(self, other: Complex<T>) -> Complex<T> {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

impl<T> SubAssign for Complex<T>
where
    T: Copy + SubAssign,
{
    fn sub_assign(&mut self, other: Complex<T>) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl<T> Mul for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    type Output = Complex<T>;
    fn mul(self, other: Complex<T>) -> Complex<T> {
        Complex::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg<Output = T>,
{
    fn mul_assign(&mut self, other: Complex<T>) {
        *self = *self * other;
    }
}

impl<T> Div for Complex<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>,
{
    type Output = Complex<T>;
    fn div(self, other: Complex<T>) -> Complex<T> {
        let denominator = (other * other.conjugate()).re;
        let numerator = self * other.conjugate();
        Complex {
            re: numerator.re / denominator,
            im: numerator.im / denominator,
        }
    }
}

impl<T> DivAssign for Complex<T>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>,
{
    fn div_assign(&mut self, other: Complex<T>) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_basic() {
        let a = Complex::new(5_i32, 6);
        let b = Complex::new(7, 3);
        let plus = Complex::new(12, 9);
        let minus = Complex::new(-2, 3);
        let mul = Complex::new(17, 57);

        assert_eq!(a + b, plus);
        assert_eq!(a - b, minus);
        assert_eq!(a * b, mul);
        assert_eq!(mul / a, b);
    }
}

use std::convert::{From, Into, AsRef};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::{PartialEq, PartialOrd, Ordering};

const EPSILON: f64 = 0.00001;

#[derive(Clone, Copy, Debug)]
pub struct Float(f64);

impl Float {
    pub fn new<S: Into<f64>>(value: S) -> Float {
        return Float(value.into());
    }
    pub fn sqrt(&self) -> Float {
        return Float(self.0.sqrt());
    }
    pub fn round(&self) -> f64 {
        return self.0.round();
    }
    pub fn powi(&self, value: i32) -> Float {
        return Float(self.0.powi(value));
    }
}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        return Float(value);
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        return value.0;
    }
}

impl From<&Float> for f64 {
    fn from(value: &Float) -> Self {
        return value.0;
    }
}

impl From<i32> for Float {
    fn from(value: i32) -> Self {
        return Float(value as f64);
    }
}

impl AsRef<Float> for Float {
    fn as_ref(&self) -> &Float {
        return self;
    }
}

impl PartialEq<f64> for Float {
    fn eq(&self, other: &f64) -> bool {
        return (self.0 - other).abs() < EPSILON;
    }
    fn ne(&self, other: &f64) -> bool {
        return !self.eq(other);
    }
}

impl PartialEq<i32> for Float {
    fn eq(&self, other: &i32) -> bool {
        return (self.0 - (*other as f64)).abs() < EPSILON;
    }
    fn ne(&self, other: &i32) -> bool {
        return !self.eq(other);
    }
}

impl PartialEq<Float> for Float {
    fn eq(&self, other: &Float) -> bool {
        return self.eq(&other.0);
    }
    fn ne(&self, other: &Float) -> bool {
        return self.ne(&other.0);
    }
}

impl PartialOrd<f64> for Float {
    fn ge(&self, other: &f64) -> bool {
        return self == other || self.0 > *other;
    }
    fn gt(&self, other: &f64) -> bool {
        return self != other && self.0 > *other;
    }
    fn le(&self, other: &f64) -> bool {
        return self == other || self.0 < *other;
    }
    fn lt(&self, other: &f64) -> bool {
        return self != other && self.0 < *other;
    }
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.0 < *other {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }
}

impl PartialOrd<Float> for Float {
    fn ge(&self, other: &Float) -> bool {
        return self.ge(&other.0);
    }
    fn gt(&self, other: &Float) -> bool {
        return self.gt(&other.0);
    }
    fn le(&self, other: &Float) -> bool {
        return self.le(&other.0);
    }
    fn lt(&self, other: &Float) -> bool {
        return self.lt(&other.0);
    }
    fn partial_cmp(&self, other: &Float) -> Option<std::cmp::Ordering> {
        return self.partial_cmp(&other.0);
    }
}

impl PartialOrd<i32> for Float {
    fn ge(&self, other: &i32) -> bool {
        return self.ge(&(*other as f64));
    }
    fn gt(&self, other: &i32) -> bool {
        return self.gt(&(*other as f64));
    }
    fn le(&self, other: &i32) -> bool {
        return self.le(&(*other as f64));
    }
    fn lt(&self, other: &i32) -> bool {
        return self.lt(&(*other as f64));
    }
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        return self.partial_cmp(&(*other as f64));
    }
}

impl <S: Into<f64>> Add<S> for Float {
    type Output = Float;
    fn add(self, rhs: S) -> Self::Output {
        return Float(self.0 + rhs.into());
    }
}

impl <S: Into<f64>> Sub<S> for Float {
    type Output = Float;
    fn sub(self, rhs: S) -> Self::Output {
        return Float(self.0 - rhs.into());
    }
}

impl <S: Into<f64>> Mul<S> for Float {
    type Output = Float;
    fn mul(self, rhs: S) -> Self::Output {
        return Float(self.0 * rhs.into());
    }
}

impl <S: Into<f64>> Div<S> for Float {
    type Output = Float;
    fn div(self, rhs: S) -> Self::Output {
        return Float(self.0 / rhs.into());
    }
}

impl <S: Into<f64>> AddAssign<S> for Float {
    fn add_assign(&mut self, rhs: S) {
        self.0 += rhs.into();
    }
}

impl <S: Into<f64>> SubAssign<S> for Float {
    fn sub_assign(&mut self, rhs: S) {
        self.0 -= rhs.into();
    }
}

impl <S: Into<f64>> MulAssign<S> for Float {
    fn mul_assign(&mut self, rhs: S) {
        self.0 *= rhs.into();
    }
}

impl <S: Into<f64>> DivAssign<S> for Float {
    fn div_assign(&mut self, rhs: S) {
        self.0 /= rhs.into();
    }
}

impl Neg for Float {
    type Output = Float;
    fn neg(self) -> Float {
        return Float(-self.0);
    }
}
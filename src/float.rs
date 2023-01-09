use std::convert::{From, Into, AsRef};
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::{PartialEq, PartialOrd, Ordering};

const EPSILON: f64 = 0.00001;

pub trait F64IsAbout {
    fn equals(&self, peer: f64) -> bool;
}

impl F64IsAbout for f64 {
    fn equals(self: &f64, peer: f64) -> bool {
        return (self - peer).abs() < EPSILON;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MyFloat(f64);

impl MyFloat {
    pub fn new<S: Into<f64>>(value: S) -> MyFloat {
        return MyFloat(value.into());
    }
    pub fn sqrt(&self) -> MyFloat {
        return MyFloat(self.0.sqrt());
    }
}

impl From<f64> for MyFloat {
    fn from(value: f64) -> Self {
        return MyFloat(value);
    }
}

impl From<MyFloat> for f64 {
    fn from(value: MyFloat) -> Self {
        return value.0;
    }
}

impl From<&MyFloat> for f64 {
    fn from(value: &MyFloat) -> Self {
        return value.0;
    }
}

impl From<i32> for MyFloat {
    fn from(value: i32) -> Self {
        return MyFloat(value as f64);
    }
}

impl AsRef<MyFloat> for MyFloat {
    fn as_ref(&self) -> &MyFloat {
        return self;
    }
}

impl PartialEq<f64> for MyFloat {
    fn eq(&self, other: &f64) -> bool {
        return (self.0 - other).abs() < EPSILON;
    }
    fn ne(&self, other: &f64) -> bool {
        return !self.eq(other);
    }
}

impl PartialEq<i32> for MyFloat {
    fn eq(&self, other: &i32) -> bool {
        return (self.0 - (*other as f64)).abs() < EPSILON;
    }
    fn ne(&self, other: &i32) -> bool {
        return !self.eq(other);
    }
}

impl PartialEq<MyFloat> for MyFloat {
    fn eq(&self, other: &MyFloat) -> bool {
        return self.eq(&other.0);
    }
    fn ne(&self, other: &MyFloat) -> bool {
        return self.ne(&other.0);
    }
}

impl PartialOrd<f64> for MyFloat {
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

impl PartialOrd<MyFloat> for MyFloat {
    fn ge(&self, other: &MyFloat) -> bool {
        return self.ge(&other.0);
    }
    fn gt(&self, other: &MyFloat) -> bool {
        return self.gt(&other.0);
    }
    fn le(&self, other: &MyFloat) -> bool {
        return self.le(&other.0);
    }
    fn lt(&self, other: &MyFloat) -> bool {
        return self.lt(&other.0);
    }
    fn partial_cmp(&self, other: &MyFloat) -> Option<std::cmp::Ordering> {
        return self.partial_cmp(&other.0);
    }
}

impl PartialOrd<i32> for MyFloat {
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

impl <S: Into<f64>> Add<S> for MyFloat {
    type Output = MyFloat;
    fn add(self, rhs: S) -> Self::Output {
        return MyFloat(self.0 + rhs.into());
    }
}

impl <S: Into<f64>> Sub<S> for MyFloat {
    type Output = MyFloat;
    fn sub(self, rhs: S) -> Self::Output {
        return MyFloat(self.0 - rhs.into());
    }
}

impl <S: Into<f64>> Mul<S> for MyFloat {
    type Output = MyFloat;
    fn mul(self, rhs: S) -> Self::Output {
        return MyFloat(self.0 * rhs.into());
    }
}

impl <S: Into<f64>> Div<S> for MyFloat {
    type Output = MyFloat;
    fn div(self, rhs: S) -> Self::Output {
        return MyFloat(self.0 / rhs.into());
    }
}

impl <S: Into<f64>> AddAssign<S> for MyFloat {
    fn add_assign(&mut self, rhs: S) {
        self.0 += rhs.into();
    }
}

impl <S: Into<f64>> SubAssign<S> for MyFloat {
    fn sub_assign(&mut self, rhs: S) {
        self.0 -= rhs.into();
    }
}

impl <S: Into<f64>> MulAssign<S> for MyFloat {
    fn mul_assign(&mut self, rhs: S) {
        self.0 *= rhs.into();
    }
}

impl <S: Into<f64>> DivAssign<S> for MyFloat {
    fn div_assign(&mut self, rhs: S) {
        self.0 /= rhs.into();
    }
}

impl Neg for MyFloat {
    type Output = MyFloat;
    fn neg(self) -> MyFloat {
        return MyFloat(-self.0);
    }
}
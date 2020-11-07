use std::fmt;
use std::ops;

use crate::core::geometry::vector::Vector;
use crate::core::math::rand::{rand, rand_between};
use crate::core::optic::{Reflect, Refract};

type Scalar = f64;

#[derive(Copy, Clone)]
pub struct Vec3(pub Scalar, pub Scalar, pub Scalar);

impl Vec3 {
    pub fn rand() -> Self {
        Self(rand(), rand(), rand())
    }

    pub fn rand_between(min: Scalar, max: Scalar) -> Self {
        Self(
            rand_between(min, max),
            rand_between(min, max),
            rand_between(min, max),
        )
    }

    pub fn x(&self) -> Scalar {
        self.0
    }

    pub fn y(&self) -> Scalar {
        self.1
    }

    pub fn z(&self) -> Scalar {
        self.2
    }
}

impl Vector for Vec3 {
    type Scalar = Scalar;

    fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    fn length(&self) -> Self::Scalar {
        self.sq_length().sqrt()
    }

    fn sq_length(&self) -> Self::Scalar {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }

    fn normalize(&mut self) -> &mut Self {
        let norm = self.length();
        *self = Self(self.0 / norm, self.1 / norm, self.2 / norm);

        self
    }

    fn normalized(&self) -> Self {
        let norm = self.length();

        Self(self.0 / norm, self.1 / norm, self.2 / norm)
    }

    fn dot(&self, v: Self) -> Self::Scalar {
        (self.0 * v.0) + (self.1 * v.1) + (self.2 * v.2)
    }

    fn cross(&self, v: Self) -> Self {
        Self(
            (self.1 * v.2) - (self.2 * v.1),
            (self.2 * v.0) - (self.0 * v.2),
            (self.0 * v.1) - (self.1 * v.0),
        )
    }
}

impl Reflect for Vec3 {
    fn reflect(&self, normal: Self) -> Self {
        *self - 2.0 * self.dot(normal) * (normal)
    }
}

impl Refract for Vec3 {
    type Scalar = Scalar;

    fn refract(&self, normal: Self, eta_in_over_eta_out: Self::Scalar) -> Vec3 {
        let cos_theta: f64 = (-(*self)).dot(normal);

        let vec_out_perp: Self = eta_in_over_eta_out * ((*self) + cos_theta * normal);
        let vec_out_par: Self = -(1.0 - vec_out_perp.sq_length()).abs().sqrt() * normal;

        vec_out_perp + vec_out_par
    }
}

// Addition operators
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

// Subtraction operators
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

// Unary negation
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

// Multiplication operators
impl ops::Mul<Scalar> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: Scalar) -> Self::Output {
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl ops::MulAssign<Scalar> for Vec3 {
    fn mul_assign(&mut self, scalar: Scalar) {
        *self = Self(self.0 * scalar, self.1 * scalar, self.2 * scalar);
    }
}

// Right hand scalar multiplication operator
impl ops::Mul<Vec3> for Scalar {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3(vec.0 * self, vec.1 * self, vec.2 * self)
    }
}

// Division operators
impl ops::Div<Scalar> for Vec3 {
    type Output = Self;

    fn div(self, scalar: Scalar) -> Self::Output {
        Self(self.0 / scalar, self.1 / scalar, self.2 / scalar)
    }
}

impl ops::DivAssign<Scalar> for Vec3 {
    fn div_assign(&mut self, scalar: Scalar) {
        *self = Self(self.0 / scalar, self.1 / scalar, self.2 / scalar);
    }
}

// Indexing operators
impl ops::Index<usize> for Vec3 {
    type Output = Scalar;

    fn index(&self, other: usize) -> &Scalar {
        match other {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index {} is not in Vec3", other),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, other: usize) -> &mut Scalar {
        match other {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index {} is not in Vec3", other),
        }
    }
}

// Format string
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

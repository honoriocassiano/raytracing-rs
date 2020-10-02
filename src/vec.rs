use std::ops;
use std::fmt;
use crate::util::{rand, rand_between};

type Scalar = f64;


#[derive(Copy, Clone)]
pub struct Vec3 (pub Scalar, pub Scalar, pub Scalar);


impl Vec3 {

	pub fn zero() -> Self {
		Self(0.0, 0.0, 0.0)
	}

	pub fn rand() -> Self {
		Self(rand(), rand(), rand())
	}

	pub fn rand_between(min: Scalar, max: Scalar) -> Self {
		Self(rand_between(min, max), rand_between(min, max), rand_between(min, max))
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

	pub fn length(&self) -> Scalar {
		self.sq_length().sqrt()
	}

	pub fn sq_length(&self) -> Scalar {
		(self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
	}

	pub fn normalize(&mut self) -> &mut Self {
		let norm = self.length();
		*self = Self (
			self.0 / norm,
			self.1 / norm,
			self.2 / norm
		);

		self
	}

	pub fn normalized(&self) -> Self {
		let norm = self.length();

		Self (self.0 / norm, self.1 / norm, self.2 / norm)
	}
}


// Addition operators
impl ops::Add for Vec3 {

	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self (
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2
		)
	}
}


impl ops::AddAssign for Vec3 {

	fn add_assign(&mut self, other: Self) {
	
		*self = Self (
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2
		);
	}
}


// Subtraction operators
impl ops::Sub for Vec3 {

	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self (
			self.0 - other.0,
			self.1 - other.1,
			self.2 - other.2
		)
	}
}


impl ops::SubAssign for Vec3 {

	fn sub_assign(&mut self, other: Self) {
	
		*self = Self (
			self.0 - other.0,
			self.1 - other.1,
			self.2 - other.2
		);
	}
}


// Unary negation
impl ops::Neg for Vec3 {

	type Output = Self;

	fn neg(self) -> Self::Output {
		Self (
			-self.0,
			-self.1,
			-self.2
		)
	}
}


// Multiplication operators
impl ops::Mul<Scalar> for Vec3 {

	type Output = Self;

	fn mul(self, scalar: Scalar) -> Self::Output {
		Self (
			self.0 * scalar,
			self.1 * scalar,
			self.2 * scalar
		)
	}
}


impl ops::Mul<Vec3> for Vec3 {

	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self (
			self.0 * other.0,
			self.1 * other.1,
			self.2 * other.2
		)
	}
}


impl ops::MulAssign<Scalar> for Vec3 {

	fn mul_assign(&mut self, scalar: Scalar) {
	
		*self = Self (
			self.0 * scalar,
			self.1 * scalar,
			self.2 * scalar
		);
	}
}


// Right hand scalar multiplication operator
impl ops::Mul<Vec3> for Scalar {

	type Output = Vec3;

	fn mul(self, vec: Vec3) -> Self::Output {
		Vec3 (
			vec.0 * self,
			vec.1 * self,
			vec.2 * self
		)
	}
}


// Division operators
impl ops::Div<Scalar> for Vec3 {

	type Output = Self;

	fn div(self, scalar: Scalar) -> Self::Output {
		Self (
			self.0 / scalar,
			self.1 / scalar,
			self.2 / scalar
		)
	}
}


impl ops::DivAssign<Scalar> for Vec3 {

	fn div_assign(&mut self, scalar: Scalar) {
	
		*self = Self (
			self.0 / scalar,
			self.1 / scalar,
			self.2 / scalar
		);
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
			_ => panic!("Index {} is not in Vec3", other)
		}
	}
}


impl ops::IndexMut<usize> for Vec3 {

	fn index_mut(&mut self, other: usize) -> &mut Scalar {
		match other {
			0 => &mut self.0,
			1 => &mut self.1,
			2 => &mut self.2,
			_ => panic!("Index {} is not in Vec3", other)
		}
	}
}


// Format string
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}


// Utilitary functions
pub fn dot(u: &Vec3, v: &Vec3) -> Scalar {
	(u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
}


pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3 (
		(u.1 * v.2) - (u.2 * v.1),
		(u.2 * v.0) - (u.0 * v.2),
		(u.0 * v.1) - (u.1 * v.0)
	)
}

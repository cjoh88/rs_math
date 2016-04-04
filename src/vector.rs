use traits::{Zero, One};
use std::ops::{Add, Sub, Mul, Div};


pub struct VectorN<T> {
	v: Vec<T>,
}

impl<T: Zero + Copy> VectorN<T> {
	pub fn new(v: &[T]) -> Self {
		VectorN {
			v: v.to_vec(),
		}
	}

	pub fn new_zero(size: usize) -> Self {
		VectorN {
			v: vec![T::zero(); size],
		}
	}

	pub fn len(&self) -> usize {
		self.v.len()
	}
}

impl<T: Add<Output=T> + Copy> VectorN<T> {
	pub fn add(&mut self, other: &VectorN<T>) {
		assert!(self.v.len() == other.v.len());
		for (x, y) in self.v.iter_mut().zip(other.v.iter()) {
			*x = *x + *y;
		}
	}
}

impl<T: Sub<Output=T> + Copy> VectorN<T> {
	pub fn sub(&mut self, other: &VectorN<T>) {
		assert!(self.v.len() == other.v.len());
		for (x, y) in self.v.iter_mut().zip(other.v.iter()) {
			*x = *x - *y;
		}
	}
}

impl<T: Mul<Output=T> + Add<Output=T> + Copy + Zero> VectorN<T> {
	pub fn scalar(&mut self, other: T) {
		for x in self.v.iter_mut() {
			*x = *x * other;
		}
	}

	pub fn dot(&self, other: &VectorN<T>) -> T {
		assert!(self.v.len() == other.v.len());
		let mut result = T::zero();
		for (x, y) in self.v.iter().zip(other.v.iter()) {
			result = result + *x * *y;
		}
		result
	}
}


/*
#[derive(Copy, Clone)]
pub struct Vector2<T> {
	x: T,
	y: T,
}

impl<T: Copy> Vector2<T> {
	pub fn new(x: T, y: T) -> Self {
		Vector2 {
			x: x,
			y: y,
		}
	}
}

impl<T: Copy + Add<Output=T> + Sub<Output=T> + Mul<Output=T>> Vector2<T> {
	pub fn add(&mut self, other: &Vector2<T>) {
		self.x = self.x + other.x;
		self.y = self.y + other.y;
	}
	
	pub fn sub(&mut self, other: &Vector2<T>) {
		self.x = self.x - other.x;
		self.y = self.y - other.y;
	}

	pub fn scalar(&mut self, other: T) {
		self.x = self.x * other;
		self.y = self.y * other;
	}

	pub fn dot(&self, other: &Vector2<T>) -> T {
		self.x * other.x + self.y * other.y
	}

	pub fn cross(&self, other: &Vector2<T>) {
		unimplemented!();
	}
}

impl<T: Add<Output = T> + Copy> Add for Vector2<T> {
	type Output = Vector2<T>;
	fn add(self, other: Vector2<T>) -> Self {
		Vector2 {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl<T: Sub<Output = T> + Copy> Sub for Vector2<T> {
	type Output = Vector2<T>;
	fn sub(self, other: Vector2<T>) -> Self {
		Vector2 {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}
}

impl<T: Mul<Output = T> + Copy> Mul for Vector2<T> {
	type Output = T;
	fn mul(self, other: Vector2<T>) -> T {
		self.x * other.c + self.y * other.y
	}
}








pub struct Vector3<T> {
	x: T,
	y: T,
	z: T,
}




*/
#[macro_use]

use traits::{Zero, One};
use std::ops::{Index, IndexMut, Add, Sub, Mul, Div};
use std::cmp::{PartialEq};
use std::slice::{Iter, IterMut};

pub struct Matrix<T> {
	nrows: usize,
	ncols: usize,
	v: Vec<T>,
}

impl<T> Matrix<T> where T:
	Copy +
	Zero +
	One +
	Add<Output = T> +
	Sub<Output = T> +
	Mul<Output = T> +
	Div<Output = T> +
	PartialEq {

	pub fn new(nrows: usize, ncols: usize, values: &[T]) -> Self {
		let v = values.to_vec();
		assert!(v.len() == nrows * ncols);
		Matrix {
			nrows: nrows,
			ncols: ncols,
			v: v
		}
	}

	pub fn new_zero(nrows: usize, ncols: usize) -> Self {
		Matrix {
			nrows: nrows,
			ncols: ncols,
			v: vec![T::zero(); nrows * ncols],
		}
	}

	pub fn new_identity(n: usize) -> Self {
		Matrix {
			nrows: n,
			ncols: n,
			v: {
				let mut v = Vec::with_capacity(n*n);
				for i in 0..(n*n) {
					if (i % (n+1)) == 0 {
						v.push(T::one());
					}
					else {
						v.push(T::zero());
					}
				}
				v
			}
		}
	}

	pub fn size(&self) -> (usize, usize) {
		(self.nrows, self.ncols)
	}

	pub fn nrows(&self) -> usize {
		self.nrows
	}

	pub fn ncols(&self) -> usize {
		self.ncols
	}

	pub fn add(&mut self, other: &Matrix<T>) {
		assert!(self.size() == other.size());
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x + other.v[i]; 
		}
	}

	pub fn sub(&mut self, other: &Matrix<T>) {
		assert!(self.size() == other.size());
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x - other.v[i]; 
		}
	}

	pub fn scalar(&mut self, other: T) {
		for x in self.v.iter_mut() {
			*x = *x * other;
		}
	}

	pub fn hadamard(&mut self, other: &Matrix<T>) {
		assert!(self.size() == other.size());
		for (x, y) in self.v.iter_mut().zip(other.v.iter()) {
			*x = *x * *y;
		}
	}

	pub fn kronecker(&self, other: &Matrix<T>) -> Self {
		let nrows = self.nrows * other.nrows;
		let ncols = self.ncols * other.ncols;
		Matrix {
			nrows: nrows,
			ncols: ncols,
			v: {
				let mut v = Vec::with_capacity(nrows * ncols);
				unsafe {v.set_len(nrows * ncols);}
				for sr in 0..self.nrows {
					for sc in 0..self.ncols {
						for or in 0..other.nrows {
							for oc in 0..other.ncols {
								let a = self[(sr,sc)] * other[(or, oc)];
								let i = sr * other.nrows + or;
								let j = sc * other.ncols + oc;
								v[i * ncols + j] = a;
							}
						}
					}
				}
				v
			}
		}
	}

	pub fn vercat(&mut self, other: &Matrix<T>) {
		assert!(self.ncols == other.ncols);
		self.nrows = self.nrows + other.nrows;
		for x in other.v.iter() {
			self.v.push(*x);
		}
	}

	pub fn horcat(&mut self, other: &Matrix<T>) {
		assert!(self.nrows == other.nrows);
		let mut v: Vec<T> = Vec::with_capacity(self.v.len() + other.v.len());
		for i in 0..self.nrows {
			for j in 0..self.ncols {
				v.push(self[(i,j)]);
			}
			for j in 0..other.ncols {
				v.push(other[(i,j)]);
			}
		}
		self.v = v;
	}
	
	pub fn transpose(&mut self) {
		let mut v = Vec::with_capacity(self.nrows * self.ncols);
		let ncols = self.ncols;
		let nrows = self.nrows;
		for col in 0..ncols {
			for row in 0..nrows {
				v.push(self.v[row * self.ncols + col]);
			}
		}
		self.v = v;
		self.ncols = nrows;
		self.nrows = ncols;
	}

	pub fn equals(&self, other: &Matrix<T>) -> bool {
		if self.size() != other.size() {
			return false;
		} 
		for (x, y) in self.v.iter().zip(other.v.iter()) {
			if *x != *y {
				return false;
			}
		}
		return true
	}

	pub fn is_square(&self) -> bool {
		self.nrows == self.ncols
	}

	pub fn iter(&self) -> Iter<T> {
		self.v.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.v.iter_mut()
	}

}

impl<T> Index<(usize, usize)> for Matrix<T> {
	type Output = T;

	fn index<'a>(&'a self, index: (usize, usize)) -> &'a T {
		&self.v[index.0 * self.ncols + index.1]
	}
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
	fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut T {
		&mut self.v[index.0 * self.ncols + index.1]
	}
}

impl<T: Clone> Clone for Matrix<T> {
	fn clone(&self) -> Self {
		Matrix {
			nrows: self.nrows,
			ncols: self.ncols,
			v: {
				let mut v = Vec::with_capacity(self.v.len());
				for x in self.v.iter() {
					v.push(x.clone());
				}
				v
			}
		}
	}
}


#[macro_export]
macro_rules! matrix {
    ($($($x:expr),+);+) => {
    	//let mut v = Vec::new();
    	{
    		let mut temp_vec = Vec::new();
    		let mut rows = 0;
    		let mut cols = 0;
 			$(
 				{
 					rows += 1;
 					let mut temp_cols = 0;
 					$(
 						temp_cols += 1;
 						temp_vec.push($x);
 						//println!("{}", $x);
 					)+
 					if cols != 0 && temp_cols != cols {
 						panic!("ERROR!");
 					}
 					cols = temp_cols;
 				}
 			)+
 			println!("Rows: {}\nCols: {}", rows, cols);
 			Matrix::new(rows, cols, temp_vec.as_slice())
 		}	
    }
}

pub struct Vector<T> {
	v: Vec<T>,
}

impl<T> Vector<T> where T:
	Copy +
	Zero +
	One +
	Add<Output = T> +
	Sub<Output = T> +
	Mul<Output = T> +
	Div<Output = T> +
	PartialEq {

	pub fn new(values: &[T]) -> Self {
		Vector {
			v: values.to_vec(),
		}
	}

	pub fn new_zero(nrows: usize, ncols: usize) -> Self {
		Vector {
			v: vec![T::zero(); nrows * ncols],
		}
	}

	pub fn size(&self) -> usize {
		self.v.len()
	}

	pub fn add(&mut self, other: &Vector<T>) {
		assert!(self.size() == other.size());
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x + other.v[i]; 
		}
	}

	pub fn sub(&mut self, other: &Vector<T>) {
		assert!(self.size() == other.size());
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x - other.v[i]; 
		}
	}

	pub fn scalar(&mut self, other: T) {
		for x in self.v.iter_mut() {
			*x = *x * other;
		}
	}

	pub fn equals(&self, other: &Vector<T>) -> bool {
		if self.size() != other.size() {
			return false;
		} 
		for (x, y) in self.v.iter().zip(other.v.iter()) {
			if *x != *y {
				return false;
			}
		}
		return true
	}	

	pub fn iter(&self) -> Iter<T> {
		self.v.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<T> {
		self.v.iter_mut()
	}

	pub fn to_row_matrix(self) -> Matrix<T> {
		Matrix {
			nrows: 1,
			ncols: self.v.len(),
			v: self.v,
		}
	}

	pub fn to_col_matrix(self) -> Matrix<T> {
		Matrix {
			nrows: self.v.len(),
			ncols: 1,
			v: self.v,
		}
	}

}

impl<T> Index<usize> for Vector<T> {
	type Output = T;

	fn index<'a>(&'a self, index: usize) -> &'a T {
		&self.v[index]
	}
}

impl<T> IndexMut<usize> for Vector<T> {
	fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
		&mut self.v[index]
	}
}

impl<T: Clone> Clone for Vector<T> {
	fn clone(&self) -> Self {
		Vector {
			v: {
				let mut v = Vec::with_capacity(self.v.len());
				for x in self.v.iter() {
					v.push(x.clone());
				}
				v
			}
		}
	}
}


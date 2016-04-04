use traits::{Zero, One};
use std::ops::{Index, IndexMut, Add, Sub, Mul};
use std::slice::{Iter};
use vector::VectorN;

/*TODO 
Vector multiplication
Inverse
MulAssign
Scalar division
Matrix division
Vector division
LUP factorization
Submatrix
Determinant
Map
Inverse
Trace
DivAssign
Concatenation
Eigen values
Eigen vectors
Iterator
RowIterator
ColIterator
*/
/*DONE
Scalar multiplication
Matrix multiplication
Add
Sub
AddAssign
SubAssign
transposition
Hadamard multiplication
Kronecker multiplication
horcat
vercat
*/

pub struct Matrix<T> {
	nrows: usize,
	ncols: usize,
	v: Vec<T>
}

impl<T: Copy> Matrix<T> {
	pub fn size(&self) -> (usize, usize) {
		(self.nrows, self.ncols)
	}

	pub fn nrows(&self) -> usize {
		self.nrows
	}

	pub fn ncols(&self) -> usize {
		self.ncols
	}
	//&self.v[index.0 * self.ncols + index.1]
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

	//TODO Should 'other' be by value?
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
}

impl<T: Zero + One + Clone> Matrix<T> {
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
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Zero> Matrix<T> {
	pub fn add(&mut self, other: &Matrix<T>) {
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x + other.v[i]; 
		}
	}

	pub fn sub(&mut self, other: &Matrix<T>) {
		for (i,x) in self.v.iter_mut().enumerate() {
			*x = *x - other.v[i]; 
		}
	}

	pub fn scalar(&mut self, other: T) {
		for x in self.v.iter_mut() {
			*x = *x * other;
		}
	}

	pub fn mul_vector(&mut self, other: &VectorN<T>) {
		assert!(self.nrows == other.size());
		for i in 0..self.nrows {
			for j in 0..self.ncols {
				self[(i,j)] = self[(i,j)] * other[i]
			}
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

impl<'a, 'b, T: Add<Output = T> + Copy> Add<&'b Matrix<T>> for &'a Matrix<T> {
	type Output = Matrix<T>;
	fn add(self, other: &Matrix<T>) -> Matrix<T> {
		assert!(self.size() == other.size());
		Matrix {
			nrows: self.nrows,
			ncols: self.ncols,
			v: {
				let mut v = Vec::with_capacity(self.nrows * self.ncols);
				for i in 0..(self.nrows * self.ncols) {
					v.push(self.v[i] + other.v[i]);
				}
				v
			}
		}
	}
}

impl<'a, 'b, T: Sub<Output = T> + Copy> Sub<&'b Matrix<T>> for &'a Matrix<T> {
	type Output = Matrix<T>;
	fn sub(self, other: &Matrix<T>) -> Matrix<T> {
		assert!(self.size() == other.size());
		Matrix {
			nrows: self.nrows,
			ncols: self.ncols,
			v: {
				let mut v = Vec::with_capacity(self.nrows * self.ncols);
				for i in 0..(self.nrows * self.ncols) {
					v.push(self.v[i] - other.v[i]);
				}
				v
			}
		}
	}
}

impl<'a, 'b, T: Mul<Output = T> + Add<Output = T> + Zero + Copy> Mul<&'b Matrix<T>> for &'a Matrix<T> {
	type Output = Matrix<T>;
	fn mul(self, other: &Matrix<T>) -> Matrix<T> {
		assert!(self.ncols() == other.nrows());
		Matrix {
			nrows: self.nrows,
			ncols: other.ncols,
			v: {
				let mut v = Vec::with_capacity(self.nrows * other.ncols);
				for i in 0..self.nrows {
					for j in 0..other.ncols {
						let mut r = T::zero();
						for k in 0..self.ncols {
							r = r + (self[(i,k)] * other[(k, j)]);
						}
						v.push(r);
					}
				}
				v
			}
		}
	}
}


impl<'a, T: Mul<Output = T> + Copy> Mul<T> for &'a Matrix<T> {
	type Output = Matrix<T>;
	fn mul(self, other: T) -> Matrix<T> {
		Matrix {
			nrows: self.nrows,
			ncols: self.ncols,
			v: {
				let mut v = Vec::with_capacity(self.v.len());
				for x in self.v.iter() {
					v.push(*x * other);
				}
				v
			}
		}
	}
}


/*
impl<'a, T: Mul<Output = T> + Copy> Mul<&'a Matrix<T>> for T {
	type Output = Matrix<T>;
	fn mul(self, other: &Matrix<T>) -> Matrix<T> {
		Matrix {
			nrows: other.nrows,
			ncols: other.ncols,
			v: {
				let mut v = Vec::with_capacity(other.v.len());
				for x in other.v.iter() {
					v.push(*x * self);
				}
				v
			}
		}
	}
}*/

impl<T> Matrix<T> {
	pub fn iter(&mut self) -> Iter<T> {
		self.v.iter()
	}
}




#[cfg(test)]
mod test {
	use matrix::Matrix;

	#[test]
	fn test_new() {
        let a: Matrix<f32> = Matrix::new_zero(3,3);
        let b: Matrix<f32> = Matrix::new_identity(3);
        let c: Matrix<f32> = Matrix::new(3,3, &[1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0]);
    }

    #[test]
    fn test_index() {
    	let a: Matrix<i32> = Matrix::new(3,3, &[1,0,0,0,1,0,0,0,1]);
    	let b: Matrix<i32> = Matrix::new_identity(3);
    	for i in 0..3 {
    		for j in 0..3 {
    			assert_eq!(a[(i,j)], b[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_add_sub() {
    	let a: Matrix<i32> = Matrix::new_zero(3,3);
    	let b: Matrix<i32> = Matrix::new(3,3, &[1,2,3,4,5,6,7,8,9]);	
    	let c = &a + &b;
    	let d: Matrix<i32> = Matrix::new(3,3, &[8,7,6,5,4,3,2,1,0]);
    	let e: Matrix<i32> = Matrix::new(3,3, &[9,9,9,9,9,9,9,9,9]);
    	let f = &d + &b;
    	let g = &f - &d;
    	for i in 0..3 {
    		for j in 0..3 {
    			assert_eq!(b[(i,j)], c[(i,j)]);
    			assert_eq!(e[(i,j)], f[(i,j)]);
    			assert_eq!(g[(i,j)], b[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_add_assign() {
    	let a: Matrix<i32> = Matrix::new_zero(3,3);
    	let mut b: Matrix<i32> = Matrix::new(3,3, &[1,2,3,4,5,6,7,8,9]);	
    	let c: Matrix<i32> = Matrix::new(3,3, &[8,7,6,5,4,3,2,1,0]);
    	let d: Matrix<i32> = Matrix::new(3,3, &[9,9,9,9,9,9,9,9,9]);
    	b.add(&c);
    	for i in 0..3 {
    		for j in 0..3 {
    			assert_eq!(b[(i,j)], d[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_mul() {
    	let a: Matrix<i32> = Matrix::new(2,4, &[1,3,5,7,2,4,6,8]);
    	let b: Matrix<i32> = Matrix::new(4,3, &[1,8,9,2,7,10,3,6,11,4,5,12]);
    	let c = &a * &b;
    	let d: Matrix<i32> = Matrix::new(2,3, &[50, 94, 178, 60, 120, 220]);
    	for i in 0..c.nrows {
    		for j in 0..c.ncols {
    			assert_eq!(c[(i,j)], d[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_mul_scalar() {
    	let mut a: Matrix<i32> = Matrix::new(3,3, &[1,2,3,4,5,6,7,8,9]);
    	let b: Matrix<i32> = Matrix::new(3,3, &[2,4,6,8,10,12,14,16,18]);
    	let c = &a * 2;
    	//let d = 2 * &a;
    	a.scalar(2);
    	for i in 0..3 {
    		for j in 0..3 {
    			assert_eq!(b[(i,j)], c[(i,j)]);
    			assert_eq!(b[(i,j)], a[(i,j)]);
    			//assert_eq!(b[(i,j)], d[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_transpose() {
    	let mut a: Matrix<i32> = Matrix::new(3,4, &[1,2,3,4,5,6,7,8,9,10,11,12]);
    	a.transpose();
    	let b: Matrix<i32> = Matrix::new(4,3, &[1,5,9,2,6,10,3,7,11,4,8,12]);
    	for i in 0..4 {
    		for j in 0..3 {
    			assert_eq!(b[(i,j)], a[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_iter() {
    	let mut a: Matrix<i32> = Matrix::new(3,3, &[1,2,3,4,5,6,7,8,9]);
    	for (i, x) in a.iter().enumerate() {
    		assert_eq!((i+1) as i32, *x);
    	}
    }

    #[test]
    fn test_vercat() {
    	let mut a: Matrix<i32> = Matrix::new(2,3, &[1,2,3,4,5,6]);
    	let b: Matrix<i32> = Matrix::new(2,3, &[7,8,9,10,11,12]);
    	a.vercat(&b);
    	assert_eq!(a.nrows, 4);
    	assert_eq!(a.ncols, 3);
    	for (i, x) in a.iter().enumerate() {
    		assert_eq!((i+1) as i32, *x);
    	}
    }

    #[test]
    fn test_horcat() {
    	let mut a: Matrix<i32> = Matrix::new(3,2, &[1,2,6,7,11,12]);
    	let b: Matrix<i32> = Matrix::new(3,3, &[3,4,5,8,9,10,13,14,15]);
    	a.horcat(&b);
    	for (i, x) in a.iter().enumerate() {
    		assert_eq!((i+1) as i32, *x);
    	}
    }

    #[test]
    fn test_hadamard() {
    	let mut a: Matrix<i32> = Matrix::new(3,3, &[1,3,2,1,0,0,1,2,2]);
    	let b: Matrix<i32> = Matrix::new(3,3, &[0,0,2,7,5,0,2,1,1]);
    	let c: Matrix<i32> = Matrix::new(3,3, &[0,0,4,7,0,0,2,2,2]);
    	a.hadamard(&b);
    	for i in 0..3 {
    		for j in 0..3 {
    			assert_eq!(a[(i,j)], c[(i,j)]);
    		}
    	}
    }

    #[test]
    fn test_kronecker() {
    	let a: Matrix<i32> = Matrix::new(2,2, &[1,2,3,4]);
    	let b: Matrix<i32> = Matrix::new(2,2, &[0,5,6,7]);
    	let c: Matrix<i32> = Matrix::new(4,4, &[0,5,0,10,6,7,12,14,0,15,0,20,18,21,24,28]);
    	let d = a.kronecker(&b);
    	for i in 0..4 {
    		for j in 0..4 {
    			assert_eq!(c[(i,j)], d[(i,j)]);
    		}
    	}
    }	

}

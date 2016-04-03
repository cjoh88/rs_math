use std::ops::{Mul};

pub struct MatrixF32 {
	m: usize,
	n: usize,
	v: Vec<f32>,
}

impl MatrixF32 {

	pub fn new(m: usize, n:usize) -> Self {
		MatrixF32 {
			m: m,
			n: n,
			v: vec![0.0; m*n],
		}
	}

	pub fn new_identity(n: usize) -> Self {
		MatrixF32 {
			m: n,
			n: n,
			v: {
				let mut v = vec![0.0; n*n];
				for i in 0..n {
					v[i * n + i] = 1.0;
				}
				v
			}
		}
	}

	pub fn get(&self, i: usize, j: usize) -> f32 {
		self.v[i * self.n + j]
	}

	pub fn set(&mut self, i: usize, j: usize, v: f32) {
		self.v[i * self.n + j] = v;
	}

	pub fn size(&self) -> (usize, usize) {
		(self.m, self.n)
	}

	pub fn add(&mut self, other: Self) {
		for (i, v) in other.v.iter().enumerate() {
			self.v[i] += *v;
		}
	}

	pub fn sub(&mut self, other: Self) {
		for (i, v) in other.v.iter().enumerate() {
			self.v[i] -= *v;
		}
	}

	/*pub fn mul(&self, other: &MatrixF32) -> MatrixF32 {
		MatrixF32 {
			m: self.m,
			n: other.n,
			v: {
				let len = self.m * other.n;
				let mut v = Vec::with_capacity(len);
				for i in 0..self.m {
					for j in 0..other.n {
						let mut r = 0.0f32;
						for k in 0..self.n {
							r += self.get(i,k) * other.get(k,j);
						}
						v.push(r);
					}
				}
				v
			}
		}
	}*/
}

impl Clone for MatrixF32 {
	fn clone(&self) -> Self {
		MatrixF32 {
			m: self.m,
			n: self.n,
			v: {
				let mut v = Vec::with_capacity(self.v.len());
				for i in self.v.iter() {
					v.push(i.clone());
				}
				v
			}
		}
	}
}

impl<'a, 'b> Mul<&'b MatrixF32> for &'a MatrixF32 {
	type Output = MatrixF32;
	fn mul(self, other: &MatrixF32) -> MatrixF32 {
		MatrixF32 {
			m: self.m,
			n: other.n,
			v: {
				let len = self.m * other.n;
				let mut v = Vec::with_capacity(len);
				for i in 0..self.m {
					for j in 0..other.n {
						let mut r = 0.0f32;
						for k in 0..self.n {
							r += self.get(i,k) * other.get(k,j);
						}
						v.push(r);
					}
				}
				v
			}
		}
	}
}

impl Mul for MatrixF32 {
	type Output = MatrixF32;
	fn mul(self, other: MatrixF32) -> MatrixF32 {
		MatrixF32 {
			m: self.m,
			n: other.n,
			v: {
				let len = self.m * other.n;
				let mut v = Vec::with_capacity(len);
				for i in 0..self.m {
					for j in 0..other.n {
						let mut r = 0.0f32;
						for k in 0..self.n {
							r += self.get(i,k) * other.get(k,j);
						}
						v.push(r);
					}
				}
				v
			}
		}
	}
}


#[cfg(test)]
mod test {
	use MatrixF32;

	#[test]
	fn test_size() {
		let a = MatrixF32::new(10,7);
		let size = a.size();
		assert_eq!(10, size.0);
		assert_eq!(7, size.1);
	}

    #[test]
    fn test_identity() {
    	let a = MatrixF32::new_identity(3);
    	assert_eq!(1.0, a.get(0,0));
    	assert_eq!(1.0, a.get(1,1));
    	assert_eq!(1.0, a.get(2,2));

    	assert_eq!(0.0, a.get(0,1));
    	assert_eq!(0.0, a.get(0,2));
    	assert_eq!(0.0, a.get(1,0));
    	assert_eq!(0.0, a.get(1,2));
    	assert_eq!(0.0, a.get(2,0));
    	assert_eq!(0.0, a.get(2,1));
    	
    }

    #[test]
    fn test_add() {
    	let mut a = MatrixF32::new_identity(3);
    	let b = a.clone();
    	a.add(b);
    	assert_eq!(2.0, a.get(0,0));
    	assert_eq!(2.0, a.get(1,1));
    	assert_eq!(2.0, a.get(2,2));

    	assert_eq!(0.0, a.get(0,1));
    	assert_eq!(0.0, a.get(0,2));
    	assert_eq!(0.0, a.get(1,0));
    	assert_eq!(0.0, a.get(1,2));
    	assert_eq!(0.0, a.get(2,0));
    	assert_eq!(0.0, a.get(2,1));
    }

    #[test]
    fn test_mul() {
    	let mut a = MatrixF32::new(2,2);
    	let mut b = MatrixF32::new(2,2);
    	let mut a_b = MatrixF32::new(2,2);
    	let mut b_a = MatrixF32::new(2,2);
    	a.set(0,0,1.0);
    	a.set(0,1,2.0);
    	a.set(1,0,3.0);
    	a.set(1,1,4.0);
    	b.set(0,0,2.0);
    	b.set(0,1,0.0);
    	b.set(1,0,1.0);
    	b.set(1,1,2.0);
		a_b.set(0,0,4.0);
    	a_b.set(0,1,4.0);
    	a_b.set(1,0,10.0);
    	a_b.set(1,1,8.0);
    	b_a.set(0,0,2.0);
    	b_a.set(0,1,4.0);
    	b_a.set(1,0,7.0);
    	b_a.set(1,1,10.0);
    	//let c = a.mul(&b);
    	//let d = b.mul(&a);
    	let c = &a * &b;
    	let d = &b * &a;
    	let e = a * b;
    	for i in 0..2 {
    		for j in 0..2 {
    			assert_eq!(a_b.get(i,j), c.get(i,j));
    			assert_eq!(a_b.get(i,j), e.get(i,j));
    			assert_eq!(b_a.get(i,j), d.get(i,j));
    		}
    	}
    }
}

mod matrix;
mod traits;

#[cfg(test)]
mod test {
	use traits::{Zero, One};

	#[test]
	fn test_zero_one() {
        let zero = i32::zero();
        let one = i32::one();
        assert_eq!(zero, 0);
        assert_eq!(one, 1);
    }

}
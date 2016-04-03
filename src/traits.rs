

pub trait Zero: Sized {
	fn zero() -> Self;
}

pub trait One {
	fn one() -> Self;
}

macro_rules! zero_one_impl {
    ($($t:ty)*) => ($(
    	impl Zero for $t {
    		#[inline]
    		fn zero() -> Self { 0 }
    	}
    	impl One for $t {
    		#[inline]
    		fn one() -> Self { 1 }
    	}
    )*)
}

zero_one_impl! {u8 u16 u32 usize i8 i16 i32 isize}


macro_rules! zero_one_float_impl {
    ($($t:ty)*) => ($(
    	impl Zero for $t {
    		#[inline]
    		fn zero() -> Self { 0.0 }
    	}
    	impl One for $t {
    		#[inline]
    		fn one() -> Self { 1.0 }
    	}
    )*)
}

zero_one_float_impl! {f32 f64}
macro_rules! midpoint_impl {
    ($($ty:ident)*) => {
        $(
            impl Midpoint for $ty {
                fn midpoint_2(a: Self, b: Self) -> Self {
                    const BITS_MINUS_ONE: u32 = <$ty>::BITS - 1;

                    let (sum, overflow) = a.overflowing_add(b);
                    ((overflow as Self) << BITS_MINUS_ONE) | (sum / 2)
                }
            }
        )*
    };
}

pub trait Midpoint {
    fn midpoint_2(a: Self, b: Self) -> Self;
}

midpoint_impl!(u8 u16 u32 u64 u128);

macro_rules! test_impl {
    ($($ty:ident)*) => {
        $(
            #[cfg(test)]
            #[test]
            fn $ty() {
                for a in (0..128).chain(<$ty>::MAX - 127..=<$ty>::MAX) {
                    for b in (0..128).chain(<$ty>::MAX - 127..=<$ty>::MAX) {
                        let mut correct_result = BigUint::ZERO;
                        correct_result += a;
                        correct_result += b;
                        correct_result /= 2_u8;

                        assert_eq!(
                            BigUint::from(<$ty>::midpoint_2(a, b)),
                            correct_result,
                        );
                    }
                }
            }
        )*
    }
}

#[cfg(test)]
use num_bigint::BigUint;

test_impl!(u8 u16 u32 u64 u128);

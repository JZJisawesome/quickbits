/*
 * File:    primitive_integer.rs
 * Brief:   TODO
 *
 * Copyright: Copyright (C) 2023-2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * Provides a trait for primitive integers used by the `BitManip` implementations.
*/

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use core::ops::*;
use core::fmt::{Display, Debug};

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

macro_rules! implement_for {
    ($unsigned:ty, $signed:ty) => {
        impl PrimitiveInteger for $unsigned {
            type Unsigned   = $unsigned;
            type Signed     = $signed;

            #[inline(always)]
            fn num_bits() -> Self {
                <$unsigned>::BITS as $unsigned
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }

            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn as_unsigned(self) -> Self::Unsigned {
                self
            }

            #[inline(always)]
            fn as_signed(self) -> Self::Signed {
                self as $signed
            }

            #[inline(always)]
            fn from_unsigned(value: Self::Unsigned) -> Self {
                value
            }

            #[inline(always)]
            fn from_signed(value: Self::Signed) -> Self {
                value as $unsigned
            }
        }

        impl PrimitiveInteger for $signed {
            type Unsigned   = $unsigned;
            type Signed     = $signed;

            #[inline(always)]
            fn num_bits() -> Self {
                <$signed>::BITS as $signed
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }

            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn as_unsigned(self) -> Self::Unsigned {
                self as $unsigned
            }

            #[inline(always)]
            fn as_signed(self) -> Self::Signed {
                self
            }

            #[inline(always)]
            fn from_unsigned(value: Self::Unsigned) -> Self {
                value as $signed
            }

            #[inline(always)]
            fn from_signed(value: Self::Signed) -> Self {
                value
            }
        }
    };
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

/**
 * TODO document
 *
 * NOTE: Only includes the features BitManip needs
*/
pub trait PrimitiveInteger:
    Sized + Add<Self, Output=Self> + Sub<Self, Output=Self> + Shl<Self, Output=Self> +
    Shr<Self, Output=Self> + BitAnd<Self, Output=Self> + BitOr<Self, Output=Self> +
    BitXor<Self, Output=Self> + Not<Output=Self> + BitOrAssign<Self> + BitAndAssign<Self> + BitXorAssign<Self> +
    AddAssign<Self> + SubAssign<Self> + ShlAssign<Self> + ShrAssign<Self> +
    Copy + Clone + Ord + Eq + Debug + Display
{
    type Unsigned:  PrimitiveInteger;
    type Signed:    PrimitiveInteger;

    /**
     * TODO document
    */
    fn num_bits() -> Self;

    /**
     * TODO document
    */
    fn one() -> Self;

    /**
     * TODO document
    */
    fn zero() -> Self;

    /**
     * TODO document
    */
    fn as_unsigned(self) -> Self::Unsigned;

    /**
     * TODO document
    */
    fn as_signed(self) -> Self::Signed;

    /**
     * TODO document
    */
    fn from_unsigned(value: Self::Unsigned) -> Self;

    /**
     * TODO document
    */
    fn from_signed(value: Self::Signed) -> Self;
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

implement_for!(u8,      i8);
implement_for!(u16,     i16);
implement_for!(u32,     i32);
implement_for!(u64,     i64);
implement_for!(u128,    i128);
implement_for!(usize,   isize);

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    const fn ensure_primitive_integer_implemented() {
        const fn test<T: PrimitiveInteger>() {}
        test::<u8>();
        test::<u16>();
        test::<u32>();
        test::<u64>();
        test::<u128>();
        test::<usize>();

        test::<i8>();
        test::<i16>();
        test::<i32>();
        test::<i64>();
        test::<i128>();
        test::<isize>();
    }

    #[test]
    fn check_num_bits() {
        assert!(u8::num_bits()      == (u8::BITS as u8));
        assert!(u16::num_bits()     == (u16::BITS as u16));
        assert!(u32::num_bits()     == (u32::BITS as u32));
        assert!(u64::num_bits()     == (u64::BITS as u64));
        assert!(u128::num_bits()    == (u128::BITS as u128));
        assert!(usize::num_bits()   == (usize::BITS as usize));
        assert!(i8::num_bits()      == (i8::BITS as i8));
        assert!(i16::num_bits()     == (i16::BITS as i16));
        assert!(i32::num_bits()     == (i32::BITS as i32));
        assert!(i64::num_bits()     == (i64::BITS as i64));
        assert!(i128::num_bits()    == (i128::BITS as i128));
        assert!(isize::num_bits()   == (isize::BITS as isize));
    }

    #[test]
    fn check_one() {
        assert!(u8::one()       == 1);
        assert!(u16::one()      == 1);
        assert!(u32::one()      == 1);
        assert!(u64::one()      == 1);
        assert!(u128::one()     == 1);
        assert!(usize::one()    == 1);
        assert!(i8::one()       == 1);
        assert!(i16::one()      == 1);
        assert!(i32::one()      == 1);
        assert!(i64::one()      == 1);
        assert!(i128::one()     == 1);
        assert!(isize::one()    == 1);
    }

    #[test]
    fn check_zero() {
        assert!(u8::zero()      == 0);
        assert!(u16::zero()     == 0);
        assert!(u32::zero()     == 0);
        assert!(u64::zero()     == 0);
        assert!(u128::zero()    == 0);
        assert!(usize::zero()   == 0);
        assert!(i8::zero()      == 0);
        assert!(i16::zero()     == 0);
        assert!(i32::zero()     == 0);
        assert!(i64::zero()     == 0);
        assert!(i128::zero()    == 0);
        assert!(isize::zero()   == 0);
    }
}

/*
 * File:    bitmanip.rs
 * Brief:   TODO
 *
 * Copyright: Copyright (C) 2023-2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * Definition and implementation of the BitManip trait for primitive integers
*/

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use crate::primitive_integer::PrimitiveInteger;

use core::ops::{Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use sealed::*;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

/**
 * TODO document
*/
macro_rules! implement_as_range_for_ranges {
    ($range:ty) => {
        impl<T> AsRange<T> for $range where T: PrimitiveInteger {
            #[inline(always)]
            fn as_range(&self) -> impl RangeBounds<T> {
                //This clone is really really cheap (even free from some ranges like RangeFull)
                self.clone()
            }
        }
    };
    ($($range:ty),+) => {
        $(implement_as_range_for_ranges!($range);)+
    };
}

/**
 * TODO document
*/
macro_rules! implement_as_range_for_integers {
    ($integer:ty) => {
        impl AsRange<$integer> for $integer {
            #[inline(always)]
            fn as_range(&self) -> impl RangeBounds<$integer> {
                *self..=*self
            }
        }
    };
    ($($integer:ty),+) => {
        $(implement_as_range_for_integers!($integer);)+
    };
}

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

/**
 * TODO document
*/
pub trait BitManip: BitManipInternal {
    //TODO make all of these const too since we only intend for this to be implemented for integers
    //TODO note in the docs we optimize the nice usage of the functions for not using bool because it's useful when chaining multiple functions together

    //TODO get rid of this and just use bits()/b() once we know that optimized just as well as bit()
    /**
     * Extracts the bit at the given index from the integer
     *
     * # Examples
     *
     * ```
     * use quickbits::BitManip;
     * assert_eq!(0b1010_1010.bit(5), 1);
     * assert_eq!(0b1010_1010.bit(6), 0);
     * ```
    */
    #[inline(always)]
    fn bit(&self, bit_index: Self) -> Self {
        debug_assert!(bit_index < Self::num_bits(), "Attempt to access bit at out of range index {} >= {}", bit_index, Self::num_bits());
        (*self >> bit_index) & Self::one()
    }

    /**
     * Like [`bit()`](Self::bit), but returns a [bool] instead of an integer
     *
     * # Examples
     *
     * ```
     * use quickbits::BitManip;
     * assert_eq!(0b1010_1010.bit_bool(5), true);
     * assert_eq!(0b1010_1010.bit_bool(6), false);
     * ```
    */
    #[inline(always)]
    fn bit_bool(&self, bit_index: Self) -> bool {
        self.bit(bit_index) == Self::one()
    }

    /**
     * Convenient alias for [`bits()`](Self::bits)
     *
     * # Examples
     *
     * ```
     * use quickbits::BitManip;
     * assert_eq!(0b1010_1010.b(3..=0), 0b1010_1010.bits(3..=0));
     * assert_eq!(0b1010_1010.b(7), 0b1010_1010.bits(7));
     * ```
    */
    #[inline(always)]
    fn b(&self, range: impl AsRange<Self>) -> Self {
        self.bits(range)
    }

    /**
     * Extracts the bits in the given range (or at the given index) from the integer
     *
     * # Examples
     *
     * ```
     * use quickbits::BitManip;
     * assert_eq!(0b1010_1010.bits(3..=0), 0b1010);
     * assert_eq!(0b1010_1010.bits(7), 1);
     * ```
    */
    #[inline(always)]
    fn bits(&self, range: impl AsRange<Self>) -> Self {
        let (_, left_shift_amt) = Self::range2bounds(&range);

        (*self & Self::bitmask(range)) >> left_shift_amt
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn set_bit(&self, bit_index: Self) -> Self {
        debug_assert!(bit_index < Self::num_bits(), "Attempt to access bit at out of range index {} >= {}", bit_index, Self::num_bits());
        *self | (Self::one() << bit_index)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn set_bits(&self, range: impl AsRange<Self>) -> Self {
        *self | Self::bitmask(range)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn set_bit_assign(&mut self, bit_index: Self) {
        *self = self.set_bit(bit_index);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn set_bits_assign(&mut self, range: impl AsRange<Self>) {
        *self = self.set_bits(range);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn clear_bit(&self, bit_index: Self) -> Self {
        *self & !(Self::one() << bit_index)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn clear_bits(&self, range: impl AsRange<Self>) -> Self {
        *self & !Self::bitmask(range)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn clear_bit_assign(&mut self, bit_index: Self) {
        *self = self.clear_bit(bit_index);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn clear_bits_assign(&mut self, range: impl AsRange<Self>) {
        *self = self.clear_bits(range);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn toggle_bit(&self, bit_index: Self) -> Self {
        *self ^ (Self::one() << bit_index)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn toggle_bits(&self, range: impl AsRange<Self>) -> Self {
        *self ^ Self::bitmask(range)
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn toggle_bit_assign(&mut self, bit_index: Self) {
        *self = self.toggle_bit(bit_index);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn toggle_bits_assign(&mut self, range: impl AsRange<Self>) {
        *self = self.toggle_bits(range);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn replace_bit(&self, bit_index: Self, value: impl Into<bool>) -> Self {
        if value.into() {
            self.set_bit(bit_index)
        } else {
            self.clear_bit(bit_index)
        }
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn replace_bits(&self, _range: impl AsRange<Self>, _value: Self) -> Self {
        todo!()
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn replace_bit_assign(&mut self, bit_index: Self, value: impl Into<bool>) {
        *self = self.replace_bit(bit_index, value);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn replace_bits_assign(&mut self, range: impl AsRange<Self>, value: Self) {
        *self = self.replace_bits(range, value);
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn bitmask(range: impl AsRange<Self>) -> Self {
        let (leftmost_inclusive, rightmost_inclusive) = Self::range2bounds(range);

        ((Self::one() << (leftmost_inclusive - rightmost_inclusive + Self::one())) - Self::one()) << rightmost_inclusive
    }

    /**
     * TODO document
    */
    //TODO support permuting ranges and bits combined!
    #[inline(always)]
    fn permute(&self, ranges: &[impl AsRange<Self>]) -> Self {
        let mut result      = Self::zero();
        let mut current_pos = Self::zero();

        for range in ranges.iter().rev() {
            result |= self.bits(range) << current_pos;
            current_pos += Self::range_length(range);
        }

        result
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn sign_extend_from_bit(&self, bit_index: Self) -> Self {
        let msb_index = Self::num_bits() - Self::one();
        let shift_amt = msb_index - bit_index;
        Self::from_signed((*self << shift_amt).as_signed() >> shift_amt.as_signed())
    }

    /**
     * TODO document
    */
    #[inline(always)]
    fn sign_extend_from_size(&self, size: Self) -> Self {
        self.sign_extend_from_bit(size - Self::one())
    }

    //TODO others
}

/**
 * TODO document
*/
mod sealed {
    use super::*;

    /**
     * TODO document
    */
    pub trait BitManipInternal: PrimitiveInteger {
        /**
         * TODO document
        */
        #[inline(always)]
        fn range2bounds(range: impl AsRange<Self>) -> (Self, Self) {
            let range = range.as_range();
            let leftmost_inclusive = match range.start_bound() {
                Bound::Included(index) => *index,
                Bound::Excluded(index) => {
                    debug_assert!(*index != Self::zero(), "Leftmost bit must be greater than or equal to 0");
                    *index - Self::one()
                },
                Bound::Unbounded => Self::num_bits() - Self::one(),
            };

            let rightmost_inclusive = match range.end_bound() {
                Bound::Included(index) => *index,
                Bound::Excluded(index) => {
                    debug_assert!(*index != (Self::num_bits() - Self::one()), "Rightmost bit must be less than or equal to {}", Self::num_bits() - Self::one());
                    *index + Self::one()
                },
                Bound::Unbounded => Self::zero(),
            };

            //TODO support reverse ranges?
            debug_assert!(leftmost_inclusive >= rightmost_inclusive, "Leftmost bit must be greater than or equal to rightmost bit");

            (leftmost_inclusive, rightmost_inclusive)
        }

        /**
         * TODO document
        */
        #[inline(always)]
        fn range_length(range: &impl AsRange<Self>) -> Self {
            let (leftmost_inclusive, rightmost_inclusive) = Self::range2bounds(range);
            leftmost_inclusive - rightmost_inclusive + Self::one()
        }
    }

    /**
     * TODO document
    */
    pub trait AsRange<T> {
        /**
         * TODO document
        */
        fn as_range(&self) -> impl RangeBounds<T>;
    }
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

impl<T> BitManip for T where T: BitManipInternal {}
impl<T> BitManipInternal for T where T: PrimitiveInteger {}

implement_as_range_for_ranges!(Range<T>, RangeFrom<T>, RangeFull, RangeInclusive<T>, RangeTo<T>, RangeToInclusive<T>);
implement_as_range_for_integers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/*
//Can't do this since Rust doesn't have specialization and this would conflict with the AsRange impl for &R below
impl<T> AsRange<T> for T where T: PrimitiveInteger {
    fn as_range(&self) -> impl RangeBounds<T> {
        *self..=*self
    }
}
*/

impl<T, R> AsRange<T> for &R where R: AsRange<T> {
    fn as_range(&self) -> impl RangeBounds<T> {
        (*self).as_range()
    }
}

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.bit(0), 0);
        assert_eq!(value.bit(1), 1);
        assert_eq!(value.bit(2), 0);
        assert_eq!(value.bit(3), 1);
        assert_eq!(value.bit(4), 0);
        assert_eq!(value.bit(5), 1);
        assert_eq!(value.bit(6), 0);
        assert_eq!(value.bit(7), 1);
    }

    #[test]
    fn test_bit_bool() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.bit_bool(0), false);
        assert_eq!(value.bit_bool(1), true);
        assert_eq!(value.bit_bool(2), false);
        assert_eq!(value.bit_bool(3), true);
        assert_eq!(value.bit_bool(4), false);
        assert_eq!(value.bit_bool(5), true);
        assert_eq!(value.bit_bool(6), false);
        assert_eq!(value.bit_bool(7), true);
    }

    #[test]
    fn test_bits() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.bits(3..=0), 0b1010);
        assert_eq!(value.bits(7..=4), 0b1010);
        assert_eq!(value.bits(7..=0), 0b1010_1010);
        assert_eq!(value.b(6..=1), 0b010_101);
        assert_eq!(value.bits(&(3..=0)), 0b1010);
        assert_eq!(value.bits(&(7..=4)), 0b1010);
        assert_eq!(value.bits(&(7..=0)), 0b1010_1010);
        assert_eq!(value.b(&(6..=1)), 0b010_101);
        assert_eq!(value.bits(7), 1);
        assert_eq!(value.b(&1), 1);
    }

    #[test]
    fn test_set_bit() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.set_bit(0), 0b1010_1011);
        assert_eq!(value.set_bit(1), 0b1010_1010);
        assert_eq!(value.set_bit(2), 0b1010_1110);
        assert_eq!(value.set_bit(3), 0b1010_1010);
        assert_eq!(value.set_bit(4), 0b1011_1010);
        assert_eq!(value.set_bit(5), 0b1010_1010);
        assert_eq!(value.set_bit(6), 0b1110_1010);
        assert_eq!(value.set_bit(7), 0b1010_1010);
    }

    #[test]
    fn test_set_bits() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.set_bits(3..=0), 0b1010_1111);
        assert_eq!(value.set_bits(7..=4), 0b1111_1010);
        assert_eq!(value.set_bits(7..=0), 0b1111_1111);
        assert_eq!(value.set_bits(6..=1), 0b1111_1110);
    }

    #[test]
    fn test_clear_bit() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.clear_bit(0), 0b1010_1010);
        assert_eq!(value.clear_bit(1), 0b1010_1000);
        assert_eq!(value.clear_bit(2), 0b1010_1010);
        assert_eq!(value.clear_bit(3), 0b1010_0010);
        assert_eq!(value.clear_bit(4), 0b1010_1010);
        assert_eq!(value.clear_bit(5), 0b1000_1010);
        assert_eq!(value.clear_bit(6), 0b1010_1010);
        assert_eq!(value.clear_bit(7), 0b0010_1010);
    }

    #[test]
    fn test_clear_bits() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.clear_bits(3..=0), 0b1010_0000);
        assert_eq!(value.clear_bits(7..=4), 0b0000_1010);
        assert_eq!(value.clear_bits(7..=0), 0);
        assert_eq!(value.clear_bits(6..=1), 0b1000_0000);
    }

    #[test]
    fn test_toggle_bit() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.toggle_bit(0), 0b1010_1011);
        assert_eq!(value.toggle_bit(1), 0b1010_1000);
        assert_eq!(value.toggle_bit(2), 0b1010_1110);
        assert_eq!(value.toggle_bit(3), 0b1010_0010);
        assert_eq!(value.toggle_bit(4), 0b1011_1010);
        assert_eq!(value.toggle_bit(5), 0b1000_1010);
        assert_eq!(value.toggle_bit(6), 0b1110_1010);
        assert_eq!(value.toggle_bit(7), 0b0010_1010);
    }

    #[test]
    fn test_toggle_bits() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.toggle_bits(3..=0), 0b1010_0101);
        assert_eq!(value.toggle_bits(7..=4), 0b0101_1010);
        assert_eq!(value.toggle_bits(7..=0), 0b0101_0101);
        assert_eq!(value.toggle_bits(6..=1), 0b1101_0100);
    }

    #[test]
    fn test_bitmask() {
        assert_eq!(u32::bitmask(3..=0), 0b1111);
        assert_eq!(u32::bitmask(7..=4), 0b1111_0000);
        assert_eq!(u32::bitmask(7..=0), 0b1111_1111);
        assert_eq!(u32::bitmask(6..=1), 0b0111_1110);
        assert_eq!(u32::bitmask(&(3..=0)), 0b1111);
        assert_eq!(u32::bitmask(&(7..=4)), 0b1111_0000);
        assert_eq!(u32::bitmask(&(7..=0)), 0b1111_1111);
        assert_eq!(u32::bitmask(&(6..=1)), 0b0111_1110);
    }

    #[test]
    fn test_permute() {
        let value: u32 = 0xABCD1234;
        assert_eq!(value.permute(&[7..=4]), 0x3);
        assert_eq!(value.permute(&[3..=0, 7..=4]), 0x43);
        assert_eq!(value.permute(&[15..=0, 31..=16]), 0x1234ABCD);
        assert_eq!(value.permute(&[23..=8, 31..=24, 7..=0]), 0xCD12AB34);
        assert_eq!(value.permute(&[3, 2, 1, 0]), 0x4);
        //assert_eq!(value.permute(&[3, 2, 1, 0, 7..=4]), 0x43);//TODO support this
    }

    #[test]
    fn test_sign_extend_from_bit() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.sign_extend_from_bit(7), 0xFFFFFFAA);
        assert_eq!(value.sign_extend_from_bit(6), 0x2A);
        assert_eq!(value.sign_extend_from_bit(5), 0xFFFFFFEA);
        assert_eq!(value.sign_extend_from_bit(4), 0xA);
        assert_eq!(value.sign_extend_from_bit(3), 0xFFFFFFFA);
        assert_eq!(value.sign_extend_from_bit(2), 0x2);
        assert_eq!(value.sign_extend_from_bit(1), 0xFFFFFFFE);
        assert_eq!(value.sign_extend_from_bit(0), 0);
    }

    #[test]
    fn test_sign_extend_from_size() {
        let value: u32 = 0b1010_1010;
        assert_eq!(value.sign_extend_from_size(8), 0xFFFFFFAA);
    }

    #[test]
    fn test_range2bounds() {
        assert_eq!(u32::range2bounds(&(3..=0)), (3, 0));
        assert_eq!(u32::range2bounds(&(7..=4)), (7, 4));
        assert_eq!(u32::range2bounds(&(7..=0)), (7, 0));
        assert_eq!(u32::range2bounds(&(6..=1)), (6, 1));

        assert_eq!(u32::range2bounds(&(..=0)), (31, 0));
        assert_eq!(u32::range2bounds(&(..=4)), (31, 4));
        assert_eq!(u32::range2bounds(&(..=7)), (31, 7));

        assert_eq!(u32::range2bounds(&(0..)), (0, 0));
        assert_eq!(u32::range2bounds(&(4..)), (4, 0));
        assert_eq!(u32::range2bounds(&(7..)), (7, 0));

        assert_eq!(u32::range2bounds(&(..)), (31, 0));
    }

    #[test]
    fn test_range_length() {
        assert_eq!(u32::range_length(&(3..=0)), 4);
        assert_eq!(u32::range_length(&(7..=4)), 4);
        assert_eq!(u32::range_length(&(7..=0)), 8);
        assert_eq!(u32::range_length(&(6..=1)), 6);

        assert_eq!(u32::range_length(&(..=0)), 32);
        assert_eq!(u32::range_length(&(..=4)), 28);
        assert_eq!(u32::range_length(&(..=7)), 25);

        assert_eq!(u32::range_length(&(0..)), 1);
        assert_eq!(u32::range_length(&(4..)), 5);
        assert_eq!(u32::range_length(&(7..)), 8);

        assert_eq!(u32::range_length(&(..)), 32);
        assert_eq!(u32::range_length(&(0..=0)), 1);
    }
}

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO

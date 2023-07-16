/*
 * File:    bitmanip.rs
 * Brief:   TODO
 *
 * Copyright: Copyright (C) TODO John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
 * TODO longer description
 *
*/

/*!
 * TODO rustdoc for this file here
*/

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

//TODO (includes "mod ..." and "pub mod ...")

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use crate::primitive_integer::PrimitiveInteger;
use crate::debug_panic::debug_panic_if;

use core::ops::RangeBounds;
use core::ops::Bound;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

macro_rules! implement_for {
    ($integer:ty) => {
        impl BitManip for $integer {
            fn bit(&self, bit_index: Self::BitIndex) -> Self {
                debug_panic_if!(bit_index >= Self::BITS, "Attempt to access bit at out of range index {} >= {}", bit_index, Self::BITS);
                (self >> bit_index) & 0b1
            }

            fn bit_bool(&self, bit_index: Self::BitIndex) -> bool {
                self.bit(bit_index) == 0b1
            }

            fn bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self {
                todo!()
            }

            fn set_bit(&self, bit_index: Self::BitIndex) -> Self {
                todo!()
            }

            fn set_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self {
                todo!()
            }

            fn clear_bit(&self, bit_index: Self::BitIndex) -> Self {
                todo!()
            }

            fn clear_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self {
                todo!()
            }

            fn toggle_bit(&self, bit_index: Self::BitIndex) -> Self {
                todo!()
            }

            fn toggle_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self {
                todo!()
            }

            fn replace_bit(&self, bit_index: Self::BitIndex, bit: impl Into<bool>) -> Self {
                todo!()
            }

            fn replace_bits(&self, range: impl RangeBounds<Self::BitIndex>, bits: Self) -> Self {
                todo!()
            }

            fn bitmask(range: impl RangeBounds<Self::BitIndex>) -> Self {
                let leftmost_inclusive = match range.start_bound() {
                    Bound::Included(&index) => index,
                    Bound::Excluded(&index) => {
                        debug_panic_if!(index == 0, "Leftmost bit must be greater than or equal to 0");
                        index - 1
                    },
                    Bound::Unbounded => Self::BITS - 1,
                };

                let rightmost_inclusive = match range.end_bound() {
                    Bound::Included(&index) => index,
                    Bound::Excluded(&index) => {
                        debug_panic_if!(index == (Self::BITS - 1), "Rightmost bit must be less than or equal to {}", Self::BITS - 1);
                        index + 1
                    },
                    Bound::Unbounded => 0,
                };

                //TODO support reverse ranges?
                debug_panic_if!(leftmost_inclusive < rightmost_inclusive, "Leftmost bit must be greater than or equal to rightmost bit");

                ((1 << (leftmost_inclusive - rightmost_inclusive + 1)) - 1) << rightmost_inclusive
            }
        }
    }
}

/* ------------------------------------------------------------------------------------------------
 * Constants
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Static Variables
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Types
 * --------------------------------------------------------------------------------------------- */

//TODO includes "type"-defs, structs, enums, unions, etc

/* ------------------------------------------------------------------------------------------------
 * Associated Functions and Methods
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Traits And Default Implementations
 * --------------------------------------------------------------------------------------------- */

pub trait BitManip: PrimitiveInteger {

    //TODO make all of these const too since we only intend for this to be implemented for integers
    //TODO note in the docs we optimize the nice usage of the functions for not using bool because it's useful when chaining multiple functions together

    fn bit(&self, bit_index: Self::BitIndex) -> Self;
    fn bit_bool(&self, bit_index: Self::BitIndex) -> bool;
    fn bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self;

    fn set_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn set_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self;
    fn set_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.set_bit(bit_index);
    }
    fn set_bits_assign(&mut self, range: impl RangeBounds<Self::BitIndex>) {
        *self = self.set_bits(range);
    }

    fn clear_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn clear_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self;
    fn clear_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.clear_bit(bit_index);
    }
    fn clear_bits_assign(&mut self, range: impl RangeBounds<Self::BitIndex>) {
        *self = self.clear_bits(range);
    }

    fn toggle_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn toggle_bits(&self, range: impl RangeBounds<Self::BitIndex>) -> Self;
    fn toggle_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.toggle_bit(bit_index);
    }
    fn toggle_bits_assign(&mut self, range: impl RangeBounds<Self::BitIndex>) {
        *self = self.toggle_bits(range);
    }

    fn replace_bit(&self, bit_index: Self::BitIndex, value: impl Into<bool>) -> Self;
    fn replace_bits(&self, range: impl RangeBounds<Self::BitIndex>, value: Self) -> Self;
    fn replace_bit_assign(&mut self, bit_index: Self::BitIndex, value: impl Into<bool>) {
        *self = self.replace_bit(bit_index, value);
    }
    fn replace_bits_assign(&mut self, range: impl RangeBounds<Self::BitIndex>, value: Self) {
        *self = self.replace_bits(range, value);
    }

    fn bitmask(range: impl RangeBounds<Self::BitIndex>) -> Self;

    //TODO others
}

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

implement_for!(u8);

/* ------------------------------------------------------------------------------------------------
 * Functions
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Tests
 * --------------------------------------------------------------------------------------------- */

//TODO

/* ------------------------------------------------------------------------------------------------
 * Benchmarks
 * --------------------------------------------------------------------------------------------- */

//TODO

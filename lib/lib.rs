/*
 * File:    lib.rs
 * Brief:   TODO
 *
 * Copyright: Copyright (C) 2023 John Jekel
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

//TODO

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

use std::ops::Range;

/* ------------------------------------------------------------------------------------------------
 * Macros
 * --------------------------------------------------------------------------------------------- */

//TODO (also pub(crate) use the_macro statements here too)

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

trait BitManip: Sized {
    type BitIndex;

    //TODO make all of these const too since we only intend for this to be implemented for integers

    fn bit(&self, bit_index: Self::BitIndex) -> Self;
    fn bits(&self, range: Range<Self::BitIndex>) -> Self;

    fn set_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn set_bits(&self, range: Range<Self::BitIndex>) -> Self;
    fn set_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.set_bit(bit_index);
    }
    fn set_bits_assign(&mut self, range: Range<Self::BitIndex>) {
        *self = self.set_bits(range);
    }

    fn clear_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn clear_bits(&self, range: Range<Self::BitIndex>) -> Self;
    fn clear_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.clear_bit(bit_index);
    }
    fn clear_bits_assign(&mut self, range: Range<Self::BitIndex>) {
        *self = self.clear_bits(range);
    }

    fn toggle_bit(&self, bit_index: Self::BitIndex) -> Self;
    fn toggle_bits(&self, range: Range<Self::BitIndex>) -> Self;
    fn toggle_bit_assign(&mut self, bit_index: Self::BitIndex) {
        *self = self.toggle_bit(bit_index);
    }
    fn toggle_bits_assign(&mut self, range: Range<Self::BitIndex>) {
        *self = self.toggle_bits(range);
    }

    fn replace_bit(&self, bit_index: Self::BitIndex, value: impl Into<bool>) -> Self;
    fn replace_bits(&self, range: Range<Self::BitIndex>, value: Self) -> Self;
    fn replace_bit_assign(&mut self, bit_index: Self::BitIndex, value: impl Into<bool>) {
        *self = self.replace_bit(bit_index, value);
    }
    fn replace_bits_assign(&mut self, range: Range<Self::BitIndex>, value: Self) {
        *self = self.replace_bits(range, value);
    }

    fn bitmask(range: Range<Self::BitIndex>) -> Self;

    //TODO others
}

/*trait BitManipBool: BitManip {
    /*const*/ fn bit(&self, bit: Self) -> bool {
        BitManip::bit(self, bit) == 1 
    }
    //TODO others
}*/

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

//TODO

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

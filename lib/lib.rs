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

//TODO

/* ------------------------------------------------------------------------------------------------
 * Trait Implementations
 * --------------------------------------------------------------------------------------------- */

trait BitManip: Sized {
    /*const*/ fn bit(&self, bit: Self) -> Self;
    /*const*/ fn bits(&self, range: Range<Self>) -> Self;

    /*const*/ fn set_bit(&self, bit: Self) -> Self;
    /*const*/ fn set_bit_assign(&mut self, bit: Self);
    /*const*/ fn set_bits(&self, range: Range<Self>) -> Self;
    /*const*/ fn set_bits_assign(&mut self, range: Range<Self>);

    /*const*/ fn clear_bit(&self, bit: Self) -> Self;
    /*const*/ fn clear_bit_assign(&mut self, bit: Self);
    /*const*/ fn clear_bits(&self, range: Range<Self>) -> Self;
    /*const*/ fn clear_bits_assign(&mut self, range: Range<Self>);

    /*const*/ fn toggle_bit(&self, bit: Self) -> Self;
    /*const*/ fn toggle_bit_assign(&mut self, bit: Self);
    /*const*/ fn toggle_bits(&self, range: Range<Self>) -> Self;
    /*const*/ fn toggle_bits_assign(&mut self, range: Range<Self>);

    /*const*/ fn replace_bit(&self, bit: Self, value: Self) -> Self;
    /*const*/ fn replace_bit_assign(&mut self, bit: Self value: Self);
    /*const*/ fn replace_bits(&self, range: Range<Self>, value: Self) -> Self;
    /*const*/ fn replace_bits_assign(&mut self, range: Range<Self>, value: Self);

    /*const*/ fn bitmask(range: Range<Self>) -> Self;

    //TODO others
}

/*trait BitManipBool: BitManip {
    /*const*/ fn bit(&self, bit: Self) -> bool {
        BitManip::bit(self, bit) == 1 
    }
    //TODO others
}*/

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

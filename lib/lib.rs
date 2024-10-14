/*
 * File:    lib.rs
 * Brief:   Top level module for quickbits
 *
 * Copyright: Copyright (C) 2023-2024 John Jekel
 * See the LICENSE file at the root of the project for licensing info.
 *
*/

#![doc = include_str!("../README.md")]

#![no_std]

/* ------------------------------------------------------------------------------------------------
 * Submodules
 * --------------------------------------------------------------------------------------------- */

mod bitmanip;
mod primitive_integer;

/* ------------------------------------------------------------------------------------------------
 * Uses
 * --------------------------------------------------------------------------------------------- */

#[doc(inline)]
pub use bitmanip::BitManip;

# quickbits

Minimal `#![no_std]` library for manipulating bits in primitive
Rust integers, with succinct Verilog-like syntax.

For when you don't need the full heft of the `bitvec` crate,
and want a bit more convenient syntax. :)

You can simply add `use quickbits::BitManip;` and start using the
methods on any primitive integer type in your code!

# Examples

```
use quickbits::BitManip;
let x: u32 = 0xABCD1234;
assert_eq!(x.b(3..=0), 0x4);
assert_eq!(x.b(7..=4), 0x3);
assert_eq!(x.b(31..=8), 0xABCD12);
assert_eq!(x.permute(&[23..=8, 7..=0, 31..=24]), 0xCD1234AB);
assert_eq!(x.toggle_bit(31), 0x2BCD1234);
assert_eq!(x.toggle_bit(31), 0x2BCD1234);
```

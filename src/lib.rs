/*!
# com-rs 0.1.4
Rust bindings for the Win32 [Component Object Model]
(https://msdn.microsoft.com/en-us/library/ms680573.aspx).

# Overview
This crate is composed of three main components:

* The [`com_interface!`] (macro.com_interface!.html) macro for
  defining new interface types.
* The [`ComPtr`](struct.ComPtr.html) type for making use of them.
* Definition of [`IUnknown`](struct.IUnknown.html), the base COM interface.
*/

// TODO:
// * Implement the rest of COM, this is just a tiny subset necessary to consume
//   IUnknown interfaces.
// * Tests for IUnknown/ComPtr, hard to test with no way of acquiring
//   IUnknown objects directly.

#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::fmt;

pub use comptr::{AsComPtr, ComInterface, ComPtr};
pub use unknown::IUnknown;

/// Interface identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct IID {
    /// First component, 32-bit value.
    pub data1: u32,
    /// Second component, 16-bit value.
    pub data2: u16,
    /// Third component, 16-bit value.
    pub data3: u16,
    /// Fourth component, array of 8-bit values.
    pub data4: [u8; 8],
}

/// Print IID in Windows registry format {XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}.
impl fmt::Display for IID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-\
                   {:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
            self.data1, self.data2, self.data3,
            self.data4[0], self.data4[1], self.data4[2], self.data4[3],
            self.data4[4], self.data4[5], self.data4[6], self.data4[7])
    }
}

#[test]
fn iid_display() {
    assert_eq!(IUnknown::iid().to_string(),
               "{00000000-0000-0000-C000-000000000046}");
}

/// Result type.
pub type HResult = i32;

#[macro_use]
mod macros;

mod comptr;
mod unknown;

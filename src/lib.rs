/*!
# com-rs 0.1.1
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

extern crate libc;
extern crate winapi;

// Re-export otherwise macro users need to have winapi in scope
pub use winapi::GUID as IID;

pub use comptr::{AsPtr, ComPtr, ComInterface};
pub use unknown::{IUnknown, Unknown};

#[macro_use]
mod macros;

mod comptr;
mod unknown;
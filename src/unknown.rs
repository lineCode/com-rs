// Copyright (c) 2016 com-rs developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::os::raw::c_void;

use super::{AsComPtr, HResult, IID};

/// Base interface for all COM types.
///
/// None of the methods on this struct should be called directly,
/// use [`ComPtr`](struct.ComPtr.html) instead.

#[derive(Debug)]
#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVtbl
}

#[allow(missing_debug_implementations)]
#[repr(C)]
#[doc(hidden)]
pub struct IUnknownVtbl {
    query_interface: extern "stdcall" fn(
        *const IUnknown, &IID, *mut *mut c_void) -> HResult,
    add_ref: extern "stdcall" fn(*const IUnknown) -> u32,
    release: extern "stdcall" fn(*const IUnknown) -> u32
}

#[link(name = "uuid")]
extern {
    static IID_IUnknown: IID;
}

impl IUnknown {
    /// Retrieves pointers to the supported interfaces on an object.
    /// Use [`ComPtr::from`](struct.ComPtr.html#method.from) instead.
    pub unsafe fn query_interface(&self, iid: &IID, object: *mut *mut c_void)
                                  -> HResult {
        ((*self.vtable).query_interface)(self, iid, object)
    }

    /// Increments the reference count for an interface on an object.
    /// Should never need to call this directly.
    pub unsafe fn add_ref(&self) -> u32 {
        ((*self.vtable).add_ref)(self)
    }

    /// Decrements the reference count for an interface on an object.
    /// Should never need to call this directly.
    pub unsafe fn release(&self) -> u32 {
        ((*self.vtable).release)(self)
    }
}

unsafe impl AsComPtr<IUnknown> for IUnknown { }

unsafe impl ::ComInterface for IUnknown {
    #[doc(hidden)]
    type Vtable = IUnknownVtbl;
    fn iid() -> ::IID { IID_IUnknown }
}

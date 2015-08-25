use std::os::raw::c_void;

use super::{AsComPtr, HResult, IID};

/// Base interface for all COM types.
///
/// None of the methods on this struct should be called directly,
/// use [`ComPtr`](struct.ComPtr.html) instead.

#[allow(raw_pointer_derive)]
#[derive(Debug)]
#[repr(C)]
pub struct IUnknown {
    vtable: *const IUnknownVtbl
}

#[repr(C)]
#[doc(hidden)]
pub struct IUnknownVtbl {
    query_interface: extern "stdcall" fn(
        *const IUnknown, &IID, *mut *mut c_void) -> HResult,
    add_ref: extern "stdcall" fn(*const IUnknown) -> u32,
    release: extern "stdcall" fn(*const IUnknown) -> u32
}

const IID_IUNKNOWN: IID = IID {
    data1: 0x00000000,
    data2: 0x0000,
    data3: 0x0000,
    data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]
};

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
    fn iid() -> ::IID { IID_IUNKNOWN }
}

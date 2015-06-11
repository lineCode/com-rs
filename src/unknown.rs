use winapi::{HRESULT, LPVOID, REFIID, ULONG};

use super::{AsPtr, IID};

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
    pub query_interface: extern "stdcall" fn(
        *const IUnknown, REFIID, *mut LPVOID) -> HRESULT,
    pub add_ref: extern "stdcall" fn(*const IUnknown) -> ULONG,
    pub release: extern "stdcall" fn(*const IUnknown) -> ULONG
}

// Interface Identifier for IUnknown
const IID_IUNKNOWN: IID = IID {
    Data1: 0x00000000,
    Data2: 0x0000,
    Data3: 0x0000,
    Data4: [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]
};

/// Base trait for all COM types.
pub trait Unknown {
    /// Retrieves pointers to the supported interfaces on an object.
    /// Use [`ComPtr::from`](struct.ComPtr.html#method.from) instead.
    unsafe fn query_interface(&self, iid: REFIID, object: *mut LPVOID)
                              -> HRESULT {
        let obj: &&IUnknown = ::std::mem::transmute(&self);
        ((*obj.vtable).query_interface)(*obj, iid, object)
    }

    /// Increments the reference count for an interface on an object.
    /// Should never need to call this directly.
    unsafe fn add_ref(&self) -> ULONG {
        let obj: &&IUnknown = ::std::mem::transmute(&self);
        ((*obj.vtable).add_ref)(*obj)
    }

    /// Decrements the reference count for an interface on an object.
    /// Should never need to call this directly.
    unsafe fn release(&self) -> ULONG {
        let obj: &&IUnknown = ::std::mem::transmute(&self);
        ((*obj.vtable).release)(*obj)
    }
}

impl Unknown for IUnknown { }

unsafe impl AsPtr<IUnknown> for IUnknown { }

unsafe impl ::ComInterface for IUnknown {
    #[doc(hidden)]
    type Vtable = IUnknownVtbl;
    fn iid() -> ::IID { IID_IUNKNOWN }
}
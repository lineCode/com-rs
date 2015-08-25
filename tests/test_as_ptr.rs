#[macro_use]
extern crate com_rs;

use com_rs::{ComPtr, IUnknown};

com_interface! {
    /// IFoo struct
    interface IFoo: IUnknown {
        iid: IID_IFOO { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
        vtable: IFooVtbl,
        /// foo fn
        fn foo() -> ();
    }
}

com_interface! {
    /// IBar struct
    interface IBar: IFoo, IUnknown {
        iid: IID_IBAR { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
        vtable: IBarVtbl,
        /// bar method
        fn bar() -> ();
    }
}

com_interface! {
    /// IBaz struct
    interface IBaz: IBar, IFoo, IUnknown {
        iid: IID_IBAZ { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
        vtable: IBazVtbl,
        /// baz method
        fn baz() -> ();
    }
}

#[test]
fn test_as_ptr() {
    use std::os::raw::c_void;

    let foo = ComPtr::<IFoo>::new();
    let bar = ComPtr::<IBar>::new();
    let baz = ComPtr::<IBaz>::new();

    let _foo_ptr1: *const c_void = foo.as_ptr();
    let _foo_ptr2: *const IUnknown = foo.as_ptr();
    let _foo_ptr3: *const IFoo = foo.as_ptr();

    let _bar_ptr1: *const c_void = bar.as_ptr();
    let _bar_ptr2: *const IUnknown = bar.as_ptr();
    let _bar_ptr3: *const IFoo = bar.as_ptr();
    let _bar_ptr4: *const IBar = bar.as_ptr();

    let _baz_ptr1: *const c_void = baz.as_ptr();
    let _baz_ptr2: *const IUnknown = baz.as_ptr();
    let _baz_ptr3: *const IFoo = baz.as_ptr();
    let _baz_ptr4: *const IBar = baz.as_ptr();
    let _baz_ptr5: *const IBaz = baz.as_ptr();
}

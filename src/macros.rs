/**
Macro for generating COM interface definitions.

# Usage
```
#[macro_use]
extern crate com_rs;
use com_rs::IUnknown;

com_interface! {
    interface IFoo: IUnknown {
        iid: IID_IFOO {
            0x12345678, 0x90AB, 0xCDEF,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF
        },
        vtable: IFooVtbl,
        fn foo() -> bool;
    }
}
# fn main() { }
```

This example defines an interface called `IFoo`. In this case, the base type is
IUnknown, the root COM type. The IID for the interface must also be defined,
along with the name of the vtable type, `IFooVtbl`. This isn't publicly exposed,
but there is currently no way to generate an ident within a macro so the callee
must define one instead.

The trait `Foo` defines the methods available for the interface, in this case
a single method named `foo`. Note that any methods that return no value
(e.g. the `void` type in C/C++) should return the unit type `()`.

## Inheritance
To define interfaces with a deeper hierarchy, add additional parent identifiers
to the type definitions. e.g:

```
# #[macro_use]
# extern crate com_rs;
# use com_rs::IUnknown;
# com_interface! {
#     interface IFoo: IUnknown {
#         iid: IID_IFOO { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
#         vtable: IFooVtbl,
#         fn foo() -> bool;
#     }
# }
com_interface! {
    interface IBar: IFoo, IUnknown {
        iid: IID_IBAR {
            0x12345678, 0x90AB, 0xCDEF,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF
        },
        vtable: IBarVtbl,
        fn bar(baz: i32) -> ();
    }
}
# fn main() { }
```

This example defines an interface called `IBar` which extends `IFoo` from the
previous example. Note that it is necessary to specify the parent types
for both the interface and trait declarations.

The interface hierarchy automates pointer conversion using the `AsPtr` trait,
and the trait hierarchy automatically implements the parent methods for the
child interface.
*/
#[macro_export]
macro_rules! com_interface {
    (
        $(#[$iface_attr:meta])*
        interface $iface:ident: $base_iface:ty {
            iid: $iid:ident { $d1:expr, $d2:expr, $d3:expr, $($d4:expr),+ },
            vtable: $vtable:ident,
            $(
                $(#[$fn_attr:meta])*
                fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
            )*
        }
    ) =>
    (
        #[allow(missing_debug_implementations)]
        #[doc(hidden)]
        #[repr(C)]
        pub struct $vtable {
            base: <$base_iface as $crate::ComInterface>::Vtable,
            $($func: extern "stdcall" fn(*const $iface, $($t),*) -> $rt),*
        }

        $(#[$iface_attr])*
        #[allow(raw_pointer_derive)]
        #[derive(Debug)]
        #[repr(C)]
        pub struct $iface {
            vtable: *const $vtable
        }

        impl $iface {
            $($(#[$fn_attr])*
            pub unsafe fn $func(&self, $($i: $t),*) -> $rt {
                ((*self.vtable).$func)(self $(,$i)*)
            })*
        }

        impl ::std::ops::Deref for $iface {
            type Target = $base_iface;
            fn deref(&self) -> &$base_iface {
                unsafe { ::std::mem::transmute(self) }
            }
        }

        const $iid: $crate::IID = $crate::IID {
            data1: $d1,
            data2: $d2,
            data3: $d3,
            data4: [$($d4),+]
        };

        unsafe impl $crate::AsComPtr<$iface> for $iface {}
        unsafe impl $crate::AsComPtr<$base_iface> for $iface {}

        unsafe impl $crate::ComInterface for $iface {
            #[doc(hidden)]
            type Vtable = $vtable;
            fn iid() -> $crate::IID { $iid }
        }
    );


    (
        $(#[$iface_attr:meta])*
        interface $iface:ident: $base_iface:ty, $($extra_base:ty),+ {
            iid: $iid:ident { $d1:expr, $d2:expr, $d3:expr, $($d4:expr),+ },
            vtable: $vtable:ident,
            $(
                $(#[$fn_attr:meta])*
                fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty;
            )*
        }
    ) => (
        com_interface! {
            $(#[$iface_attr])*
            interface $iface: $base_iface {
                iid: $iid { $d1, $d2, $d3, $($d4),+ },
                vtable: $vtable,
                $($(#[$fn_attr])* fn $func($($i: $t),*) -> $rt;)*
            }
        }

        $(unsafe impl $crate::AsComPtr<$extra_base> for $iface {})*
    )
}

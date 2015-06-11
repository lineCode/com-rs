/**
Macro for generating COM interface definitions.

# Usage
```
#[macro_use]
extern crate com_rs;
use com_rs::{IUnknown, Unknown};

com_interface! {
    struct IFoo: IUnknown {
        iid: IID_IFOO {
            0x12345678, 0x90AB, 0xCDEF,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF
        },
        vtable: IFooVtbl
    }
    trait Foo: Unknown {
        fn foo() -> bool
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
# use com_rs::{IUnknown, Unknown};
# com_interface! {
#     struct IFoo: IUnknown {
#         iid: IID_IFOO { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
#         vtable: IFooVtbl
#     }
#     trait Foo: Unknown { fn foo() -> bool }
# }
com_interface! {
    struct IBar: IFoo, IUnknown {
        iid: IID_IBAR {
            0x12345678, 0x90AB, 0xCDEF,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF
        },
        vtable: IBarVtbl
    }
    trait Bar: Foo, Unknown {
        fn bar(baz: i32) -> ()
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
        struct $iface:ident: $($base_iface:ty),* {
            iid: $iid:ident { $d1:expr, $d2:expr, $d3:expr, $($d4:expr),+ },
            vtable: $vtable:ident
        }
        trait $tr:ident: $($base_tr:ident),+ {
            $($(#[$fn_attr:meta])*
            fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty),*
        }
    ) =>
    (
        __com_struct! {
            $(#[$iface_attr])*
            #[allow(raw_pointer_derive)]
            #[derive(Debug)]
            struct $iface: $($base_iface),* {
                vtable: $vtable {
                    $(fn $func($($t),*) -> $rt),*
                }
            }
        }

        __com_trait! {
            struct $iface;
            trait $tr: $($base_tr),* {
                $($(#[$fn_attr])*
                fn $func($($i: $t),*) -> $rt),*
            }
        }

        __iid!($iid = $d1, $d2, $d3, $($d4),+);

        // Implement interface traits
        impl $tr for $iface { }
        $(impl $base_tr for $iface { })*

        // Implement pointer conversion trait
        unsafe impl $crate::AsPtr<$iface> for $iface { }
        $(unsafe impl $crate::AsPtr<$base_iface> for $iface { })*

        // Implement helper trait
        unsafe impl $crate::ComInterface for $iface {
            #[doc(hidden)]
            type Vtable = $vtable;
            fn iid() -> $crate::IID { $iid }
        }
    );
}

// NOTE: all macros below this line are hidden because they shouldn't be used
// directly, but they have to be exported so com_interface can use them.

#[doc(hidden)]
#[macro_export]
macro_rules! __iid {
    ($id:ident = $d1:expr, $d2:expr, $d3:expr, $($d4:expr),+) => (
        const $id: $crate::IID = $crate::IID {
            Data1: $d1,
            Data2: $d2,
            Data3: $d3,
            Data4: [$($d4),+]
        };
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __com_struct {
    (
        $(#[$attr:meta])*
        struct $name:ident: $base_name:ty {
            vtable: $vtable:ident {
                $(fn $func:ident($($t:ty),*) -> $rt:ty),*
            }
        }
    ) => (
        #[repr(C)]
        $(#[$attr])*
        pub struct $name {
            vtable: *const $vtable
        }

        #[repr(C)]
        #[doc(hidden)]
        pub struct $vtable {
            base: <$base_name as $crate::ComInterface>::Vtable,
            $(pub $func: extern "stdcall" fn(*const $name, $($t),*) -> $rt),*
        }
    );
    (
        $(#[$attr:meta])* struct $name:ident: $base_name:ty, $($x:ty),* {
            vtable: $vtable:ident {
                $(fn $func:ident($($t:ty),*) -> $rt:ty),*
            }
        }
    ) => (
        // Discard additional base interfaces
        __com_struct! {
            $(#[$attr])* struct $name: $base_name {
                vtable: $vtable {
                    $(fn $func($($t),*) -> $rt),*
                }
            }
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __com_trait {
    (
        struct $iface:ident;
        trait $tr:ident: $base_tr:ident {
            $($(#[$fn_attr:meta])*
            fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty),*
        }
    ) => (
        pub trait $tr: $base_tr {
            $($(#[$fn_attr])*
            unsafe fn $func(&self, $($i: $t),*) -> $rt{
                let obj: &&$iface = ::std::mem::transmute(&self);
                ((*obj.vtable).$func)(*obj $(,$i)*)
            })*
        }
    );
    (
        struct $iface:ident;
        trait $tr:ident: $base_tr:ident, $($x:ident),* {
            $($(#[$fn_attr:meta])*
            fn $func:ident($($i:ident: $t:ty),*) -> $rt:ty),*
        }
    ) => (
        // Discard additional base traits
        __com_trait! {
            struct $iface;
            trait $tr: $base_tr {
                $($(#[$fn_attr])* fn $func($($i: $t),*) -> $rt),*
            }
        }
    )
}
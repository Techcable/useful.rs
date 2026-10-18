//! Macros to mark code and imports as used, avoiding `#[warn(dead_code)]`.
//!
//! # Examples
//! This can be used as a cleaner alternative to `#[deny(unused)]`.
//! The following code gives no warnings, even though `Debug` is not used in the program.
//! ```
//! #[deny(warnings)]
//! use core::fmt::Debug;
//! useful::used!(use Debug);
//! ```
//!
//! When used with functions, it is more precise than `#[allow(unused)]` and avoids
//! suppressing dead code warnings for the body of the functions.
//! The following code gives a warning on the unused variable `x`,
//! without warning about the unused function `foo`.
//! ```
//! fn foo() {
//!     let x = 3;
//! }
//! useful::used!(use foo);
//! ```
//!
//! # Prefixes
//! Each item must be prefixed with either `use` or `let`,
//! which determines how it is marked as used:
//! - `use path` imports the path, expanding to `use path as _;`.
//!   This works for anything that can be imported,
//!   including modules, types, traits, functions, and constants.
//! - `let expr` evaluates an expression, expanding to `let _ = expr;`.
//!   This is usually just a path, which is needed for associated functions,
//!   methods, and associated constants, because they cannot be imported with `use`.
//!   Other expressions are useful for disambiguating generics, such as `let foo("")`.
//!
//! ```
//! #![deny(unused)]
//! # #![allow(unused_extern_crates)]
//! use core::fmt::Debug;
//!
//! struct Foo;
//! impl Foo {
//!     fn new() -> Self {
//!         Foo
//!     }
//! }
//! useful::used!(use Debug, use Foo, let Foo::new);
//! ```
//!
//! Support for paths without a prefix has been removed in v0.2.0.
//!
//! # Documentation-Only Imports
//! Currently, rustc will warn about unused imports if those imports are only used in doc comments.
//! This can be worked around by using the [`used_for_docs!`] macro.
//! It works exactly the same as [`used!`], but it is clearer why you are using it.
//! ```
//! #![deny(unused_imports)]
//! use std::collections::HashMap;
//! useful::used_for_docs!(use HashMap);
//!
//! /// This is almost as awesome as a [`HashMap`]
//! struct AwesomeStruct(u32, u32);
//! ```
#![cfg_attr(not(test), no_std)]

/// Mark the specified paths as being used,
/// avoiding lints for dead code or unused imports.
///
/// Accepts a comma-separated list of paths prefixed with `use`,
/// or expressions prefixed with `let`:
/// ```
/// #![deny(unused)]
/// # #![allow(unused_extern_crates)]
/// use core::fmt::Debug;
///
/// struct Foo;
/// impl Foo {
///     const BAR: u32 = 3;
///     fn new() -> Self {
///         Foo
///     }
/// }
/// useful::used!(use Debug, use Foo, let Foo::new, let Foo::BAR);
/// ```
///
/// # Prefixes
/// The prefix determines how each path is marked as used.
///
/// Support for unprefixed paths has been removed in v0.2.0.
///
/// ## `use path`
/// Imports the path, expanding to `#[allow(unused)] use path as _;`.
/// This works for anything that can be imported,
/// including modules, types, traits, functions, and constants.
///
/// ## `let expr`
/// Evaluates an expression, expanding to `let _ = expr;`.
/// The expression is never actually executed, only type checked.
///
/// This is usually just a path, which is needed for associated functions,
/// methods, and associated constants, because they cannot be imported with `use`.
///
/// Generic functions need explicit type arguments, such as `let Vec::<u8>::new`.
/// Functions with `impl Trait` arguments cannot be given explicit type arguments,
/// so call them with constant arguments to assist type inference instead:
/// ```
/// #![deny(unused)]
/// # #![allow(unused_extern_crates)]
/// use core::fmt::Debug;
///
/// fn foo(_x: impl Debug) {}
/// useful::used!(let foo(""));
/// ```
/// Since the call is never executed, the arguments only need to have the right types.
/// Casting to a function pointer also works, such as `let foo as fn(&'static str)`.
///
/// Be careful, because expressions that don't reference anything will
/// be accepted without marking anything as used, such as `let 1 + 2`.
///
/// # Advantages
/// This is more precise than applying `#[allow(unused)]` to an item because
/// it only applies to the function/struct directly and not its children.
///
/// For example, this following code succeeds without warning or error
/// on the unused variable `x` because the `#[allow(unused)]` has a recursive effect.
/// ```
/// #[allow(unused)]
/// fn foo() {
///     let x = 3;
/// }
/// ```
///
/// In contrast, the following code gives the error expected for the unused variable
/// without giving any warning for the `foo` function being unused.
/// ```compile_fail
/// #![deny(unused)]
/// # #![allow(unused_extern_crates)]
///
/// fn foo() {
///     let x = 3;
/// }
/// useful::used!(use foo);
/// ```
///
/// # Limitations
/// Cannot work with local variables inside an expression contact.
/// Use `let _ = var` instead to achieve a similar effect.
///
/// Cannot work with `#[forbid(unused)]`.
#[macro_export]
macro_rules! used {
    // implementation detail, which processes one item at a time
    //
    // `$attrs` is a bracketed list of attributes, which is applied to each generated item.
    // It is forwarded as a single token tree because `$(#[$attr])*`
    // cannot be repeated inside the `$(...)?` used for recursion.
    // Errors are never subject to these attributes,
    // so syntax is checked even if the items are disabled by `#[cfg(...)]`.
    (@with_attrs $attrs:tt @impl) => {};
    (@with_attrs $attrs:tt @impl let $x:expr $(, $($rest:tt)*)?) => {
        // Before Rust 1.89, the dead code lint ignored the body of `const _`,
        // but it has always treated items with `#[allow(dead_code)]` as used.
        // The block gives each generated function its own scope, avoiding name conflicts.
        {
            $crate::used!(@emit $attrs
                #[allow(unused)]
                fn __useful_let_used() {
                    let _ = $x;
                }
            );
        }
        $($crate::used!(@with_attrs $attrs @impl $($rest)*);)?
    };
    (@with_attrs $attrs:tt @impl use $x:path $(, $($rest:tt)*)?) => {
        $crate::used!(@emit $attrs
            #[allow(unused)]
            use $x as _;
        );
        $($crate::used!(@with_attrs $attrs @impl $($rest)*);)?
    };
    (@with_attrs $attrs:tt @impl $x:path $(, $($rest:tt)*)?) => {
        compile_error!(concat!(
            "depending on their type, paths must be prefixed by `let` or `use` (got `",
            stringify!($x),
            "`). Support for unprefixed paths was removed in v0.2.0",
        ));
    };
    // must not fall through to the catch-all branch below, which would recurse forever
    (@with_attrs $attrs:tt @impl $($invalid:tt)+) => {
        compile_error!(concat!(
            "expected a path, prefixed by `let` or `use`, found `",
            stringify!($($invalid)+),
            "`",
        ));
    };
    (@emit [$(#[$attr:meta])*] $item:item) => {
        $(#[$attr])*
        $item
    };
    (@with_attrs $attrs:tt) => {};
    (@with_attrs $attrs:tt $($items:tt)+) => {
        const _: () = {
            $crate::used!(@with_attrs $attrs @impl $($items)+);
        };
    };
    ($($items:tt)*) => {
        $crate::used!(@with_attrs [] $($items)*);
    };
}

/// Mark the specified paths as being used for documentation comments.
///
/// Behaves exactly the same as [`used!`],
/// but with a different name for clarity of purpose.
/// Items should be prefixed with `use` or `let`, as described in [`used!`].
///
/// # Example
/// ```
/// #![deny(unused_imports)]
/// use std::collections::HashMap;
/// useful::used_for_docs!(use HashMap);
///
/// /// This is almost as awesome as a [`HashMap`]
/// struct AwesomeStruct(u32, u32);
/// ```
pub use crate::used as used_for_docs;

/// Mark the specified paths as being used for tests.
///
/// Behaves like [`used!`] if and only if `#[cfg(not(test))]` is true,
/// meaning you will get warnings if the code is not actually used in tests.
/// Items should be prefixed with `use` or `let`, as described in [`used!`].
///
/// The syntax is checked even when `#[cfg(test)]` is true.
///
/// # Example
/// ```
/// #![deny(unused)]
/// # #![allow(unused_extern_crates)]
///
/// struct Foo;
/// impl Foo {
///     fn new() -> Self {
///         Foo
///     }
/// }
/// // only called from `#[cfg(test)]` code
/// fn helper() {}
/// useful::used_for_tests!(use helper, let Foo::new);
/// ```
#[macro_export]
macro_rules! used_for_tests {
    ($($x:tt)*) => {
        // The `cfg` must not be applied to the macro invocation,
        // because then the input would never be checked when `cfg(test)` is set.
        $crate::used!(@with_attrs [#[cfg(not(test))]] $($x)*);
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn unused_imports() {
        use std::collections::HashMap;
        use std::hash::Hash;
        used!(use HashMap, use Hash);
    }
    #[test]
    fn unused_functions() {
        fn unused() {}
        used!(use unused);
    }
}

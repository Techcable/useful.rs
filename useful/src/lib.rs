//! Macros to mark code and imports as used, avoiding `#[warn(dead_code)]`.
//!
//! # Examples
//! This can be used as a cleaner alternative to `#[deny(unused)]`.
//! The following code gives no warnings, even though `Debug` is not used in the program.
//! ```
//! #[deny(warnings)]
//! use core::fmt::Debug;
//! useful::used!(Debug);
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
//! useful::used!(foo);
//! ```
//!
//! # Documentation-Only Imports
//! Currently, rustc will warn about unused imports if those imports are only used in doc comments.
//! This can be worked around by using the [`used_for_docs!`] macro.
//! It works exactly the same as [`used!`], but it is clearer why you are using it.
//! ```
//! #![deny(unused_imports)]
//! use std::collections::HashMap;
//! useful::used_for_docs!(HashMap);
//!
//! /// This is almost as awesome as a [`HashMap`]
//! struct AwesomeStruct(u32, u32);
//! ```
#![cfg_attr(not(test), no_std)]

/// Mark the specified paths as being used,
/// avoiding lints for dead code or unused imports.
///
/// # Advantages
/// This is more precise than applying `#[allow(unused)]` because
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
///
/// fn foo() {
///     let x = 3;
/// }
/// useful::used!(foo);
/// ```
///
/// # Limitations
/// Cannot work with local variables inside an expression contact.
/// Use `let _ = var` instead to achieve a similar effect.
///
/// Cannot work with `#[forbid(unused)]`.
#[macro_export]
macro_rules! used {
    () => {};
    ($($x:path),+ $(,)?) => {
        const _: () = {
            $(#[allow(unused)]
            use $x as _;)*
        };
    };
}

/// Mark the specified paths as being used for documentation comments.
///
/// Behaves exactly the same as [`used!`],
/// but with a different name for clarity of purpose.
///
/// # Example
/// ```
/// #![deny(unused_imports)]
/// use std::collections::HashMap;
/// useful::used_for_docs!(HashMap);
///
/// /// This is almost as awesome as a [`HashMap`]
/// struct AwesomeStruct(u32, u32);
/// ```
pub use crate::used as used_for_docs;

#[cfg(test)]
mod test {
    #[test]
    fn unused_imports() {
        use std::collections::HashMap;
        use std::hash::Hash;
        used!(HashMap, Hash);
    }
    #[test]
    fn unused_functions() {
        fn unused() {}
        used!(unused);
    }
}

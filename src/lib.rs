//! libc - Raw FFI bindings to platforms' system libraries
#![crate_name = "libc_mips"]
#![crate_type = "rlib"]
#![allow(
    renamed_and_removed_lints, // Keep this order.
    unknown_lints, // Keep this order.
    bad_style,
    overflowing_literals,
    improper_ctypes,
    // This lint is renamed but we run CI for old stable rustc so should be here.
    redundant_semicolon,
    redundant_semicolons,
    unused_macros,
    unused_macro_rules,
)]
#![cfg_attr(libc_deny_warnings, deny(warnings))]
// Attributes needed when building as part of the standard library
#![cfg_attr(feature = "rustc-dep-of-std", feature(link_cfg, no_core))]
#![cfg_attr(libc_thread_local, feature(thread_local))]
// Enable extra lints:
#![cfg_attr(feature = "extra_traits", deny(missing_debug_implementations))]
#![deny(missing_copy_implementations, safe_packed_borrows)]
#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]

#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(feature = "rustc-dep-of-std")] {
        extern crate rustc_std_workspace_core as core;
        #[allow(unused_imports)]
        use core::iter;
        #[allow(unused_imports)]
        use core::ops;
        #[allow(unused_imports)]
        use core::option;
    }
}

#[doc(hidden)]
#[allow(unused_imports)]
use core::clone::Clone;
#[allow(unused_imports)]
use core::ffi;
#[allow(unused_imports)]
use core::fmt;
#[allow(unused_imports)]
use core::hash;
#[doc(hidden)]
#[allow(unused_imports)]
use core::marker::{Copy, Send, Sync};
#[allow(unused_imports)]
use core::mem;
#[allow(unused_imports)]
use core::num;
#[doc(hidden)]
#[allow(unused_imports)]
use core::option::Option;

mod fixed_width_ints;
pub use fixed_width_ints::*;

mod unix;
pub use unix::*;

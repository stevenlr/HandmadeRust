#![feature(
    unsize,
    coerce_unsized,
    optin_builtin_traits,
    format_args_nl,
    allow_internal_unstable,
    wrapping_int_impl
)]
#![no_std]

pub mod alloc;
mod ashared;
mod bitflags;
pub mod containers;
pub mod dl;
pub mod fmt;
pub mod fs;
pub mod hash;
pub mod io;
pub mod linalg;
pub mod num;
pub mod serde;
mod shared;
pub mod str;
pub mod sync;
pub mod thread;
mod unique;
pub mod zlib;

pub use ashared::{AShared, AWeak};
pub use bitflags::*;
pub use shared::{Shared, Weak};
pub use unique::Unq;

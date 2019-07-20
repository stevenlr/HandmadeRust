#![feature(unsize, coerce_unsized, optin_builtin_traits)]
#![no_std]

pub mod alloc;
mod ashared;
mod bitflags;
pub mod containers;
pub mod dl;
pub mod fs;
pub mod hash;
pub mod io;
pub mod linalg;
pub mod num;
pub mod serde;
mod shared;
pub mod str;
pub mod thread;
mod unique;
pub mod zlib;

pub use ashared::{AShared, AWeak};
pub use bitflags::*;
pub use shared::{Shared, Weak};
pub use unique::Unq;

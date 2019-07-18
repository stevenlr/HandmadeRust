#![feature(unsize, coerce_unsized, optin_builtin_traits)]
#![no_std]

pub mod alloc;
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
mod unique;
pub mod zlib;

pub use bitflags::*;
pub use shared::{Shared, Weak};
pub use unique::Unq;

#![feature(unsize, coerce_unsized, optin_builtin_traits)]

pub mod alloc;
mod bitflags;
pub mod containers;
pub mod dl;
pub mod fs;
pub mod hash;
pub mod io;
pub mod linalg;
pub mod num;
mod shared;
pub mod str;
mod unique;
pub mod zlib;

pub use bitflags::*;
pub use shared::{Shared, Weak};
pub use unique::Unq;

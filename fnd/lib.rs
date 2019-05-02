#![feature(unsize, coerce_unsized, optin_builtin_traits)]

pub mod alloc;
mod unique;
mod shared;
pub mod containers;
pub mod hash;
pub mod str;

pub use unique::Unq;
pub use shared::Shared;
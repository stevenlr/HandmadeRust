#![feature(unsize, coerce_unsized, optin_builtin_traits)]

pub mod alloc;
pub mod containers;
pub mod hash;
mod shared;
pub mod str;
mod unique;

pub use shared::Shared;
pub use unique::Unq;

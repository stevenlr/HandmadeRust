Handmade Rust [![Build Status](https://travis-ci.org/stevenlr/HandmadeRust.svg?branch=master)](https://travis-ci.org/stevenlr/HandmadeRust)
=====================

This is a project to create a Vulkan rendering engine using only the Rust `core` library, no `std`, `alloc`, or any other external dependencies. It is accompanied by a [blog series](http://stevenlr.com).

Right now it only targets Windows. This may change in this future.

Crates
----------------

 - [`fnd`](fnd): A standard library replacement with allocator-aware containers.
 - [`hash_macro`](hash_macro): Compile-time string literal hashing. Requires `proc_macro_hygiene` at the moment.
 - [`main`](main): The main application.
 - [`vk`](vk): Vulkan bindings, generated using [stevenlr/VkXml](https://github.com/stevenlr/VkXml).
 - [`win32`](win32): Raw Win32 API bindings.

Blog series
----------------

 - [Part 1: Introduction & Allocators](http://stevenlr.com/posts/handmade-rust-1-allocators/)
 - [Part 2: Unq, an allocator-aware Box](http://stevenlr.com/posts/handmade-rust-2-unq/)
 - [Part 3: Containers](http://stevenlr.com/posts/handmade-rust-3-containers/)
 - [Part 4: Generating Vulkan bindings](http://stevenlr.com/posts/handmade-rust-4-vulkan-bindings/)
[![BSD-2-Clause License](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![docs.rs](https://docs.rs/checked_array/badge.svg)](https://docs.rs/checked_array)
[![crates.io](https://img.shields.io/crates/v/checked_array.svg)](https://crates.io/crates/checked_array)
[![Download numbers](https://img.shields.io/crates/d/checked_array.svg)](https://crates.io/crates/checked_array)
[![dependency status](https://deps.rs/crate/checked_array/0.1.0/status.svg)](https://deps.rs/crate/checked_array/0.1.0)
[![Travis CI](https://travis-ci.org/KizzyCode/checked_array-rust.svg?branch=master)](https://travis-ci.org/KizzyCode/checked_array-rust)
[![Appveyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/checked_array-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/checked-array-rust)


# `checked_array`
This crate provides an API abstraction for array-like linear collections which exports *checked APIs only*.


## Why `checked_array`
Rust's `std::vec` and `std::slice` modules have the problem, that they expose APIs which will implicitly panic if called
with a wrong argument. Common examples are:
 - `split_at*`
 - `rotate*`
 - `clone_from_slice`
 - `copy_from_slice`
 - etc.

This is a problem because it does not only violate common design principles for safe languages like "explicit is better
than implicit", but it is also not very typical for Rust itself, which usually provides fallible APIs using `Option` or
`Result`.

`checked_array` tries to address this problem by defining checked APIs and providing an opaque generic wrapper which
only implements these checked APIs.


## `checked_array` and `alloc`
There is one exception to the safety guarantees of `checked_array`: if the wrapped type uses `alloc`/`std::alloc`, __we
cannot catch any allocation errors__.

To make the user aware of this problem, we introduce the `WillPanic` error type if for wrapped types that will panic
allocation errors.
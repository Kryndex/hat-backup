// Copyright 2014 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! High level Hat API

// Unstable APIs:
#![cfg_attr(feature = "benchmarks", feature(test))]

// Standard Rust imports.
#[macro_use]
extern crate log;
extern crate rand;
#[cfg(feature = "benchmarks")]
extern crate test;
extern crate time;

// Rust crates.
extern crate byteorder;
extern crate capnp;
extern crate sodiumoxide;
extern crate libsodium_sys;
extern crate rustc_serialize;
extern crate scoped_pool;
extern crate void;
extern crate filetime;

// Error definition macros.
#[macro_use]
extern crate error_type;

// Diesel supplies our SQLite wrapper.
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

// Testing utilities.
#[cfg(test)]
extern crate quickcheck;

// Submodules
pub mod backend;
mod blob;
mod crypto;
mod db;
mod errors;
mod gc;
mod hash;
pub mod hat;
mod key;
mod snapshot;
mod tags;
mod util;

// Re-export the main type

pub use hat::Hat;

// The capnp module generated by build.rs and used internally
#[allow(dead_code)]
mod root_capnp {
    include!(concat!(env!("OUT_DIR"), "/root_capnp.rs"));
}

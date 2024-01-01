#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,

    clippy::style,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,

    //clippy::restriction,
    clippy::todo,
    clippy::mem_forget,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::exit,
    clippy::verbose_file_reads,
    clippy::str_to_string,
    clippy::unwrap_used,
    clippy::expect_used,

    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    unused
)]
#![forbid(unsafe_code)]
#![allow(elided_lifetimes_in_paths)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used,))]

pub use types::Database as DatabaseType;
pub mod app;

mod auth;
mod controller;
mod repository;
mod service;
mod types;

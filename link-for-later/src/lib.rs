#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub use crate::types::Database as DatabaseType;
pub mod app;

mod controller;
mod repository;
mod service;
mod state;
mod types;

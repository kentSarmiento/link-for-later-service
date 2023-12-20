#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub use crate::types::Database as RepositoryType;
pub mod app;

mod controller;
mod repository;
mod service;
mod types;

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

pub use types::Database as DatabaseType;
pub mod app;

mod controller;
mod dto;
mod entity;
mod repository;
mod service;
mod types;

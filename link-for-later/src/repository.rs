use crate::types::repository::Links;

pub mod mongodb;

#[derive(Default)]
pub struct Base {}

impl Links for Base {}

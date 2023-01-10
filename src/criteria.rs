use crate::args;
use clap::{Parser};


pub struct Criteria {
  size: (u64, u64),
  include_extensions: Vec<String>,
  exclude_extensions: Vec<String>,
  max_depth: u8,
  age: f64,
  newer: bool,
}

impl Criteria {
  pub fn new(args:Args) -> Criteria {
    let args = Args::parse();
  }
}
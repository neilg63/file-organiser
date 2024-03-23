extern crate chrono;

mod args;
mod resource_row;
mod utils;
mod path_info;
mod criteria;
mod matches;
mod run;
mod start;
mod manage;

use crate::start::init;

/// Initialise the command prompt
/// By default it will give an overview of the current directory
/// to a max of 5 levels of nested directories
fn main() {
	init();
}
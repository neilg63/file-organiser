extern crate chrono;

mod args; // Manage command line arguments
mod resource_row; // Manage flattened directory contents
mod utils; // Miscellaneous functions
mod path_info; // Custom path info
mod criteria; // Scan criteria and operations
mod matches; // Build optional regular expression for pattern matching
mod run; // Scan the work directory after processing all options
mod start; // Initialise the utility after validating core arguments
mod manage; // Handle copy, move and delete operations

use crate::start::init;

/// Initialise the command prompt
/// By default it will give an overview of the current directory
/// to a max of 5 levels of nested directories
fn main() {
	init();
}
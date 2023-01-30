extern crate chrono;
// use std::rc::Rc;
use std::io::Write;
use clap::{Parser};
use args::Args;
use color_print::cprintln;
use utils::pluralize_64;

mod args;
mod resource_row;
mod utils;
mod path_info;
mod criteria;
mod matches;
mod run;

use crate::path_info::PathInfo;
use crate::resource_row::*;
use crate::criteria::*;
use crate::run::*;

pub fn action_prompt(text: &str) -> bool {
    let mut line = String::new();
    print!("{} (Y/n)", text);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    match line.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false
    }
}

fn main() {
    let args = Args::parse();
    let path_info = PathInfo::new_from_args(&args);
    let criteria = Criteria::new(&args, path_info.pattern);
    if path_info.exists {
        let details = DetailLevel::new(&args.list, &args.groups, &args.void);
        let resource_tree = scan_directory(&path_info.canonical, &details, &criteria, false);
        criteria.show();
        
        if criteria.delete_with_prompt() {
            let num_matched_files = resource_tree.num_files();
            if num_matched_files > 0 {
                let file_word = pluralize_64("file", "s", num_matched_files as u64);
                if action_prompt(format!("Are you sure you want to delete the {} above {}?", num_matched_files, file_word).as_str()) {
                    scan_directory(&path_info.canonical, &details, &criteria, true);
                    cprintln!("<green>deleted</green>");
                } else {
                    cprintln!("<red>Not deleted</red>");
                }
            } else {
                cprintln!("<red>No matched files to delete</red>");
            }
        }
    } else {
       cprintln!("The target directory <red>{}</red> does not exist", path_info.input); 
    }
    
}

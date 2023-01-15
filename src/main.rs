extern crate chrono;
// use std::rc::Rc;
use std::io::Write;
use walkdir::{WalkDir, DirEntry};
//use clap::{Parser, command, Arg, ArgAction};
use clap::{Parser};
use args::Args;
use color_print::cprintln;

mod args;
mod resource_row;
mod utils;
mod path_info;
mod criteria;
mod matches;

use crate::path_info::PathInfo;
use crate::resource_row::*;
use crate::criteria::*;
use crate::matches::*;


pub fn scan_directory(path_str:String, details: DetailLevel, criteria: &Criteria, delete_confirmed: bool) {
    let mut root_ref:Option<DirEntry> = None;
    let mut resource_tree: ResourceTree = ResourceTree::new(criteria.max_depth);
    let target_dir = WalkDir::new(path_str).min_depth(0).max_depth(criteria.max_depth as usize).follow_links(true);
    for file in target_dir.into_iter().filter_map(|file| file.ok()) {
        let ft = file.file_type();
        let mut not_excluded = true;
        if ft.is_dir() {
            if root_ref.is_none() {
                root_ref = Some(file.clone());
                resource_tree.add_root(&file);
            }
            let r_set = ResourceSet::new(&file);
            if ft.is_dir() && root_ref.is_some() {
               not_excluded = r_set.is_not_excluded_dir(&criteria.exclude_directories, &root_ref);
            }
            if not_excluded {
                resource_tree.push(&r_set);
            }
        } else {
            let resource = ResourceRow::new(&file);
            if resource.matches_criteria(&criteria) {
                if resource.depth() < 2 {
                    resource_tree.add_to_parent(&resource);
                }  else {
                    if resource.is_not_in_excluded_dir(&criteria.exclude_directories, &root_ref) { 
                        resource_tree.add_to_sub(&resource);
                    }
                }
            }
        }
    }
    resource_tree.show(details);
}

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
    let criteria = Criteria::new(&args);

    if path_info.exists {
        let details = DetailLevel::new(&args.list, &args.groups, &args.void);
        scan_directory(path_info.canonical, details, &criteria, false);
        criteria.show();
        if criteria.delete_with_prompt() {
            if action_prompt("Are you sure you want to delete the above files?") {
                cprintln!("<green>OK</green>");
            } else {
                cprintln!("<red>Not OK</red>");
            }
        }
    } else {
       cprintln!("<red>The target directory {}</red> does not exist", path_info.input); 
    }
    
    

    if criteria.has_pattern() {
        let source_str = "long-123.build.jpeg";
        println!("{:?}", match_string_simple(source_str.to_owned(), &criteria.pattern.unwrap() ) );
    }
    
}

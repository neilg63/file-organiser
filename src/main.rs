extern crate chrono;
// use std::rc::Rc;
use walkdir::{WalkDir, DirEntry};
//use clap::{Parser, command, Arg, ArgAction};
use clap::{Parser};
use args::Args;
use std::env;
use std::path::Path;
use color_print::cprintln;

mod args;
mod resource_row;
mod utils;
mod path_info;

use crate::utils::*;
use crate::resource_row::*;
use crate::path_info::*;


pub fn scan_directory(path_str:String, show_files: bool, max_depth: u8, target_days: f64, is_after: bool, extensions: &Vec<String>, &sizes: &(u64, u64)) {
    let mut root_ref:Option<DirEntry> = None;
    let mut resource_tree: ResourceTree = ResourceTree::new(max_depth);
    let target_dir = WalkDir::new(path_str).min_depth(0).max_depth(max_depth as usize).follow_links(true);
    for file in target_dir.into_iter().filter_map(|file| file.ok()) {
        let ft = file.file_type();
        if ft.is_dir() {
            if root_ref.is_none() {
                root_ref = Some(file.clone());
                resource_tree.add_root(&file);
            }
            let r_set = ResourceSet::new(&file);
            resource_tree.push(&r_set);
        } else {
            let resource = ResourceRow::new(&file);
            if resource.is_ranges(target_days, is_after, &sizes, &extensions) {
                if resource.depth() < 2 {
                    resource_tree.add_to_parent(&resource);
                }  else {
                    resource_tree.add_to_sub(&resource);
                }
            }
        }
    }
    resource_tree.show(show_files);
}


fn main() {
    let args = Args::parse();
    let curr_ref = ".".to_string() ;
    let path_arg = args.path.unwrap_or(curr_ref.clone());
    let curr_path = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let path_str = if path_arg == curr_ref { curr_path.clone().to_owned() } else if is_full_path(path_arg.as_str()) { path_arg.to_owned() } else { format!("{}/{}", curr_path, path_arg) };
    let before = extract_age(&args.before);
    let after = extract_age(&args.after);
    let is_after = after > 0f64;
    let time = if is_after { after } else { before };
    let max_depth = if args.max_depth > 0 { args.max_depth } else { 1 };

    let path_info = PathInfo::new(&path_str);
    
    let extensions:Vec<String> = extract_extensions(&args.ext);
    let sizes = extract_sizes(&args.size);

    let target_days = time as f64;

    
    let (move_target, move_mode) = extract_move_target(args.r#move);
    
    let delete_mode = !move_mode && args.remove;
    if path_info.exists {
        scan_directory(path_info.canonical, args.list, max_depth, target_days, is_after, &extensions, &sizes);
    } else {
       cprintln!("<red>The target directory {}</red> does not exist", path_str); 
    }
    

    let min_size_display = size_display(sizes.0, "min.");
    let max_size_display = size_display(sizes.1, "max.");
    let has_size_constraint = sizes.0 > 0 || sizes.0 > 0;
    let size_display = if has_size_constraint { format!("{} {}", min_size_display, max_size_display) } else { "All sizes".to_owned() };
    
    // cprintln!("total\t<green>{}</green>\tsize\t<blue>{}</blue>\t{}\t{}", num_files, smart_size(total_bytes), extensions.join(","), size_display);
    // let ext_text = if extensions.len() > 0 { extensions.join(", ") } else { "all".to_owned() };
    let days = if is_after { after } else { before };
    let age_range = days_age_display(days, is_after);
    cprintln!("<yellow>{: <15}</yellow>", age_range);
    cprintln!("{: >10} <blue>{: >9}</blue>", "size range", size_display);
    cprintln!("{: >10} <cyan>{: >9}</cyan>", "extensions", extensions.join(", "));
    let action_text = build_action_text(delete_mode, move_mode, &move_target);
    cprintln!("{} <yellow>{: <15}</yellow>", "action", action_text);
    
}

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

use crate::utils::*;
use crate::resource_row::*;


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
    let before = args.before;
    let after = args.after;
    let is_after = after > 0;
    let time = if is_after { after } else { before };
    let max_depth = if args.max_depth > 0 { args.max_depth } else { 1 };

    let has_root = Path::new(&path_str).exists();

    let extensions:Vec<String> = extract_extensions(&args.ext);
    let sizes = extract_sizes(&args.size);

    let target_days = time as f64;

    

    let move_mode = args.r#move.is_some() && args.r#move.unwrap().len() > 0;
    
    let delete_mode = !move_mode && args.remove;
    if has_root {
        scan_directory(path_str, args.list, max_depth, target_days, is_after, &extensions, &sizes);
    } else {
       cprintln!("<red>The target directory {}</red> does not exist", path_str); 
    }
    

    let min_size_display = size_display(sizes.0, "min.");
    let max_size_display = size_display(sizes.1, "max.");
    let size_display = format!("{} {}", min_size_display, max_size_display);
    
    // cprintln!("total\t<green>{}</green>\tsize\t<blue>{}</blue>\t{}\t{}", num_files, smart_size(total_bytes), extensions.join(","), size_display);

    println!("#size: {}\textensions: {}\tmove: {}, delete: {}", size_display, extensions.join(","), move_mode, delete_mode);
    
}

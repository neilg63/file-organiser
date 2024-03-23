use std::io::Write;
use clap::Parser;
use crate::args::Args;
use color_print::cprintln;
use crate::utils::pluralize_64;

use crate::path_info::PathInfo;
use crate::resource_row::*;
use crate::criteria::*;
use crate::run::*;

/// Called to confirm risky operations such as move or delete
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

/// Start the command line prompt and parse the core options
pub fn init() {
  let args = Args::parse();
  let path_info = PathInfo::new_from_args(&args);
  let mut criteria = Criteria::new(&args, path_info.pattern);
  if path_info.exists {
      let details = DetailLevel::new(&args.list, &args.groups, &args.void);
      let resource_tree = scan_directory(&path_info.canonical, &details, &mut criteria);
      criteria.show();
      if criteria.delete_with_prompt() {
          let num_matched_files = resource_tree.num_files();
          if num_matched_files > 0 {
              let file_word = pluralize_64("file", "s", num_matched_files as u64);
              if action_prompt(format!("Are you sure you want to delete the {} above {}?", num_matched_files, file_word).as_str()) {
                  resource_tree.run(ActionMode::Delete, None);
              } else {
                  cprintln!("<red>Not deleted</red>");
              }
          } else {
              cprintln!("<red>No matched files to delete</red>");
          }
      } else if criteria.move_or_copy_mode() && !criteria.has_target() {
          if action_prompt(&format!("The directory {} does not exist. Do you want to create it", criteria.target_ref() )) {
              if criteria.create_target() {
                  resource_tree.run(criteria.action, Some(criteria.target_info().path));
              } else {
                   cprintln!("<red>New target directory ({}) could be created</red>", criteria.target_ref());
              }
          } else {
              cprintln!("{}", criteria.action.to_not_past());
          }
      }
  } else {
     cprintln!("The target directory <red>{}</red> does not exist", path_info.input); 
  }
  
}
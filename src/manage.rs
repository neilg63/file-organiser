use std::fs::{rename, copy, create_dir_all};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use walkdir::{DirEntry};
use crate::resource_row::*;

pub fn move_file(resource: &ResourceRow, target: &Option<Box<PathBuf>>, root_ref: &Option<DirEntry>) -> (bool, String) {
  copy_move_file(resource, target, root_ref, true)
}

pub fn copy_file(resource: &ResourceRow, target: &Option<Box<PathBuf>>, root_ref: &Option<DirEntry>) -> (bool, String) {
  copy_move_file(resource, target, root_ref, false)
}

fn copy_move_file(resource: &ResourceRow, target: &Option<Box<PathBuf>>, root_ref: &Option<DirEntry>, move_mode: bool) -> (bool, String) {
  let mut moved = false;
  let mut new_path_str = "".to_string();
  if let Some(mp) = target {
   let target_base_string = mp.to_str().unwrap().to_owned();
   new_path_str = [target_base_string.clone(), resource.relative_path(root_ref)].join(MAIN_SEPARATOR.to_string().as_str());
   let needs_parent = resource.depth() > 1;
   let new_parent_dir = if needs_parent {
    [target_base_string, resource.relative_parent_path(root_ref)].join(MAIN_SEPARATOR.to_string().as_str())
   } else {
    target_base_string
   };
   let new_parent_path = Path::new(new_parent_dir.as_str());
   let mut has_parent = new_parent_path.exists();
   if  !has_parent {
    if let Ok(_ok) = create_dir_all(new_parent_path) {
      has_parent = true;
    }
   }
   if has_parent {
    let new_path = Path::new(new_path_str.as_str());
    if move_mode {
      if let Ok(_success) = rename(resource.path_ref(), new_path) {
        moved = true;
      }
    } else {
      if let Ok(_success) = copy(resource.path_ref(), new_path) {
        moved = true;
      }
    }
   }
  }
  (moved, new_path_str)
}
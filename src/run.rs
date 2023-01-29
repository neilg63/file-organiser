use std::fs::{remove_file,rename, create_dir_all};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};
use walkdir::{WalkDir, DirEntry};
use crate::resource_row::*;
use crate::criteria::*;
use crate::path_info::*;


pub fn scan_directory(path_str: &str, details: &DetailLevel, criteria: &Criteria, delete_confirmed: bool) -> ResourceTree {
    let mut root_ref:Option<DirEntry> = None;
    let mut resource_tree: ResourceTree = ResourceTree::new(criteria.max_depth);
    let target_dir = WalkDir::new(path_str).min_depth(0).max_depth(criteria.max_depth as usize).follow_links(true).same_file_system(true);
    let mut may_move = false;
    let mut move_path: Option<Box<PathBuf>> = None;
    if criteria.move_mode() {
      let move_dir_info = PathInfo::new(criteria.target.clone().unwrap().as_str());
      may_move = move_dir_info.exists;
      if may_move {
        move_path = Some(move_dir_info.path);
      }
    }
    let may_delete = criteria.delete_mode() && delete_confirmed;

    for file in target_dir.into_iter().filter_map(|file| file.ok()) {
        let ft = file.file_type();
        let mut not_excluded = true;
        if ft.is_dir() {
            if root_ref.is_none() {
                root_ref = Some(file.clone());
                resource_tree.add_root(&file);
            }
            let r_set = ResourceSet::new(&file  );
            if ft.is_dir() && root_ref.is_some() {
               not_excluded = r_set.is_not_excluded_dir(&criteria.exclude_directories, &root_ref);
            }
            if not_excluded {
              resource_tree.push(&r_set);
            }
        } else {
            let mut resource = ResourceRow::new(&file);
            if resource.matches_criteria(&criteria, &root_ref) {
                if resource.depth() < 2 {
                    resource_tree.add_to_parent(&resource);
                }  else {
                    if resource.is_not_in_excluded_dir(&criteria, &root_ref) { 
                        resource_tree.add_to_sub(&resource);
                    }
                }
                if may_move {
                  let (moved, new_path) = move_file(&resource, &move_path, &root_ref);
                  if moved {
                    resource.set_move_target(new_path.as_str());
                  }
                } else if may_delete {
                  if let Ok(_ok) = remove_file(resource.path_ref()) {
                    resource.set_deleted();
                  }
                }
            }
        }
    }
    resource_tree.show(details);
    resource_tree
}

fn move_file(resource: &ResourceRow, target: &Option<Box<PathBuf>>, root_ref: &Option<DirEntry>) -> (bool, String) {
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
    if let Ok(_success) = rename(resource.path_ref(), new_path) {
      moved = true;
    }
   }
  }
  (moved, new_path_str)
}
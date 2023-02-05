use std::fs::{remove_file};
use walkdir::{WalkDir, DirEntry};
use crate::resource_row::*;
use crate::criteria::*;
use crate::manage::{move_file, copy_file};

pub fn scan_directory(path_str: &str, details: &DetailLevel, criteria: &mut Criteria) -> ResourceTree {
    let mut root_ref:Option<DirEntry> = None;
    let mut resource_tree: ResourceTree = ResourceTree::new(criteria.max_depth);
    let target_dir = WalkDir::new(path_str).min_depth(0).max_depth(criteria.max_depth as usize).follow_links(true).same_file_system(true);
    let target_path = criteria.apply_action_permissions();
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
                if criteria.may_copy() {
                  let (copied, new_path) = copy_file(&resource, &target_path, &root_ref);
                  if copied {
                    resource.set_target(new_path.as_str());
                  }
                } else if criteria.may_move() {
                  let (moved, new_path) = move_file(&resource, &target_path, &root_ref);
                  if moved {
                    resource.set_target(new_path.as_str());
                  }
                } else if criteria.may_delete() {
                  if let Ok(_ok) = remove_file(resource.path_ref()) {
                    resource.set_deleted();
                  }
                }
                // Add first level files to the parent directories
                if resource.depth() < 2 {
                    resource_tree.add_to_parent(&resource);
                }  else {
                  // Add the path of nested files to their subdirectory, unless excluded
                    if resource.is_not_in_excluded_dir(&criteria, &root_ref) { 
                        resource_tree.add_to_sub(&resource);
                    }
                }
            }
        }
    }
    resource_tree.show(details);
    resource_tree
}

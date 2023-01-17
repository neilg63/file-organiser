use crate::utils::*;
use walkdir::{DirEntry};
use std::path::{Path};
use color_print::cprintln;
extern crate chrono;
use chrono::prelude::*;
use std::{os::unix::prelude::MetadataExt, collections::HashMap};
use crate::matches::*;
use crate::criteria::*;

#[derive(Debug, Clone)]
pub struct DetailLevel {
    pub show_files: bool,
    pub show_extension_groups: bool,
    pub show_void_directories: bool,
}

impl DetailLevel {
    pub fn new (show_files: &bool, show_extension_groups: &bool, show_void_directories: &bool) -> Self {
        DetailLevel { 
          show_files: show_files.to_owned(),
          show_extension_groups: show_extension_groups.to_owned(),
          show_void_directories: show_void_directories.to_owned()
        }
    }
}


#[derive(Debug, Clone)]
pub struct ResourceRow {
    pub file: DirEntry,
    pub extension: String,
    pub ts: u64,
    pub move_target: Option<String>,
    pub deleted: bool,
}

impl ResourceRow {
    pub fn new(file: &DirEntry) -> Self {
        ResourceRow { 
            file: file.to_owned(), 
            extension: extract_extension(file),
            ts: extract_timestamp(file),
            move_target: None,
            deleted: false,
         }
    }

    pub fn set_move_target(&mut self, target: &str) {
      self.move_target = Some(target.to_owned());
    }

    pub fn set_deleted(&mut self) {
      self.deleted = true;
    }

    pub fn seconds_old(&self) -> u64 {
        current_timestamp() as u64 - self.ts
    }

    pub fn days_old(&self) -> f64 {
        self.seconds_old() as f64 / 86400f64
    }

    pub fn has_valid_extension(&self, extensions: &Vec<String>, exclusions: &Vec<String>) -> bool {
        is_in_extensions(&self.extension, extensions) && is_not_in_extensions(&self.extension, exclusions)
    }

    pub fn is_not_in_excluded_dir(&self, e_dirs: &Vec<String>, root_ref: &Option<DirEntry>) -> bool {
      is_not_excluded_dir(&self.file, e_dirs, root_ref)
    }

    pub fn file_name(&self) -> String {
      self.file.file_name().to_str().unwrap_or("").to_owned()
    }

    pub fn size(&self) -> u64 {
        if let Ok(meta) = self.file.metadata() {
            meta.size()
        } else {
            0u64
        }
    }

    pub fn smart_size(&self) -> String {
        smart_size(self.size())
    }

    pub fn path_ref(&self) -> &Path {
      self.file.path()
    }

    pub fn matches(&self, pattern: &Option<String>, bounds: MatchBounds, mode: MatchMode) -> bool {
      if let Some(pattern_str) = pattern {
        match_string(self.file_name(), pattern_str, true, bounds, mode) 
      } else {
        true
      }
    }

    pub fn is_in_size_range(&self, sizes: &(u64, u64)) -> bool {
        let (min, max) = sizes.to_owned();
        let size = self.size();
        (size >= min || min < 1) && (size <= max || max < 1) 
    }

    pub fn is_in_day_range(&self, target_days: f64, is_after: bool) -> bool {
        (is_after && self.days_old() <= target_days) || (!is_after && self.days_old() >= target_days)
    }

    pub fn matches_criteria(&self, criteria: &Criteria, root_ref: &Option<DirEntry>) -> bool {
        self.is_in_day_range(criteria.age, criteria.newer) 
        && self.is_in_size_range(&criteria.sizes)
        && self.has_valid_extension(&criteria.include_extensions, &criteria.exclude_extensions)
        && (!criteria.has_pattern() || self.matches(&criteria.pattern, criteria.bounds, criteria.match_mode))
        && (!criteria.has_omit_pattern() || self.matches(&criteria.exclude_pattern, criteria.bounds, criteria.match_mode) == false)
        && self.show_if_hidden(criteria.show_hidden, root_ref)
    }

    pub fn show_if_hidden(&self, show_hidden: bool, root_ref: &Option<DirEntry>) -> bool {
      show_hidden || (self.file_name().starts_with(".") || self.relative_parts(root_ref).into_iter().any(|s| s.starts_with("."))) == false
    }
/*     pub fn day_display(&self) -> String {
        format!("{: >9}", format!("{:.2}", self.days_old()))
    } */

    pub fn age_display(&self) -> String {
        seconds_to_day_hours_min_secs(self.seconds_old())
    }

    pub fn file_display(&self, root_ref: &Option<DirEntry>) -> String {
        to_relative_path(&self.file, root_ref)
    }

    pub fn relative_path(&self, root_ref: &Option<DirEntry>) -> String {
      self.file_display(root_ref)
    }

    pub fn relative_parent_path(&self, root_ref: &Option<DirEntry>) -> String {
      path_to_relative_path(&self.file.path().parent().unwrap(), root_ref)
    }

    pub fn relative_parts(&self, root_ref: &Option<DirEntry>) -> Vec<String> {
        to_relative_parts(&self.file, root_ref)
    } 

    pub fn directory_path_string(&self) -> String {
      if let Some(parent_dir) = &self.file.path().parent() {
        path_to_string(parent_dir)
      } else {
        "".to_string()
      }
    }

    pub fn depth(&self) -> usize {
       self.file.depth()
    }

    pub fn modified_date(&self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp_opt(self.ts as i64, 0).unwrap()
    }

    pub fn modified_display(&self) -> String {
        self.modified_date().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn show(&self, root_ref: &Option<DirEntry>) {
        cprintln!("{: >9}\t<green>{}</green>\t<cyan>{: >9}</cyan>\t{}\t{}\t<yellow>{}</yellow>", self.age_display(), self.modified_display(), self.smart_size(), self.extension, self.depth(), self.file_display(root_ref));
    }

}

#[derive(Debug, Clone)]
pub struct ResourceSet {
  pub parent: DirEntry,
  pub resources: Vec<ResourceRow>,
  pub depth: usize,
}

impl ResourceSet {
  pub fn new(parent: &DirEntry) -> Self {
    ResourceSet { parent: parent.to_owned(), resources: vec![], depth: parent.depth() }
  }

  pub fn push(&mut self, resource: &ResourceRow) {
    self.resources.push(resource.to_owned());
  }

  pub fn count(&self) -> usize {
    self.resources.len()
  }



  pub fn is_not_excluded_dir(&self, e_dirs: &Vec<String>, root_ref: &Option<DirEntry>) -> bool {
    is_not_excluded_dir(&self.parent, e_dirs, root_ref)
  }
  
  pub fn size(&self) -> u64 {
    let mut size = 0u64;
    for row in &self.resources {
      size += row.size();
    }
    size
  }

  pub fn depth(&self) -> u8 {
    self.parent.depth() as u8
  }

  pub fn path_display(&self, root_ref: &Option<DirEntry>) -> String {
    to_relative_path(&self.parent, root_ref)
  }

  pub fn full_path_string(&self) -> String {
    path_to_string(&self.parent.path())
  }

  pub fn smart_size(&self) -> String {
      smart_size(self.size())
  }

  pub fn show(&self, root_ref: &Option<DirEntry>, show_files: bool) {  
    if show_files {
      for row in &self.resources {
        row.show(root_ref);
      }
    }
    let files_word = if self.count() == 1 { "file" } else { "files" };
    cprintln!("<cyan>{: >8}</cyan> {}\t{: >10}\t<yellow>{: >9}</yellow>", self.count(), files_word, self.smart_size(), self.path_display(root_ref));
  }

}

#[derive(Debug, Clone)]
pub struct ExtensionStats {
  pub key: String,
  pub count: u32,
  pub size: u64
}

impl ExtensionStats {
  pub fn new(key: String, count: u32, size: u64) -> Self {
    ExtensionStats { key, count, size }
  }
}

#[derive(Debug, Clone)]
pub struct ResourceTree {
  parent: Option<DirEntry>,
  directories: Vec<Box<ResourceSet>>,
  max_depth: u8
}

impl ResourceTree {
  pub fn new(max_depth: u8) -> Self {
    ResourceTree { max_depth, parent: None, directories: vec![] }
  }

  pub fn parent_dir(&mut self) -> Option<Box<&mut ResourceSet>> {
    if self.directories.len() > 0 {
      let first_opt = self.directories.get_mut(0);

              
       if first_opt.is_some() {
        // let par = first_opt.unwrap().to_owned().as_mut();
        Some(Box::new(first_opt.unwrap().as_mut()))
       } else {
        None
       }
    } else {
      None
    }
  }

 /*  pub fn curr_sub_dir(&mut self) -> Option<Box<&mut ResourceSet>> {
    if self.directories.len() > 1 {
      let last_opt = self.directories.last_mut();    
       if last_opt.is_some() {
        Some(Box::new(last_opt.unwrap().as_mut()))
       } else {
        None
       }
    } else {
      None
    }
  } */

   pub fn matched_sub_dir(&mut self, row: &ResourceRow) -> Option<Box<&mut ResourceSet>> {
    if self.directories.len() > 1 {
      let matched_opt = self.directories.iter_mut().find(|rs| rs.full_path_string() == row.directory_path_string());
       if let Some(matched_box)  = matched_opt {
        Some(Box::new(matched_box.as_mut()))
       } else {
        None
       }
    } else {
      None
    }
  }

  pub fn add_to_parent(&mut self, row: &ResourceRow) {
    if let Some(parent) = self.parent_dir() {
      let _ = &parent.push(row);
    }
  }

   pub fn add_to_sub(&mut self, row: &ResourceRow) {
    if let Some(curr_dir) = self.matched_sub_dir(row) {
      let _ = &curr_dir.push(row);
    }
  }

  pub fn add_root(&mut self, parent: &DirEntry) {
    self.parent = Some(parent.to_owned());
  }

  pub fn push(&mut self, resource_set: &ResourceSet) {
    self.directories.push(Box::new(resource_set.to_owned()));
  }

  pub fn num_dirs(&self) -> usize { 
    self.directories.len()
  }

  pub fn num_sub_dirs(&self) -> usize { 
    if self.num_dirs() > 0 {
      self.directories.len() - 1
    } else {
      0
    }
  }

  pub fn num_sub_dirs_display(&self) -> String {
    if self.num_sub_dirs() > 0 {
      let word = if self.num_sub_dirs() == 1 { "subdirectory" } else { "subdirectories" };
      format!("{} {}", self.num_sub_dirs(), word)
    } else {
      "".to_owned()
    }
  }

  pub fn size(&self) -> u64 {
    let mut size = 0u64;
    for row in &self.directories {
      size += row.size();
    }
    size
  }

  pub fn get_min_max_files(&self) -> (Option<ResourceRow>, Option<ResourceRow>) {
    let mut min_val = 0;
    let mut max_val = 0;
    let mut min_row: Option<ResourceRow> = None;
    let mut max_row: Option<ResourceRow> = None;
    for row in &self.directories {
      for resource in row.resources.clone() {
        let size_val = resource.size();
        if size_val > max_val {
          max_row = Some(resource);
          max_val = size_val;
        } else if size_val > 0 && (size_val < min_val || min_val < 1) {
          min_row = Some(resource);
          min_val = size_val;
        }
      }
    }
    (min_row, max_row)
  }

  pub fn num_files(&self) -> usize {
    let mut num = 0;
    for row in &self.directories {
      num += row.count();
    }
    num
  }

  pub fn path_display(&self) -> String {
      if let Some(root) = &self.parent {
        if let Some(root_path) = root.to_owned().into_path().to_str() {
          root_path.to_owned()
        } else {
          "".to_owned()  
        }
      } else {
        "".to_owned()
      }
  }

  pub fn smart_size(&self) -> String {
      smart_size(self.size())
  }

  pub fn build_extension_map(&self) -> Vec<ExtensionStats> {
    let mut map: HashMap<String, (u32, u64)> = HashMap::new();
    for directory in &self.directories {
      if directory.count() > 0 {
        for file in &directory.resources {
          let mut ext_count: u32 = 1;
          let mut ext_size: u64 = 1;
          if map.contains_key(&file.extension) {
            let (curr_count, curr_size) = map.get_mut(&file.extension).unwrap().to_owned();
            ext_count = curr_count + 1;
            ext_size = curr_size + file.size();
          }
          map.insert(file.extension.to_owned(), (ext_count, ext_size));
        }
      }
    }
    let mut ext_stats: Vec<ExtensionStats> = vec![];
    for (key, item) in map.into_iter() {
      ext_stats.push(ExtensionStats::new(key, item.0, item.1));
    }
    ext_stats.sort_by(|a, b| b.size.cmp(&a.size));
    ext_stats
  }

  pub fn show_extension_stats(&self) {
    for row in self.build_extension_map().into_iter() {
      let file_word = pluralize_64("file", "s", row.count as u64);
      let ext_text = if row.key.len() > 0 { row.key } else { "[none]".to_owned() };
      cprintln!("<yellow>{: >10}</yellow>\t<cyan>{: >9}</cyan> {}\t{}", ext_text, row.count, file_word, smart_size(row.size));
    }
  }

  pub fn show(&self, details: &DetailLevel) {
    for directory in &self.directories {
      if self.parent.is_some() {
        if directory.as_ref().depth() < self.max_depth {
          if details.show_void_directories || directory.count() > 0 {
            directory.as_ref().show(&self.parent, details.show_files);
          }
        }
      }
    }
    if details.show_extension_groups {
      self.show_extension_stats();
    }
    let num_files = self.num_files();
    cprintln!("{: <10} <yellow>{}</yellow>", "path", self.path_display());
    cprintln!("{: <10} <green>{}</green>", "total #", num_files);
    if num_files > 0 {
      cprintln!("{: <10} <cyan>{}</cyan>", "tot. size", self.smart_size());
      if num_files > 2 {
        let (min_file, max_file) = self.get_min_max_files();
        if let Some(min_resource) = min_file {
          cprintln!("{: <10} <cyan>{}</cyan> ({})", "smallest", min_resource.smart_size(), min_resource.relative_path(&self.parent));
        }
        if let Some(max_resource) = max_file {
          cprintln!("{: <10} <cyan>{}</cyan> ({})", "largest", max_resource.smart_size(), max_resource.relative_path(&self.parent));
        }
      }
      cprintln!("{: <10} {}", "max depth", self.max_depth);
    }
  }

}

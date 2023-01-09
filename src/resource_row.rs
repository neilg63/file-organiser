use crate::utils::*;
use walkdir::{DirEntry};
use color_print::cprintln;
extern crate chrono;
use chrono::prelude::*;
use std::{os::unix::prelude::MetadataExt};

#[derive(Debug, Clone)]
pub struct ResourceRow {
    file: DirEntry,
    extension: String,
    ts: u64,
}

impl ResourceRow {
    pub fn new(file: &DirEntry) -> Self {
        ResourceRow { 
            file: file.to_owned(), 
            extension: extract_extension(file),
            ts: extract_timestamp(file)
         }
    }

    pub fn seconds_old(&self) -> u64 {
        current_timestamp() as u64 - self.ts
    }

    pub fn days_old(&self) -> f64 {
        self.seconds_old() as f64 / 86400f64
    }

    pub fn has_valid_extension(&self, extensions: &Vec<String>) -> bool {
        is_in_extensions(&self.extension, extensions)
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

    pub fn is_in_size_range(&self, sizes: &(u64, u64)) -> bool {
        let (min, max) = sizes.to_owned();
        let size = self.size();
        (size >= min || min < 1) && (size <= max || max < 1) 
    }

    pub fn is_in_day_range(&self, target_days: f64, is_after: bool) -> bool {
        (is_after && self.days_old() <= target_days) || (!is_after && self.days_old() >= target_days)
    }

    pub fn is_ranges(&self, target_days: f64, is_after: bool, sizes: &(u64, u64), extensions: &Vec<String>) -> bool {
        self.is_in_day_range(target_days, is_after) && self.is_in_size_range(&sizes) && self.has_valid_extension(&extensions)
    }

    pub fn day_display(&self) -> String {
        format!("{: >9}", format!("{:.2}", self.days_old()))
    }

    pub fn file_display(&self, root_ref: &Option<DirEntry>) -> String {
        to_relative_path(&self.file, root_ref)
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
        cprintln!("{} days\t<green>{}</green>\t<blue>{: >9}</blue>\t{}\t{}\t<yellow>{}</yellow>", self.day_display(), self.modified_display(), self.smart_size(), self.extension, self.depth(), self.file_display(root_ref));
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
    cprintln!("<blue>{}</blue> {}\t{}\t<yellow>{}</yellow>", self.count(), files_word, self.smart_size(), self.path_display(root_ref));
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

  pub fn curr_sub_dir(&mut self) -> Option<Box<&mut ResourceSet>> {
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
  }

  pub fn add_to_parent(&mut self, row: &ResourceRow) {
    if let Some(parent) = self.parent_dir() {
      let _ = &parent.push(row);
    }
  }

   pub fn add_to_sub(&mut self, row: &ResourceRow) {
    if let Some(curr_dir) = self.curr_sub_dir() {
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
      let word = if self.num_sub_dirs() == 1 { "subdirectory" } else { "subdirectoris" };
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

  pub fn path_display(&self) -> String {
      if let Some(root) = &self.parent {
        to_relative_path(&root, &self.parent)
      } else {
        "".to_owned()
      }
  }

  pub fn smart_size(&self) -> String {
      smart_size(self.size())
  }

  pub fn show(&self, show_files: bool) {
    let mut num_files: usize = 0;
    let mut total_bytes: u64 = 0;
    for directory in &self.directories {
      if self.parent.is_some() {
        if directory.as_ref().depth() < self.max_depth {
          directory.as_ref().show(&self.parent, show_files);
        }
        num_files += directory.count();
        total_bytes += directory.size();
      }
    }
    cprintln!("total: <green>{}</green>\tsize: <blue>{}</blue>\t{}\t<yellow>{}</yellow>\t{}", num_files, smart_size(total_bytes), self.smart_size(), self.path_display(),self.num_sub_dirs_display());
  }

}

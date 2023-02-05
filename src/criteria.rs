use crate::args::Args;
use crate::utils::*;
use crate::matches::{MatchBounds, string_ends_with};
use crate::path_info::PathInfo;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use color_print::{cprintln,cformat};

#[derive(Debug, Copy, Clone)]
pub enum MatchMode {
  Simple,
  Regex
}
#[derive(Debug, Copy, Clone)]
pub enum ActionMode {
  List,
  Move,
  Copy,
  Delete,
  DirectDelete // unprompted
}

impl ActionMode {
  pub fn to_past_string(&self, not_mode: bool) -> String {
    let prefix = if not_mode { "not "} else { ""};
    match self {
      ActionMode::List => cformat!("<yellow>{}{}</yellow>", prefix, "listed"),
      ActionMode::Move => cformat!("<cyan>{}{}</cyan>", prefix, "moved"),
      ActionMode::Copy => cformat!("<green>{}{}</green>", prefix, "copied"),
      ActionMode::Delete | ActionMode::DirectDelete => cformat!("<red>{}{}</red>", prefix, "deleted"),
    }
  }
  pub fn to_past(&self) -> String {
    self.to_past_string(false)
  }

  pub fn to_not_past(&self) -> String {
    self.to_past_string(true)
  }

  pub fn delete_confirmed(&self) -> bool {
    match self {
      ActionMode::DirectDelete => true,
      _ => false
    }
  }
}

#[derive(Debug, Clone)]
pub struct Criteria {
  pub sizes: (u64, u64),
  pub include_extensions: Vec<String>,
  pub exclude_extensions: Vec<String>,
  pub exclude_directories: Vec<String>,
  pub pattern: Option<String>,
  pub exclude_pattern: Option<String>,
  pub match_mode: MatchMode,
  pub bounds: MatchBounds,
  pub max_depth: u8,
  pub min_age: f64,
  pub max_age: f64,
  pub show_hidden: bool,
  pub action: ActionMode,
  pub may: ActionMode,
  pub target: Option<String>,
}

impl Criteria {
  pub fn new(args: &Args, file_pattern: Option<String>) -> Criteria {
    // accept -separated range or single for --before
    let before_parts = extract_string_parts(&args.before);
    let has_before_range = before_parts.len() > 1 && before_parts.get(1).is_some();
    let mut before_ref = if has_before_range { before_parts.get(0).unwrap().to_owned() } else { "".to_owned() };
    // use either -a, --after value for min. age or first value in the before range
    let after_ref = if has_before_range { before_parts.get(1).unwrap().to_owned() } else { args.after.to_owned() };
    if has_before_range && !string_ends_with(&before_ref, "[mdhdwy]") && string_ends_with(&after_ref, "[mdhdwy]") {
      let suffix = extract_first_suffix_letter(&after_ref);
      before_ref = format!("{},{}", before_ref, suffix);
    }

    let before = extract_age(&before_ref);
    let after = extract_age(&after_ref);
    let max_depth = if args.max_depth > 0 && file_pattern.is_none() { args.max_depth } else { 1 };

    let include_extensions:Vec<String> = extract_extensions(&args.ext);

    let exclude_extensions:Vec<String> = extract_extensions(&args.not_ext);

    let exclude_directories:Vec<String> = extract_from_list(&args.exclude_dirs);

    let sizes = extract_sizes(&args.size);

    let (target, copy_mode) = extract_move_target(args.copy.clone());
    let (target, move_mode) = if copy_mode { (target, false ) } else { extract_move_target(args.r#move.clone()) };
    let target = if move_mode { Some(target) } else { None };

    let has_start_pattern = args.starts_with.len() > 0;
    let has_end_pattern = !has_start_pattern && args.ends_with.len() > 0;
    let pattern_str = if has_start_pattern { args.starts_with.clone() } else if has_end_pattern { args.ends_with.clone() } else { args.pattern.clone() };
    
    let pattern = if file_pattern.is_some() { 
      file_pattern
    } else if pattern_str.len() > 0 {
      Some(pattern_str.clone())
    } else { 
      None
    };

    let bounds = if has_start_pattern { MatchBounds::Start } else if has_end_pattern { MatchBounds::End } else { MatchBounds::Open };
    
    let exclude_pattern = if args.omit_pattern.len() > 0 { Some(args.omit_pattern.clone()) } else { None };
    
    let delete_mode = !move_mode && args.delete;

    let force_delete = delete_mode && args.force;
    let action = if copy_mode {
      ActionMode::Copy
    } else if move_mode {
      ActionMode::Move
    } else if force_delete {
      ActionMode::DirectDelete
    } else if delete_mode {
      ActionMode::Delete
    } else {
      ActionMode::List
    };

    let match_mode = if args.regex_mode { MatchMode::Regex } else { MatchMode::Simple };
    let show_hidden = args.hidden;
    
    Criteria { 
      sizes,
      include_extensions,
      exclude_extensions,
      exclude_directories,
      pattern,
      exclude_pattern,
      match_mode,
      bounds,
      max_depth,
      min_age: before,
      max_age: after,
      show_hidden,
      action,
      target,
      may: ActionMode::List
    }
  }

  pub fn min_size(&self) -> u64 {
    self.sizes.0
  }

  pub fn max_size(&self) -> u64 {
    self.sizes.1
  }

  pub fn has_pattern(&self) -> bool {
    self.pattern.is_some()
  }

  pub fn has_omit_pattern(&self) -> bool {
    self.exclude_pattern.is_some()
  }

  pub fn has_max_size(&self) -> bool {
    self.max_size() > self.min_size()
  }

  pub fn target_info(&self) -> PathInfo {
    if let Some(tg) = self.target.clone() {
      PathInfo::new(tg.clone().as_str())
    } else {
      PathInfo::new_empty()
    }
  }

  pub fn has_target(&self) -> bool {
    self.target_info().exists
  }

  pub fn create_target(&self) -> bool {
    if let Some(tg) = self.target.clone() {
      let new_parent_path = Path::new(&tg).to_owned();
      if let Ok(_created) = create_dir_all(new_parent_path) {
        true
      } else {
        false
      }
    } else {
      false
    }
  }

  pub fn apply_action_permissions(&mut self) -> Option<Box<PathBuf>> {
    let mut target_path: Option<Box<PathBuf>> = None;
    let delete_confirmed = self.action.delete_confirmed();
    if self.move_or_copy_mode() {
      let move_dir_info = self.target_info();
      if move_dir_info.exists {
        target_path = Some(move_dir_info.path);
        if self.copy_mode() {
          self.set_may_copy();
        } else {
          self.set_may_move();
        }
      }
    } else if self.delete_mode() {
      if self.delete_mode() && delete_confirmed {
        self.set_may_delete();
      }
    }
    target_path
  }

  pub fn has_size_limits(&self) -> bool {
    self.min_size() > 0 || self.has_max_size()
  }

  pub fn delete_mode(&self) -> bool {
    match self.action {
      ActionMode::Delete | ActionMode::DirectDelete => true,
      _ => false,
    }
  }

  pub fn delete_with_prompt(&self) -> bool {
    match self.action {
      ActionMode::Delete => true,
      _ => false,
    }
  }

  pub fn move_mode(&self) -> bool {
    match self.action {
      ActionMode::Move => true,
      _ => false,
    }
  }

  pub fn copy_mode(&self) -> bool {
    match self.action {
      ActionMode::Copy => true,
      _ => false,
    }
  }

  pub fn set_may_move(&mut self) {
    self.may = ActionMode::Move;
  }

  pub fn set_may_copy(&mut self) {
    self.may = ActionMode::Copy;
  }

  pub fn set_may_delete(&mut self) {
    self.may = ActionMode::Delete;
  }

  pub fn move_or_copy_mode(&self) -> bool {
    self.move_mode() || self.copy_mode()
  }

  pub fn may_copy(&self) -> bool {
    match self.may {
      ActionMode::Copy => true,
      _ => false
    }
  }

  pub fn may_move(&self) -> bool {
    match self.may {
      ActionMode::Move => true,
      _ => false
    }
  }

  pub fn may_delete(&self) -> bool {
    match self.may {
      ActionMode::Delete => true,
      _ => false
    }
  }

  pub fn target_ref(&self) -> Box<String> {
    if let Some(tg) = self.target.clone() {
      Box::new(tg)
    } else {
      Box::new( "".to_owned())
    }
  }

  pub fn target_mode(&self) -> bool {
    self.move_mode() || self.copy_mode()
  }

  pub fn filter_by_age(&self) -> bool {
    self.min_age > 0f64 || self.max_age > 0f64
  }

  pub fn has_min_age(&self) -> bool {
    self.min_age > 0f64
  }

  pub fn has_max_age(&self) -> bool {
    self.max_age > 0f64 && self.max_age > self.min_age
  }

  pub fn to_text(&self) -> String {
    let action = match self.action {
      ActionMode::Move => "move to",
      ActionMode::Copy => "copy to",
      ActionMode::Delete => if self.delete_mode() {
        "delete"
      } else {
        "list"
      },
      _ => "list"
    };
    let target = if self.target_mode() {
        format!(" {}", self.target_ref())
    } else {
      "".to_owned()
    };
    format!("{}{}", action, target)
  }

  pub fn show(&self) {
    let min_size_display = size_display(self.min_size(), "min.");
    let max_size_display = size_display(self.max_size(), "max.");
    let has_size_constraint = self.has_size_limits();
    let size_display = if has_size_constraint { format!("{} {}", min_size_display, max_size_display) } else { "[all]".to_owned() };
    let age_range = days_age_display(self.min_age, self.max_age);
    cprintln!("<cyan,italics>CRITERIA</cyan,italics>");
    cprintln!("<yellow>{}</yellow>", age_range);
    cprintln!("{: <12} <cyan>{}</cyan>", "size range", size_display);
    let ext_text = if self.include_extensions.len() > 0 { self.include_extensions.join(", ") } else { "[all]".to_owned()  };
    cprintln!("{: <12} <cyan>{}</cyan>", "extensions", ext_text);
    if self.has_pattern() || self.has_omit_pattern() {
      let mut parts: Vec<String> = vec![];
      if self.has_pattern() {
        if let Some(pattern) = self.pattern.clone() {
          let short_pattern = to_short_pattern(&pattern);
          parts.push(cformat!("matching <cyan>{}</cyan>", short_pattern));
        }
      }
      if self.has_omit_pattern() {
        if let Some(not_pattern) = self.exclude_pattern.clone() {
          parts.push(cformat!("not matching <cyan>{}</cyan>", not_pattern));
        }
      }
      if parts.len() > 0 {
        cprintln!("{: <12} {}", "file names", parts.join(" and "));
      }
    }
    let action_text = self.to_text();
    cprintln!("{} <yellow>{: <12}</yellow>", "action", action_text);
  }

}

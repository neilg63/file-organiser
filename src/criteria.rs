use crate::args::Args;
use crate::utils::*;
use crate::matches::{MatchBounds, string_ends_with};
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
  Delete,
  DirectDelete // unprompted
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

    let (move_target, move_mode) = extract_move_target(args.r#move.clone());
    let target = if move_mode { Some(move_target) } else { None };

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
    let action = if move_mode {
      ActionMode::Move
    } else if force_delete {
      ActionMode::DirectDelete
    } else if delete_mode {
      ActionMode::Delete
    } else {
      ActionMode::List
    };

    let match_mode = if args.regex_mode { MatchMode::Simple } else { MatchMode::Regex };
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
      target
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

/*   pub fn show_prompt(&self) -> bool {
    match self.action {
      ActionMode::Delete => true,
      _ => false,
    }
  } */

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

  pub fn filter_by_age(&self) -> bool {
    self.min_age > 0f64 || self.max_age > 0f64
  }

  pub fn has_min_age(&self) -> bool {
    self.min_age > 0f64
  }

  pub fn has_max_age(&self) -> bool {
    self.max_age > 0f64 && self.max_age > self.min_age
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
    let action_text = build_action_text(self.delete_mode(), self.move_mode(), &self.target);
    cprintln!("{} <yellow>{: <12}</yellow>", "action", action_text);
  }

}

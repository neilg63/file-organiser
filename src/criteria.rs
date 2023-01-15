use crate::args::Args;
use crate::utils::*;
use color_print::cprintln;

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
  pub max_depth: u8,
  pub age: f64,
  pub newer: bool,
  pub show_hidden: bool,
  pub action: ActionMode,
  pub target: Option<String>,
}

impl Criteria {
  pub fn new(args: &Args) -> Criteria {
    let before = extract_age(&args.before);
    let after = extract_age(&args.after);
    let is_after = after > 0f64;
    let time = if is_after { after } else { before };
    let max_depth = if args.max_depth > 0 { args.max_depth } else { 1 };

    
    let include_extensions:Vec<String> = extract_extensions(&args.ext);

    let exclude_extensions:Vec<String> = extract_extensions(&args.not_ext);

    let exclude_directories:Vec<String> = extract_from_list(&args.exclude_dirs);

    let sizes = extract_sizes(&args.size);

    let target_days = time as f64;
    let (move_target, move_mode) = extract_move_target(args.r#move.clone());
    let target = if move_mode { Some(move_target) } else { None };

    let pattern = if args.pattern.len() > 0 { Some(args.pattern.clone()) } else { None };

    let exclude_pattern = if args.exclude_pattern.len() > 0 { Some(args.exclude_pattern.clone()) } else { None };
    
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
      max_depth,
      age: target_days,
      newer: is_after,
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

  pub fn has_max_size(&self) -> bool {
    self.max_size() > self.min_size()
  }

  pub fn show_prompt(&self) -> bool {
    match self.action {
      ActionMode::Delete => true,
      _ => false,
    }
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

  pub fn show(&self) {
    let min_size_display = size_display(self.min_size(), "min.");
    let max_size_display = size_display(self.max_size(), "max.");
    let has_size_constraint = self.has_size_limits();
    let size_display = if has_size_constraint { format!("{} {}", min_size_display, max_size_display) } else { "All sizes".to_owned() };
    let age_range = days_age_display(self.age, self.newer);
    cprintln!("<yellow>{: <15}</yellow>", age_range);
    cprintln!("{: >10} <cyan>{: >9}</cyan>", "size range", size_display);
    cprintln!("{: >10} <cyan>{: >9}</cyan>", "extensions", self.include_extensions.join(", "));
    let action_text = build_action_text(self.delete_mode(), self.move_mode(), &self.target);
    cprintln!("{} <yellow>{: <15}</yellow>", "action", action_text);
  }

}

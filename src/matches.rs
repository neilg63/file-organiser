use string_patterns::*;
use crate::criteria::MatchMode;

#[derive(Debug, Copy,Clone)]
pub enum MatchBounds {
  Open,
  Start,
  End
}

pub fn match_string(source: String, pattern: &str, case_insensitive: bool, bounds: MatchBounds, mode: MatchMode) -> bool {
  let start_bounds = match bounds {
    MatchBounds::Start => if pattern.starts_with("^") { "" } else { "^" },
    _ => ""
  };
  let end_bounds = match bounds {
    MatchBounds::End => if pattern.ends_with("$") { "" } else { "(\\.\\w+)?$" },
    _ => ""
  };
  let parsed_pattern = match mode {
    MatchMode::Simple => pattern.replace(".", "\\.").replace("*", ".*"),
    _ => pattern.to_owned()
  };
  let corrected_pattern = [start_bounds, parsed_pattern.as_str(), end_bounds].concat();
  source.pattern_match(&corrected_pattern, case_insensitive)
}


pub fn string_ends_with(source: &str, pattern: &str) -> bool {
  match_string(source.to_owned(), &pattern.to_owned(), false, MatchBounds::End,MatchMode::Regex)
}
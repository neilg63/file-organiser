use string_patterns::{build_regex, Regex};
use crate::criteria::MatchMode;

#[derive(Debug, Copy,Clone)]
pub enum MatchBounds {
  Open,
  Start,
  End
}

pub fn build_matcher(pattern: &str, case_insensitive: bool, bounds: MatchBounds, mode: MatchMode) -> Option<Regex> {
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
  if let Ok(rgx) = build_regex(&corrected_pattern, case_insensitive) {
    Some(rgx)
  } else {
    None
  }
}

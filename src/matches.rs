use string_patterns::PatternMatch;
use crate::criteria::MatchMode;

#[derive(Debug, Copy,Clone)]
pub enum MatchBounds {
  Open,
  Start,
  End
}

/// Build the anchored regex pattern string used to match file names against.
/// Validated up front by probing an empty string via the shared regex cache in
/// string-patterns, so the pattern is compiled once and reused for every file
/// matched against it via PatternMatch::pattern_match_ci, rather than file-organiser
/// building and holding its own Regex.
pub fn build_match_pattern(pattern: &str, bounds: MatchBounds, mode: MatchMode) -> Option<String> {
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
  if "".pattern_match_result(&corrected_pattern, true).is_ok() {
    Some(corrected_pattern)
  } else {
    None
  }
}

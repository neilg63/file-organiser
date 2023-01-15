use regex::Regex;
use crate::criteria::MatchMode;

#[derive(Debug, Copy,Clone)]
pub enum MatchBounds {
  Open,
  Start,
  End
}

pub fn match_string(source: String, pattern: &String, case_insensitive: bool, bounds: MatchBounds, mode: MatchMode) -> bool {
  let prefix = if case_insensitive { "(?i)" } else { "" };
  let start_bounds = match bounds {
    MatchBounds::Start => if pattern.starts_with("^") { "" } else { "^" },
    _ => ""
  };
  let end_bounds = match bounds {
    MatchBounds::End => if pattern.ends_with("$") { "" } else { "$" },
    _ => ""
  };
  let parsed_pattern = match mode {
    MatchMode::Simple => pattern.as_str().replace(".", "\\.").replace("*", ".*"),
    _ => pattern.clone().to_owned()
  };
  let corrected_pattern = [prefix, start_bounds, parsed_pattern.as_str(), end_bounds].join("");
  let re = Regex::new(corrected_pattern.as_str()).unwrap();
  re.is_match(source.as_str())
}

pub fn match_string_simple(source: String, pattern: &String) -> bool {
  match_string(source, pattern, false, MatchBounds::Open, MatchMode::Simple)
}

pub fn match_string_sensitive(source: String, pattern: &String) -> bool {
  match_string(source, pattern, true, MatchBounds::Open, MatchMode::Simple)
}

pub fn match_string_advanced(source: String, pattern: &String, case_insensitive: bool) -> bool {
  match_string(source, pattern, case_insensitive, MatchBounds::Open, MatchMode::Regex)
}

pub fn starts_with_simple(source: String, pattern: &String, case_insensitive: bool) -> bool {
  match_string(source, pattern, case_insensitive, MatchBounds::Start, MatchMode::Simple)
}

pub fn starts_with_advanced(source: String, pattern: &String, case_insensitive: bool) -> bool {
  match_string(source, pattern, case_insensitive, MatchBounds::Start, MatchMode::Regex)
}
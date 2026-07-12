use std::collections::HashMap;
use std::sync::OnceLock;

/// Built-in English defaults, embedded at compile time so the binary never
/// depends on a file being present at runtime.
const DEFAULT_LANG: &str = include_str!("lang/en.env");

/// Parse simple KEY=value lines, ignoring blank lines and lines starting
/// with '#'. Shared by the embedded defaults and an optional override file
/// so both follow exactly the same rules.
fn parse_lang_content(content: &str) -> HashMap<String, String> {
  let mut map = HashMap::new();
  for line in content.lines() {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue;
    }
    if let Some((key, value)) = trimmed.split_once('=') {
      map.insert(key.trim().to_string(), value.trim().to_string());
    }
  }
  map
}

/// Messages start from the built-in English defaults; if FO_LANG_FILE points
/// at a readable file, its entries are overlaid on top, so a translation only
/// needs to list the keys it wants to change.
fn messages() -> &'static HashMap<String, String> {
  static MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();
  MESSAGES.get_or_init(|| {
    let mut map = parse_lang_content(DEFAULT_LANG);
    if let Ok(path) = std::env::var("FO_LANG_FILE") {
      if let Ok(content) = std::fs::read_to_string(&path) {
        for (key, value) in parse_lang_content(&content) {
          map.insert(key, value);
        }
      }
    }
    map
  })
}

/// Look up a message by its machine name. Falls back to the key itself if
/// missing from both the override file and the built-in defaults, so a typo
/// in a key shows up as obviously wrong output rather than being swallowed.
pub fn t(key: &str) -> String {
  messages().get(key).cloned().unwrap_or_else(|| key.to_owned())
}

/// Look up a template message and substitute {0}, {1}, ... with args in
/// order. Templates use positional placeholders rather than a fixed word
/// order so a translation is free to rearrange them.
pub fn tf(key: &str, args: &[&str]) -> String {
  let mut result = t(key);
  for (index, arg) in args.iter().enumerate() {
    result = result.replace(&format!("{{{}}}", index), arg);
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn looks_up_known_default_keys() {
    assert_eq!(t("FO_HEADER_OVERVIEW"), "OVERVIEW");
    assert_eq!(t("FO_ACTION_NOT_DELETED"), "not deleted");
  }

  #[test]
  fn falls_back_to_the_key_for_unknown_messages() {
    assert_eq!(t("FO_DOES_NOT_EXIST"), "FO_DOES_NOT_EXIST");
  }

  #[test]
  fn substitutes_positional_placeholders_in_order() {
    assert_eq!(tf("FO_AGE_OLD", &["3h"]), "3h old");
    assert_eq!(tf("FO_AGE_BETWEEN", &["1d", "5d"]), "between 1d and 5d old");
  }
}

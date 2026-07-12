use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Built-in English defaults, embedded at compile time so the binary never
/// depends on a file being present at runtime.
const DEFAULT_LANG: &str = include_str!("lang/en.env");

/// Parse simple KEY=value lines, ignoring blank lines and lines starting
/// with '#'. Shared by the embedded defaults and every locale/override file
/// so they all follow exactly the same rules.
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

/// Extract a lowercase language code from a POSIX locale value, e.g. "fr"
/// from "fr_FR.UTF-8". Returns None for the unset/default "C"/"POSIX" locale.
fn parse_locale_code(raw: &str) -> Option<String> {
  let code = raw.split(['_', '.']).next()?.trim().to_lowercase();
  if code.is_empty() || code == "c" || code == "posix" {
    None
  } else {
    Some(code)
  }
}

/// The user's system language, read from the standard POSIX LC_ALL/LANG
/// environment variables (LC_ALL takes precedence, per POSIX convention).
fn system_locale() -> Option<String> {
  let raw = std::env::var("LC_ALL").or_else(|_| std::env::var("LANG")).ok()?;
  parse_locale_code(&raw)
}

/// Where a developer can drop a new `<code>.env` file to add a language
/// without touching any source code: FO_LANG_DIR if set, otherwise a `lang`
/// directory next to the running executable (e.g. next to a packaged
/// release binary, alongside its own lang/en.env-style files).
fn locale_dir() -> Option<PathBuf> {
  if let Ok(dir) = std::env::var("FO_LANG_DIR") {
    return Some(PathBuf::from(dir));
  }
  std::env::current_exe().ok()?.parent().map(|exe_dir| exe_dir.join("lang"))
}

/// Messages are layered: built-in English defaults, then an auto-detected
/// system-locale file if one is found in the locale directory, then
/// FO_LANG_FILE if set - each layer only needs to list the keys it changes,
/// anything missing falls through to the layer below.
fn messages() -> &'static HashMap<String, String> {
  static MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();
  MESSAGES.get_or_init(|| {
    let mut map = parse_lang_content(DEFAULT_LANG);

    if let (Some(dir), Some(code)) = (locale_dir(), system_locale()) {
      let path = dir.join(format!("{}.env", code));
      if let Ok(content) = std::fs::read_to_string(&path) {
        for (key, value) in parse_lang_content(&content) {
          map.insert(key, value);
        }
      }
    }

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

  #[test]
  fn parses_language_code_from_posix_locale_values() {
    assert_eq!(parse_locale_code("fr_FR.UTF-8"), Some("fr".to_string()));
    assert_eq!(parse_locale_code("en_US.UTF-8"), Some("en".to_string()));
    assert_eq!(parse_locale_code("de"), Some("de".to_string()));
  }

  #[test]
  fn treats_unset_or_default_locale_as_no_preference() {
    assert_eq!(parse_locale_code("C"), None);
    assert_eq!(parse_locale_code("POSIX"), None);
    assert_eq!(parse_locale_code(""), None);
  }
}

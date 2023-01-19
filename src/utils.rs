use walkdir::{DirEntry};
use std::path::{Path};
use std::time::UNIX_EPOCH;
use size::Size;
use std::fs;

pub fn is_full_path(path_arg: &str) -> bool {
    path_arg.starts_with("/") || path_arg.starts_with("~/")
}

pub fn current_timestamp() -> i64 {
  chrono::offset::Utc::now().timestamp()
}

pub fn smart_size(byte_size: u64) -> String {
  let size = Size::from_bytes(byte_size);
  size.to_string()
}

pub fn extract_extension(file: &DirEntry) -> String {
    let file_ext = file.path().extension();
    if let Some(ext) = file_ext { 
        if let Some(ext_str) = ext.to_str() {
            ext_str.to_lowercase().to_owned()
        } else {
            "".to_owned()    
        }
    } else { 
        "".to_owned()
    }
}

pub fn extract_extensions<'a>(ext_list: &str) -> Vec<String> {
  extract_from_list(ext_list)
}

pub fn extract_from_list<'a>(str_list: &str) -> Vec<String> {
  if str_list.clone().len() > 0 { str_list.split(",").into_iter().map(|s| s.to_owned()).collect() } else { vec![] }
}

pub fn extract_move_target(move_opt: Option<String>) -> (String, bool) {
  let move_target = move_opt.unwrap_or("".to_owned());
  (move_target.clone(), move_target.len() > 0)
}

pub fn extract_timestamp(file: &DirEntry) -> u64 {
    let mut ts = 0u64;
    if let Ok(meta) = file.metadata() {
        if let Ok(mod_time) = meta.modified() {
            if let Ok(ts_val) = mod_time.duration_since(UNIX_EPOCH) {
                ts = ts_val.as_secs();
            }
        }
    }
    ts
}

pub fn build_action_text(delete_mode: bool, move_mode: bool, move_target: &Option<String>) -> String {
  let mut action_parts: Vec<&str> = vec![];
  if move_mode {
      action_parts.push("move to");
      if let Some(tg) = move_target {
        action_parts.push(tg.as_str());
      }
  } else if delete_mode { 
      action_parts.push("delete");
  } else {
    action_parts.push("list");
  }
  action_parts.join(" ").to_owned()
}

pub fn is_in_extensions(ext: &String, extensions: &Vec<String>) -> bool {
    if extensions.len() > 0 {
        extensions.iter().any(|e| matches_empty_extension_ref(e, ext))
    } else {
        true
    }
}

pub fn is_not_in_extensions(ext: &String, extensions: &Vec<String>) -> bool {
    extensions.iter().any(|e| matches_empty_extension_ref(e, ext)) == false
}

fn matches_empty_extension_ref(match_ext: &String, file_ext: &String) -> bool {
  match_ext == file_ext || (match_ext == "_" && file_ext == "")
}

pub fn numeric_string_to_f64(num_chars: &Vec<char>) -> f64 {
    let min_str = num_chars.iter().collect::<String>();
    if let Ok(min_val) = min_str.parse::<f64>() {
        min_val
    } else {
        0f64
    }
}

pub fn num_unit_to_bytes_u64(num: f64, unit: char) -> u64 {
    let unit_multiplier = match unit {
        'k' => 1024f64,
        'm' => 1024f64 * 1024f64,
        'g' => 1024f64 * 1024f64 * 1024f64,
        _ => 1f64
    };
    (num * unit_multiplier) as u64
}

pub fn extract_size_val(num_chars: &Vec<char>, unit: char) -> u64 {
    let mut int_val = 0u64;
    if num_chars.len() > 0 {
        let num_val = numeric_string_to_f64(&num_chars);
        if num_val > 0f64 {
            int_val = num_unit_to_bytes_u64(num_val, unit);
        }
    }
    int_val
}

pub fn extract_age(size_str: &String) -> f64 {
  let chars: Vec<char> = size_str.to_lowercase().chars().into_iter().collect();
  let mut num = 0f64;
  let mut num_chars: Vec<char> = vec![];
  let mut has_number = false;
  let mut div = 1f64;
  let mut has_unit = false;
  for (index, char) in chars.into_iter().enumerate() {
    if char.is_numeric() {
      has_number = true;
    }
    if (char.is_numeric() || (char == '.' && has_number)) && !has_unit {
      num_chars.push(char);
    } else if index > 0 && has_number && char != ',' && !char.is_numeric() && div == 1f64 {
      div = match char {
        's' => 86400f64,
        'm' => 1440f64,
        'h' => 24f64,
        'w' => 1f64/7f64,
        'y' => 1f64/365.25f64,
        _ => 1f64
      };
      has_unit = true;
    }
  }
  let num_str = num_chars.iter().collect::<String>();
  if let Ok(num_fl) = num_str.parse::<f64>() {
    num = num_fl;
  }
  num / div
}

pub fn extract_sizes(size_str: &String) -> (u64, u64) {
    let mut min = 0u64;
    let mut max = 0u64;
    let ref_str = size_str.trim().to_lowercase();
    if ref_str.len() > 0 {
        let mut is_in_num = false;
        let mut first_char = '#';
        let mut has_min = false;
        let mut min_chars: Vec<char> = vec![];
        let mut min_unit = 'b';
        let mut max_unit = 'b';
        let mut max_chars: Vec<char> = vec![];
        for (ci, char) in  ref_str.chars().into_iter().enumerate() {
            if ci == 0 {
                first_char = char;
            }
            let is_max = has_min || first_char == ',';
            let mut capture = false;
            if char.is_numeric() {
                is_in_num = true;
                capture = true;
            } else if is_in_num {
                if char == '.' {
                    capture = true;
                } else if char != ',' {
                    is_in_num = false;
                    let unit = match char {
                        'k' | 'm' | 'g' => char,
                        _ => '#'
                    };
                    if unit != '#' {
                        if is_max {
                            max_unit = unit;
                        } else {
                            min_unit = unit;
                        }
                    }
                }
            } else {
                is_in_num = false;
            }
            if ci > 0 && min_chars.len() > 0 && !is_in_num {
                has_min = true;
            }
            if capture {
                if is_max {
                    max_chars.push(char);
                } else {
                    min_chars.push(char);
                }
            }
        }
        let min_ref_unit = if min_unit == 'b' && max_unit != 'b' { max_unit } else { min_unit };
        min = extract_size_val(&min_chars, min_ref_unit);
        if max_chars.len() > 0 {
            let max_ref_unit = if min_ref_unit != 'b' && max_unit == 'b' { min_ref_unit } else { max_unit };
            let max_ref_val = extract_size_val(&max_chars, max_ref_unit);
            if max_ref_val > min {
                max = max_ref_val;
            }
        }
    }
    (min, max)
}

pub fn size_display(size: u64, prefix: &str) -> String {
    if size > 0 { format!("{} {}", prefix, smart_size(size)) } else { "".to_string() }
}

pub fn to_relative_parts(current: &DirEntry, root: &Option<DirEntry>) -> Vec<String> {
  path_to_relative_parts(current.path(), root)
}

pub fn path_to_relative_parts(current_path: &Path, root: &Option<DirEntry>) -> Vec<String> {
    if let Some(root_ref) = root {
        let root_comps = root_ref.path().components().into_iter().collect::<Vec<_>>();
        let num_root_parts = root_comps.len();
        let mut parts: Vec<String> = vec![];
        for (ci, item) in current_path.components().into_iter().enumerate() {
            let item_str = item.as_os_str().to_str().unwrap_or("");
            let par_ref = if ci < num_root_parts { root_comps.get(ci) } else { None };
            let is_root_part = par_ref.is_some() && par_ref.unwrap().as_os_str().to_str().unwrap_or("") == item_str;
            if !is_root_part {
                parts.push(item_str.to_string());
            }
        }
        parts.to_owned()
    } else {
        vec![]
    }
}

pub fn get_num_subdirectories(parent: &DirEntry) -> usize {
  let mut num_subs:usize = 0;
  let result: Result<fs::ReadDir, std::io::Error>  = fs::read_dir(parent.clone().into_path());
  if let Ok(rd) = result {
    for entry in  rd {
      if let Ok(sub_entry) = entry {
        if sub_entry.path().is_dir() {
          num_subs += 1;
        }
      }
    }
  }
  num_subs
}

pub fn path_to_string(ref_path: &Path) -> String {
  let parts = ref_path.components().into_iter().map(|c| c.as_os_str().to_str().unwrap_or("")).collect::<Vec<_>>();
  parts.join("/")
}

pub fn to_relative_path(current: &DirEntry, root: &Option<DirEntry>) -> String {
  path_to_relative_path(current.path(), root)
}

pub fn path_to_relative_path(current_path: &Path, root: &Option<DirEntry>) -> String {
  let parts = path_to_relative_parts(current_path, root);
    if parts.len() > 0 {
        parts.join("/").to_owned()
    } else {
        current_path.to_str().unwrap_or("").to_string()
    }
}

pub fn is_not_excluded_dir(resource: &DirEntry, e_dirs: &Vec<String>, root_ref: &Option<DirEntry>) -> bool {
  if e_dirs.len() > 0 {
    let dirs = to_relative_parts(resource, root_ref);
    dirs.into_iter().any(|d| e_dirs.contains(&d)) == false
  } else {
    true
  }
}

pub fn extract_day_ref_pairs(days: f64) -> (f64, String) {
  let mut unit = "day";
  let mut num = days;
  if days < 0.5 {
    if days >= 1f64/24f64 {
      num *= 24f64;
      unit = "hour";
    } else if days >= 1f64/1440f64 {
      num *= 1440f64;
      unit = "min";
    } else {
      num *= 86400f64;
      unit = "sec";
    }
  } else if days > 730.5 {
    unit = "year";
    num /= 362.25;
  }
  (num, unit.to_owned())
}

pub fn smart_dec_format(num: f64) -> String {
  let num_fmt = format!("{:.3}", num);
  let num_parts = num_fmt.split(".").into_iter().collect::<Vec<&str>>();
  let base_num = num_parts.get(0).unwrap().to_owned();
  let mut dec_chars:Vec<char> = vec![];
  if num_parts.len() > 1 {
    let second = num_parts.get(1).unwrap().chars().rev().into_iter();
    let mut is_zero = true;
    for (_index, digit) in second.enumerate() {
      if digit != '0' && is_zero {
        dec_chars.push(digit);
        is_zero = false;
      }
    }
  }
  let has_decimals = dec_chars.len() > 0;
  if has_decimals { 
    format!("{}.{}", base_num, dec_chars.into_iter().collect::<String>())
  } else { 
    base_num.to_owned()
  }
}

pub fn days_age_display(days: f64, is_after: bool) -> String {
    if days > 0f64 {
        let start = if is_after { "newer"} else { "older" };
        let pl = if days == 1f64 { "" } else { "s"};
        let (num, unit) = extract_day_ref_pairs(days);
        let num_display = smart_dec_format(num);
        format!("{} than {} {}{}", start, num_display, unit, pl)
    } else {
        "All ages".to_owned()
    }
}

pub fn pluralize_64(single_form: &str, plural_form: &str, count: u64) -> String {
  if count == 1 {
    single_form.to_owned()
  } else {
    if plural_form == "s" || plural_form == "es" && plural_form.len() < single_form.len() {
      format!("{}{}", single_form, plural_form)
    } else {
      plural_form.to_owned()
    }
  }
}

pub fn seconds_to_day_hours_min_secs(seconds: u64) -> String {
  let secs_per_hour = 3600;
  let secs_per_day = secs_per_hour * 24;
  let has_days = seconds >= secs_per_day;
  let show_hours = seconds < secs_per_day * 3;
  let has_hours = seconds >= secs_per_hour;
  let show_minutes = seconds < secs_per_hour * 6;
  let has_minutes = seconds >= 60;
  let show_seconds = seconds < 60 * 5;
  if has_days {
    let days = seconds as f64 / secs_per_day as f64;
    if show_hours {
      let hours = (days % 1f64) * 24f64;
      format!("{:.0} {} {:.0}{}", days, pluralize_64("day", "s", days as u64), hours, "h")
    } else {
      format!("{:.0} {}", days, pluralize_64("day", "s", days as u64))
    }
  } else if has_hours {
    let hours = seconds as f64 / secs_per_hour as f64;
    if show_minutes {
      let minutes = (hours % 1f64) * 60f64;
      format!("{:.0}{} {:.0}{}", hours, "h", minutes, "m")
    } else {
      format!("{:.0}{}", hours, "h")
    }
  } else if has_minutes {
    let minutes = seconds as f64 / 60.0;
    if show_seconds {
      let secs = seconds % 60;
      format!("{:.0}{} {:.0}{}", minutes, "m", secs, "s")
    } else {
      format!("{:.0}{}", minutes, "m")
    }
  } else {
    format!("{:.0}{}", seconds, "s")
  }
}
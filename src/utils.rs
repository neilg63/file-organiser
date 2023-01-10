use walkdir::{DirEntry};
use std::time::UNIX_EPOCH;
use size::Size;

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
  if ext_list.clone().len() > 2 { ext_list.split(",").into_iter().map(|s| s.to_owned()).collect() } else { vec![] }
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

pub fn is_in_extensions(ext: &String, extensions: &Vec<String>) -> bool {
    if extensions.len() > 0 {
        extensions.iter().any(|s| s == ext)
    } else {
        true
    }
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

pub fn to_relative_path(current: &DirEntry, root: &Option<DirEntry>) -> String {
    if let Some(root_ref) = root {
        let root_comps = root_ref.path().components().into_iter().collect::<Vec<_>>();
        let num_root_parts = root_comps.len();
        let mut parts: Vec<String> = vec![];
        for (ci, item) in current.path().components().into_iter().enumerate() {
            let item_str = item.as_os_str().to_str().unwrap_or("");
            let par_ref = if ci < num_root_parts { root_comps.get(ci) } else { None };
            let is_root_part = par_ref.is_some() && par_ref.unwrap().as_os_str().to_str().unwrap_or("") == item_str;
            if !is_root_part {
                parts.push(item_str.to_string());
            }
        }
        parts.join("/").to_owned()
    } else {
        current.path().to_str().unwrap_or("").to_string()
    }
}

pub fn days_age_display(days: u32, is_after: bool) -> String {
    if days > 0 {
        let start = if is_after { "newer"} else { "older" };
        let day_word = if days == 1 { "day" } else { "days"};
        format!("{} than {} {} old", start, days, day_word)
    } else {
        "All ages".to_owned()
    }
}
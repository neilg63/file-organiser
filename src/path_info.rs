use std::path::{Path, PathBuf};
use std::env;
use crate::args::Args;
use crate::utils::{is_full_path,path_string_to_file_name,path_string_to_head};

/// Overview of a resource path
#[derive(Debug, Clone)]
pub struct PathInfo {
  pub path: Box<PathBuf>,
  pub canonical: String,
  pub exists: bool,
  pub input: String,
  pub pattern: Option<String>,
}

fn parse_expanded_path_args(paths: &Vec<String>) -> String {
  let names: Vec<String> = paths.into_iter().map(|p| path_string_to_file_name(p)).collect();
  let head = path_string_to_head(paths.get(0).unwrap());
  format!("{}({})", head, names.join("|"))
}

/// Build path information from std::path::Path
impl PathInfo {
  pub fn new(in_str: &str) -> Self {
    let mut path = Path::new(in_str);
    let input = in_str.to_owned();
    let mut exists = path.exists();
    let is_dir = path.is_dir();
    
    let mut pattern : Option<String> = None;
    if !is_dir {
      if let Some(par_path) = path.parent() {
        if let Some(p_os_str) = path.file_name() {
          pattern = Some(p_os_str.to_str().unwrap_or("").to_owned());
        }
        path = par_path;
        exists = par_path.exists() && par_path.is_dir();
      }
      
    }

    let mut canonical = "".to_owned();
    if exists {
        if let Ok(os_path) = path.canonicalize() {
            if let Some(full_path_str) = os_path.to_str() {
                canonical = full_path_str.to_owned();
            }
        }
    }
    PathInfo {
      path: Box::new(path.to_owned()),
      canonical,
      exists,
      input,
      pattern
    }
  }

  /// Default empty constructor
  pub fn new_empty() -> Self {
    PathInfo {
      path: Box::new(Path::new("").to_owned()),
      canonical: "".to_owned(),
      exists: false,
      input: "".to_owned(),
      pattern: None
    }
  }

  /// Build from command line arguments
  pub fn new_from_args(args: &Args) -> Self {
    let curr_ref = "".to_owned();
    let path_args = args.path.clone().unwrap_or(vec![".".to_string()]);
    let path_arg = if path_args.len() > 1 {
      parse_expanded_path_args(&path_args)
    } else if path_args.len() > 0 {
      path_args.get(0).unwrap().to_owned()
    } else {
      "".to_string()
    };

    let curr_path = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let path_str = if path_arg == curr_ref { curr_path.clone().to_owned() } else if is_full_path(path_arg.as_str()) { path_arg.to_owned() } else { format!("{}/{}", curr_path, path_arg) };
    PathInfo::new(&path_str)
  }

}
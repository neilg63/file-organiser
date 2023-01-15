use std::path::{Path, PathBuf};
use std::env;
use crate::args::Args;
use crate::utils::is_full_path;

#[derive(Debug, Clone)]
pub struct PathInfo {
  pub path: Box<PathBuf>,
  pub canonical: String,
  pub exists: bool,
  pub input: String,
}

impl PathInfo {
  pub fn new(in_str: &str) -> Self {
    let path = Path::new(in_str);
    let input = in_str.to_owned();
    let exists = path.exists();

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
    }
  }

  pub fn new_from_args(args: &Args) -> Self {
    let curr_ref = ".".to_string() ;
    let path_arg = args.path.clone().unwrap_or(curr_ref.clone());
    let curr_path = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let path_str = if path_arg == curr_ref { curr_path.clone().to_owned() } else if is_full_path(path_arg.as_str()) { path_arg.to_owned() } else { format!("{}/{}", curr_path, path_arg) };
    PathInfo::new(&path_str)
  }

}
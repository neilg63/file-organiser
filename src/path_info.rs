use std::path::{Path, PathBuf};

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
}
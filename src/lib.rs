use std::{
  collections::HashMap,
  path::{Path, PathBuf},
};

pub enum FindUpKind {
  File,
  Dir,
}

pub enum FindUpResult {
  Found(PathBuf),
  Continue,
  Stop,
}

/// FindUp is a utility for finding files or directories upward in the directory tree.
///
/// # Example
///
/// ```rust
/// use find_up::FindUp;
/// use find_up::FindUpKind;
///
/// let find_up = FindUp::new(".", FindUpKind::File);
/// let paths = find_up.find_up(&["package.json"]);
/// ```
///
/// # Example
///
/// ```rust
/// use find_up::FindUp;
/// use find_up::FindUpKind;
///
/// let find_up = FindUp::new(".", FindUpKind::Dir);
/// let paths = find_up.find_up(&["a"]);
/// ```
pub struct FindUp<P: AsRef<Path>> {
  pub cwd: P,
  pub kind: FindUpKind,
}

impl<P: AsRef<Path>> FindUp<P> {
  pub fn new(cwd: P, kind: FindUpKind) -> Self {
    Self { cwd, kind }
  }

  /// Find files or directories upward in the directory tree.
  ///
  /// # Example
  ///
  /// ```rust
  /// use find_up::FindUp;
  /// use find_up::FindUpKind;
  ///
  /// let find_up = FindUp::new(".", FindUpKind::File);
  /// let paths = find_up.find_up(&["package.json"]);
  /// ```
  pub fn find_up(&self, names: &[&str]) -> HashMap<String, Vec<PathBuf>> {
    self.find_up_with(names, move |p| {
      if !p.exists() {
        return FindUpResult::Continue;
      }

      let file_matched = matches!(self.kind, FindUpKind::File) && p.is_file();
      let dir_matched = matches!(self.kind, FindUpKind::Dir) && p.is_dir();

      if !(file_matched || dir_matched) {
        return FindUpResult::Continue;
      }

      FindUpResult::Found(p.to_path_buf())
    })
  }

  fn find_up_with<F>(&self, names: &[&str], f: F) -> HashMap<String, Vec<PathBuf>>
  where
    F: Fn(&PathBuf) -> FindUpResult,
  {
    let mut paths = HashMap::new();

    let mut cwd = self.cwd.as_ref();

    self.scan_directory(cwd, names, &mut paths, &f);

    while let Some(parent) = cwd.parent() {
      self.scan_directory(parent, names, &mut paths, &f);
      cwd = parent;
    }

    paths
  }

  fn scan_directory<F>(
    &self,
    cwd: &Path,
    names: &[&str],
    paths: &mut HashMap<String, Vec<PathBuf>>,
    f: &F,
  ) where
    F: Fn(&PathBuf) -> FindUpResult,
  {
    for name in names {
      let vecs = paths.entry(name.to_string()).or_default();
      let file = cwd.join(name);
      match f(&file) {
        FindUpResult::Found(path_buf) => {
          vecs.push(path_buf);
        }
        FindUpResult::Continue => {}
        FindUpResult::Stop => {
          break;
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_multiple_files_when_searching_upward() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let find_up = FindUp::new("fixtures/a/b/c/d", FindUpKind::File);
    let paths = find_up.find_up(&[package_json_name, node_version_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 2);

    if let Some(paths) = paths.get(package_json_name) {
      assert_eq!(paths.len(), 4);
    }

    if let Some(paths) = paths.get(node_version_name) {
      assert_eq!(paths.len(), 1);
    }
  }

  #[test]
  fn should_not_find_files_when_searching_for_directories() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let find_up = FindUp::new("fixtures/a/b/c/d", FindUpKind::Dir);
    let paths = find_up.find_up(&[package_json_name, node_version_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 2);

    if let Some(paths) = paths.get(package_json_name) {
      assert_eq!(paths.len(), 0);
    }

    if let Some(paths) = paths.get(node_version_name) {
      assert_eq!(paths.len(), 0);
    }
  }

  #[test]
  fn should_find_directory_in_parent_path() {
    let dir_name = "a";

    let find_up = FindUp::new("fixtures/a/b/c/d", FindUpKind::Dir);
    let paths = find_up.find_up(&[dir_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 1);

    if let Some(paths) = paths.get(dir_name) {
      assert_eq!(paths.len(), 1);
    }
  }
}

use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq)]
pub enum FindUpKind {
  File,
  Dir,
}

pub enum FindUpResult {
  Saved(PathBuf),
  Continue,
  Stop,
}

/// A builder for the `find_up` function.
///
/// # Example
///
/// ```rust
/// use find_up::{UpFinder, FindUpKind};
///
/// let find_up = UpFinder::builder().cwd(".").kind(FindUpKind::File).build();
/// let paths = find_up.find_up("package.json");
///
/// println!("{:#?}", paths);
/// ```
#[derive(Debug, PartialEq, TypedBuilder)]
pub struct UpFinder<P: AsRef<Path>> {
  /// The current working directory.
  cwd: P,
  /// The kind of file to search for.
  #[builder(default = FindUpKind::File)]
  kind: FindUpKind,
}

impl<P: AsRef<Path>> UpFinder<P> {
  /// Find a file in the current working directory and all parent directories.
  ///
  /// # Example
  ///
  /// ```rust
  /// use find_up::{UpFinder, FindUpKind};
  ///
  /// let find_up = UpFinder::builder().cwd(".").kind(FindUpKind::File).build();
  /// let paths = find_up.find_up("package.json");
  ///
  /// println!("{:#?}", paths);
  /// ```
  pub fn find_up(&self, name: &str) -> Vec<PathBuf> {
    let paths = self.find_up_multi(&[name]);

    if let Some(paths) = paths.get(name) {
      paths.clone()
    } else {
      vec![]
    }
  }

  /// Find multiple files in the current working directory and all parent directories.
  ///
  /// # Example
  ///
  /// ```rust
  /// use find_up::{UpFinder, FindUpKind};
  ///
  /// let find_up = UpFinder::builder().cwd(".").kind(FindUpKind::File).build();
  /// let paths = find_up.find_up_multi(&["package.json", ".node-version"]);
  ///
  /// println!("{:#?}", paths);
  /// ```
  pub fn find_up_multi(&self, names: &[&str]) -> FxHashMap<String, Vec<PathBuf>> {
    self.find_up_with_impl(self.cwd.as_ref().to_path_buf(), names, FindUpResult::Saved)
  }

  fn find_up_with_impl<F>(
    &self,
    cwd: PathBuf,
    names: &[&str],
    matcher: F,
  ) -> FxHashMap<String, Vec<PathBuf>>
  where
    F: Fn(PathBuf) -> FindUpResult,
  {
    let mut paths: FxHashMap<&str, Vec<PathBuf>> = FxHashMap::default();

    let mut cwd = cwd;

    loop {
      for &name in names {
        let vecs = paths.entry(name).or_default();

        let file = cwd.join(name);

        if !file.exists() {
          continue;
        }

        let matches_criteria = match self.kind {
          FindUpKind::File => file.is_file(),
          FindUpKind::Dir => file.is_dir(),
        };

        if !matches_criteria {
          continue;
        }

        match matcher(file) {
          FindUpResult::Saved(path) => {
            vecs.push(path);
          }
          FindUpResult::Continue => {
            continue;
          }
          FindUpResult::Stop => {
            break;
          }
        }
      }

      let Some(parent) = cwd.parent() else {
        break;
      };

      cwd = parent.to_path_buf();
    }

    paths
      .into_iter()
      .map(|(name, paths)| (name.to_string(), paths))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use insta::assert_debug_snapshot;

  use super::*;

  #[test]
  fn should_find_files_when_searching_upward() {
    let find_up = UpFinder::builder()
      .cwd("fixtures/a/b/c/d")
      .kind(FindUpKind::File)
      .build();

    let paths = find_up.find_up("package.json");

    assert_eq!(paths.len(), 4);

    assert_debug_snapshot!(paths);
  }

  #[test]
  fn should_find_multiple_files_when_searching_upward() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let find_up = UpFinder::builder()
      .cwd("fixtures/a/b/c/d")
      .kind(FindUpKind::File)
      .build();

    let paths = find_up.find_up_multi(&[package_json_name, node_version_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 2);

    if let Some(paths) = paths.get(package_json_name) {
      assert_eq!(paths.len(), 4);
    }

    if let Some(paths) = paths.get(node_version_name) {
      assert_eq!(paths.len(), 1);
    }

    assert_debug_snapshot!(paths);
  }

  #[test]
  fn should_not_find_files_when_searching_for_directories() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let find_up = UpFinder::builder()
      .cwd("fixtures/a/b/c/d")
      .kind(FindUpKind::Dir)
      .build();

    let paths = find_up.find_up_multi(&[package_json_name, node_version_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 2);

    if let Some(paths) = paths.get(package_json_name) {
      assert_eq!(paths.len(), 0);
    }

    if let Some(paths) = paths.get(node_version_name) {
      assert_eq!(paths.len(), 0);
    }

    assert_debug_snapshot!(paths);
  }

  #[test]
  fn should_find_directory_in_parent_path() {
    let dir_name = "a";

    let find_up = UpFinder::builder()
      .cwd("fixtures/a/b/c/d")
      .kind(FindUpKind::Dir)
      .build();

    let paths = find_up.find_up_multi(&[dir_name]);

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 1);

    if let Some(paths) = paths.get(dir_name) {
      assert_eq!(paths.len(), 1);
    }

    assert_debug_snapshot!(paths);
  }
}

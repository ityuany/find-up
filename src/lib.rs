use std::path::{Path, PathBuf};

use rustc_hash::FxHashMap;

pub enum FindUpKind {
  File,
  Dir,
}

pub enum FindUpResult {
  Saved(PathBuf),
  Continue,
  Stop,
}

pub struct FindUpOptions<P: AsRef<Path>> {
  pub cwd: P,
  pub kind: FindUpKind,
}

/// Find a file or directory upward in the directory tree.
///
/// # Examples
///
/// ```rust
/// use find_up::{find_up, FindUpOptions, FindUpKind};
///
/// let paths = find_up("package.json", FindUpOptions { cwd: ".", kind: FindUpKind::File });
///
/// ```
pub fn find_up<P: AsRef<Path>>(name: &str, options: FindUpOptions<P>) -> Vec<PathBuf> {
  let paths = find_up_multi(&[name], options);

  if let Some(paths) = paths.get(name) {
    paths.clone()
  } else {
    vec![]
  }
}

fn find_up_multi<P: AsRef<Path>>(
  names: &[&str],
  options: FindUpOptions<P>,
) -> FxHashMap<String, Vec<PathBuf>> {
  find_up_with_impl(
    options.cwd.as_ref().to_path_buf(),
    names,
    |path| FindUpResult::Saved(path),
    options.kind,
  )
}

fn find_up_with_impl<F>(
  cwd: PathBuf,
  names: &[&str],
  matcher: F,
  find_kind: FindUpKind,
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

      let matches_criteria = match find_kind {
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

#[cfg(test)]
mod tests {
  use insta::assert_debug_snapshot;

  use super::*;

  #[test]
  fn should_find_files_when_searching_upward() {
    let paths = find_up(
      "package.json",
      FindUpOptions {
        cwd: "fixtures/a/b/c/d",
        kind: FindUpKind::File,
      },
    );

    assert_eq!(paths.len(), 4);

    assert_debug_snapshot!(paths);
  }

  #[test]
  fn should_find_multiple_files_when_searching_upward() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let paths = find_up_multi(
      &[package_json_name, node_version_name],
      FindUpOptions {
        cwd: "fixtures/a/b/c/d",
        kind: FindUpKind::File,
      },
    );

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

    let paths = find_up_multi(
      &[package_json_name, node_version_name],
      FindUpOptions {
        cwd: "fixtures/a/b/c/d",
        kind: FindUpKind::Dir,
      },
    );

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

    let paths = find_up_multi(
      &[dir_name],
      FindUpOptions {
        cwd: "fixtures/a/b/c/d",
        kind: FindUpKind::Dir,
      },
    );

    println!("{:#?}", paths);

    assert_eq!(paths.len(), 1);

    if let Some(paths) = paths.get(dir_name) {
      assert_eq!(paths.len(), 1);
    }

    assert_debug_snapshot!(paths);
  }
}

use std::path::{Path, PathBuf};

use rustc_hash::FxHashMap;

pub enum FindUpKind {
  File,
  Dir,
}

pub enum FindUpResult {
  Found(PathBuf),
  Continue,
  Stop,
}

pub struct FindUpOptions<P: AsRef<Path>> {
  pub cwd: P,
  pub kind: FindUpKind,
}

pub fn find_up<P: AsRef<Path>>(
  names: &[&str],
  options: FindUpOptions<P>,
) -> FxHashMap<String, Vec<PathBuf>> {
  find_up_with(
    options.cwd.as_ref().to_path_buf(),
    names,
    move |p, kind| {
      if !p.exists() {
        return FindUpResult::Continue;
      }

      let file_matched = matches!(kind, FindUpKind::File) && p.is_file();
      let dir_matched = matches!(kind, FindUpKind::Dir) && p.is_dir();

      if !(file_matched || dir_matched) {
        return FindUpResult::Continue;
      }

      FindUpResult::Found(p.to_path_buf())
    },
    options.kind,
  )
}

fn find_up_with<F>(
  cwd: PathBuf,
  names: &[&str],
  f: F,
  find_kind: FindUpKind,
) -> FxHashMap<String, Vec<PathBuf>>
where
  F: Fn(&PathBuf, &FindUpKind) -> FindUpResult,
{
  let mut paths: FxHashMap<String, Vec<PathBuf>> = FxHashMap::default();

  let mut cwd = cwd;

  for name in names {
    let vecs = paths.entry(name.to_string()).or_default();

    let file = cwd.join(name);

    match f(&file, &find_kind) {
      FindUpResult::Found(path_buf) => {
        vecs.push(path_buf);
      }
      FindUpResult::Continue => {
        continue;
      }
      FindUpResult::Stop => {}
    }
  }

  while let Some(parent) = cwd.parent() {
    cwd = parent.to_path_buf();

    for name in names {
      let vecs = paths.entry(name.to_string()).or_default();

      let file = cwd.join(name);

      match f(&file, &find_kind) {
        FindUpResult::Found(path_buf) => {
          vecs.push(path_buf);
        }
        FindUpResult::Continue => {
          continue;
        }
        FindUpResult::Stop => {
          break;
        }
      }
    }
  }

  paths
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_multiple_files_when_searching_upward() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let paths = find_up(
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
  }

  #[test]
  fn should_not_find_files_when_searching_for_directories() {
    let package_json_name = "package.json";
    let node_version_name = ".node-version";

    let paths = find_up(
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
  }

  #[test]
  fn should_find_directory_in_parent_path() {
    let dir_name = "a";

    let paths = find_up(
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
  }
}

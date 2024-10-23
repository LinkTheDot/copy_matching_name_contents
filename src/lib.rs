use anyhow::anyhow;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Config {
  comparing_directory: PathBuf,
  copying_directory: PathBuf,
  destination: PathBuf,
}

impl Config {
  pub fn new<P: AsRef<Path>>(comparing_directory: P, copying_directory: P, destination: P) -> Self {
    Self {
      comparing_directory: comparing_directory.as_ref().to_path_buf(),
      copying_directory: copying_directory.as_ref().to_path_buf(),
      destination: destination.as_ref().to_path_buf(),
    }
  }

  /// Copies the files with matching names between comparing and copying from copying into destination.
  /// Any extensions are ignored.
  pub fn copy_matching_with_comparing(&self) -> anyhow::Result<()> {
    fs::DirBuilder::new()
      .recursive(true)
      .create(&self.destination)?;

    let matching_paths_from_copying_directory: Vec<PathBuf> = self.get_matching_paths()?;

    matching_paths_from_copying_directory
      .into_iter()
      .for_each(|path| {
        if let Err(error) = fs::copy(&path, self.destination.join(path.file_name().unwrap())) {
          log::error!("Failed to copy file {:?}, Reason: {:?}", path, error);
        }
      });

    Ok(())
  }

  /// Returns a list of files that were in copy but weren't in compare.
  pub fn get_missing_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
    let comparing_paths = self.file_paths_under_comparing()?;
    let copying_paths = self.file_paths_under_copying()?;

    let comparing_file_names: Vec<String> = paths_to_file_names(&comparing_paths)
      .into_iter()
      .flatten()
      .collect();
    let copying_file_names: Vec<(Option<String>, PathBuf)> = paths_to_file_names(&copying_paths)
      .into_iter()
      .zip(copying_paths)
      .collect();

    Ok(
      copying_file_names
        .into_iter()
        .filter_map(|(file_name, path)| {
          let file_name = file_name?;

          (!comparing_file_names.contains(&file_name)).then_some(path)
        })
        .collect(),
    )
  }

  fn get_matching_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
    let comparing_paths = self.file_paths_under_comparing()?;
    let copying_paths = self.file_paths_under_copying()?;

    let comparing_file_names: HashSet<String> = paths_to_file_names(&comparing_paths)
      .into_iter()
      .flatten()
      .collect();
    let copying_file_names: Vec<(Option<String>, PathBuf)> = paths_to_file_names(&copying_paths)
      .into_iter()
      .zip(copying_paths)
      .collect();

    Ok(
      copying_file_names
        .into_iter()
        .filter_map(|(file_name, path)| {
          let file_name = file_name?;

          if comparing_file_names.contains(&file_name) {
            Some(path)
          } else {
            log::info!("File name `{:?}` was not matching.", file_name);

            None
          }
        })
        .collect(),
    )
  }

  fn file_paths_under_comparing(&self) -> anyhow::Result<Vec<PathBuf>> {
    file_paths_under(&self.comparing_directory)
  }

  fn file_paths_under_copying(&self) -> anyhow::Result<Vec<PathBuf>> {
    file_paths_under(&self.copying_directory)
  }
}

/// Only files, directories are ignored.
fn file_paths_under<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
  let path: &Path = path.as_ref();

  if !path.is_dir() {
    return Err(anyhow!("Attempted to read a file as a directory."));
  }

  fs::read_dir(path)
    .map(|read_dir| {
      read_dir
        .filter_map(|entry_result| {
          let path = match entry_result {
            Ok(entry) => entry.path(),
            Err(error) => {
              log::error!("Failed to read a path. Reason: `{:?}`", error);
              return None;
            }
          };

          path.is_file().then_some(path)
        })
        .collect()
    })
    .map_err(Into::into)
}

/// Removes directories and extensions from paths and returns the resulting file name as a string.
fn paths_to_file_names(list: &[PathBuf]) -> Vec<Option<String>> {
  list
    .iter()
    .filter_map(|path| {
      let file_name = path
        .with_extension("")
        .file_name()
        .map(|osstr| osstr.to_str().map(|str| str.to_string()));

      if let Some(Some(_)) = &file_name {
      } else {
        log::warn!(
          "Failed to reduce path to just a file name. Path: `{:?}`",
          path
        );
      }

      file_name
    })
    .collect()
}

#![allow(dead_code)]

use clap::{Arg, Command};
use std::{path::PathBuf, sync::OnceLock};

pub struct Args {
  args: clap::ArgMatches,
}

impl Args {
  const COPY: &'static str = "copy";
  const COMPARE: &'static str = "compare";
  const DESTINATION: &'static str = "destination";
  const LOG_DIFF: &'static str = "log difference";
  const DEFAULT_DESTINATION_PATH: &'static str = "copy_dest";

  pub fn new() -> Self {
    let args = Self::setup_args();

    Self { args }
  }

  pub fn get_copy(&self) -> &'static PathBuf {
    static COPY_DIR: OnceLock<PathBuf> = OnceLock::new();

    COPY_DIR.get_or_init(|| {
      let Some(value) = self.args.get_one::<String>(Self::COPY) else {
        log::error!("Missing copy directory.");
        panic!("Missing copy directory.");
      };

      PathBuf::from(value)
    })
  }

  pub fn get_compare(&self) -> &'static PathBuf {
    static COMPARE_DIR: OnceLock<PathBuf> = OnceLock::new();

    COMPARE_DIR.get_or_init(|| {
      let Some(value) = self.args.get_one::<String>(Self::COMPARE) else {
        log::error!("Missing compare directory.");
        panic!("Missing compare directory.");
      };

      PathBuf::from(value)
    })
  }

  pub fn get_dest(&self) -> &'static PathBuf {
    static DEST_DIR: OnceLock<PathBuf> = OnceLock::new();

    DEST_DIR.get_or_init(|| {
      self
        .args
        .get_one::<String>(Self::DESTINATION)
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(Self::DEFAULT_DESTINATION_PATH))
        .clone()
    })
  }

  pub fn log_diff_flag(&self) -> bool {
    self.args.get_flag(Self::LOG_DIFF)
  }

  fn setup_args() -> clap::ArgMatches {
    Command::new("Copy Matching Name Contents")
      .arg(
        Arg::new(Self::COPY)
          .short('c')
          .long("copy")
          .action(clap::ArgAction::Set)
          .help("Sets the directory to copy files from after comparison."),
      )
      .arg(
        Arg::new(Self::COMPARE)
          .short('p')
          .long("comp")
          .action(clap::ArgAction::Set)
          .help("Sets the directory to compare files against for copying."),
      )
      .arg(
        Arg::new(Self::DESTINATION)
          .short('d')
          .long("dest")
          .action(clap::ArgAction::Set)
          .help("Sets the destination directory for files to copy into. The destination is recursively built if it doesn't already exist."),
      )
        .arg(
        Arg::new(Self::LOG_DIFF)
          .short('l')
          .long("logdiff")
          .action(clap::ArgAction::SetTrue)
          .help("With this flag set, the difference between copy and compare will be logged instead of running the program."),
      )
      .get_matches()
  }
}

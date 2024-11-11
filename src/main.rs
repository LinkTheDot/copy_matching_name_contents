use clap::Args;
use copy_matching_name_contents::*;

mod clap;
mod logger;

fn main() {
  let args = Args::new();

  if args.logs_disabled_flag() && args.log_diff_flag() {
    panic!("Logs cannot be disabled (-n) when the `log_diff` flag (-l) is set");
  }

  if !args.logs_disabled_flag() {
    if let Err(error) = crate::logger::setup_file_logger() {
      panic!("Failed to start logger. Reason: `{:?}`", error);
    }
  }

  let config = Config::new(args.get_compare(), args.get_copy(), args.get_dest());

  if !args.log_diff_flag() {
    log::info!("Copying matching files.");

    let result = config.copy_matching_with_comparing();

    if let Err(error) = result {
      log::error!("An error occurred when running the program: `{:?}`", error);
    }
  } else {
    log::info!("Logging missing paths");

    let result = config.get_missing_paths();

    match result {
      Ok(paths) => {
        log::info!("Missing total: {}", paths.len());
        log::info!("Missing paths: {:#?}", paths);
      }

      Err(error) => log::error!("Failed to get missing paths. Reason: `{:?}`", error),
    }
  }
}

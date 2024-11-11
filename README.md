This is a CLI tool that will copy matching file names between two files, and
copy the contents of a select one into a third, new directory.

Very niche, really only meant for myself.

# Building for your own target

```
cargo build --release
```

# Usage

```
./copy_matching_name_contents --copy copy_from_path --comp compare_against_path --dest destination_path
```

When the comparison is made, any parent directories are ignored, file extensions
are ignored,

# Some things to know

- File comparisons ignore parent directories (of course)
- File extensions are ignored
- Everything that matches is copied from the copying directory into the
  destination
- If the destination doesn't exist, it is created
- Any sub directories in the paths given are ignored (I didn't need a recursive
  feature)
- By default, logs will be sent to a file at the run location. Run with the -n
  flag to stop that.
- If you want to compare two directories to make sure you got everything, the -l
  flag will only run to write the difference between the copy and compare
  directories into the log file.

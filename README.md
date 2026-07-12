[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/file-organiser)
[![crates.io](https://img.shields.io/crates/v/file-organiser.svg)](https://crates.io/crates/file-organiser)
[![docs.rs](https://docs.rs/file-organiser/badge.svg)](https://docs.rs/file-organiser)

# _FileOrganiser_: Informative File Management Utility

FileOrganiser (fileorg) is a command line tool that lets you quickly list, move or delete large numbers of files in nested folders filtered by age, file extension, file name pattern and/or size range.

It does not seek to replace common utilities such as _ls_, (_dir_) and _find_ combined with _mv_ and _rm_ (_move_ or _del_), but provides a more transparent overview and streamlined workflow when managing large volumes of files.

This crate is still under development and I welcome feedback on its performance with different file systems. The utility uses the cross-platform [WalkDir](https://crates.io/crates/walkdir) crate and builds and runs on recent versions of Linux, Mac and Windows (verified via cross-compilation for `x86_64-pc-windows-gnu`, `x86_64-unknown-linux-gnu` and `x86_64-unknown-linux-musl`).

I have mainly used the development version on Linux servers to reorganise uploaded media files. Although it can only work within one file system at a time, it has no problems with mounted block storage volumes or S3 object-storage buckets that may use different file systems from the host operating system.

## Primary use cases

- Summarise file directory contents by size, age and extensions (-g flag)
- Filter file listings by age, size, extension(s) and/or file name pattern
- Move filtered files to another directory
- Delete filtered files (prompted without the -f flag)

## Known Issues

- Reading deeply nested directories with large numbers of files can be slow. The default max depth is thus set to 10. If you just want to find out the total disk usage, use `du -ch --max-depth 1` instead.
- If the target path ends in a filename with a wildcard, the command line interpreter will expand it internally into an array all matching file names. This is inefficient for 100 or more matching file names. Instead use the `-e jpeg,jpg` extension or `-p file_name_pattern` options when filtering by name or extension on thousands of files.
- The current implementation has to scan all directories and files before applying post-filters such as pattern matching. The standard _find . -name '[pattern]'_ is much faster if all you need to do is to find a file.

The following command will give you an overview of all jpeg, gif and png files in the target directories and subdirectories thereof to a max depth of 3 with a minimum file size of 5M and minimum age of 30 days.

These examples assume a system wide alias of _fileorg_

`fileorg -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30`

The -l flag reveals individual file entries with their age, date, type and relative path.

`fileorg -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l`

Should you wish to move these files to a target directory, respecting the original nested file structure, add a --move flag.

`fileorg -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --move /extended-drive/media`

Should you wish to delete these files, add a `--delete` or `-u` flag (`-d` stands for max depth)

`fileorg -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --delete`

## Arguments

- **--before, -b** only files modified before the specified number of days ago, `--before 30` _older than 30 days_. For other periods, you may use the suffixes `s` for seconds, `m` for minutes, `h` for hours, `w` for weeks or `y` for years, e.g. `5m` _5 minutes_ . You may add a range either via -a (--after) or simply with a dash, e.g. `-b 7-14` means between 7 and 14 days old while `-b 30m-12h` means between 30 minutes and 12 hours old.
- **--after, -a** only files modified after the specified number of days ago, `--after 30` _newer than 30 days_ . This may be combined with -b (--before) for an age range.
- **--size, -s** file size range with k (KB), m (MB) or g (GB) unit suffixes. e.g. 1-2M = 1MB to 2MB. One size alone is assumed to be the minimum. To set only a maximum prefix with a comma ( ,5MB) or use a 0-5M range.
- **--ext, -e** extensions, omit to allow all extensions
- **--not-ext, -n** extensions to be excluded, e.g. move or delete all files that do not include these extensions
- **--exclude-dirs, -q** directories to be excluded. These are relative to the target directory. If prefixed by your system's directory separator (`/` on Linux and Mac and `\` on Windows), it will exclude all subdirectories starting from the parent directory, otherwise it will exclude all subdirectories at any nesting level. You may exclude multiple subdirectory path with comma-separated lists e.g. `/node_modules,/dist` will exclude all files nested in these subdirectories.
- **--list, -l** Flag to show individual file details rather than just the overview
- **--groups, -g** Flag to show stats by extension groups before the main overview
- **--max-depth, -d** Max depth of subdirectories to scan. Defaults to 10 to limit overhead of parsing deeply nested directories. Max value is 255.
- **--pattern, -p** Match pattern for the file name. Add the `-x` flag to use full regular expressions in quotes.
- **--omit-pattern, -o** Omit file names matching this pattern. This may be combined with `--pattern, -p` or `--ext, -e` for more advanced pattern matching.
- **--starts-with** Match pattern from the start of the file name
- **--ends-with** Match pattern from the end of the file name, with or without the extension
- **--regex-mode, -x** Flag to interpret the above pattern as a full regular expression, e.g. where `a*` means any number of the preceding character, otherwise _\*_ is a wildcard for any characters, which in full regex mode is `.*`. For simple pattern matches `.` is interpreted literally, while in full regex mode it means any character and must be escaped to match a dot.
- **--copy, -c** Copy to specified new target directory. Takes precedence over `--move, -m`;
- **--move, -m** Move to specified new target directory
- **--delete, -u** Delete files filtered by the above criteria
- **--force, -f** Bypass prompt for bulk deletion (useful for cron jobs)
- **--hidden, -y** Match hidden files and directories, e.g. `.git` as folder or `.gitignore` as a file
- **--completions** Print a shell completion script for the given shell (`bash`, `zsh`, `fish`, `elvish` or `powershell`) to stdout and exit. See [Shell Completion](#shell-completion) below.

## Installation

- First ensure you have installed the [Rust Cargo compiler](https://doc.rust-lang.org/cargo/getting-started/installation.html) for your operating system
- checkout out the repository and change into its directory
- Run `cargo build --release`
- The executable will be at `target/release/file-organiser`
- Add an alias to the file-organiser, e.g. **fileorg**, or add a symbolic link to it a directory already in your system's export path.

### Shell Completion

Source or target path arguments and flags support tab completion via [clap_complete](https://crates.io/crates/clap_complete). Generate the script for your shell and source it, e.g. in `.bashrc`:

`source <(fileorg --completions bash)`

Substitute `zsh`, `fish`, `elvish` or `powershell` for other shells. The scan path completes to any file or directory, while `--move`/`-m` and `--copy`/`-c` complete to directories only. This only covers local paths for now; completion against remote storage such as S3 will need a dynamic completion engine once that's supported.

### Localization

All output text (headers, labels, prompts, action verbs, unit words) is looked up by machine name rather than hard-coded, with English defaults built into the binary from `src/lang/en.env`. Templates use positional placeholders (`{0}`, `{1}`, ...) so a translation can reorder values within a sentence.

New languages can be added without touching any source code:

- Copy `src/lang/en.env` to `lang/<code>.env` (e.g. `lang/fr.env`) in a `lang` directory next to the file-organiser executable, and translate the keys you care about; anything you leave out falls back to English.
- It's picked up automatically when the system's `LC_ALL`/`LANG` locale matches `<code>`, e.g. `LANG=fr_FR.UTF-8` loads `lang/fr.env` if present.
- `FO_LANG_DIR` redirects where that `lang` directory is looked up, if you'd rather not place it next to the binary.
- `FO_LANG_FILE` points at a single file that always wins, regardless of the detected locale - useful for testing a translation or tweaking a handful of strings.

### Screenshots

Basic listing with the -g option to show stats by extension:
![Screenshot 1](https://github.com/neilg63/file-organiser/blob/main/screenshots/file-org-1.png?raw=true)

Show only png files older than 2 years (-b before, -e extension):
![Screenshot 2](https://github.com/neilg63/file-organiser/blob/main/screenshots/file-org-2.png?raw=true)

Show full listing of png files older than 2 years and larger than 50MB (-b before, -e extension, -s size, -l full listing):
![Screenshot 2](https://github.com/neilg63/file-organiser/blob/main/screenshots/file-org-3.png?raw=true)

### Dev Notes

Version 0.1.6 corrects a reporting bug for files newer than 5 minutes old where 1m 25s was incorrectly reported as 2m 45s owing to rounding anomaly in the f64 to u64 conversion. I added a test for the `days_to_day_hours_min_secs()` function.

This is an alpha release. If anyone finds this useful, I may package it for release for the major operating systems.

Version 0.1.8 has a minor bug fix for Windows compatibility.

Version 0.2.0 updates all dependencies and includes several fixes and new features:

- Fixed a Windows compile error (`MetadataExt::size()` doesn't exist on Windows; it's `file_size()`), verified with a real cross-compiled and linked build rather than just type-checking.
- Fixed a bug where `--move`/`-m` to a target directory that didn't exist yet would silently move files into the wrong (parent) directory with no prompt. It now always prompts before creating a missing target, naming the shallowest missing path component.
- All output text is now localisable, with automatic system-locale detection and per-language files that can be added without touching source code; see [Localization](#localization).
- Added shell tab completion via `--completions <shell>`; see [Shell Completion](#shell-completion).
- Fixed an off-by-one in `--max-depth`/`-d`: it previously scanned one level shallower than requested (`-d 5` only reached 4 levels of nested subdirectories). Verified against a real move/delete run, not just listing/counting, since this gate also controls which directories get operated on.
- Raised the default `--max-depth` from 5 to 10. Benchmarked scanning an 18,000-file, 20-level-deep tree: time scales linearly with file count regardless of depth, no blowup, thanks in part to the cloning fixes above.
- Various internal performance fixes to avoid unnecessary cloning when summarising large directory trees.

[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/file-organiser)
[![crates.io](https://img.shields.io/crates/v/file-organiser.svg)](https://crates.io/crates/file-organiser)
[![docs.rs](https://docs.rs/file-organiser/badge.svg)](https://docs.rs/file-organiser)

# *FileOrganiser*: Informative File Management Utility

FileOrganiser (fileorg) is a command line tool that lets you quickly list, move or delete large numbers of files in nested folders filtered by age, file extension, file name pattern and/or size range.

It does not seek to replace common utilities such as _ls_, (_dir_) and _find_ combined with _mv_ and _rm_ (_move_ or _del_), but provides a more transparent overview and streamlined workflow when managing large volumes of files.

This crate is still under development and I welcome feedback on its performance with different file systems. The utility uses the cross-platform [WalkDir](https://crates.io/crates/walkdir) crate and should work on recent versions of Linux, Mac and Windows.

I have mainly used the development version on Linux servers to reorganise uploaded media files. Although it can only work within one file system at a time, it has no problems with mounted block storage volumes or S3 object-storage buckets that may use different file systems from the host operating system.

## Primary use cases

- Summarise file directory contents by size, age and extensions (-g flag)
- Filter file listings by age, size, extension(s) and/or file name pattern
- Move filtered files to another directory
- Delete filtered files (prompted without the -f flag)

## Known Issues

- Reading deeply nested directories with large numbers of files can be slow. The default max depth is thus set to 5. If you just want to find out the total disk usage, use `du -ch --max-depth 1` instead.
- If the target path ends in a filename with a wildcard, the command line interpreter will expand it internally into an array all matching file names. This is inefficient for 100 or more matching file names. Instead use the `-e jpeg,jpg` extension or `-p file_name_pattern` options when filtering by name or extension on thousands of files.
- The current implementation has to scan all directories and files before applying post-filters such as pattern matching. The standard *find . -name '[pattern]'* is much faster if all you need to do is to find a file.

The following command will give you an overview of all jpeg, gif and png files in the target directories and subdirectories thereof to a max depth of 3 with a minimum file size of 5M and minimum age of 30 days.

These examples assume a system wide alias of *fileorg*

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
- **--max-depth, -d** Max depth of subdirectories to scan. Defaults to 5 to limit overhead of parsing deeply nested directories. Max value is 255.
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

## Installation

- First ensure you have installed the [Rust Cargo compiler](https://doc.rust-lang.org/cargo/getting-started/installation.html) for your operating system
- checkout out the repository and change into its directory
- Run `cargo build --release`
- The executable will be at `target/release/file-organiser`
- Add an alias to the file-organiser, e.g. file-org, or add a symbolic link to it a directory already in your system's export path.

### Screenshots

Basic listing with the -g option to show stats by extension:
![Screenshot 1](https://github.com/neilg63/file-organiser/screenshots/file-org-1.png)

Show only png files older than 2 years (-b before, -e extension):
![Screenshot 2](https://github.com/neilg63/file-organiser/screenshots/file-org-2.png)

Show full listing of png files older than 2 years and larger than 50MB (-b before, -e extension, -s size, -l full listing):
![Screenshot 2](https://github.com/neilg63/file-organiser/screenshots/file-org-2.png)

### Dev Notes

This is an alpha release. 
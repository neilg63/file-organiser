# SmartMove Quick Bulk File Management Utility

Smartmove is a command line tool that lists, moves or deletes large numbers of files filtered by age, file extension and/or size range.
It does not seek to replace common utilities such as _ls_, _find_ and _exa_ combined with _mv_ and _rm_, but provides a more transparent streamlined workflow when managing large volumes of files.

NB: This is currently under development in its alpha and should be used with caution. I have now added the move and delete functionaliy, but still need to improved feedback and test on different file systems and operating systems. The application leverages contributed packages which are all cross-platform and should work on recent versions of Linux, Mac and Windows.

The following command will give you and overview of all jpeg, gif and png files in the target directories and subdirectories thereof to a max depth of 3 with a minimum file size of 5M and minimum age of 30 days
`smartmove -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30`

The -l flag reveals individual file entries with their age, date, type, and relative path.
`smartmove -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l`

Should you wish to move these files to a target directory, respecting the original nested file structure, add a --move flag.
`smartmove -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --move /extended-drive/media`

Should you wish to delete these files, add a remove flag
`smartmove -e jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --delete`

## Arguments

- --before, -b only files modified before the specified number of days ago, `--before 30` _older than 30 days_. For other periods, you may use the suffixes `s` for seconds, `m` for minutes, `h` for hours, `w` for weeks or `y` for years, e.g. `5m` _5 minutes_
- --after, -a only files modified after the specified number of days ago, `--newer 30` _newer than 30 days_
- --size, -s file size range with k (KB), m (MB) or g (GB) unit suffixes. e.g. 1-2M = 1MB to 2MB. One size alone is assumed to be the minimum. To set only a maximum prefix with a comma ( ,5MB) or use a 0-5M range.
- --ext, -e extensions, omit to allow all extensions
- --not_ext, -n extensions to be excluded, e.g. move or delete all files that do not include these extensions
- --exclude-dirs, -q directories at any nesting level to be excluded
- --list, -l Flag to show individual file details rather than just the overview
- --groups, -g Flag to show stats by extension groups before the main overview
- --max-depth, -d Max depth of subdirectories to scan. Defaults to 255 (pratcically unlimited).
- --pattern, -p Match pattern for the file name
- --starts-with Match pattern from the start of the file name
- --ends-with Match pattern from the end of the file name, with or without the extension
- --regex-mode, -x Flag to interpret the above pattern as a full regular express, e.g. `a*` means any number of the preceding character in full regex mode, but otherwise a wildcard for `a` followewd by any characters, which in full regex mode is `a.*`. For simple pattern matches `.` is interpreted literally, while in full regex mode it means any character and must be escaped to match a dot.
- --move, -m Move to specified new target directory
- --delete, -u Delete files filtered by the above criteria
- --force, -f Bypass prompt for bulk deletion (useful for cron jobs)
- --hidden, -c Match hidden files and directories, e.g. `.git` as folder or `.gitignore` as a file

## Installation

- First ensure you have installed the [Rust Cargo compiler](https://doc.rust-lang.org/cargo/getting-started/installation.html) for your operating system
- checkout out repository and change into the repository directory
- Run `cargo build --release`
- The executable will be at `target/release/smartmove`
- Add an alias to the file or ideally move it into a directory already in your system's export path. On Linux and Mac, this may be `/usr/local/bin`.

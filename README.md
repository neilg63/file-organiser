# SmartMove Quick Bulk File Management Utility

Smartmove is a command line tool to supplement _ls_, _exa_ and _find_ and let you list, move or delete large numbers of files filtered by age, file type and/or size range.

NB: This is currently under development and have yet to add the deletion and movement features. It only shows an overview and/or lists files. The other features will come soon.

The following command will give you and overview of all jpeg, gif and png files in the target directories and subdirectories thereof to a max depth of 3 with a minimum file size of 5M and minimum age of 30 days
`smartmove -ext jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30`

The -l flag reveals individual file entries with their age, date, type, and relative path.
`smartmove -ext jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l`

Should you wish to move these files to a target directory, respecting the original nested file structure, add a --move flag.
`smartmove -ext jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --move /extended-drive/media`

Should you wish to delete these files, add a remove flag
`smartmove -ext jpg,jpeg,gif,png --size 5M --max-depth 3 --before 30  -l --remove`

## Arguments

- --before, -b only files modified before the specified number of days ago, --before 30 older than 30 days
- --after, -a only files modified after the specified number of days ago, --newer 30 newer than 30 days
- --size, -s file size range with k (KB), m (MB) or g (GB) unit suffixes. e.g. 1-2M = 1MB to 2MB. One size alone is assumed to be the minimum. To set only a maximum prefix with a comma ( ,5MB) or use a 0-5M range.
- --ext, -e extensions, omit to allow all extensions
- --list, -l Flag to show individual file details rather than just the overview
- --max-depth, -d Max depth of subdirectories to scan. Defaults to 1.
- --move, -m Move to specified new target directory
- --remove, -x Delete files filtered by the above criteria

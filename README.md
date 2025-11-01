# File Organizer

A command-line tool for organizing and managing files based on various criteria.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/file-organizer`.

## Usage

```bash
file-organizer <COMMAND> <PATH>
```

### Available Commands

- `organize-by-creation-date` - Organize files into subdirectories based on creation date
- `separate-live-photo-videos` - Separate potential live photo videos from images

For help on any command:
```bash
file-organizer --help
file-organizer <COMMAND> --help
```

## Commands

### organize-by-creation-date

Organizes files in a directory into subdirectories based on their creation date. Files are moved into an `organized` folder with the structure: `organized/YYYY/MM/DD/`.

**Usage:**
```bash
file-organizer organize-by-creation-date <PATH>
```

**Example:**
```bash
file-organizer organize-by-creation-date /path/to/photos
```

**Behavior:**
- Shows a preview of the first 10 files to be organized
- Asks for confirmation before moving files
- Creates directory structure: `organized/2024/01/15/` for files created on January 15, 2024
- Skips files already in the `organized` folder
- Validates that no destination files already exist before moving

### separate-live-photo-videos

Identifies and separates potential Live Photo video files (.MOV) that have corresponding image files (.HEIC or .JPG) with matching filenames. This uses a heuristic approach to help isolate Live Photo videos for review.

**Usage:**
```bash
file-organizer separate-live-photo-videos <PATH>
```

**Example:**
```bash
file-organizer separate-live-photo-videos /path/to/photos
```

**Behavior:**
- Scans for .MOV files (uppercase extension only) that have a matching .HEIC or .JPG file
  - Example: `IMG_1234.MOV` matches with `IMG_1234.HEIC` or `IMG_1234.JPG`
- Shows a preview of the first 10 matching files
- Asks for confirmation before moving files
- Moves matching .MOV files to `potential_live_photo_videos` folder inside the provided path
- Leaves the corresponding image files (.HEIC/.JPG) in place
- Skips files already in `potential_live_photo_videos` or `organized` folders

## Notes

- Both commands skip `.DS_Store` files
- Symlinks are not supported
- Files are moved using `std::fs::rename` to preserve timestamps (requires same filesystem)
- The tool only processes files in the top-level directory (max depth: 1, no subdirectories)

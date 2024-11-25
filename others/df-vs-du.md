`df` and `du` are both command-line utilities used to report disk usage, but they serve different purposes and provide different types of information:

1. **df (Disk Free)**:
   - **Purpose**: Reports the amount of disk space available on the file system as a whole.
   - **Usage**: Typically used to get an overview of the disk space usage and availability on mounted file systems.
   - **Output**: Shows the total size, used space, available space, and the percentage of used space for each mounted file system.
   - **Example**:
     ```sh
     df -h
     ```
     This command provides a human-readable summary of disk space usage for all mounted file systems.

2. **du (Disk Usage)**:
   - **Purpose**: Estimates the disk space used by files and directories.
   - **Usage**: Typically used to find out how much space a particular directory or file is consuming.
   - **Output**: Shows the disk space used by each specified file and directory, and can recursively summarize the space used by directories.
   - **Example**:
     ```sh
     du -h /path/to/directory
     ```
     This command provides a human-readable summary of disk space usage for the specified directory and its contents.

### Key Differences:
- **Scope**:
  - `df` provides a high-level view of disk space usage across entire file systems.
  - `du` provides a detailed view of disk space usage at the file and directory level.

- **Use Case**:
  - Use `df` when you need to check overall disk space availability and usage on your system.
  - Use `du` when you need to identify which files or directories are consuming the most space.

- **Output**:
  - `df` outputs information about file systems.
  - `du` outputs information about individual files and directories.

Both tools are useful for managing disk space, but they are used in different contexts depending on the level of detail required.

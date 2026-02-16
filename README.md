# Organizer - File Organizer in Rust

A cross-platform desktop application that scans a selected folder, groups loose files by their extension, and organizes them into categorized subfolders within an `Organizer/` directory.

Built with **Rust**, **egui** (for the GUI interface) and **rfd** (for native file dialogs).

## Features

- **GUI Interface**: Easy folder selection via native dialog
- **Smart Organization**: Scans loose files (not folders) in the selected directory
- **Auto-Categorization**: Creates folders named by file extension (e.g., `PDF`, `JPG`, `MP3`) inside `Organizer/`
- **No Extension Files**: Handles files without extensions in a `Sin_Extension` folder
- **Safe Moving**: Files are moved to their respective folders
- **Detailed Statistics**: Shows a complete report of the organization process
- **Error Handling**: Manages filesystem errors and displays them to the user

## Project Structure

```
src/
├── main.rs          # Entry point — launches the GUI interface
├── ui.rs            # UI layer — egui components and event handling
└── core.rs          # Core logic — scanning, organizing and moving files
```

## How It Works

1. **Select Folder**: User selects a target folder using the "Seleccionar Carpeta..." button
2. **List Files**: Click "Listar Archivos" to view all files and folders in the directory
3. **Organize Files**: Click "Organizar por Extension" to:
   - Scan the folder for files (ignores subdirectories and the Organizer folder)
   - Create `Organizer/` directory if it doesn't exist
   - For each file, extract its extension and create the corresponding folder
   - Move files without extensions to the `Sin_Extension/` folder
   - Move files to their respective extension folders
4. **View Results**: See moved files, created folders, and any errors found

## Directory Structure Created

After organization, the selected folder will contain:

```
SelectedFolder/
├── Organizer/
│   ├── PDF/          # All .pdf files
│   ├── JPG/          # All .jpg/.jpeg files
│   ├── TXT/          # All .txt files
│   ├── ZIP/          # All .zip files
│   ├── EXE/          # All .exe files
│   └── Sin_Extension/# Files without extensions
└── (remaining folders and the Organizer folder)
```

## Prerequisites

- Rust (stable version) — https://rustup.rs
- Cargo (Rust package manager)

## Build and Run

```bash
cargo run
```

## Using the Application

1. Run the application with `cargo run`
2. Click "Seleccionar Carpeta..." to choose the folder you want to organize
3. Optional: Click "Listar Archivos" to see current contents
4. Click "Organizar por Extension" to start the organization process
5. Review the summary with operation statistics
6. Use "Limpiar" to reset the interface

## Important Notes

- The application only organizes files at the top level of the selected directory
- Does not organize files inside subfolders
- Does not modify the `Organizer/` folder if it already exists
- Files are moved, not copied (permanent operation)
- It's recommended to backup important files before organizing

## Technologies Used

- **Rust**: Main programming language
- **egui**: GUI library
- **rfd**: Native file dialogs
- **std::fs**: Rust filesystem operations

## Contributing

1. Fork the repository
2. Create a branch for your feature (`git checkout -b new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin new-feature`)
5. Create a Pull Request

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
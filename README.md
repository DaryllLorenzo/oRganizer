# 📁 Organizer — Rust File Organizer

> A simple, cross-platform desktop application that scans a selected folder, groups loose files by their extension, and organizes them into categorized subfolders inside an `organizer/` directory.

Built with **Rust**, **egui** (for UI), and **rfd** (for native file dialogs).

---

## 🎯 Features

- 🖱️ **Graphical Interface**: Easy folder selection via native dialog.
- 🧹 **Smart Organization**: Scans for loose files (non-folders) in the selected directory.
- 📂 **Auto-Categorization**: Creates folders named by file extension (e.g., `.pdf`, `.jpg`, `.mp3`) inside `organizer/`.
- 🚚 **Safe Move**: Files are moved (not copied) into their respective folders.
- ✅ **User Confirmation**: Requires explicit “Accept” before performing any action.

---

## 🧱 Project Structure
src/
├── main.rs          # Entry point — launches the UI
├── ui.rs            # UI layer — egui components and event handling
├── core.rs          # Core logic — file scanning, organizing, moving


---

## ⚙️ How It Works

1. User selects a target folder via the UI.
2. Clicks “Accept” to trigger the organizer.
3. App:
   - Scans the folder for files (ignores subdirectories).
   - Creates `organizer/` if it doesn’t exist.
   - For each file, creates a subfolder named after its extension (e.g., `.png` → `organizer/.png/`).
   - Moves the file into its corresponding folder.
4. Logs result to console (can be extended with UI feedback).

---

## 🚀 Quick Start

### Prerequisites

- Rust (stable) — https://rustup.rs
- Cargo

### Build & Run

```bash
git clone <your-repo-url>
cd organizer
cargo run

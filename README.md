# 📸 Metadata Extractor (Rust)

A simple command-line tool written in Rust that extracts image metadata and EXIF information.

## ✨ Features

### 📁 Basic Metadata

* File Size
* Image Format
* Width & Height
* Aspect Ratio

### 📷 EXIF Metadata

* Camera Make
* Camera Model
* Date Taken
* ISO
* Exposure Time
* Aperture
* Focal Length
* Lens Information

### 🛡️ Error Handling

* Gracefully handles images without EXIF metadata.
* Prevents crashes when EXIF data is missing.

---

## 🦀 What I Learned

While building this project, I explored:

* Rust ownership and borrowing
* References (`&Path`)
* Pattern matching with `match`
* Error handling using `Result`
* File I/O
* Buffered readers (`BufReader`)
* Working with external crates
* Image processing basics
* EXIF metadata standards

---

## 🔧 Tech Stack

* Rust
* image crate
* exif crate

---

## 📦 Requirements & Installation

Before running the project, make sure you have the following installed:

### Requirements

* Rust (latest stable version)
* Cargo (comes bundled with Rust)

### Install Rust

#### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

Download and install Rust from:
https://www.rust-lang.org/tools/install

Verify the installation:

```bash
rustc --version
cargo --version
```

### Clone the Repository

```bash
git clone <repository-url>
cd <repository-folder>
```

### Install Dependencies

Cargo will automatically download and build all required dependencies from `Cargo.toml` when you run the project for the first time.

## ▶️ Run Locally

```bash
cargo run
```

Enter the image path when prompted:

```text
/path/to/image.jpg
```

---

## 🌱 Future Improvements

* GPS location extraction
* JSON output support
* Batch processing multiple images
* Better formatting for EXIF values
* Export metadata to file

---

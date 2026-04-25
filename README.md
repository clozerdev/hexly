<div align="center">
  <h1>Hexly</h1>
  <p><em>A modern, native GUI for inspecting NFC smart card memory</em></p>

  <p><em>Built with:</em></p>
  <img alt="Rust" src="https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=rust&logoColor=white">
  <img alt="GTK4" src="https://img.shields.io/badge/GTK4-4A86CF.svg?style=flat&logo=gtk&logoColor=white">
  <img alt="libadwaita" src="https://img.shields.io/badge/libadwaita-3D3D3D.svg?style=flat">

<p>
  <a href="#-overview">🌌 Overview</a> •
  <a href="#-features">✨ Features</a> •
  <a href="#-design-goals">🎯 Design Goals</a> •
  <a href="#-installation">📥 Installation</a>
</p>
</div>

---

> 🚧 This project is still under development, so some features may not yet be available.

## 🌌 Overview

**Hexly** is a **modern, native GUI application for inspecting NFC smart card memory**, focused on **MIFARE Classic cards**.

Built with **Rust**, **GTK4**, and **libadwaita**, Hexly provides a clean and intuitive interface to explore low-level card data without sacrificing clarity or control.

Hexly focuses on:

- **clarity over abstraction**
- **visibility over hidden processing**
- **native desktop integration**

It is designed for developers, tinkerers, and security enthusiasts who want to **inspect, understand, and experiment with card memory structures**.

---

## ✨ Features

- **Smart Card Interaction**
  - Connect to PC/SC-compatible readers
  - Read NFC card memory

- **Structured Memory View**
  - Sector and block visualization
  - Clear grouping of memory regions
  - Highlighted data layout

- **Hex & Raw Data Inspection**
  - View raw bytes in hex format
  - Inspect data block-by-block
  - Easy-to-read layout for debugging

- **Modern GTK Interface**
  - Built with GTK4 + libadwaita
  - Native look on GNOME and Wayland desktops
  - Responsive and clean UI

- **Safe by Design**
  - Focus on **read-only inspection**
  - No hidden writes or destructive operations
  - Transparent interaction with hardware

---

## 🎯 Design Goals

Hexly aims to provide a balance between:

- low-level control
- visual clarity
- modern UX

### 🧠 Philosophy

> *Expose the data clearly — without getting in the way.*

Hexly avoids:

- unnecessary abstractions
- hidden transformations
- overcomplicated workflows

---

## 📥 Installation

### Requirements

Note that `cargo build` and `cargo run` are no longer supported, as additional build steps are required.

- gtk4 >= 4.18
- libadwaita >= 1.7
- pcsclite >= 2.3

### Using Meson

Make sure you have Meson installed on your system:
```bash
sudo apt install meson
sudo dnf install meson
sudo pacman -S meson
```

Then use the following commands to build and install the application

```bash
meson setup build
ninja -C build
ninja -C build install
```

If you do not want to install the application system-wide, you can use a prefix:

```bash
meson setup build --prefix=$HOME/.local
ninja -C build
ninja -C build install
```

---

## 🧠 Notes

- Hexly uses **PC/SC** to communicate with smart card readers
- Make sure your reader is supported and `pcscd` is running:

```bash
systemctl status pcscd
```

---

## 📄 License

MIT License.

---

## ⭐ Support

If Hexly is useful to you:

- ⭐ Star the repository
- 🐞 Report issues
- 💡 Suggest features
- 🔧 Open pull requests

---

## 🔮 Future Plans

- Write support (optional, explicit)
- Better sector decoding
- Key management tools
- Advanced analysis features

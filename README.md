# ![hanifx](https://via.placeholder.com/50) hanifx Rust Module

[![Crates.io](https://img.shields.io/crates/v/hanifx)](https://crates.io/crates/hanifx)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80+-orange)](https://www.rust-lang.org/)

`hanifx` is a **Rust-based encoding and decoding tool** designed for **Linux**, **Ubuntu**, **Arch Linux**, and **Termux (Android)**.  
It provides a safe and efficient solution for **text and file encoding/decoding**.

---

## 🔧 Features

- Text and file encoding & decoding
- Safe data handling
- Cross-platform: Linux and Android (Termux)
- Simple and easy-to-use API

---

## 🖥 Supported Platforms

- Linux (Ubuntu / Debian)
- Arch Linux
- Termux (Android)

---

## 🛠 Installation

<details>
<summary>1. Install Rust</summary>

To use `hanifx`, you first need **Rust** and **Cargo** installed.


### Ubuntu / Debian:

```bash
sudo apt update
sudo apt install rust ('error problem try this⬇️')
apt install rust

### Arch linux sudoudo pacman -Syu
sudo pacman -S rust
rustc --versicargorgo --version

### Termux (Android):

pkg update && pkg upgrade
pkg install rust
rustc --version
cargo --version

> ⚠️ If rustup is not available in Termux, installing via pkg install rust works fine.



</details><details>
<summary>2. Install Hanifx Crate</summary>Once Rust and Cargo are installed, you can install hanifx:

cargo install hanifx

> ⚠️ To use hanifx in your own project, add it to your Cargo.toml:



[dependencies]
hanifx = "0.1.0"

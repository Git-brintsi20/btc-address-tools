<div align="center">
  <img src="https://raw.githubusercontent.com/yourusername/btc-address-tools/main/assets/banner.png" alt="BTC Address Tools Banner" width="800"/>
  <br/>
  <h1>Bitcoin Address Toolkit</h1>
  <p>
    <strong>A comprehensive, educational command-line toolkit for exploring Bitcoin addresses, HD wallets, and cryptographic principles. Built with Rust for safety and performance.</strong>
  </p>
  <p>
    <a href="https://github.com/yourusername/btc-address-tools/actions/workflows/rust.yml">
      <img src="https://github.com/yourusername/btc-address-tools/actions/workflows/rust.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/rust-1.70%2B-orange.svg" alt="Rust Version">
    </a>
    <a href="LICENSE">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License">
    </a>
     <a href="https://crates.io/crates/btc-address-tools">
      <img src="https://img.shields.io/crates/v/btc-address-tools.svg" alt="Crates.io">
    </a>
  </p>
</div>

---

## 🧡 A Summer of Bitcoin Project
This toolkit was developed as a learning exercise to deeply understand the mechanics of Bitcoin addresses and wallets. It serves as a practical, hands-on educational resource for anyone interested in the technical foundations of Bitcoin, making it a perfect companion for aspiring Bitcoin developers.

### Core Philosophy: Learn by Doing
The best way to understand Bitcoin is to build with it. This tool was created with an educational-first mindset. Every feature, from the detailed validation output to the step-by-step `learn` command, is designed to demystify the "magic" behind Bitcoin's cryptographic foundations. It encourages exploration and provides a safe environment to experiment with concepts that are often theoretical.

## ⚠️ Security Warning
**FOR EDUCATIONAL PURPOSES ONLY.** This tool is designed for learning and experimentation on Bitcoin's mainnet and testnet.
- 🚫 **Never use generated private keys for real funds.**
- 🎓 **The primary goal is education, not secure storage.**
- 🔒 **For actual Bitcoin storage, always use a reputable hardware wallet (e.g., Ledger, Trezor).**

---

## ✨ Features Overview
This toolkit provides a suite of commands to demystify Bitcoin addresses and wallets.

| Feature | Description | Supported Standards |
|---|---|---|
| 🔍 **Address Validator** | Validate and dissect any Bitcoin address (P2PKH, P2SH, P2WPKH, P2TR). | All major address types |
| 🎲 **HD Wallet Generator** | Create new BIP39-compliant wallets with mnemonic phrases. | BIP32, BIP39, BIP44 |
| 📥 **Mnemonic Recovery** | Import and recover wallets from existing mnemonic phrases. | BIP39 |
| 🔄 **Format Converter** | Convert public keys into all corresponding address formats. | P2PKH, P2SH, P2WPKH, P2TR |
| ✨ **Example Generator** | Quickly generate sample data like public keys for testing. | N/A |
| 📚 **Educational Mode** | An interactive, step-by-step guide to creating a Bitcoin address from scratch. | Core crypto principles |

---

## 🎬 Interactive Demo
The best way to experience the toolkit is through its interactive mode.

*(This is a placeholder. You can record a terminal session and convert it to a GIF using tools like `asciinema` and `agg`.)*
![BTC Address Tools Demo GIF](https://raw.githubusercontent.com/yourusername/btc-address-tools/main/assets/demo.gif)

```bash
btc-tools interactive
```

---

## 🚀 Getting Started

### Prerequisites
- [Rust 1.70+](https://www.rust-lang.org/tools/install)
- `git` for cloning the repository.

### Installation & Build
```bash
# 1. Clone the repository
git clone https://github.com/yourusername/btc-address-tools.git
cd btc-address-tools

# 2. Build the project in release mode
cargo build --release

# 3. Run the executable directly
./target/release/btc-tools --help

# (Optional) 4. Install it on your system path
cargo install --path .
btc-tools --help
```

---

## 📖 Command-Line Usage
Here are some examples of how to use the tool directly from your terminal.

### 1. Validate an Address
```bash
# Get detailed info on a Taproot address
btc-tools validate bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297
```

### 2. Generate a New HD Wallet
```bash
# Generate a standard 12-word mnemonic wallet
btc-tools generate

# Create a 24-word mnemonic and show private keys (for educational use!)
btc-tools generate --words 24 --show-private-keys
```

### 3. Convert a Public Key
```bash
# Generate all address types from a public key for mainnet
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5

# Generate testnet addresses from the same public key
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5 --testnet
```

### 4. Learn About Address Creation
```bash
# Start the interactive educational tutorial
btc-tools learn
```

---

## 🎓 Understanding Derivation Paths
This tool uses the BIP44 standard for derivation paths: `m / purpose' / coin_type' / account' / change / address_index`.

| Component | Description | Bitcoin Value |
|---|---|---|
| `m` | Denotes the master key | `m` |
| `purpose'` | The standard being used | `44'` (for BIP44) |
| `coin_type'` | The cryptocurrency | `0'` (Bitcoin Mainnet), `1'` (Testnet) |
| `account'` | Organizes funds into separate accounts | `0'` (for the first account) |
| `change` | Distinguishes receiving vs. change addresses | `0` (Receiving), `1` (Change) |
| `address_index` | The sequential address number | `0, 1, 2, ...` |

**Example:** `m/44'/0'/0'/0/5` is the 6th receiving address of the first mainnet account.

---

## 🛠️ Technical Stack & Standards
- **Language:** Rust (2021 Edition)
- **Core Libraries:**
  - `bitcoin` v0.31 for core data structures.
  - `secp256k1` v0.28 for cryptographic functions.
  - `bip39` v2.0 for mnemonic phrase generation.
- **CLI Framework:** `clap` v4.4 for robust argument parsing.
- **Interactive UI:** `dialoguer` and `colored` for a user-friendly experience.

---

## 🧪 Testing
The project includes a comprehensive test suite to ensure correctness.
```bash
# Run all tests
cargo test

# Run with detailed output
cargo test -- --nocapture
```

## 🤝 Contributing
Contributions are welcome! Please see the [Contributing Guidelines](CONTRIBUTING.md) for more details on how to get involved.

## 📝 License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

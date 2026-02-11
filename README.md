# 🔑 Bitcoin Address Toolkit

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/yourusername/btc-address-tools/actions/workflows/rust.yml/badge.svg)](https://github.com/yourusername/btc-address-tools/actions)

A comprehensive Bitcoin address toolkit for validation, HD wallet generation, format conversion, and education. Built with Rust for security and performance.

## ⚠️ Security Warning

**FOR EDUCATIONAL PURPOSES ONLY**

- Never use generated keys for real Bitcoin funds
- This tool is designed for learning and testing
- Use hardware wallets (Ledger, Trezor) for actual Bitcoin storage
- Private keys generated here should never hold real value

## ✨ Features

### 1. 🔍 Address Validator
Validate any Bitcoin address and get detailed information:
- **Supported Formats:**
  - Legacy (P2PKH): `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa`
  - Script Hash (P2SH): `3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy`
  - Native SegWit (Bech32): `bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq`
  - Taproot (Bech32m): `bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297`
- **Network Support:** Mainnet and Testnet.
- **Validation Checks:**
  - ✅ Checksum verification
  - ✅ Address type detection
  - ✅ Network identification (mainnet/testnet)
  - ✅ Compatibility information
  - ✅ Fee comparison

### 2. 🎲 HD Wallet Generator
Generate hierarchical deterministic (HD) wallets using BIP39:
- Generate 12 or 24-word mnemonic phrases
- Derive multiple addresses using BIP44 derivation paths
- Display addresses, public keys, and private keys (optional)
- Support for custom derivation paths
- Full BIP32/BIP39/BIP44 compliance

### 3. 📥 Mnemonic Import & Recovery
Import existing wallets from mnemonic phrases:
- Validate 12 or 24-word BIP39 mnemonics
- Custom derivation path support
- Recover multiple addresses from seed
- Detailed path explanation
- Compatible with major wallet software

### 4. 🔄 Address Format Converter
Convert between different Bitcoin address formats:
- Input: Public key (hex) or any address
- Output: All equivalent address formats
  - Legacy (P2PKH)
  - P2SH-SegWit (Wrapped)
  - Native SegWit (P2WPKH)
  - Taproot (P2TR) info
- **Testnet Support:** Generate testnet addresses with the `--testnet` flag.
- Transaction fee comparison
- Format recommendations

### 5. ✨ Example Data Generator
Generate sample data for testing purposes:
- Create a valid, random compressed public key.
- Useful for quickly testing the `convert` command without needing to generate a full wallet.

### 6. 📚 Educational Mode
Interactive tutorial explaining Bitcoin address generation:
- Step-by-step address creation process
- Private key → Public key → Address
- Explanation of hashing (SHA-256, RIPEMD-160)
- Encoding formats (Base58Check, Bech32)
- Visual diagrams and flow
- Perfect for learning Bitcoin fundamentals

## 🚀 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/btc-address-tools.git
cd btc-address-tools

# Build the project
cargo build --release

# The binary will be in target/release/btc-tools
```

### Quick Install

```bash
cargo install --path .
```

## 📖 Usage

### Interactive Mode (Recommended)

```bash
btc-tools interactive
```

This launches a user-friendly menu where you can choose any feature.

### Command Line Interface

#### Validate an Address
```bash
btc-tools validate 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
btc-tools validate bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
```

#### Generate New HD Wallet
```bash
# Generate 12-word mnemonic, 5 addresses
btc-tools generate

# Generate 24-word mnemonic, 10 addresses
btc-tools generate --words 24 --count 10

# Show private keys (⚠️ USE WITH CAUTION)
btc-tools generate --show-private-keys
```

#### Import Existing Mnemonic
```bash
# Import with default path
btc-tools import --mnemonic "your twelve word mnemonic phrase here..."

# Custom derivation path
btc-tools import \
  --mnemonic "your mnemonic here" \
  --path "m/44'/0'/0'/0" \
  --count 5

# Show private keys
btc-tools import --mnemonic "..." --show-private-keys
```

#### Convert Address Formats
```bash
# From public key (Mainnet)
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5

# From public key (Testnet)
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5 --testnet

# From existing address (shows info only)
btc-tools convert bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
```

#### Generate Example Data
```bash
btc-tools example
```

#### Educational Mode
```bash
btc-tools learn
```

### Help & Options

```bash
btc-tools --help
btc-tools generate --help
btc-tools import --help
```

## 🎓 Understanding Derivation Paths

BIP44 derivation path format: `m / purpose' / coin_type' / account' / change / address_index`

| Component | Description | Bitcoin Value |
|-----------|-------------|---------------|
| m | Master key | Always 'm' |
| purpose' | BIP number | 44' (BIP44) |
| coin_type' | Cryptocurrency | 0' (Bitcoin), 1' (Testnet) |
| account' | Account number | 0' (first account) |
| change | Address type | 0 (receiving), 1 (change) |
| address_index | Sequential index | 0, 1, 2, ... |

**Examples:**
- `m/44'/0'/0'/0/0` - First receiving address, first account
- `m/44'/0'/0'/0/1` - Second receiving address
- `m/44'/0'/0'/1/0` - First change address
- `m/44'/1'/0'/0/0` - Testnet address

## 🛠️ Technical Details

### Technology Stack
- **Language:** Rust 2021 Edition
- **Bitcoin Library:** `bitcoin` v0.31
- **BIP39:** `bip39` v2.0
- **Cryptography:** `secp256k1` v0.28
- **CLI Framework:** `clap` v4.4
- **Terminal UI:** `dialoguer` + `colored`

### Supported Standards
- ✅ BIP32 (Hierarchical Deterministic Wallets)
- ✅ BIP39 (Mnemonic Code for HD Wallets)
- ✅ BIP44 (Multi-Account Hierarchy)
- ✅ BIP141/173 (SegWit)
- ✅ BIP350 (Bech32m for Taproot)

### Address Types

| Type | Prefix | Encoding | Fee Savings | Introduced |
|------|--------|----------|-------------|------------|
| P2PKH | 1... | Base58Check | Baseline | 2009 |
| P2SH | 3... | Base58Check | ~25% | 2012 |
| P2WPKH | bc1q... | Bech32 | ~40% | 2017 |
| P2TR | bc1p... | Bech32m | ~40% + privacy | 2021 |

## 📊 Example Output

### Address Validation
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🔍 VALIDATING BITCOIN ADDRESS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ VALID ADDRESS

  Address: bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
  Type: P2WPKH (Pay-to-Witness-PubKey-Hash)
  Description: Native SegWit, lower fees
  Network: Mainnet (Production)

  💡 Characteristics:
  • Native SegWit (recommended)
  • ~40% lower transaction fees
  • Starts with 'bc1q'
  • Bech32 encoding
```

### HD Wallet Generation
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🎲 GENERATING NEW HD WALLET
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  🔑 BIP39 Mnemonic Seed Phrase:
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   1. abandon     2. ability     3. able        4. about
   5. above       6. absent      7. absorb      8. abstract
   9. absurd     10. abuse      11. access     12. accident
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  📊 DERIVED ADDRESSES:
  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  Address #0
  Path: m/44'/0'/0'/0/0
  Address: bc1q...
  Public Key: 02c604...
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with verbose output:

```bash
cargo test -- --nocapture
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

This project uses `rustfmt` and `clippy`:

```bash
cargo fmt
cargo clippy
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔒 Security

This tool is designed for educational purposes. Key security considerations:

- ✅ All operations run locally (no network calls)
- ✅ Uses cryptographically secure randomness (`OsRng`)
- ✅ No telemetry or data collection
- ⚠️ Not audited for production use
- ⚠️ Never use for real Bitcoin storage

## 🙏 Acknowledgments

- Bitcoin Core developers
- rust-bitcoin team
- BIP39 specification authors
- The entire Bitcoin community

## 📚 Resources

### Learn More About Bitcoin
- [Bitcoin Developer Guide](https://developer.bitcoin.org/)
- [Mastering Bitcoin](https://github.com/bitcoinbook/bitcoinbook) by Andreas Antonopoulos
- [BIP39 Specification](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP32 Specification](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP44 Specification](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)

### Rust Bitcoin Development
- [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin)
- [rust-secp256k1](https://github.com/rust-bitcoin/rust-secp256k1)
- [rust-bip39](https://github.com/rust-bitcoin/rust-bip39)

## 📧 Contact

For questions or feedback, please open an issue on GitHub.

---

**Remember: This tool is for learning only. Always use hardware wallets for real Bitcoin! 🔐**

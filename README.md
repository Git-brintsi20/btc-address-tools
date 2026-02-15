<div align="center">

  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=28&duration=3000&pause=1000&color=F7931A&center=true&vCenter=true&multiline=true&repeat=false&width=600&height=80&lines=%E2%82%BF+Bitcoin+Address+Toolkit">
    <img alt="Bitcoin Address Toolkit" src="https://readme-typing-svg.demolab.com?font=Fira+Code&weight=700&size=28&duration=3000&pause=1000&color=F7931A&center=true&vCenter=true&multiline=true&repeat=false&width=600&height=80&lines=%E2%82%BF+Bitcoin+Address+Toolkit">
  </picture>

  <br/>

  <em>A comprehensive, educational CLI & web toolkit for exploring Bitcoin addresses,<br/>HD wallets, and cryptographic principles. Built with Rust.</em>

  <br/><br/>

  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-2021_Edition-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License">
  </a>
  <img src="https://img.shields.io/badge/Bitcoin-Education-f7931a?style=for-the-badge&logo=bitcoin&logoColor=white" alt="Bitcoin Education">
  <a href="https://btc-address-tools.vercel.app">
    <img src="https://img.shields.io/badge/Live_Demo-▶_Try_It-28a745?style=for-the-badge&logo=vercel&logoColor=white" alt="Live Demo">
  </a>

  <br/><br/>

  <a href="https://btc-address-tools.vercel.app"><strong>🌐 Live Demo</strong></a> &nbsp;&nbsp;•&nbsp;&nbsp;
  <a href="#-getting-started-cli"><strong>🚀 Quick Start</strong></a> &nbsp;&nbsp;•&nbsp;&nbsp;
  <a href="#-features-at-a-glance"><strong>✨ Features</strong></a> &nbsp;&nbsp;•&nbsp;&nbsp;
  <a href="https://github.com/Git-brintsi20/btc-address-tools"><strong>📦 GitHub</strong></a>

</div>

<br/>

<div align="center">
  <table>
    <tr>
      <td align="center"><b>🔍 Validate</b><br/><sub>P2PKH · P2SH · SegWit · Taproot</sub></td>
      <td align="center"><b>🎲 Generate</b><br/><sub>BIP39 HD Wallets · 12-24 words</sub></td>
      <td align="center"><b>🔄 Convert</b><br/><sub>Pubkey → All address formats</sub></td>
      <td align="center"><b>📚 Learn</b><br/><sub>Step-by-step crypto tutorial</sub></td>
    </tr>
  </table>
</div>

---

> [!WARNING]
> **FOR EDUCATIONAL PURPOSES ONLY.** Never use generated keys for real funds.
> Use hardware wallets (Ledger, Trezor) for actual Bitcoin storage.

---

## 🧡 Summer of Bitcoin

This toolkit was built as a hands-on learning project to deeply understand the mechanics behind Bitcoin addresses and wallets. It goes beyond theory — every feature is designed to let you **see, touch, and experiment** with the cryptographic primitives that power Bitcoin.

<details>
<summary><b>🎯 Core Philosophy: Learn by Doing</b></summary>
<br/>

The best way to understand Bitcoin is to build with it. This toolkit takes an educational-first approach:

- **Transparent outputs** — Every validation shows the full script breakdown, hash values, and encoding details
- **Step-by-step mode** — The `learn` command walks through private key → public key → address creation
- **Safe experimentation** — Generate wallets, convert keys, and validate addresses without risk
- **Dual interface** — Both a native Rust CLI and a zero-dependency browser app for accessibility

</details>

---

## ✨ Features at a Glance

<table>
  <thead>
    <tr>
      <th width="50">🏷️</th>
      <th width="200">Feature</th>
      <th>Description</th>
      <th width="180">Standards</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>🔍</td>
      <td><b>Address Validator</b></td>
      <td>Validate and dissect any Bitcoin address with full type detection, network info, script breakdown, and encoding details.</td>
      <td><code>P2PKH</code> <code>P2SH</code> <code>P2WPKH</code> <code>P2WSH</code> <code>P2TR</code></td>
    </tr>
    <tr>
      <td>🎲</td>
      <td><b>HD Wallet Generator</b></td>
      <td>Generate BIP39-compliant hierarchical deterministic wallets with 12–24 word mnemonics.</td>
      <td><code>BIP32</code> <code>BIP39</code> <code>BIP44</code></td>
    </tr>
    <tr>
      <td>📥</td>
      <td><b>Mnemonic Recovery</b></td>
      <td>Import and recover wallets from existing mnemonic seed phrases.</td>
      <td><code>BIP39</code></td>
    </tr>
    <tr>
      <td>🔄</td>
      <td><b>Format Converter</b></td>
      <td>Convert any compressed public key into all 4 address formats simultaneously.</td>
      <td><code>P2PKH</code> <code>P2SH</code> <code>P2WPKH</code> <code>P2TR</code></td>
    </tr>
    <tr>
      <td>📚</td>
      <td><b>Educational Mode</b></td>
      <td>Interactive, step-by-step walkthrough of Bitcoin address creation from scratch.</td>
      <td>ECDSA · SHA-256 · RIPEMD-160 · Base58 · Bech32</td>
    </tr>
    <tr>
      <td>🌐</td>
      <td><b>Web Interface</b></td>
      <td>All tools accessible from the browser. Pure JS with zero dependencies — crypto implemented from scratch.</td>
      <td><a href="https://btc-address-tools.vercel.app">Try it live →</a></td>
    </tr>
  </tbody>
</table>

---

## 🌐 Web App

<div align="center">

  **[▶ btc-address-tools.vercel.app](https://btc-address-tools.vercel.app)**

  <sub>Beautiful dark UI · Zero dependencies · All crypto implemented from scratch</sub>

</div>

<br/>

The web interface mirrors the CLI's functionality directly in the browser. Every piece of Bitcoin cryptography — SHA-256, RIPEMD-160, Base58Check, Bech32, Bech32m — is written in **pure JavaScript from scratch** for full educational transparency. No npm packages, no build step, no black boxes.

---

## 🚀 Getting Started (CLI)

### Prerequisites
- [Rust 1.70+](https://www.rust-lang.org/tools/install)

### Quick Install
```bash
git clone https://github.com/Git-brintsi20/btc-address-tools.git
cd btc-address-tools
cargo build --release
```

### Run
```bash
./target/release/btc-tools --help
```

<details>
<summary><b>📦 Optional: Install system-wide</b></summary>

```bash
cargo install --path .
btc-tools --help
```
</details>

---

## 📖 Usage Examples

<details open>
<summary><b>🔍 Validate an Address</b></summary>

```bash
btc-tools validate bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297
```
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  🔍 VALIDATING BITCOIN ADDRESS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✅ VALID ADDRESS

  Address:  bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297
  Type:     Taproot (P2TR)
  Network:  Bitcoin Mainnet

  📋 Technical Details:
  Script Type:   Witness V1 Taproot
  Encoding:      Bech32m
```
</details>

<details>
<summary><b>🎲 Generate a New HD Wallet</b></summary>

```bash
# Standard 12-word wallet
btc-tools generate

# 24-word wallet with private keys visible
btc-tools generate --words 24 --show-private-keys
```
</details>

<details>
<summary><b>🔄 Convert a Public Key</b></summary>

```bash
# All address formats from a public key
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5

# Testnet addresses
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5 --testnet
```
</details>

<details>
<summary><b>📚 Learn Bitcoin Address Creation</b></summary>

```bash
btc-tools learn
```
An interactive step-by-step walkthrough: Private Key → Public Key (ECDSA) → Hash160 → Address Encoding.
</details>

---

## 🎓 Derivation Paths (BIP44)

```
m / purpose' / coin_type' / account' / change / address_index
```

| Component | Meaning | Bitcoin Value |
|:---:|---|---|
| `m` | Master key | — |
| `purpose'` | Standard | `44'` (BIP44) |
| `coin_type'` | Cryptocurrency | `0'` Mainnet · `1'` Testnet |
| `account'` | Account index | `0'` |
| `change` | Receiving / Change | `0` / `1` |
| `address_index` | Address number | `0, 1, 2 …` |

> **Example:** `m/44'/0'/0'/0/5` → 6th receiving address of the first mainnet account.

---

## 🛠️ Tech Stack

<table>
  <tr>
    <td><b>🦀 Language</b></td>
    <td>Rust (2021 Edition)</td>
  </tr>
  <tr>
    <td><b>₿ Bitcoin</b></td>
    <td><code>bitcoin</code> v0.31 — core data structures & encoding</td>
  </tr>
  <tr>
    <td><b>🔐 Crypto</b></td>
    <td><code>secp256k1</code> v0.28 — elliptic curve operations</td>
  </tr>
  <tr>
    <td><b>🌱 BIP39</b></td>
    <td><code>bip39</code> v2.0 — mnemonic generation & recovery</td>
  </tr>
  <tr>
    <td><b>🖥️ CLI</b></td>
    <td><code>clap</code> v4.4 + <code>dialoguer</code> + <code>colored</code></td>
  </tr>
  <tr>
    <td><b>🌐 Web</b></td>
    <td>Vanilla HTML/CSS/JS — zero dependencies, all crypto from scratch</td>
  </tr>
</table>

---

## 🧪 Testing

```bash
cargo test                  # Run all tests
cargo test -- --nocapture   # With detailed output
```

---

## 🤝 Contributing

Contributions are welcome! Please see the [Contributing Guidelines](CONTRIBUTING.md) for details.

## 📝 License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.

<div align="center">
  <br/>
  <sub>Built with 🧡 for the Bitcoin community</sub>
</div>

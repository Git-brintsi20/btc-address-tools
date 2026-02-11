# Quick Start Guide

## Installation & Build

```bash
# Clone or navigate to the project
cd btc-address-tools

# Build in release mode (optimized)
cargo build --release

# The binary is now at: target/release/btc-tools.exe (Windows) or target/release/btc-tools (Linux/Mac)
```

## Quick Test Commands

### 1. Show Help
```bash
btc-tools --help
```

### 2. Validate Bitcoin Addresses

```bash
# Validate a Legacy address
btc-tools validate 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

# Validate a SegWit address
btc-tools validate bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq

# Validate a P2SH address
btc-tools validate 3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy

# Test invalid address
btc-tools validate invalid_address_test
```

### 3. Generate New HD Wallet

```bash
# Generate with defaults (12 words, 5 addresses)
btc-tools generate

# Generate 24-word mnemonic
btc-tools generate --words 24

# Generate 10 addresses
btc-tools generate --count 10

# Show private keys (⚠️ EDUCATIONAL ONLY!)
btc-tools generate --show-private-keys
```

### 4. Import from Mnemonic

```bash
# Test mnemonic (DO NOT USE FOR REAL FUNDS)
btc-tools import --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# With custom derivation path
btc-tools import \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" \
  --path "m/44'/0'/0'/0" \
  --count 3

# Show private keys
btc-tools import \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" \
  --show-private-keys
```

### 5. Convert Address Formats

```bash
# From compressed public key (example)
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5

# From an existing address (shows info only)
btc-tools convert bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
```

### 6. Educational Mode

```bash
# Run interactive tutorial
btc-tools learn
```

### 7. Interactive Mode (Easiest!)

```bash
# Launch menu-driven interface
btc-tools interactive
```

## Expected Output Examples

### Address Validation Output
```
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

### HD Wallet Generation Output
```
🔑 BIP39 Mnemonic Seed Phrase:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 1. word1    2. word2    3. word3    4. word4
 5. word5    6. word6    7. word7    8. word8
 9. word9   10. word10  11. word11  12. word12
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 DERIVED ADDRESSES:
  
Address #0
Path: m/44'/0'/0'/0/0
Address: bc1q...
Public Key: 02...
```

## Testing with Known Values

### Test Mnemonic (BIP39 Standard Test Vector)
```
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
```

**Expected First Address:**
- Path: m/44'/0'/0'/0/0
- Address: bc1qcr8te4kr609gcawutmrza0j4xv80jy8z306fyu

### Test Public Key
```
02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5
```

## Common Use Cases

### 1. Verify a Customer's Bitcoin Address
```bash
btc-tools validate <address>
```

### 2 Generate Test Wallets for Development
```bash
btc-tools generate --count 20
```

### 3. Recover Addresses from Backup Phrase
```bash
btc-tools import --mnemonic "<your 12 or 24 words>"
```

### 4. Learn Bitcoin Address Mechanics
```bash
btc-tools learn
```

## Performance

- Address validation: < 1ms
- HD wallet generation: ~50ms for 5 addresses
- Interactive mode: Instant response

## Troubleshooting

### Build Issues
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Runtime Issues
- Ensure you're using Rust 1.70+
- Check that all dependencies are compatible
- On Windows, use PowerShell or CMD, not Git Bash for colored output

## Safety Reminders

⚠️ **NEVER** use generated mnemonics or private keys for real Bitcoin!

✅ **DO**:
- Use this tool for learning
- Test with testnet addresses
- Use hardware wallets for real funds

❌ **DON'T**:
- Store generated keys for production use
- Share mnemonics or private keys
- Trust software-generated keys with real money

## Next Steps

1. ✅ Try all commands above
2. 📚 Read the [main README](README.md) for full documentation
3. 🎓 Run `btc-tools learn` for educational mode
4. 🔍 Validate your own addresses
5. 💻 Explore the [source code](src/) to learn implementation details

---

**Happy Learning! 🚀**

For issues or questions, please open an issue on GitHub.

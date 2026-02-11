# Examples

This directory contains example usage of the Bitcoin Address Toolkit.

## Quick Start Examples

### 1. Validate a Bitcoin Address

```bash
# Validate a Legacy address
btc-tools validate 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

# Validate a SegWit address
btc-tools validate bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq

# Validate a P2SH address
btc-tools validate 3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy
```

### 2. Generate HD Wallet

```bash
# Generate with default settings (12 words, 5 addresses)
btc-tools generate

# Generate with 24-word mnemonic
btc-tools generate --words 24

# Generate 10 addresses
btc-tools generate --count 10

# Show private keys (educational only!)
btc-tools generate --show-private-keys
```

### 3. Import Existing Wallet

```bash
# Import from interactive prompt
btc-tools import

# Import with mnemonic as argument (use quotes)
btc-tools import --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Custom derivation path
btc-tools import \
  --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about" \
  --path "m/44'/0'/0'/0" \
  --count 5
```

### 4. Convert Address Formats

```bash
# Convert from public key (compressed, 33 bytes hex)
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5

# Check address info (shows what format an address is)
btc-tools convert bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq
```

### 5. Learn Mode

```bash
# Run interactive tutorial
btc-tools learn
```

## Test Mnemonics (BIP39)

**⚠️ These are test mnemonics - NEVER use for real funds!**

### 12-word test mnemonic:
```
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about
```

### 24-word test mnemonic:
```
abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art
```

## Example Derivation Paths

| Path | Description |
|------|-------------|
| `m/44'/0'/0'/0/0` | First receiving address, first account |
| `m/44'/0'/0'/0/1` | Second receiving address |
| `m/44'/0'/1'/0/0` | First address, second account |
| `m/44'/1'/0'/0/0` | First testnet address |
| `m/84'/0'/0'/0/0` | First native SegWit address (alternative standard) |

## Sample Public Keys for Testing

```bash
# Example compressed public keys:
btc-tools convert 02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5
btc-tools convert 03cbcaa9c98c877a26977d00825c956a238e8dddfbd322cce4f74b0b5bd6ace4a7
```

## Interactive Mode

For the easiest experience, use interactive mode:

```bash
btc-tools interactive
```

This will show you a menu with all available options:
- 🔍 Validate Bitcoin Address
- 🎲 Generate New HD Wallet
- 📥 Import Existing Mnemonic
- 🔄 Convert Address Formats
- 📚 Educational Mode (Learn)
- ❌ Exit

## Real-World Use Cases (Educational)

### Use Case 1: Address Validation Service
```bash
# Validate user input before processing
btc-tools validate $USER_PROVIDED_ADDRESS
```

### Use Case 2: Wallet Recovery
```bash
# Recover addresses from backup mnemonic
btc-tools import \
  --mnemonic "your backup phrase here" \
  --count 20
```

### Use Case 3: Format Conversion
```bash
# Show all format equivalents for a public key
btc-tools convert $PUBLIC_KEY_HEX
```

## Tips

1. **Always use quotes** when passing mnemonics as arguments
2. **Never share** output that contains private keys
3. **Use hardware wallets** for real Bitcoin storage
4. **Test thoroughly** with testnet before mainnet
5. **Keep backups** of important mnemonics (on paper, not digitally)

## Next Steps

- Read the [main README](../README.md) for full documentation
- Check out the [source code](../src/) to understand implementation
- Learn about [BIP39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- Study [BIP32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki) and [BIP44](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki)

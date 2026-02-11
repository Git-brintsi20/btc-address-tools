# Contributing to Bitcoin Address Toolkit

Thank you for your interest in contributing! This project is designed for educational purposes and welcomes contributions that enhance learning about Bitcoin addresses.

## 📋 Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Areas for Contribution](#areas-for-contribution)

## 🤝 Code of Conduct

This project adheres to a code of conduct emphasizing:
- Respectful communication
- Educational focus
- Security awareness
- Constructive feedback
- Inclusive environment

## 🚀 Getting Started

1. **Fork the repository**
2. **Clone your fork**
   ```bash
   git clone https://github.com/yourusername/btc-address-tools.git
   cd btc-address-tools
   ```
3. **Build the project**
   ```bash
   cargo build
   ```
4. **Run tests**
   ```bash
   cargo test
   ```

## 💻 Development Setup

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)
- Git
- Text editor or IDE (VS Code with rust-analyzer recommended)

### Recommended Tools
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install formatting tool
rustup component add rustfmt

# Install linter
rustup component add clippy
```

### Project Structure
```
btc-address-tools/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── validator.rs     # Address validation
│   ├── hd_wallet.rs     # HD wallet generation
│   ├── converter.rs     # Format conversion
│   ├── educational.rs   # Tutorial mode
│   └── utils.rs         # Helper functions
├── Cargo.toml           # Dependencies
├── README.md            # Main documentation
└── examples/            # Usage examples
```

## 🔧 How to Contribute

### Types of Contributions

1. **Bug Reports**
   - Check existing issues first
   - Provide detailed description
   - Include steps to reproduce
   - Share error messages/logs

2. **Feature Requests**
   - Explain the use case
   - Describe expected behavior
   - Consider educational value
   - Keep security in mind

3. **Code Contributions**
   - Fix bugs
   - Add features
   - Improve documentation
   - Enhance user experience
   - Add tests

4. **Documentation**
   - Fix typos
   - Improve clarity
   - Add examples
   - Translate content

## 📝 Coding Standards

### Rust Style Guide
Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/):

```rust
// Use meaningful names
fn validate_bitcoin_address(address: &str) -> Result<()> {
    // Function implementation
}

// Document public APIs
/// Validates a Bitcoin address and displays detailed information
pub fn validate_address(address_str: &str) -> Result<()> {
    // Implementation
}

// Handle errors properly
match address.parse() {
    Ok(addr) => process_address(&addr),
    Err(e) => return Err(anyhow!("Invalid address: {}", e)),
}
```

### Code Quality Checklist
- [ ] Follows Rust naming conventions
- [ ] Includes doc comments for public items
- [ ] Handles errors appropriately
- [ ] No compiler warnings
- [ ] Passes `cargo clippy`
- [ ] Formatted with `cargo fmt`
- [ ] Security warnings where appropriate

### Security Guidelines
- **ALWAYS** include warnings for private key display
- **NEVER** remove security warnings
- **EMPHASIZE** educational purpose
- **AVOID** encouraging real fund usage
- **USE** cryptographically secure randomness (`OsRng`)

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin
```

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_address() {
        let result = validate_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_address() {
        let result = validate_address("invalid");
        assert!(result.is_ok()); // Function handles error internally
    }
}
```

### Test Coverage Goals
- Core functionality: 80%+
- Error handling: 70%+
- Edge cases: 60%+

## 🔄 Pull Request Process

### Before Submitting
1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make changes**
   - Write clear commit messages
   - Keep commits focused
   - Test your changes

3. **Run checks**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   cargo build --release
   ```

4. **Update documentation**
   - Update README if needed
   - Add code comments
   - Update CHANGELOG

### Submitting PR
1. Push to your fork
   ```bash
   git push origin feature/your-feature-name
   ```

2. Open Pull Request on GitHub
   - Use descriptive title
   - Explain what and why
   - Reference related issues
   - Include testing details

3. Wait for review
   - Address feedback
   - Update as needed
   - Be patient and respectful

### PR Checklist
- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] No new compiler warnings
- [ ] Code formatted with `rustfmt`
- [ ] Clippy checks pass
- [ ] Documentation updated
- [ ] Security considerations addressed
- [ ] Educational value maintained

## 🎯 Areas for Contribution

### High Priority
- [ ] Add unit tests for all modules
- [ ] Add integration tests
- [ ] Improve error messages
- [ ] Add more address validation tests
- [ ] Performance optimization

### Medium Priority
- [ ] Add QR code generation
- [ ] Support testnet addresses
- [ ] Add batch processing mode
- [ ] Improve educational content
- [ ] Add more examples

### Nice to Have
- [ ] GUI version
- [ ] Web version (WASM)
- [ ] Multi-language support
- [ ] Advanced cryptography explanation
- [ ] Bitcoin Script tutorials

### Documentation
- [ ] More usage examples
- [ ] Video tutorials
- [ ] API documentation
- [ ] Architecture diagrams
- [ ] Learning path guide

## 🐛 Bug Report Template

```markdown
**Describe the bug**
A clear and concise description.

**To Reproduce**
Steps to reproduce:
1. Run command '...'
2. Enter input '...'
3. See error

**Expected behavior**
What you expected to happen.

**Actual behavior**
What actually happened.

**Environment:**
 - OS: [e.g. Windows 11]
 - Rust version: [e.g. 1.75]
 - Project version: [e.g. 1.0.0]

**Additional context**
Any other relevant information.
```

## ✨ Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear description of the problem.

**Describe the solution you'd like**
What you want to happen.

**Describe alternatives you've considered**
Other solutions you've thought about.

**Educational value**
How does this help users learn?

**Additional context**
Any other relevant information.
```

## 📖 Resources

### Learning Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Bitcoin Development
- [Bitcoin Developer Guide](https://developer.bitcoin.org/)
- [Mastering Bitcoin](https://github.com/bitcoinbook/bitcoinbook)
- [BIP Repository](https://github.com/bitcoin/bips)

### Project-Specific
- [rust-bitcoin docs](https://docs.rs/bitcoin/)
- [secp256k1 docs](https://docs.rs/secp256k1/)
- [bip39 docs](https://docs.rs/bip39/)

## 🏅 Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Mentioned in release notes
- Credited in documentation

## 📞 Getting Help

- **GitHub Issues**: For bugs and features
- **Discussions**: For questions and ideas
- **Documentation**: Read README and examples first

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Quick Checklist for Contributors

```bash
# 1. Fork and clone
git clone https://github.com/yourusername/btc-address-tools.git

# 2. Create branch
git checkout -b feature/my-feature

# 3. Make changes and test
cargo fmt
cargo clippy
cargo test
cargo build --release

# 4. Commit and push
git add .
git commit -m "Add: brief description"
git push origin feature/my-feature

# 5. Open PR on GitHub
```

---

**Thank you for helping make Bitcoin education better! 🚀**

Questions? Open an issue or discussion on GitHub.

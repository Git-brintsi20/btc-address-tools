use anyhow::Result;
use bitcoin::{Address, Network, PublicKey, PrivateKey};
use bitcoin::secp256k1::Secp256k1;
use colored::*;
use std::io::{self, Write};

/// Run interactive educational tutorial
pub fn run_tutorial() -> Result<()> {
    println!("\n{}", "═".repeat(70).bright_magenta());
    println!("{}", "  📚 BITCOIN ADDRESS GENERATION TUTORIAL".bright_yellow().bold());
    println!("{}", "  Learn How Bitcoin Addresses Work Step-by-Step".white());
    println!("{}", "═".repeat(70).bright_magenta());
    
    println!("\n{}", "  Welcome to the interactive Bitcoin address tutorial!".bright_white());
    println!("  {}", "We'll generate a Bitcoin address from scratch and explain each step.".white());
    
    // Step 1: Generate Private Key
    wait_for_enter("\n  Press Enter to start with Step 1: Private Key Generation...");
    step1_private_key()?;
    
    // Step 2: Derive Public Key
    wait_for_enter("\n  Press Enter to continue to Step 2: Public Key Derivation...");
    let pubkey = step2_public_key()?;
    
    // Step 3: Hash the Public Key
    wait_for_enter("\n  Press Enter to continue to Step 3: Hashing...");
    step3_hashing(&pubkey)?;
    
    // Step 4: Generate Address
    wait_for_enter("\n  Press Enter to continue to Step 4: Address Generation...");
    step4_address_generation(&pubkey)?;
    
    // Step 5: Encoding Explained
    wait_for_enter("\n  Press Enter to continue to Step 5: Encoding Formats...");
    step5_encoding()?;
    
    // Summary
    display_summary();
    
    Ok(())
}

fn step1_private_key() -> Result<()> {
    println!("\n{}", "  ━".repeat(35).bright_magenta());
    println!("{}", "  STEP 1: Private Key Generation".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).bright_magenta());
    
    println!("\n  {} What is a Private Key?", "📖".bright_yellow());
    println!("  {}", "• A random 256-bit (32-byte) number".white());
    println!("  {}", "• Must be kept SECRET - it controls your Bitcoin".white());
    println!("  {}", "• Generated using cryptographically secure randomness".white());
    println!("  {}", "• Range: 1 to ~2^256 (astronomically large)".white());
    
    // Generate a private key
    let secp = Secp256k1::new();
    let (_secret_key, _) = secp.generate_keypair(&mut rand::rngs::OsRng);
    let private_key = PrivateKey::new(_secret_key, Network::Bitcoin);
    
    println!("\n  {} Generated Private Key:", "🔑".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    println!("  {} {}", "Hex:".bold(), _secret_key.display_secret().to_string().bright_red());
    println!("  {} {} bytes", "Length:".bold(), "32".bright_white());
    println!("  {} WIF (Wallet Import Format)", "Format:".bold());
    println!("  {} {}", "WIF:".bold(), private_key.to_wif().dimmed());
    
    println!("\n  {} Security Note:", "⚠️".bright_red());
    println!("  {}", "• Anyone with this key can spend your Bitcoin".red());
    println!("  {}", "• Never share or store it insecurely".red());
    println!("  {}", "• This example is for learning only!".red());
    
    Ok(())
}

fn step2_public_key() -> Result<PublicKey> {
    println!("\n{}", "  ━".repeat(35).bright_magenta());
    println!("{}", "  STEP 2: Public Key Derivation".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).bright_magenta());
    
    println!("\n  {} How is the Public Key Derived?", "📖".bright_yellow());
    println!("  {}", "• Uses Elliptic Curve Cryptography (ECC)".white());
    println!("  {}", "• Curve: secp256k1 (same as used by Bitcoin)".white());
    println!("  {}", "• Formula: Public Key = Private Key × G".white());
    println!("  {}", "  - G is the generator point on the curve".dimmed());
    println!("  {}", "• One-way function: Can't reverse to get private key".white());
    
    // Generate keys
    let secp = Secp256k1::new();
    let (_secret_key, public_key) = secp.generate_keypair(&mut rand::rngs::OsRng);
    let pubkey = PublicKey::new(public_key);
    
    println!("\n  {} Public Key Formats:", "🔓".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    
    println!("  {} {}", "Compressed (33 bytes):".bold(), pubkey.to_string().bright_green());
    println!("  {} Starts with 02 or 03", "Note:".dimmed());
    println!("  {} Indicates y-coordinate parity", "     ".dimmed());
    
    println!("\n  {} Why Compressed?", "💡".bright_blue());
    println!("  {}", "• Reduces size from 65 to 33 bytes".white());
    println!("  {}", "• Saves space in transactions".white());
    println!("  {}", "• Can reconstruct full point from compressed form".white());
    
    Ok(pubkey)
}

fn step3_hashing(pubkey: &PublicKey) -> Result<()> {
    println!("\n{}", "  ━".repeat(35).bright_magenta());
    println!("{}", "  STEP 3: Hashing the Public Key".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).bright_magenta());
    
    println!("\n  {} Why Hash the Public Key?", "📖".bright_yellow());
    println!("  {}", "• Adds extra layer of security".white());
    println!("  {}", "• Makes addresses shorter".white());
    println!("  {}", "• Protects against future quantum computers".white());
    
    println!("\n  {} Hashing Process:", "🔄".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    
    // Step 1: SHA256
    use bitcoin::hashes::{Hash, sha256, hash160};
    let sha256_hash = sha256::Hash::hash(pubkey.to_bytes().as_slice());
    println!("\n  {} SHA-256 Hash", "1️⃣".bright_cyan());
    println!("  {} {}", "Input:".bold(), pubkey.to_string().dimmed());
    println!("  {} {}", "Output:".bold(), sha256_hash.to_string().bright_white());
    println!("  {} 32 bytes", "Length:".dimmed());
    
    // Step 2: RIPEMD160
    let pubkey_hash = hash160::Hash::hash(pubkey.to_bytes().as_slice());
    println!("\n  {} RIPEMD-160 Hash", "2️⃣".bright_cyan());
    println!("  {} {}", "Input:".bold(), sha256_hash.to_string().dimmed());
    println!("  {} {}", "Output:".bold(), pubkey_hash.to_string().bright_white());
    println!("  {} 20 bytes (final pubkey hash)", "Length:".dimmed());
    
    println!("\n  {} This 20-byte hash is the core of the address!", "💡".bright_blue());
    
    Ok(())
}

fn step4_address_generation(pubkey: &PublicKey) -> Result<()> {
    println!("\n{}", "  ━".repeat(35).bright_magenta());
    println!("{}", "  STEP 4: Address Generation".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).bright_magenta());
    
    println!("\n  {} Three Main Address Types:", "📖".bright_yellow());
    
    // Legacy P2PKH
    println!("\n  {} Legacy (P2PKH)", "1️⃣".bright_cyan());
    let p2pkh = Address::p2pkh(pubkey, Network::Bitcoin);
    println!("  {} {}", "Address:".bold(), p2pkh.to_string().bright_green());
    println!("  {} Starts with '1'", "Format:".dimmed());
    println!("  {} Base58Check encoding", "Encoding:".dimmed());
    println!("  {} 1 (version) + 20 (hash) + 4 (checksum)", "Structure:".dimmed());
    
    // Native SegWit
    println!("\n  {} Native SegWit (P2WPKH)", "2️⃣".bright_cyan());
    let p2wpkh = Address::p2wpkh(pubkey, Network::Bitcoin)
        .expect("Failed to create P2WPKH address");
    println!("  {} {}", "Address:".bold(), p2wpkh.to_string().bright_green());
    println!("  {} Starts with 'bc1q'", "Format:".dimmed());
    println!("  {} Bech32 encoding", "Encoding:".dimmed());
    println!("  {} Lower case, error detection", "Features:".dimmed());
    
    // Taproot
    println!("\n  {} Taproot (P2TR)", "3️⃣".bright_cyan());
    println!("  {} bc1p... (example format)", "Address:".bright_green());
    println!("  {} Starts with 'bc1p'", "Format:".dimmed());
    println!("  {} Bech32m encoding", "Encoding:".dimmed());
    println!("  {} Most advanced privacy", "Features:".dimmed());
    
    Ok(())
}

fn step5_encoding() -> Result<()> {
    println!("\n{}", "  ━".repeat(35).bright_magenta());
    println!("{}", "  STEP 5: Encoding Formats Explained".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).bright_magenta());
    
    println!("\n  {} Base58Check Encoding (Legacy)", "🔤".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    println!("  {} Uses characters: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz", "Alphabet:".bold());
    println!("  {} Excludes: 0, O, I, l (avoid confusion)", "Note:".dimmed());
    println!("  {} Includes 4-byte checksum for error detection", "Safety:".white());
    println!("  {} Used for Legacy (1...) and P2SH (3...) addresses", "Usage:".white());
    
    println!("\n  {} Bech32 Encoding (SegWit)", "🔤".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    println!("  {} All lowercase", "Format:".bold());
    println!("  {} Better error detection than Base58", "Advantage:".white());
    println!("  {} Can detect and locate typos", "Feature:".white());
    println!("  {} More efficient for QR codes", "Benefit:".white());
    println!("  {} Used for SegWit addresses (bc1q...)", "Usage:".white());
    
    println!("\n  {} Bech32m Encoding (Taproot)", "🔤".bright_yellow());
    println!("  {}", "─".repeat(35).dimmed());
    println!("  {} Improved version of Bech32", "Evolution:".bold());
    println!("  {} Fixes edge case in original Bech32", "Fix:".white());
    println!("  {} Used for Taproot addresses (bc1p...)", "Usage:".white());
    
    Ok(())
}

fn display_summary() {
    println!("\n{}", "═".repeat(70).bright_magenta());
    println!("{}", "  📊 SUMMARY: Bitcoin Address Generation".bright_cyan().bold());
    println!("{}", "═".repeat(70).bright_magenta());
    
    println!("\n  {} The Complete Process:", "🔄".bright_yellow());
    println!();
    println!("  {}", "  1. Generate Random Private Key (256 bits)".white());
    println!("  {}", "           ↓".bright_blue());
    println!("  {}", "  2. Derive Public Key (ECDSA secp256k1)".white());
    println!("  {}", "           ↓".bright_blue());
    println!("  {}", "  3. Hash Public Key (SHA-256 + RIPEMD-160)".white());
    println!("  {}", "           ↓".bright_blue());
    println!("  {}", "  4. Add Version Byte + Checksum".white());
    println!("  {}", "           ↓".bright_blue());
    println!("  {}", "  5. Encode (Base58Check or Bech32)".white());
    println!("  {}", "           ↓".bright_blue());
    println!("  {}", "  6. Final Bitcoin Address! 🎉".bright_green().bold());
    
    println!("\n  {} Key Takeaways:", "💡".bright_yellow());
    println!("  {} Private key must be kept SECRET", "•".red());
    println!("  {} Public key can be shared safely", "•".green());
    println!("  {} Address is a hash of the public key", "•".blue());
    println!("  {} Process is one-way (cannot reverse)", "•".yellow());
    println!("  {} Multiple address formats exist (Legacy, SegWit, Taproot)", "•".cyan());
    println!("  {} Use Native SegWit (bc1q) for best fees", "•".bright_green());
    
    println!("\n{}", "═".repeat(70).bright_magenta());
    println!("  {}", "Tutorial Complete! You now understand Bitcoin addresses! 🎓".bright_green().bold());
    println!("{}", "═".repeat(70).bright_magenta());
    println!();
}

fn wait_for_enter(prompt: &str) {
    print!("{}", prompt.bright_blue());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tutorial_steps() {
        // Test individual steps don't panic
        let result = step1_private_key();
        assert!(result.is_ok());
    }
}

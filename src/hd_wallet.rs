use anyhow::{Result, anyhow};
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::{Secp256k1, All};
use bitcoin::{Address, Network};
use bip39::{Mnemonic, Language};
use colored::*;

/// Generate a new HD wallet with BIP39 mnemonic
pub fn generate_wallet(word_count: usize, address_count: usize, show_private: bool) -> Result<()> {
    println!("\n{}", "━".repeat(70).bright_blue());
    println!("{}", "  🎲 GENERATING NEW HD WALLET".bright_cyan().bold());
    println!("{}", "━".repeat(70).bright_blue());
    
    // Validate word count and generate entropy
    let entropy_length = match word_count {
        12 => 16, // 128 bits
        24 => 32, // 256 bits
        _ => return Err(anyhow!("Word count must be 12 or 24")),
    };
    
    // Generate random entropy
    let mut entropy = vec![0u8; entropy_length];
    use rand::RngCore;
    rand::rngs::OsRng.fill_bytes(&mut entropy);
    
    // Generate new mnemonic
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy)
        .map_err(|e| anyhow!("Failed to generate mnemonic: {:?}", e))?;
    let mnemonic_str: String = mnemonic.words().collect::<Vec<&str>>().join(" ");
    
    // Display security warning
    display_security_warning();
    
    // Display mnemonic
    println!("\n{}", "  🔑 BIP39 Mnemonic Seed Phrase:".bright_yellow().bold());
    println!("{}", "  ━".repeat(35).yellow());
    
    let words: Vec<&str> = mnemonic_str.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        if (i + 1) % 4 == 0 {
            println!("  {:2}. {}", i + 1, word.bright_white().bold());
        } else {
            print!("  {:2}. {}    ", i + 1, word.bright_white().bold());
        }
    }
    println!("\n{}", "  ━".repeat(35).yellow());
    
    // Generate seed from mnemonic
    let seed = mnemonic.to_seed("");
    println!("\n  {} {}", "Master Seed (hex):".bold(), hex::encode(&seed[..32]).dimmed());
    
    // Derive addresses
    let secp = Secp256k1::new();
    derive_and_display_addresses(&seed, &secp, address_count, show_private, "m/44'/0'/0'/0")?;
    
    Ok(())
}

/// Import and recover wallet from existing mnemonic
pub fn import_wallet(
    mnemonic_str: Option<String>,
    derivation_path: &str,
    address_count: usize,
    show_private: bool,
) -> Result<()> {
    println!("\n{}", "━".repeat(70).bright_blue());
    println!("{}", "  📥 IMPORTING WALLET FROM MNEMONIC".bright_cyan().bold());
    println!("{}", "━".repeat(70).bright_blue());
    
    // Get mnemonic from user if not provided
    let mnemonic_str = match mnemonic_str {
        Some(m) => m,
        None => {
            println!("\n  {} Enter your BIP39 mnemonic phrase:", "📝".bold());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };
    
    // Validate and parse mnemonic
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, &mnemonic_str)
        .map_err(|e| anyhow!("Invalid mnemonic: {:?}", e))?;
    
    println!("\n  {} {}", "✓".bright_green(), "Mnemonic validated successfully!".green());
    println!("  {} {} words", "Word count:".bold(), mnemonic_str.split_whitespace().count());
    
    // Generate seed
    let seed = mnemonic.to_seed("");
    
    // Explain derivation path
    explain_derivation_path(derivation_path);
    
    // Derive addresses
    let secp = Secp256k1::new();
    derive_and_display_addresses(&seed, &secp, address_count, show_private, derivation_path)?;
    
    Ok(())
}

fn derive_and_display_addresses(
    seed: &[u8],
    secp: &Secp256k1<All>,
    count: usize,
    show_private: bool,
    base_path: &str,
) -> Result<()> {
    println!("\n{}", "  📊 DERIVED ADDRESSES:".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).cyan());
    
    // Parse base derivation path
    let base_derivation: DerivationPath = base_path.parse()
        .map_err(|e| anyhow!("Invalid derivation path: {:?}", e))?;
    
    // Create master key
    let master_key = Xpriv::new_master(Network::Bitcoin, seed)?;
    
    // Derive addresses
    for i in 0..count {
        // Create full path: base_path/i
        let mut full_path = base_derivation.clone();
        full_path = full_path.child(bitcoin::bip32::ChildNumber::from_normal_idx(i as u32)?);
        
        // Derive private key
        let derived_private_key = master_key.derive_priv(secp, &full_path)?;
        
        // Get public key
        let derived_public_key = Xpub::from_priv(secp, &derived_private_key);
        let secp_pubkey = derived_public_key.public_key;
        
        // Convert secp256k1::PublicKey to bitcoin::PublicKey
        let public_key = bitcoin::PublicKey::new(secp_pubkey);
        
        // Generate address (P2WPKH - Native SegWit)
        let address = Address::p2wpkh(&public_key, Network::Bitcoin)
            .expect("Failed to create address");
        
        // Display address info
        println!("\n  {} #{}", "Address".bright_yellow().bold(), i);
        println!("  {} {}", "Path:".bold(), full_path.to_string().dimmed());
        println!("  {} {}", "Address:".bold(), address.to_string().bright_green());
        println!("  {} {}", "Public Key:".bold(), public_key.to_string().bright_white());
        
        if show_private {
            println!("  {} {}", "Private Key:".bold().red(), 
                     derived_private_key.to_string().bright_red());
            println!("  {} {}", "⚠️".yellow(), "KEEP THIS SECRET - NEVER SHARE!".red().bold());
        }
    }
    
    if !show_private {
        println!("\n  {} Use --show-private-keys flag to display private keys (⚠️ DANGEROUS)", 
                 "ℹ".bright_blue());
    }
    
    println!("\n{}", "  ━".repeat(35).cyan());
    
    Ok(())
}

fn display_security_warning() {
    println!("\n{}", "  ⚠️  SECURITY WARNING".bright_red().bold());
    println!("{}", "  ━".repeat(35).red());
    println!("  {} Write down this mnemonic on paper", "•".red());
    println!("  {} NEVER store it digitally or take screenshots", "•".red());
    println!("  {} Anyone with this phrase can access your funds", "•".red());
    println!("  {} This is for EDUCATIONAL purposes only", "•".red());
    println!("  {} Use hardware wallets for real Bitcoin", "•".red());
    println!("{}", "  ━".repeat(35).red());
}

fn explain_derivation_path(path: &str) {
    println!("\n{}", "  🗺️  Derivation Path Explanation:".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).cyan());
    println!("  {} {}", "Path:".bold(), path.bright_white());
    println!();
    
    // Parse and explain each part
    let parts: Vec<&str> = path.split('/').collect();
    
    for (i, part) in parts.iter().enumerate() {
        let explanation = match (i, part.trim_end_matches('\'')) {
            (0, "m") => "m = Master key (root of the tree)",
            (1, "44") => "44' = Purpose (BIP44 - Multi-Account Hierarchy)",
            (2, "0") => "0' = Coin type (0 = Bitcoin, 1 = Testnet)",
            (2, "1") => "1' = Coin type (1 = Bitcoin Testnet)",
            (3, _) => "Account number (0' = first account)",
            (4, "0") => "0 = External chain (receiving addresses)",
            (4, "1") => "1 = Internal chain (change addresses)",
            (5, _) => "Address index (increments for each address)",
            _ => "Unknown component",
        };
        
        println!("  {} {}", format!("{:12}", part).bright_yellow(), explanation.white());
    }
    
    println!("{}", "  ━".repeat(35).cyan());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_wallet() {
        let result = generate_wallet(12, 3, false);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_valid_mnemonic_import() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let result = import_wallet(Some(mnemonic.to_string()), "m/44'/0'/0'/0", 2, false);
        assert!(result.is_ok());
    }
}

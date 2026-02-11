use anyhow::{Result, anyhow};
use bitcoin::{Address, Network, PublicKey};
use bitcoin::secp256k1::Secp256k1;
use colored::*;
use std::str::FromStr;

/// Convert between different Bitcoin address formats
pub fn convert_formats(input: &str, testnet: bool) -> Result<()> {
    let network = if testnet { Network::Testnet } else { Network::Bitcoin };

    println!("\n{}", "━".repeat(70).bright_blue());
    println!("{}", "  🔄 ADDRESS FORMAT CONVERTER".bright_cyan().bold());
    println!("  {} {}", "Network:".bold(), format!("{:?}", network).bright_yellow());
    println!("{}", "━".repeat(70).bright_blue());
    
    // Try to parse as public key first (hex format)
    if let Ok(pubkey) = parse_public_key(input) {
        display_all_formats_from_pubkey(&pubkey, network)?;
    } 
    // Try to parse as address
    else if let Ok(address) = Address::from_str(input) {
        let address_checked = address.assume_checked();
        // Extract public key from address if possible
        println!("\n  {} Input address detected: {}", "ℹ".bright_blue(), address_checked.to_string().bright_white());
        println!("  {} Address type: {}", "ℹ".bright_blue(), 
                 format!("{:?}", address_checked.address_type()).bright_yellow());
        
        // Note: We can't extract the public key from an address alone
        // We can only show what formats would exist IF we had the pubkey
        println!("\n  {} Cannot generate other formats from address alone", "⚠".yellow());
        println!("  {} Addresses are one-way hashes of public keys", "ℹ".bright_blue());
        println!("  {} To convert formats, please provide the public key (hex)", "💡".bright_yellow());
        
        display_address_info(&address_checked);
    }
    else {
        return Err(anyhow!("Invalid input. Please provide:\n  • Public key in hex format (66 chars, compressed)\n  • Bitcoin address (any format)"));
    }
    
    Ok(())
}

fn parse_public_key(hex_str: &str) -> Result<PublicKey> {
    // Remove any whitespace or 0x prefix
    let cleaned = hex_str.trim().trim_start_matches("0x");
    
    // Decode hex
    let bytes = hex::decode(cleaned)
        .map_err(|_| anyhow!("Invalid hex string"))?;
    
    // Parse as PublicKey
    PublicKey::from_slice(&bytes)
        .map_err(|e| anyhow!("Invalid public key: {}", e))
}

fn display_all_formats_from_pubkey(pubkey: &PublicKey, network: Network) -> Result<()> {
    let _secp = Secp256k1::new();
    
    println!("\n  {} {}", "Public Key (compressed):".bold(), pubkey.to_string().bright_white());
    println!();
    
    println!("{}", "  📋 ALL ADDRESS FORMATS:".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).cyan());
    
    // 1. Legacy P2PKH
    let p2pkh_address = Address::p2pkh(pubkey, network);
    let p2pkh_features = if network == Network::Testnet {
        "• Starts with 'm' or 'n'\n  • For Testnet use only\n  • Base58Check encoding"
    } else {
        "• Starts with '1'\n  • Most compatible (all wallets support)\n  • Highest transaction fees\n  • Base58Check encoding"
    };
    display_format_box(
        "1️⃣  Legacy (P2PKH)",
        &p2pkh_address.to_string(),
        p2pkh_features,
        "🟡",
    );
    
    // 2. P2SH-SegWit (wrapped)
    let wpubkey_hash = pubkey.wpubkey_hash()
        .ok_or_else(|| anyhow!("Failed to generate wpubkey hash"))?;
    let p2wpkh_script = bitcoin::script::Builder::new()
        .push_slice(wpubkey_hash)
        .into_script();
    let p2sh_address = Address::p2sh(&p2wpkh_script, network)?;
    let p2sh_features = if network == Network::Testnet {
        "• Starts with '2'\n  • For Testnet use only\n  • Lower fees than legacy"
    } else {
        "• Starts with '3'\n  • Backward compatible with old wallets\n  • Lower fees than legacy (~25% savings)\n  • SegWit benefits in P2SH wrapper"
    };
    display_format_box(
        "2️⃣  P2SH-SegWit (Wrapped)",
        &p2sh_address.to_string(),
        p2sh_features,
        "🟢",
    );
    
    // 3. Native SegWit (Bech32)
    let p2wpkh_address = Address::p2wpkh(pubkey, network)?;
    let p2wpkh_features = if network == Network::Testnet {
        "• Starts with 'tb1q'\n  • RECOMMENDED for Testnet\n  • Lowest fees\n  • Bech32 encoding"
    } else {
        "• Starts with 'bc1q'\n  • RECOMMENDED for new wallets\n  • ~40% lower fees than legacy\n  • Bech32 encoding (lowercase)\n  • Most efficient format"
    };
    display_format_box(
        "3️⃣  Native SegWit (P2WPKH)",
        &p2wpkh_address.to_string(),
        p2wpkh_features,
        "🟢",
    );
    
    // 4. Taproot (if we can create it)
    let taproot_prefix = if network == Network::Testnet { "tb1p" } else { "bc1p" };
    println!("\n  {} Taproot (P2TR) addresses require key tweaking", "ℹ".bright_blue());
    println!("    {} Cannot be directly derived from unmodified pubkey", "•".blue());
    println!("    {} Would start with '{}'", "•".blue(), taproot_prefix);
    println!("    {} Most advanced privacy and efficiency", "•".blue());
    
    // Fee comparison
    if network == Network::Bitcoin {
        display_fee_comparison();
    }
    
    Ok(())
}

fn display_format_box(title: &str, address: &str, features: &str, icon: &str) {
    println!("\n  {}", title.bright_yellow().bold());
    println!("  {}", "─".repeat(35).dimmed());
    println!("  {} {}", "Address:".bold(), address.bright_green());
    println!("  {} Features:", "Info:".bold());
    for line in features.lines() {
        println!("  {}", line.white());
    }
    println!("  {}", icon);
}

fn display_fee_comparison() {
    println!("\n{}", "  💰 TRANSACTION FEE COMPARISON (MAINNET):".bright_cyan().bold());
    println!("{}", "  ━".repeat(40).cyan());
    println!("  {} (Baseline: 100%)", "Legacy P2PKH:      ".bright_white());
    println!("  {} (~75%)", "P2SH-SegWit:       ".bright_white());
    println!("  {} (~60%) ✨ BEST", "Native SegWit:     ".bright_green().bold());
    println!("  {} (~60%) 🔒 Most Private", "Taproot:           ".bright_green().bold());
    println!();
    println!("  {} Fee savings are approximate", "Note:".bold());
    println!("  {} Actual savings depend on transaction complexity", "•".dimmed());
    println!("  {} Native SegWit (bc1q) is recommended for most users", "•".dimmed());
    println!("{}", "  ━".repeat(40).cyan());
}

fn display_address_info(address: &Address) {
    println!("\n{}", "  📊 ADDRESS INFORMATION:".bright_cyan().bold());
    println!("{}", "  ━".repeat(35).cyan());
    
    println!("  {} {}", "Network:".bold(), 
             format!("{:?}", address.network()).bright_yellow());
    
    let script = address.script_pubkey();
    println!("  {} {} bytes", "Script Size:".bold(), script.len());
    
    // Show what we know about this address type
    match address.address_type() {
        Some(bitcoin::AddressType::P2pkh) => {
            println!("\n  {} This is a Legacy address", "Type:".bold());
            println!("  {} Could be converted to SegWit for lower fees", "Tip:".bright_blue());
        }
        Some(bitcoin::AddressType::P2sh) => {
            println!("\n  {} This is a P2SH address", "Type:".bold());
            println!("  {} May contain SegWit or multisig script", "Note:".bright_blue());
        }
        Some(bitcoin::AddressType::P2wpkh) => {
            println!("\n  {} This is a Native SegWit address", "Type:".bold());
            println!("  {} Optimal for fees! 👍", "Status:".bright_green());
        }
        Some(bitcoin::AddressType::P2tr) => {
            println!("\n  {} This is a Taproot address", "Type:".bold());
            println!("  {} Most advanced format! 🚀", "Status:".bright_green());
        }
        _ => {}
    }
    
    println!("{}", "  ━".repeat(35).cyan());
}

/// Generate example public key for testing
pub fn generate_example_pubkey() -> Result<PublicKey> {
    use bitcoin::secp256k1::Secp256k1;
    use rand::rngs::OsRng;
    
    let secp = Secp256k1::new();
    let (_secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    
    Ok(PublicKey::new(public_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example_conversion_mainnet() {
        // Test with a known public key
        let pubkey_hex = "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5";
        let result = convert_formats(pubkey_hex, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_example_conversion_testnet() {
        let pubkey = generate_example_pubkey().unwrap();
        let result_testnet = convert_formats(&pubkey.to_string(), true);
        assert!(result_testnet.is_ok());
    }
    
    #[test]
    fn test_address_input() {
        let address = "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq";
        let result = convert_formats(address, false);
        assert!(result.is_ok());

        let testnet_address = "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7";
        let result_testnet = convert_formats(testnet_address, true);
        assert!(result_testnet.is_ok());
    }
}

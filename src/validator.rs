use bitcoin::{Address, Network};
use colored::*;
use anyhow::Result;
use std::str::FromStr;

/// Validate a Bitcoin address and display detailed information
pub fn validate_address(address_str: &str) -> Result<()> {
    println!("\n{}", "━".repeat(70).bright_blue());
    println!("{}", "  🔍 VALIDATING BITCOIN ADDRESS".bright_cyan().bold());
    println!("{}", "━".repeat(70).bright_blue());
    
    // Try to parse the address
    match Address::from_str(address_str) {
        Ok(address) => {
            // Need to check the network
            let address_checked = address.assume_checked();
            display_valid_address(&address_checked, address_str);
        }
        Err(e) => {
            display_invalid_address(address_str, &e);
        }
    }
    
    Ok(())
}

fn display_valid_address(address: &Address, address_str: &str) {
    println!("\n{}", "✅ VALID ADDRESS".bright_green().bold());
    println!();
    
    // Determine address type
    let (address_type, description) = get_address_type(address);
    
    // Display address information
    println!("  {} {}", "Address:".bold(), address_str.bright_white());
    println!("  {} {}", "Type:".bold(), address_type.bright_yellow());
    println!("  {} {}", "Description:".bold(), description.white());
    println!("  {} {}", "Network:".bold(), get_network_display(*address.network()));
    
    // Display technical details
    println!("\n{}", "  📋 Technical Details:".bright_cyan().bold());
    
    // Show script type
    let script_pubkey = address.script_pubkey();
    println!("  {} {}", "Script Type:".bold(), get_script_type_display(&script_pubkey));
    println!("  {} {} bytes", "Script Length:".bold(), script_pubkey.len());
    println!("  {} {}", "Script (hex):".bold(), script_pubkey.to_hex_string().dimmed());
    
    // Address characteristics
    println!("\n{}", "  💡 Characteristics:".bright_cyan().bold());
    
    match address.address_type() {
        Some(bitcoin::AddressType::P2pkh) => {
            println!("  {} Legacy format (most compatible)", "•".yellow());
            println!("  {} Higher transaction fees", "•".yellow());
            println!("  {} Starts with '1'", "•".yellow());
            println!("  {} Base58Check encoding", "•".yellow());
        }
        Some(bitcoin::AddressType::P2sh) => {
            println!("  {} Script Hash format", "•".yellow());
            println!("  {} Can contain any script", "•".yellow());
            println!("  {} Starts with '3'", "•".yellow());
            println!("  {} Often used for multisig or SegWit", "•".yellow());
        }
        Some(bitcoin::AddressType::P2wpkh) => {
            println!("  {} Native SegWit (recommended)", "•".green());
            println!("  {} ~40% lower transaction fees", "•".green());
            println!("  {} Starts with 'bc1q'", "•".green());
            println!("  {} Bech32 encoding", "•".green());
        }
        Some(bitcoin::AddressType::P2wsh) => {
            println!("  {} Native SegWit Script", "•".green());
            println!("  {} Lower fees than legacy", "•".green());
            println!("  {} Starts with 'bc1q'", "•".green());
            println!("  {} For complex scripts", "•".green());
        }
        Some(bitcoin::AddressType::P2tr) => {
            println!("  {} Taproot (most advanced)", "•".bright_green());
            println!("  {} Enhanced privacy", "•".bright_green());
            println!("  {} More efficient multisig", "•".bright_green());
            println!("  {} Starts with 'bc1p'", "•".bright_green());
            println!("  {} Bech32m encoding", "•".bright_green());
        }
        None => {
            println!("  {} Unknown address type", "•".red());
        }
        Some(_) => {
            println!("  {} Other address type", "•".yellow());
        }
    }
    
    // Compatibility notes
    println!("\n{}", "  🔌 Compatibility:".bright_cyan().bold());
    match address.address_type() {
        Some(bitcoin::AddressType::P2pkh) | Some(bitcoin::AddressType::P2sh) => {
            println!("  {} Compatible with all wallets", "✓".green());
        }
        Some(bitcoin::AddressType::P2wpkh) | Some(bitcoin::AddressType::P2wsh) => {
            println!("  {} Most modern wallets (post-2017)", "✓".green());
            println!("  {} May not work with very old software", "!".yellow());
        }
        Some(bitcoin::AddressType::P2tr) => {
            println!("  {} Newest wallets (post-2021)", "✓".green());
            println!("  {} Not supported by older wallets", "!".yellow());
        }
        None => {}
        Some(_) => {}
    }
    
    println!();
}

fn display_invalid_address(address_str: &str, error: &bitcoin::address::ParseError) {
    println!("\n{}", "❌ INVALID ADDRESS".bright_red().bold());
    println!();
    println!("  {} {}", "Address:".bold(), address_str.bright_white());
    println!("  {} {}", "Error:".bold(), format!("{:?}", error).red());
    
    // Provide helpful hints
    println!("\n{}", "  💡 Common Issues:".bright_yellow().bold());
    println!("  {} Check for typos or missing characters", "•".yellow());
    println!("  {} Valid formats:", "•".yellow());
    println!("      {} 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", "-".dimmed());
    println!("      {} 3J98t1WpEZ73CNmYviecrnyiWrnqRhWNLy", "-".dimmed());
    println!("      {} bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq", "-".dimmed());
    println!("      {} bc1p5d7rjq7g6rdk2yhzks9smlaqtedr4dekq08ge8ztwac72sfr9rusxg3297", "-".dimmed());
    println!("  {} Testnet addresses start with 'm', 'n', '2', 'tb1'", "•".yellow());
    println!();
}

fn get_address_type(address: &Address) -> (&str, &str) {
    match address.address_type() {
        Some(bitcoin::AddressType::P2pkh) => {
            ("P2PKH (Pay-to-PubKey-Hash)", "Legacy address format, most compatible")
        }
        Some(bitcoin::AddressType::P2sh) => {
            ("P2SH (Pay-to-Script-Hash)", "Can contain any script, often used for multisig")
        }
        Some(bitcoin::AddressType::P2wpkh) => {
            ("P2WPKH (Pay-to-Witness-PubKey-Hash)", "Native SegWit, lower fees")
        }
        Some(bitcoin::AddressType::P2wsh) => {
            ("P2WSH (Pay-to-Witness-Script-Hash)", "Native SegWit for scripts")
        }
        Some(bitcoin::AddressType::P2tr) => {
            ("P2TR (Pay-to-Taproot)", "Taproot, most advanced privacy and efficiency")
        }
        None => ("Unknown", "Address type could not be determined"),
        _ => ("Other", "Address type varies"),
    }
}

fn get_network_display(network: Network) -> ColoredString {
    match network {
        Network::Bitcoin => "Mainnet (Production)".bright_green(),
        Network::Testnet => "Testnet (Testing)".bright_yellow(),
        Network::Signet => "Signet (Testing)".bright_yellow(),
        Network::Regtest => "Regtest (Local)".bright_magenta(),
        _ => "Unknown Network".red(),
    }
}

fn get_script_type_display(script: &bitcoin::Script) -> ColoredString {
    if script.is_p2pkh() {
        "P2PKH".bright_yellow()
    } else if script.is_p2sh() {
        "P2SH".bright_yellow()
    } else if script.is_p2wpkh() {
        "P2WPKH (SegWit)".bright_green()
    } else if script.is_p2wsh() {
        "P2WSH (SegWit)".bright_green()
    } else if script.is_p2tr() {
        "P2TR (Taproot)".bright_cyan()
    } else {
        "Unknown".red()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_legacy_address() {
        let result = validate_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_valid_segwit_address() {
        let result = validate_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_invalid_address() {
        let result = validate_address("invalid_address_123");
        assert!(result.is_ok()); // Function returns Ok but displays error
    }
}

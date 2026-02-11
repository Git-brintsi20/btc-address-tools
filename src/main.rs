use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;

mod validator;
mod hd_wallet;
mod converter;
mod educational;
mod utils;

#[derive(Parser)]
#[command(
    name = "btc-tools",
    version = "1.0.0",
    about = "🔑 Bitcoin Address Toolkit - Validator, HD Wallet Generator & Educational Tool",
    long_about = "A comprehensive Bitcoin address toolkit for validation, HD wallet generation, and learning.\n⚠️  FOR EDUCATIONAL PURPOSES ONLY - Never use generated keys for real funds!"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a Bitcoin address (supports Legacy, P2SH, SegWit, Taproot)
    Validate {
        /// Bitcoin address to validate
        address: String,
    },
    
    /// Generate a new HD wallet with BIP39 mnemonic
    Generate {
        /// Number of words in mnemonic (12 or 24)
        #[arg(short, long, default_value = "12")]
        words: usize,
        
        /// Number of addresses to derive
        #[arg(short, long, default_value = "5")]
        count: usize,
        
        /// Show private keys (⚠️ USE WITH CAUTION)
        #[arg(long)]
        show_private_keys: bool,
    },
    
    /// Import and recover wallet from existing mnemonic
    Import {
        /// BIP39 mnemonic phrase (12 or 24 words)
        #[arg(short, long)]
        mnemonic: Option<String>,
        
        /// Custom derivation path (default: m/44'/0'/0'/0)
        #[arg(short, long, default_value = "m/44'/0'/0'/0")]
        path: String,
        
        /// Number of addresses to derive
        #[arg(short, long, default_value = "5")]
        count: usize,
        
        /// Show private keys (⚠️ USE WITH CAUTION)
        #[arg(long)]
        show_private_keys: bool,
    },
    
    /// Convert between different Bitcoin address formats
    Convert {
        /// Public key (hex) or any Bitcoin address
        input: String,
        /// Generate testnet addresses instead of mainnet
        #[arg(long)]
        testnet: bool,
    },
    
    /// Generate example data for testing (e.g., public key)
    Example,

    /// Interactive educational mode - Learn how Bitcoin addresses work
    Learn,
    
    /// Interactive mode - User-friendly menu interface
    Interactive,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Display security warning banner
    display_banner();
    
    match cli.command {
        Commands::Validate { address } => {
            validator::validate_address(&address)?;
        }
        Commands::Generate { words, count, show_private_keys } => {
            hd_wallet::generate_wallet(words, count, show_private_keys)?;
        }
        Commands::Import { mnemonic, path, count, show_private_keys } => {
            hd_wallet::import_wallet(mnemonic, &path, count, show_private_keys)?;
        }
        Commands::Convert { input, testnet } => {
            converter::convert_formats(&input, testnet)?;
        }
        Commands::Example => {
            let pubkey = converter::generate_example_pubkey()?;
            println!("\n{}", "🔑 Example Data Generated".bright_cyan().bold());
            println!("{}", "━".repeat(30).cyan());
            println!("{} {}", "Sample Public Key (hex):".bold(), pubkey.to_string().bright_white());
            println!("{}", "━".repeat(30).cyan());
            println!("You can use this public key with the 'convert' command.");
            println!("Example: {} convert {}", "btc-tools".green(), pubkey.to_string().bright_white());
        }
        Commands::Learn => {
            educational::run_tutorial()?;
        }
        Commands::Interactive => {
            run_interactive_mode()?;
        }
    }
    
    Ok(())
}

fn display_banner() {
    println!("\n{}", "═".repeat(70).bright_cyan());
    println!("{}", "    🔑  BITCOIN ADDRESS TOOLKIT".bright_yellow().bold());
    println!("{}", "═".repeat(70).bright_cyan());
    println!("{}", "    ⚠️  WARNING: FOR EDUCATIONAL PURPOSES ONLY".bright_red().bold());
    println!("{}", "    Never use generated keys for real funds!".yellow());
    println!("{}", "    Use hardware wallets for actual Bitcoin storage.".yellow());
    println!("{}", "═".repeat(70).bright_cyan());
    println!();
}

fn run_interactive_mode() -> Result<()> {
    use dialoguer::{Select, theme::ColorfulTheme};
    
    loop {
        println!();
        let options = vec![
            "🔍 Validate Bitcoin Address",
            "🎲 Generate New HD Wallet",
            "📥 Import Existing Mnemonic",
            "🔄 Convert Address Formats",
            "📚 Educational Mode (Learn)",
            "✨ Generate Example Data",
            "❌ Exit"
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .items(&options)
            .default(0)
            .interact()?;
        
        match selection {
            0 => {
                let address: String = dialoguer::Input::new()
                    .with_prompt("Enter Bitcoin address to validate")
                    .interact_text()?;
                validator::validate_address(&address)?;
            }
            1 => {
                let words_options = vec!["12 words", "24 words"];
                let words_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select mnemonic length")
                    .items(&words_options)
                    .default(0)
                    .interact()?;
                let words = if words_selection == 0 { 12 } else { 24 };
                
                let count: usize = dialoguer::Input::new()
                    .with_prompt("Number of addresses to generate")
                    .default(5)
                    .interact_text()?;
                
                let show_private = dialoguer::Confirm::new()
                    .with_prompt("⚠️  Show private keys? (DANGEROUS - educational only)")
                    .default(false)
                    .interact()?;
                
                hd_wallet::generate_wallet(words, count, show_private)?;
            }
            2 => {
                let mnemonic: String = dialoguer::Input::new()
                    .with_prompt("Enter your BIP39 mnemonic (12 or 24 words)")
                    .interact_text()?;
                
                let path: String = dialoguer::Input::new()
                    .with_prompt("Derivation path")
                    .default("m/44'/0'/0'/0".to_string())
                    .interact_text()?;
                
                let count: usize = dialoguer::Input::new()
                    .with_prompt("Number of addresses to derive")
                    .default(5)
                    .interact_text()?;
                
                let show_private = dialoguer::Confirm::new()
                    .with_prompt("⚠️  Show private keys?")
                    .default(false)
                    .interact()?;
                
                hd_wallet::import_wallet(Some(mnemonic), &path, count, show_private)?;
            }
            3 => {
                let input: String = dialoguer::Input::new()
                    .with_prompt("Enter public key (hex) or Bitcoin address")
                    .interact_text()?;
                
                let testnet = dialoguer::Confirm::new()
                    .with_prompt("Generate for Testnet?")
                    .default(false)
                    .interact()?;

                converter::convert_formats(&input, testnet)?;
            }
            4 => {
                educational::run_tutorial()?;
            }
            5 => {
                let pubkey = converter::generate_example_pubkey()?;
                println!("\n{}", "🔑 Example Data Generated".bright_cyan().bold());
                println!("{}", "━".repeat(30).cyan());
                println!("{} {}", "Sample Public Key (hex):".bold(), pubkey.to_string().bright_white());
                println!("{}", "━".repeat(30).cyan());
                println!("You can use this public key with the 'convert' command.");
            }
            6 => {
                println!("\n{}\n", "👋 Goodbye! Stay safe with your Bitcoin!".bright_green());
                break;
            }
            _ => unreachable!(),
        }
    }
    
    Ok(())
}

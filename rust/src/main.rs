// PyRouterSploit - Main Entry Point
// Cross-platform security exploitation framework

use anyhow::Result;
use clap::{Parser, Subcommand};
use pyroutersploit::{init, shutdown};

#[derive(Parser)]
#[command(name = "pyroutersploit")]
#[command(about = "Cross-platform security exploitation framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the REST API server
    Serve {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },

    /// Start the MCP server
    Mcp {
        #[arg(long, default_value = "stdio")]
        transport: String,
    },

    /// Initialize the database and populate defaults
    Init {
        #[arg(long)]
        populate_cryptex: bool,
    },

    /// Query the cryptex dictionary
    Cryptex {
        #[arg(short, long)]
        search: Option<String>,
        #[arg(long)]
        list_all: bool,
    },

    /// Hash data with multiple algorithms
    Hash {
        #[arg(short, long)]
        data: String,
        #[arg(short, long)]
        algorithm: Option<String>,
        #[arg(long)]
        all: bool,
    },

    /// QKD encryption operations
    Qkd {
        #[command(subcommand)]
        operation: QkdOperations,
    },
}

#[derive(Subcommand)]
enum QkdOperations {
    /// Encrypt data
    Encrypt {
        #[arg(short, long)]
        data: String,
    },
    /// Decrypt data
    Decrypt {
        #[arg(short, long)]
        ciphertext: String,
    },
    /// Generate a new key
    GenerateKey {
        #[arg(short, long, default_value_t = 32)]
        size: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    // Initialize PyRouterSploit
    init().await?;

    match cli.command {
        Commands::Serve { host, port } => {
            serve_api(&host, port).await?;
        }
        Commands::Mcp { transport } => {
            start_mcp_server(&transport).await?;
        }
        Commands::Init { populate_cryptex } => {
            println!("✓ Database initialized");
            if populate_cryptex {
                pyroutersploit::db::CryptexDictionary::populate_defaults()?;
                println!("✓ Cryptex dictionary populated with default entries");
            }
        }
        Commands::Cryptex { search, list_all } => {
            handle_cryptex_command(search, list_all)?;
        }
        Commands::Hash { data, algorithm, all } => {
            handle_hash_command(&data, algorithm, all)?;
        }
        Commands::Qkd { operation } => {
            handle_qkd_command(operation).await?;
        }
    }

    shutdown().await?;
    Ok(())
}

async fn serve_api(host: &str, port: u16) -> Result<()> {
    use axum::Router;
    use tokio::net::TcpListener;

    println!("🚀 Starting PyRouterSploit API server at http://{}:{}", host, port);

    let app = pyroutersploit::api::rest::create_router().await;
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn start_mcp_server(transport: &str) -> Result<()> {
    println!("🔌 Starting MCP server (transport: {})", transport);
    pyroutersploit::mcp_server::start(transport).await?;
    Ok(())
}

fn handle_cryptex_command(search: Option<String>, list_all: bool) -> Result<()> {
    use pyroutersploit::db::CryptexDictionary;

    if list_all {
        let entries = CryptexDictionary::list_all()?;
        println!("\n📚 Cryptex Dictionary ({} entries):\n", entries.len());
        for entry in entries {
            println!("  • {} → {}", entry.function_name, entry.branding_name);
            println!("    Category: {:?}", entry.category);
            println!("    {}\n", entry.pseudo_code);
        }
    } else if let Some(query) = search {
        let results = CryptexDictionary::search(&query)?;
        println!("\n🔍 Search results for '{}': {} found\n", query, results.len());
        for entry in results {
            println!("  • {} → {}", entry.function_name, entry.branding_name);
            println!("    {}\n", entry.pseudo_code);
        }
    } else {
        println!("Use --search <query> or --list-all");
    }

    Ok(())
}

fn handle_hash_command(data: &str, algorithm: Option<String>, all: bool) -> Result<()> {
    use pyroutersploit::crypto::{HashAlgorithm, MultiHasher};

    if all {
        let results = MultiHasher::hash_all(data.as_bytes())?;
        println!("\n🔐 Hashing '{}' with all algorithms:\n", data);
        for result in results {
            println!("  {:<15} : {}", result.algorithm.to_string(), result.hex);
        }
    } else if let Some(algo_str) = algorithm {
        let algo = match algo_str.to_uppercase().as_str() {
            "SHA256" => HashAlgorithm::SHA256,
            "SHA512" => HashAlgorithm::SHA512,
            "BLAKE3" => HashAlgorithm::BLAKE3,
            "SHA3_256" => HashAlgorithm::SHA3_256,
            _ => {
                println!("Unknown algorithm. Using SHA256");
                HashAlgorithm::SHA256
            }
        };
        let result = MultiHasher::hash(algo, data.as_bytes())?;
        println!("\n{} hash: {}", algo, result.hex);
    } else {
        let result = MultiHasher::hash(HashAlgorithm::SHA256, data.as_bytes())?;
        println!("\nSHA256 hash: {}", result.hex);
    }

    Ok(())
}

async fn handle_qkd_command(operation: QkdOperations) -> Result<()> {
    use pyroutersploit::crypto::{QKDEncryption, QKDKeyGenerator};

    match operation {
        QkdOperations::Encrypt { data } => {
            let qkd = QKDEncryption::new_session(32)?;
            let ciphertext = qkd.encrypt(data.as_bytes())?;
            let ciphertext_hex = hex::encode(&ciphertext);
            println!("\n🔐 Encrypted (hex): {}", ciphertext_hex);
            println!("Session ID: {}", qkd.session_id);
            qkd.save_session()?;
            println!("✓ Session saved to database");
        }
        QkdOperations::Decrypt { ciphertext } => {
            let ciphertext_bytes = hex::decode(&ciphertext)?;
            // For demo, create a new session - in real use, would load from DB
            println!("⚠ Note: This is a demo. In production, load session from DB");
        }
        QkdOperations::GenerateKey { size } => {
            let key = QKDKeyGenerator::generate_hybrid_key(size)?;
            let key_hex = hex::encode(&key);
            println!("\n🔑 Generated QKD key ({} bytes):", size);
            println!("{}", key_hex);
        }
    }

    Ok(())
}

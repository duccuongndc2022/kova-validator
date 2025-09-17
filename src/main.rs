//! Kova Validator - Data validation service for the Kova ecosystem

use clap::Parser;
use kova_validator::{ValidatorService, ValidationConfig};
use std::net::SocketAddr;
use tracing::info;

#[derive(Parser)]
#[command(name = "kova-validator")]
#[command(about = "Data validation service for the Kova ecosystem")]
struct Args {
    /// API server port
    #[arg(long, default_value = "8080")]
    api_port: u16,
    
    /// WebSocket server port
    #[arg(long, default_value = "8081")]
    ws_port: u16,
    
    /// Bind address
    #[arg(long, default_value = "0.0.0.0")]
    bind: String,
    
    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
    
    /// Configuration file path
    #[arg(long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(&args.log_level)
        .init();
    
    info!("Starting Kova Validator");
    info!("API port: {}", args.api_port);
    info!("WebSocket port: {}", args.ws_port);
    
    // Load configuration
    let config = if let Some(config_path) = args.config {
        ValidationConfig::from_file(&config_path)?
    } else {
        ValidationConfig::default()
    };
    
    // Create validation service
    let validator = ValidatorService::new(config).await?;
    
    // Start API server
    let api_addr = format!("{}:{}", args.bind, args.api_port).parse::<SocketAddr>()?;
    let ws_addr = format!("{}:{}", args.bind, args.ws_port).parse::<SocketAddr>()?;
    
    info!("Starting API server on {}", api_addr);
    info!("Starting WebSocket server on {}", ws_addr);
    
    // Start servers
    tokio::try_join!(
        validator.start_api_server(api_addr),
        validator.start_websocket_server(ws_addr)
    )?;
    
    Ok(())
}

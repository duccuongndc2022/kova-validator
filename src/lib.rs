//! # Kova Validator
//!
//! Data validation service for the Kova ecosystem.
//!
//! This library provides comprehensive data validation capabilities for sensor data
//! within the Kova decentralized robotics network, including quality assessment,
//! anomaly detection, and temporal consistency validation.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod api;
pub mod anomaly;
pub mod config;
pub mod metrics;
pub mod sensors;
pub mod validation;

/// Re-export commonly used types
pub use config::ValidationConfig;
pub use validation::{ValidatorService, ValidationResult, QualityMetrics};
pub use sensors::{SensorData, SensorType};

/// Initialize the Kova Validator system
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::info!("Initializing Kova Validator");
    Ok(())
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

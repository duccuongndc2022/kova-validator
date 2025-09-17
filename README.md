# Kova Validator

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Documentation](https://docs.rs/kova-validator/badge.svg)](https://docs.rs/kova-validator)
[![Build Status](https://github.com/kovasystems/kova-validator/workflows/CI/badge.svg)](https://github.com/kovasystems/kova-validator/actions)

**Data validation service for the Kova ecosystem**

Kova Validator is a high-performance data validation service designed to assess the quality and integrity of sensor data within the Kova decentralized robotics network. It provides comprehensive validation algorithms, anomaly detection, and quality scoring for various sensor types.

## Features

- **Multi-Sensor Validation**: Camera, LiDAR, IMU, GPS, and thermal data validation
- **Quality Assessment**: Comprehensive quality scoring and metrics
- **Anomaly Detection**: Advanced anomaly detection algorithms
- **Temporal Consistency**: Time-series data consistency validation
- **Real-time Processing**: High-performance async validation pipeline
- **Configurable Rules**: Customizable validation rules and thresholds
- **REST API**: HTTP API for validation requests
- **WebSocket Support**: Real-time validation streaming
- **Metrics Export**: Prometheus metrics for monitoring

## Quick Start

### Installation

Add Kova Validator to your `Cargo.toml`:

```toml
[dependencies]
kova-validator = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use kova_validator::{
    ValidatorService, ValidationConfig, SensorData, ValidationResult,
    QualityMetrics, AnomalyDetector,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create validation service
    let config = ValidationConfig::default();
    let mut validator = ValidatorService::new(config).await?;
    
    // Create sensor data
    let sensor_data = SensorData {
        sensor_id: "camera-001".to_string(),
        sensor_type: SensorType::Camera,
        timestamp: chrono::Utc::now(),
        data: image_data,
        metadata: HashMap::new(),
    };
    
    // Validate data
    let result = validator.validate(&sensor_data).await?;
    
    println!("Quality Score: {:.2}", result.quality_score);
    println!("Is Valid: {}", result.is_valid);
    
    Ok(())
}
```

## Architecture

### Validation Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Raw Data  │───▶│ Preprocess  │───▶│  Validate   │───▶│   Score     │
│             │    │             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
         │                   │                   │                   │
         ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Metadata   │    │  Noise      │    │  Anomaly    │    │  Quality    │
│  Extraction │    │  Reduction  │    │  Detection  │    │  Metrics    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### Quality Metrics

- **Completeness**: Data completeness and missing value analysis
- **Consistency**: Temporal and spatial consistency checks
- **Accuracy**: Data accuracy and precision assessment
- **Noise Level**: Signal-to-noise ratio analysis
- **Anomaly Score**: Outlier and anomaly detection score
- **Temporal Coherence**: Time-series data coherence validation

## API Reference

### Validation Service

```rust
use kova_validator::{ValidatorService, ValidationConfig};

// Create validation service with custom configuration
let config = ValidationConfig {
    min_quality_score: 0.8,
    enable_anomaly_detection: true,
    enable_temporal_consistency: true,
    max_noise_threshold: 0.05,
    validation_timeout: Duration::from_secs(30),
};

let mut validator = ValidatorService::new(config).await?;

// Validate sensor data
let result = validator.validate(&sensor_data).await?;
```

### Quality Metrics

```rust
use kova_validator::QualityMetrics;

let metrics = QualityMetrics {
    completeness: 0.95,
    consistency: 0.88,
    accuracy: 0.92,
    noise_level: 0.03,
    anomaly_score: 0.01,
    temporal_coherence: 0.90,
};

let overall_score = metrics.calculate_overall_score();
```

### Anomaly Detection

```rust
use kova_validator::{AnomalyDetector, AnomalyConfig};

let config = AnomalyConfig {
    algorithm: AnomalyAlgorithm::IsolationForest,
    contamination: 0.1,
    random_state: 42,
};

let detector = AnomalyDetector::new(config);
let anomalies = detector.detect(&sensor_data).await?;
```

### REST API

```bash
# Validate sensor data
curl -X POST http://localhost:8080/api/v1/validate \
  -H "Content-Type: application/json" \
  -d '{
    "sensor_id": "camera-001",
    "sensor_type": "camera",
    "data": "base64_encoded_data",
    "metadata": {}
  }'

# Get validation metrics
curl http://localhost:8080/api/v1/metrics

# Get validation history
curl http://localhost:8080/api/v1/history?limit=100
```

### WebSocket API

```javascript
const ws = new WebSocket('ws://localhost:8080/ws/validate');

ws.onopen = () => {
    // Send validation request
    ws.send(JSON.stringify({
        type: 'validate',
        data: sensorData
    }));
};

ws.onmessage = (event) => {
    const result = JSON.parse(event.data);
    console.log('Validation result:', result);
};
```

## Configuration

### Validation Configuration

```rust
use kova_validator::ValidationConfig;

let config = ValidationConfig {
    // Quality thresholds
    min_quality_score: 0.7,
    max_noise_threshold: 0.1,
    
    // Feature flags
    enable_anomaly_detection: true,
    enable_temporal_consistency: true,
    enable_spatial_consistency: true,
    
    // Timeouts
    validation_timeout: Duration::from_secs(30),
    preprocessing_timeout: Duration::from_secs(10),
    
    // Algorithm settings
    anomaly_algorithm: AnomalyAlgorithm::IsolationForest,
    quality_weights: QualityWeights::default(),
    
    // Performance settings
    max_concurrent_validations: 100,
    cache_size: 1000,
    enable_caching: true,
};
```

### Sensor-Specific Configuration

```rust
use kova_validator::sensors::{CameraConfig, LiDARConfig, IMUConfig};

// Camera validation configuration
let camera_config = CameraConfig {
    min_resolution: (640, 480),
    max_resolution: (4096, 4096),
    supported_formats: vec![ImageFormat::RGB, ImageFormat::RGBA],
    max_noise_threshold: 0.05,
    enable_blur_detection: true,
    enable_exposure_validation: true,
};

// LiDAR validation configuration
let lidar_config = LiDARConfig {
    min_points: 1000,
    max_points: 1000000,
    range_min: 0.1,
    range_max: 100.0,
    angular_resolution: 0.1,
    max_noise_threshold: 0.02,
    enable_outlier_detection: true,
};

// IMU validation configuration
let imu_config = IMUConfig {
    sample_rate_min: 10.0,
    sample_rate_max: 1000.0,
    acceleration_range: (-16.0, 16.0),
    gyroscope_range: (-2000.0, 2000.0),
    max_drift_threshold: 0.01,
    enable_calibration_check: true,
};
```

## Examples

### Basic Validation

```bash
cargo run --example basic_validation
```

### Anomaly Detection

```bash
cargo run --example anomaly_detection
```

### REST API Server

```bash
cargo run --bin kova-validator -- --api-port 8080
```

### WebSocket Server

```bash
cargo run --bin kova-validator -- --ws-port 8081
```

### Batch Processing

```bash
cargo run --example batch_validation -- --input data/ --output results/
```

## Testing

Run all tests:

```bash
cargo test
```

Run specific test categories:

```bash
cargo test --features camera
cargo test --features lidar
cargo test --features imu
cargo test --features anomaly
```

Run integration tests:

```bash
cargo test --test integration
```

Run performance tests:

```bash
cargo test --test performance
```

## Performance

Kova Validator is optimized for high-performance validation:

- **Throughput**: 10,000+ validations per second
- **Latency**: Sub-millisecond validation for simple cases
- **Memory**: Efficient memory usage with streaming processing
- **Scalability**: Horizontal scaling with load balancing

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench validation_throughput
cargo bench --bench anomaly_detection
```

## Monitoring

### Metrics

Kova Validator exposes Prometheus metrics:

- `kova_validator_validations_total`: Total number of validations
- `kova_validator_validation_duration_seconds`: Validation duration histogram
- `kova_validator_quality_score_histogram`: Quality score distribution
- `kova_validator_anomalies_detected_total`: Total anomalies detected
- `kova_validator_errors_total`: Total validation errors

### Health Checks

```bash
# Health check endpoint
curl http://localhost:8080/health

# Readiness check
curl http://localhost:8080/ready

# Liveness check
curl http://localhost:8080/live
```

## Deployment

### Docker

```dockerfile
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/kova-validator /usr/local/bin/
EXPOSE 8080 8081
CMD ["kova-validator"]
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kova-validator
spec:
  replicas: 3
  selector:
    matchLabels:
      app: kova-validator
  template:
    metadata:
      labels:
        app: kova-validator
    spec:
      containers:
      - name: kova-validator
        image: kovasystems/kova-validator:latest
        ports:
        - containerPort: 8080
        - containerPort: 8081
        env:
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "1Gi"
            cpu: "500m"
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Clone your fork
3. Create a feature branch
4. Make your changes
5. Add tests
6. Run the test suite
7. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Links

- [Website](https://www.kova.systems/)
- [Documentation](https://docs.rs/kova-validator)
- [Discord](https://discord.gg/kova)
- [Twitter](https://twitter.com/KovaSystems)

## Acknowledgments

- The Rust community for excellent tooling and ecosystem
- The machine learning community for anomaly detection algorithms
- The Kova Systems team for the validation requirements

---

**Made with ❤️ by the Kova Systems team**

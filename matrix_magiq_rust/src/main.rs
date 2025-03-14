use anyhow::Result;
use core::{
    ai::{GeminiClient, GeminiConfig, optimization::CircuitOptimizer, optimization::OptimizationConfig},
    quantum::Circuit,
};
use std::env;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Matrix-Magiq Quantum Runtime...");

    // Initialize Gemini client
    let gemini_config = GeminiConfig {
        api_key: env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY environment variable must be set"),
        model: "gemini-pro".to_string(),
        temperature: 0.7,
        max_output_tokens: 2048,
    };

    let gemini_client = GeminiClient::new(gemini_config);

    // Initialize circuit optimizer
    let optimization_config = OptimizationConfig {
        optimization_level: core::ai::optimization::OptimizationLevel::Aggressive,
        max_depth: 100,
        error_threshold: 0.01,
    };

    let optimizer = CircuitOptimizer::new(gemini_client.clone(), optimization_config);

    // Example: Create and optimize a quantum circuit
    let circuit = Circuit::new(); // You'll need to implement this
    info!("Created initial quantum circuit");

    match optimizer.optimize_circuit(&circuit).await {
        Ok(optimized_circuit) => {
            info!(
                "Successfully optimized circuit. New depth: {}",
                optimized_circuit.depth()
            );
        }
        Err(e) => {
            warn!("Failed to optimize circuit: {}", e);
        }
    }

    // Example: Analyze error rates
    match optimizer.analyze_error_rates(&circuit).await {
        Ok(analysis) => {
            info!(
                "Error analysis complete. Confidence score: {}",
                analysis.confidence_score
            );
            for recommendation in analysis.recommendations {
                info!("Recommendation: {}", recommendation);
            }
        }
        Err(e) => {
            warn!("Failed to analyze error rates: {}", e);
        }
    }

    info!("Matrix-Magiq Quantum Runtime initialized successfully");
    Ok(())
}

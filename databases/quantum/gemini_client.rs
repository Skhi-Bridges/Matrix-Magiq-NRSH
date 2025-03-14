use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiOptimization {
    pub optimization_result: String,
    pub error_analysis: String,
}

pub struct GeminiClient {
    python_path: PathBuf,
    api_key: String,
}

impl GeminiClient {
    pub fn new(python_path: PathBuf, api_key: String) -> Self {
        Self {
            python_path,
            api_key,
        }
    }

    pub async fn optimize_circuit(&self, circuit_json: &str) -> Result<GeminiOptimization> {
        // Call Python script with circuit JSON
        let output = Command::new(&self.python_path)
            .arg("quantum_runtime.py")
            .env("GEMINI_API_KEY", &self.api_key)
            .env("CIRCUIT_JSON", circuit_json)
            .output()
            .await?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            warn!("Gemini optimization failed: {}", error);
            anyhow::bail!("Gemini optimization failed: {}", error);
        }

        let result = String::from_utf8_lossy(&output.stdout);
        let optimization: GeminiOptimization = serde_json::from_str(&result)?;
        
        info!("Successfully received Gemini optimization");
        Ok(optimization)
    }
}

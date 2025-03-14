//! Nourish Chain Telemetry Implementation

#![cfg_attr(not(feature = "std"), no_std)]

/// Telemetry for Nourish Chain
pub struct NourishTelemetry;

impl NourishTelemetry {
    pub fn record_metrics() {
        #[cfg(feature = "std")]
        log::info!("Recording Nourish Chain metrics");
    }
}

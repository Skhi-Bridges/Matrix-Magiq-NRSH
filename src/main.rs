// Matrix-Magiq NRSH Entrypoint 
// Main entry point for the Nourish Chain parachain

#![warn(missing_docs)]

//! Nourish Chain node implementation.
//! 
//! Parachain focused on spirulina cultivation tracking with registry, oracle, and telemetry pallets.
//! Provides transparent and verifiable spirulina cultivation tracking from cultivation to distribution.

use nourish_chain_runtime::{
    opaque::Block, AccountId, Balance, BlockNumber, Hash, Header, Index, 
    RuntimeApi, Signature, WASM_BINARY,
};
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::{Configuration, DatabaseConfig, Role, RpcHandlers, TaskManager};
use std::sync::Arc;

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
    let version = sc_cli::VersionInfo {
        name: "Nourish Chain Node",
        version: env!("CARGO_PKG_VERSION"),
        commit: env!("SUBSTRATE_CLI_IMPL_VERSION"),
        executable_name: "nourish-chain",
        author: "Matrix-Magiq",
        description: "Nourish Chain (NRSH) - Spirulina Tracking Parachain",
        support_url: "https://github.com/Skhi-Bridges/Matrix-Magiq-NRSH",
        copyright_start_year: 2024,
    };

    command::run(version)
}

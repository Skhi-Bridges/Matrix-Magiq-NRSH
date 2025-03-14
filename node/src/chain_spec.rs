//! Nourish Chain parachain specification

use cumulus_primitives_core::ParaId;
use nourish_runtime::{AccountId, AuraId, Balance, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized ChainSpec for the nourish parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [ChainSpec].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecExtension, ChainSpecGroup)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given ChainSpec.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Development parachain config (to connect to a locally running relay chain)
pub fn development_config() -> ChainSpec {
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "NRSH".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Nourish Development",
        "nourish_dev",
        ChainType::Development,
        || {
            // Genesis config
            // This is a placeholder for the actual genesis configuration
            ()
        },
        Vec::new(),
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You can use "rococo-local" for local testing
            para_id: 2000,
        },
    )
}

/// Start a node with the given parachain id and relay chain chain_spec.
///
/// This is the actual implementation that is abstract over the executor.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RB, BIQ, BIC>(
    parachain_id: ParaId,
    relay_chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
    para_chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
    base_path: Option<&std::path::Path>,
    relay_chain_args: impl Iterator<Item = String>,
    parachain_args: impl Iterator<Item = String>,
    collator_options: CollatorOptions,
    hwbench: Option<sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient<RB, BIQ, BIC>>)>
where
    // Placeholder for the actual implementation
    RB: sc_client_api::Backend<Block> + 'static,
    BIQ: sc_client_api::BackendInfoProvider<Block> + 'static,
    BIC: sc_client_api::BlockImportProvider<Block> + 'static,
{
    // In a real implementation, this would contain the logic to start the parachain node
    // For our testing purpose, this is left as a placeholder
    Ok((TaskManager::new(Arc::new(Registry::new().unwrap()), None).unwrap(), Arc::new(ParachainClient::new())))
}

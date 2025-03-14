// Matrix-Magiq NRSH Library
// Top-level library file for the Nourish Chain parachain

#![cfg_attr(not(feature = "std"), no_std)]

/// Re-export pallet components
pub use pallet;

// Export key types for NRSH chain
pub mod types {
    use sp_runtime::traits::{IdentifyAccount, Verify};
    use sp_runtime::MultiSignature;

    /// An index to a block.
    pub type BlockNumber = u32;

    /// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
    pub type Signature = MultiSignature;

    /// Some way of identifying an account on the chain.
    pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

    /// The type for looking up accounts.
    pub type AccountIndex = u32;

    /// Balance of an account.
    pub type Balance = u128;

    /// Type used for expressing timestamp.
    pub type Moment = u64;

    /// Index of a transaction in the chain.
    pub type Index = u32;

    /// A hash of some data used by the chain.
    pub type Hash = sp_core::H256;
}

// Export key functionality for spirulina supply chain tracking
pub mod supply_chain;

// Export registry functionality
pub mod registry;

// Export oracle functionality for nutritional content verification
pub mod oracle;

// Export telemetry functionality for real-time cultivation metrics
pub mod telemetry;

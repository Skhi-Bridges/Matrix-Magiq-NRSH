//! Runtime API definition for nourish pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;

/// The Nourish runtime API used to access spirulina batch verification data.
sp_api::decl_runtime_apis! {
    pub trait NourishRuntimeApi<AccountId, Balance> where
        AccountId: Codec,
        Balance: Codec,
    {
        /// Get details of a specific spirulina batch by ID
        fn get_batch_details(batch_id: Vec<u8>) -> Option<BatchDetails<AccountId, Balance>>;
        
        /// Check the verification status of a spirulina batch
        fn batch_verification_status(batch_id: Vec<u8>) -> Option<VerificationStatus>;
        
        /// Get all batch IDs produced by a specific account
        fn get_producer_batches(producer: AccountId) -> Vec<Vec<u8>>;
    }
}

/// Details of a spirulina production batch
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Debug)]
pub struct BatchDetails<AccountId, Balance> {
    /// Identifier of the batch
    pub id: Vec<u8>,
    /// Account ID of the producer
    pub producer: AccountId,
    /// Quantity produced in grams
    pub quantity: u64,
    /// Production timestamp
    pub timestamp: u64,
    /// Current verification status
    pub status: VerificationStatus,
    /// Nutritional metrics of the batch
    pub metrics: NutritionalMetrics,
    /// Production facility ID
    pub facility_id: Vec<u8>,
    /// Current reward value
    pub reward_value: Balance,
    /// Verification result data from oracles
    pub verification_data: Vec<VerificationData>,
}

/// Status of a spirulina batch verification
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Debug)]
pub enum VerificationStatus {
    /// Batch is pending verification
    Pending,
    /// Batch is currently being verified
    InProgress,
    /// Batch has been verified as valid
    Verified,
    /// Batch verification failed
    Failed,
    /// Batch verification has been disputed
    Disputed,
}

/// Nutritional metrics for a spirulina batch
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Debug)]
pub struct NutritionalMetrics {
    /// Protein content percentage
    pub protein_content: u8,
    /// Chlorophyll content in mg/g
    pub chlorophyll_content: u8,
    /// Phycocyanin content in mg/g
    pub phycocyanin_content: u8,
    /// Beta-carotene content in mg/g
    pub beta_carotene_content: u8,
    /// Iron content in mg/g
    pub iron_content: u8,
    /// Overall quality score (0-100)
    pub quality_score: u8,
}

/// Verification data from an oracle
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Debug)]
pub struct VerificationData {
    /// Oracle account ID
    pub oracle: Vec<u8>,
    /// Verification timestamp
    pub timestamp: u64,
    /// Verification result (0-100)
    pub score: u8,
    /// Optional comments
    pub comments: Vec<u8>,
    /// Signature using CRYSTALS-Dilithium
    pub signature: Vec<u8>,
}

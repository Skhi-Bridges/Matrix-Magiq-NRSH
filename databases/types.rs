//! Type definitions for the Nourish pallet.

use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use sp_std::prelude::*;

/// Status of a spirulina batch verification
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
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
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
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
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct VerificationData<AccountId> {
    /// Oracle account ID
    pub oracle: AccountId,
    /// Verification timestamp
    pub timestamp: u64,
    /// Verification result (0-100)
    pub score: u8,
    /// Optional comments
    pub comments: Vec<u8>,
    /// Signature using CRYSTALS-Dilithium
    pub signature: Vec<u8>,
}

/// Details of a spirulina production batch
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
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
    pub verification_data: Vec<VerificationData<AccountId>>,
}

/// Registration information for a production facility
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct FacilityInfo<AccountId> {
    /// Facility ID
    pub id: Vec<u8>,
    /// Facility name
    pub name: Vec<u8>,
    /// Geographic location (latitude, longitude)
    pub location: (i32, i32),
    /// Owner account
    pub owner: AccountId,
    /// Maximum production capacity in grams per day
    pub capacity: u64,
    /// Registration timestamp
    pub registered_at: u64,
    /// ISO certification IDs
    pub certifications: Vec<Vec<u8>>,
    /// Whether the facility is currently active
    pub active: bool,
}

/// Registration information for a verifier
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct VerifierInfo<AccountId, Balance> {
    /// Verifier account
    pub account: AccountId,
    /// Verifier name
    pub name: Vec<u8>,
    /// Verification records count
    pub verification_count: u64,
    /// Success rate (0-100)
    pub success_rate: u8,
    /// Staked balance
    pub stake: Balance,
    /// Whether the verifier is currently active
    pub active: bool,
}

/// Parameters for a cultivation protocol
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct CultivationParameters {
    /// Protocol ID
    pub id: Vec<u8>,
    /// Protocol name
    pub name: Vec<u8>,
    /// pH range (min, max)
    pub ph_range: (u8, u8),
    /// Temperature range in Celsius (min, max)
    pub temperature_range: (u8, u8),
    /// Light intensity range (min, max)
    pub light_range: (u16, u16),
    /// Nutrient mix reference
    pub nutrient_mix: Vec<u8>,
    /// Growth period in days
    pub growth_period: u16,
    /// Expected protein content range
    pub protein_range: (u8, u8),
}

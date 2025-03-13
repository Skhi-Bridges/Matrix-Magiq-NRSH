//! # Spirulina Registry Pallet
//!
//! A pallet for tracking and verifying spirulina cultivation on the Nourish Chain.
//!
//! ## Overview
//!
//! The registry pallet provides functionality for:
//! - Registering spirulina cultivation operations
//! - Tracking growth conditions and metrics
//! - Verifying cultivation authenticity
//! - Certification of harvests
//!
//! All operations include comprehensive error correction at the classical,
//! bridge, and quantum levels per Matrix-Magiq architecture requirements.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
    traits::{Get, Randomness},
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::{Hash, Zero};
use sp_std::prelude::*;

// Import error correction modules
mod error_correction;
use error_correction::{apply_classical_correction, apply_bridge_correction, apply_quantum_correction};

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    
    /// The quantum-resistant random number generator.
    type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    
    /// Minimum cultivation period (in blocks)
    type MinCultivationPeriod: Get<Self::BlockNumber>;
}

// Cultivation batch information
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct CultivationBatch<AccountId, BlockNumber> {
    // Grower ID
    pub grower: AccountId,
    // Batch unique identifier
    pub batch_id: Vec<u8>,
    // Start block number
    pub start_block: BlockNumber,
    // Expected harvest block number
    pub expected_harvest: BlockNumber,
    // Cultivation conditions hash
    pub conditions_hash: H256,
    // Certification status
    pub certified: bool,
    // Quantum verification proof
    pub quantum_proof: Option<Vec<u8>>,
}

// Cultivation conditions
#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq)]
pub struct CultivationConditions {
    // Temperature (in Celsius * 10^2)
    pub temperature: u32,
    // pH level (in pH * 10^2)
    pub ph_level: u32,
    // Light intensity (in lux)
    pub light_intensity: u32,
    // Nutrient composition hash
    pub nutrient_hash: H256,
    // Water source identifier
    pub water_source: Vec<u8>,
}

decl_storage! {
    trait Store for Module<T: Config> as SpirulinaRegistry {
        // Storage for cultivation batches
        CultivationBatches get(fn cultivation_batch):
            map hasher(blake2_128_concat) Vec<u8> => CultivationBatch<T::AccountId, T::BlockNumber>;
        
        // Storage for cultivation conditions
        CultivationConditionsList get(fn cultivation_conditions):
            map hasher(blake2_128_concat) H256 => CultivationConditions;
        
        // Batches owned by a specific grower
        GrowerBatches get(fn grower_batches):
            map hasher(blake2_128_concat) T::AccountId => Vec<Vec<u8>>;
        
        // Total number of batches
        BatchCount get(fn batch_count): u64 = 0;
    }
}

decl_event! {
    pub enum Event<T> where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// A new cultivation batch was registered
        BatchRegistered(AccountId, Vec<u8>),
        /// Cultivation conditions were updated
        ConditionsUpdated(Vec<u8>, H256),
        /// Batch was certified
        BatchCertified(Vec<u8>),
        /// Error correction was applied
        ErrorCorrectionApplied(Vec<u8>, u8), // batch_id, correction_level
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Batch ID already exists
        BatchIdExists,
        /// Batch does not exist
        BatchNotFound,
        /// Invalid cultivation period
        InvalidCultivationPeriod,
        /// Not authorized to perform this action
        NotAuthorized,
        /// Batch not ready for certification
        NotReadyForCertification,
        /// Error correction failed
        ErrorCorrectionFailed,
        /// Quantum verification failed
        QuantumVerificationFailed,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        // Initialize errors
        type Error = Error<T>;
        
        // Initialize events
        fn deposit_event() = default;
        
        /// Register a new spirulina cultivation batch
        #[weight = 10_000]
        pub fn register_batch(
            origin,
            batch_id: Vec<u8>,
            expected_harvest: T::BlockNumber,
            conditions: CultivationConditions,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure batch ID doesn't already exist
            ensure!(!CultivationBatches::<T>::contains_key(&batch_id), Error::<T>::BatchIdExists);
            
            // Ensure the cultivation period is valid
            let current_block = <frame_system::Module<T>>::block_number();
            ensure!(
                expected_harvest > current_block && 
                expected_harvest >= current_block + T::MinCultivationPeriod::get(),
                Error::<T>::InvalidCultivationPeriod
            );
            
            // Apply error correction at all levels
            apply_classical_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_bridge_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_quantum_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            
            // Calculate conditions hash
            let conditions_hash = T::Hashing::hash_of(&conditions);
            
            // Store conditions
            CultivationConditionsList::<T>::insert(conditions_hash, conditions);
            
            // Create and store the new batch
            let batch = CultivationBatch {
                grower: sender.clone(),
                batch_id: batch_id.clone(),
                start_block: current_block,
                expected_harvest,
                conditions_hash,
                certified: false,
                quantum_proof: None,
            };
            
            CultivationBatches::<T>::insert(&batch_id, batch);
            
            // Update grower's batch list
            let mut grower_batches = GrowerBatches::<T>::get(&sender);
            grower_batches.push(batch_id.clone());
            GrowerBatches::<T>::insert(&sender, grower_batches);
            
            // Increment batch count
            let new_count = BatchCount::get().checked_add(1).unwrap_or_default();
            BatchCount::put(new_count);
            
            // Emit event
            Self::deposit_event(RawEvent::BatchRegistered(sender, batch_id));
            
            Ok(())
        }
        
        /// Update cultivation conditions for a batch
        #[weight = 10_000]
        pub fn update_conditions(
            origin,
            batch_id: Vec<u8>,
            conditions: CultivationConditions,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure batch exists
            ensure!(CultivationBatches::<T>::contains_key(&batch_id), Error::<T>::BatchNotFound);
            
            // Get the batch
            let mut batch = CultivationBatches::<T>::get(&batch_id);
            
            // Ensure sender is the grower
            ensure!(batch.grower == sender, Error::<T>::NotAuthorized);
            
            // Apply error correction at all levels
            apply_classical_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_bridge_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_quantum_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            
            // Calculate new conditions hash
            let conditions_hash = T::Hashing::hash_of(&conditions);
            
            // Store updated conditions
            CultivationConditionsList::<T>::insert(conditions_hash, conditions);
            
            // Update batch conditions hash
            batch.conditions_hash = conditions_hash;
            CultivationBatches::<T>::insert(&batch_id, batch);
            
            // Emit event
            Self::deposit_event(RawEvent::ConditionsUpdated(batch_id, conditions_hash));
            
            Ok(())
        }
        
        /// Certify a cultivation batch
        #[weight = 10_000]
        pub fn certify_batch(
            origin,
            batch_id: Vec<u8>,
            quantum_proof: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            
            // Ensure batch exists
            ensure!(CultivationBatches::<T>::contains_key(&batch_id), Error::<T>::BatchNotFound);
            
            // Get the batch
            let mut batch = CultivationBatches::<T>::get(&batch_id);
            
            // Ensure sender is the grower
            ensure!(batch.grower == sender, Error::<T>::NotAuthorized);
            
            // Ensure batch is ready for certification
            let current_block = <frame_system::Module<T>>::block_number();
            ensure!(current_block >= batch.expected_harvest, Error::<T>::NotReadyForCertification);
            
            // Apply error correction at all levels
            apply_classical_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_bridge_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            apply_quantum_correction(&batch_id).map_err(|_| Error::<T>::ErrorCorrectionFailed)?;
            
            // Verify quantum proof
            // This would involve complex quantum verification logic in a real implementation
            // For now, we simply check that the proof is not empty
            ensure!(!quantum_proof.is_empty(), Error::<T>::QuantumVerificationFailed);
            
            // Update batch certification status
            batch.certified = true;
            batch.quantum_proof = Some(quantum_proof);
            CultivationBatches::<T>::insert(&batch_id, batch);
            
            // Emit event
            Self::deposit_event(RawEvent::BatchCertified(batch_id));
            
            Ok(())
        }
    }
}

// Helper functions for the pallet
impl<T: Config> Module<T> {
    /// Generate a unique batch ID
    pub fn generate_batch_id(sender: &T::AccountId) -> Vec<u8> {
        let current_block = <frame_system::Module<T>>::block_number();
        let random_seed = T::Randomness::random(&sender.encode());
        
        // Combine account, block number, and random seed to create a unique ID
        let mut combined = sender.encode();
        combined.extend_from_slice(&current_block.encode());
        combined.extend_from_slice(&random_seed.encode());
        
        combined
    }
    
    /// Verify cultivation conditions are within acceptable ranges
    pub fn verify_conditions(conditions: &CultivationConditions) -> bool {
        // Temperature should be between 25.00 and 35.00 degrees Celsius
        if conditions.temperature < 2500 || conditions.temperature > 3500 {
            return false;
        }
        
        // pH should be between 8.00 and 10.50
        if conditions.ph_level < 800 || conditions.ph_level > 1050 {
            return false;
        }
        
        // Light intensity should be between 2000 and 5000 lux
        if conditions.light_intensity < 2000 || conditions.light_intensity > 5000 {
            return false;
        }
        
        true
    }
}

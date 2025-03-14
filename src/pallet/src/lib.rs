// NRSH Pallet Implementation
// Core functionality for the Nourish Chain parachain

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    /// The pallet configuration trait
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The maximum length of a spirulina batch identifier
        #[pallet::constant]
        type MaxBatchIdLength: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// Storage for spirulina batches
    #[pallet::storage]
    #[pallet::getter(fn spirulina_batches)]
    pub type SpirulinaBatches<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxBatchIdLength>,
        SpirulinaBatch<T>,
        OptionQuery,
    >;

    /// Storage for registered producers
    #[pallet::storage]
    #[pallet::getter(fn registered_producers)]
    pub type RegisteredProducers<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Producer<T>,
        OptionQuery,
    >;

    /// Storage for nutritional verification
    #[pallet::storage]
    #[pallet::getter(fn nutritional_verifications)]
    pub type NutritionalVerifications<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxBatchIdLength>,
        NutritionalContent,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new spirulina batch has been registered
        BatchRegistered(T::AccountId, BoundedVec<u8, T::MaxBatchIdLength>),
        /// A new producer has been registered
        ProducerRegistered(T::AccountId),
        /// Nutritional content has been verified
        NutritionalContentVerified(BoundedVec<u8, T::MaxBatchIdLength>),
        /// Batch has been processed
        BatchProcessed(BoundedVec<u8, T::MaxBatchIdLength>),
        /// Batch has been shipped
        BatchShipped(BoundedVec<u8, T::MaxBatchIdLength>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Batch ID is too long
        BatchIdTooLong,
        /// Batch already exists
        BatchAlreadyExists,
        /// Producer not registered
        ProducerNotRegistered,
        /// Producer already registered
        ProducerAlreadyRegistered,
        /// Batch not found
        BatchNotFound,
        /// Unauthorized action
        Unauthorized,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new spirulina batch
        #[pallet::weight(10_000)]
        pub fn register_batch(
            origin: OriginFor<T>,
            batch_id: Vec<u8>,
            cultivation_date: u64,
            location: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure producer is registered
            ensure!(RegisteredProducers::<T>::contains_key(&who), Error::<T>::ProducerNotRegistered);

            // Convert to bounded vec
            let bounded_batch_id: BoundedVec<u8, T::MaxBatchIdLength> = batch_id
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            // Ensure batch doesn't already exist
            ensure!(!SpirulinaBatches::<T>::contains_key(&bounded_batch_id), Error::<T>::BatchAlreadyExists);

            // Create new batch
            let bounded_location: BoundedVec<u8, T::MaxBatchIdLength> = location
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            let batch = SpirulinaBatch::<T> {
                producer: who.clone(),
                cultivation_date,
                location: bounded_location,
                status: BatchStatus::Registered,
                processing_steps: vec![].try_into().unwrap(),
            };

            // Store batch
            SpirulinaBatches::<T>::insert(&bounded_batch_id, batch);

            // Emit event
            Self::deposit_event(Event::BatchRegistered(who, bounded_batch_id));

            Ok(())
        }

        /// Register a new producer
        #[pallet::weight(10_000)]
        pub fn register_producer(
            origin: OriginFor<T>,
            name: Vec<u8>,
            location: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure producer is not already registered
            ensure!(!RegisteredProducers::<T>::contains_key(&who), Error::<T>::ProducerAlreadyRegistered);

            // Convert to bounded vecs
            let bounded_name: BoundedVec<u8, T::MaxBatchIdLength> = name
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            let bounded_location: BoundedVec<u8, T::MaxBatchIdLength> = location
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            // Create new producer
            let producer = Producer::<T> {
                account_id: who.clone(),
                name: bounded_name,
                location: bounded_location,
                verified: false,
            };

            // Store producer
            RegisteredProducers::<T>::insert(&who, producer);

            // Emit event
            Self::deposit_event(Event::ProducerRegistered(who));

            Ok(())
        }

        /// Process a batch
        #[pallet::weight(10_000)]
        pub fn process_batch(
            origin: OriginFor<T>,
            batch_id: Vec<u8>,
            process_description: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Convert to bounded vec
            let bounded_batch_id: BoundedVec<u8, T::MaxBatchIdLength> = batch_id
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            // Ensure batch exists
            let mut batch = SpirulinaBatches::<T>::get(&bounded_batch_id)
                .ok_or(Error::<T>::BatchNotFound)?;

            // Ensure processor is the batch producer
            ensure!(batch.producer == who, Error::<T>::Unauthorized);

            // Convert process description to bounded vec
            let bounded_process: BoundedVec<u8, T::MaxBatchIdLength> = process_description
                .try_into()
                .map_err(|_| Error::<T>::BatchIdTooLong)?;

            // Add processing step
            let mut steps = batch.processing_steps.to_vec();
            steps.push(bounded_process);
            batch.processing_steps = steps.try_into().unwrap_or_default();

            // Update batch status
            batch.status = BatchStatus::Processed;

            // Update batch
            SpirulinaBatches::<T>::insert(&bounded_batch_id, batch);

            // Emit event
            Self::deposit_event(Event::BatchProcessed(bounded_batch_id));

            Ok(())
        }
    }

    /// Spirulina batch information
    #[derive(Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct SpirulinaBatch<T: Config> {
        /// The producer of the batch
        pub producer: T::AccountId,
        /// The date of cultivation
        pub cultivation_date: u64,
        /// The location of cultivation
        pub location: BoundedVec<u8, T::MaxBatchIdLength>,
        /// The status of the batch
        pub status: BatchStatus,
        /// Processing steps
        pub processing_steps: BoundedVec<BoundedVec<u8, T::MaxBatchIdLength>, T::MaxBatchIdLength>,
    }

    /// Producer information
    #[derive(Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Producer<T: Config> {
        /// The account ID of the producer
        pub account_id: T::AccountId,
        /// The name of the producer
        pub name: BoundedVec<u8, T::MaxBatchIdLength>,
        /// The location of the producer
        pub location: BoundedVec<u8, T::MaxBatchIdLength>,
        /// Whether the producer is verified
        pub verified: bool,
    }

    /// Nutritional content information
    #[derive(Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct NutritionalContent {
        /// Protein content (%)
        pub protein: u8,
        /// Vitamin A content (%)
        pub vitamin_a: u8,
        /// Vitamin B12 content (%)
        pub vitamin_b12: u8,
        /// Iron content (%)
        pub iron: u8,
        /// Verification timestamp
        pub timestamp: u64,
    }

    /// Batch status
    #[derive(Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, MaxEncodedLen, PartialEq)]
    pub enum BatchStatus {
        #[default]
        /// Batch is registered
        Registered,
        /// Batch is processed
        Processed,
        /// Batch is shipped
        Shipped,
        /// Batch is delivered
        Delivered,
    }
}

/// Implementation of error correction for the NRSH pallet
pub mod error_correction {
    use super::*;
    use sp_std::prelude::*;

    /// Apply Reed-Solomon error correction to data
    pub fn apply_reed_solomon(data: &[u8]) -> Vec<u8> {
        // This would be a real implementation of Reed-Solomon
        // For now, we just return the data
        data.to_vec()
    }

    /// Verify Reed-Solomon error correction
    pub fn verify_reed_solomon(data: &[u8], parity: &[u8]) -> bool {
        // This would be a real implementation of Reed-Solomon verification
        // For now, we just return true
        true
    }

    /// Apply error correction to batch data
    pub fn correct_batch_data<T: pallet::Config>(
        batch_id: &BoundedVec<u8, T::MaxBatchIdLength>,
    ) -> Result<pallet::SpirulinaBatch<T>, &'static str> {
        // Get batch data
        let batch = pallet::SpirulinaBatches::<T>::get(batch_id)
            .ok_or("Batch not found")?;

        // In a real implementation, we would apply error correction
        // to the batch data here
        // For now, we just return the batch

        Ok(batch)
    }
}

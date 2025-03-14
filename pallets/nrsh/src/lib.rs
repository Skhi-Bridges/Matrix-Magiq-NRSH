//! Nourish Pallet Implementation

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new spirulina batch has been registered
        SpirulinaBatchRegistered { batch_id: u32 },
    }

    #[pallet::storage]
    pub type SpirulinaBatches<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<u8>, OptionQuery>;
}

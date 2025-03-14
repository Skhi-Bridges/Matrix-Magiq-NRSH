//! # ActorX Core
//! 
//! Core traits and implementations for ActorX framework
//! integrated with Matrix-Magiq quantum-resistant architecture

mod error;
mod actor;
mod message;
mod context;
mod crypto;
mod actor_token;

pub use error::{ClassicalError, BridgeError, QuantumError, ErrorCorrectionLayer};
pub use actor::Actor;
pub use message::Message;
pub use context::Context;
pub use crypto::{QuantumCrypto, Blake3Hasher};
pub use actor_token::{ActorToken, ActorTokenRegistry, MultiCurrencyWallet, PermawebProfile, DexIntegration};

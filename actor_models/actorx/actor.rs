//! Actor trait definition for ActorX framework

use crate::message::Message;
use crate::context::Context;
use crate::error::{ClassicalError, BridgeError, QuantumError};

/// Core Actor trait for ActorX framework
/// Implements Fill and Kill operations for quantum keys
pub trait Actor: Send + 'static {
    /// Process a message with Fill semantics (persistent state)
    fn receive<M: Message>(&self, msg: M);
    
    /// Process a message with Kill semantics (ephemeral state)
    fn receive_once<M: Message>(&self, msg: M);
    
    /// Get the actor's address for messaging
    fn address(&self) -> String;
    
    /// Apply quantum correction to actor's state
    fn quantum_correct(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Actor Reference for interacting with actors
pub struct ActorRef<A: Actor> {
    inner: A,
    error_correction: crate::error::ErrorCorrectionLayer,
}

impl<A: Actor> ActorRef<A> {
    pub fn new(actor: A) -> Self {
        Self {
            inner: actor,
            error_correction: crate::error::ErrorCorrectionLayer::new(),
        }
    }
    
    pub fn send<M: Message>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        // Apply three-level error correction
        let corrected_msg = self.error_correction.apply_classical_correction(msg)
            .map_err(|e| ClassicalError::CorrectionFailed(e.to_string()))?;
            
        let bridge_corrected_msg = self.error_correction.apply_bridge_correction(corrected_msg)
            .map_err(|e| BridgeError::CorrectionFailed(e.to_string()))?;
            
        let quantum_corrected_msg = self.error_correction.apply_quantum_correction(bridge_corrected_msg)
            .map_err(|e| QuantumError::CorrectionFailed(e.to_string()))?;
            
        self.inner.receive(quantum_corrected_msg);
        Ok(())
    }
    
    pub fn send_kill<M: Message>(&self, msg: M) -> Result<(), Box<dyn std::error::Error>> {
        // Apply three-level error correction with Kill semantics
        let corrected_msg = self.error_correction.apply_classical_correction(msg)
            .map_err(|e| ClassicalError::CorrectionFailed(e.to_string()))?;
            
        let bridge_corrected_msg = self.error_correction.apply_bridge_correction(corrected_msg)
            .map_err(|e| BridgeError::CorrectionFailed(e.to_string()))?;
            
        let quantum_corrected_msg = self.error_correction.apply_quantum_correction(bridge_corrected_msg)
            .map_err(|e| QuantumError::CorrectionFailed(e.to_string()))?;
            
        self.inner.receive_once(quantum_corrected_msg);
        Ok(())
    }
}

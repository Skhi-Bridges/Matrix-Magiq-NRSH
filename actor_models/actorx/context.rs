//! Context trait definition for ActorX framework

use crate::actor::Actor;
use crate::message::Message;
use std::any::Any;

/// Actor execution context
pub trait Context {
    /// Get the actor's address
    fn address(&self) -> String;
    
    /// Get the actor's current state
    fn state(&self) -> Box<dyn Any>;
    
    /// Create a child actor
    fn create_child<A: Actor>(&self, actor: A) -> String;
    
    /// Send a message to another actor
    fn send<M: Message>(&self, recipient: &str, msg: M) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Send a kill message to another actor
    fn send_kill<M: Message>(&self, recipient: &str, msg: M) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Stop the current actor
    fn stop(&self);
    
    /// Apply quantum correction to context
    fn quantum_correct(&self) -> Result<(), Box<dyn std::error::Error>>;
}

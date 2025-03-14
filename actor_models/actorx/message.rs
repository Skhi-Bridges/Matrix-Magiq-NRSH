//! Message trait definition for ActorX framework

/// Core Message trait for ActorX framework
pub trait Message: Send + 'static + Clone {}

/// Implement Message for common types
impl<T: Send + 'static + Clone> Message for T {}

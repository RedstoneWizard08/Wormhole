//! Utility types.

/// A type that represents a shortened form of `Send + Sync + 'static`.
pub trait TripleS = Send + Sync + 'static;

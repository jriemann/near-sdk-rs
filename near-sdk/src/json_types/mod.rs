//! Helper types for JSON serialization.

mod account;
mod integers;
mod public_key;
mod vector;

pub use account::ValidAccountId;
pub use integers::{I128, I64, U128, U64};
pub use public_key::{Base58PublicKey, CurveType};
pub use vector::Base64VecU8;

/// Raw type for duration in nanoseconds
pub type Duration = u64;

/// Duration in nanosecond wrapped into a struct for JSON serialization as a string.
pub type WrappedDuration = U64;

/// Raw type for timestamp in nanoseconds
pub type Timestamp = u64;

/// Timestamp in nanosecond wrapped into a struct for JSON serialization as a string.
pub type WrappedTimestamp = U64;

/// Balance wrapped into a struct for JSON serialization as a string.
pub type WrappedBalance = U128;

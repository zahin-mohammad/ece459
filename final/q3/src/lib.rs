//! Tallystick is a rust library for tallying votes.
//!
//! ## Compatibility
//!
//! Tallystick works with both nightly and stable rust, but more functionality is enabled on nightly. Tally methods that must be run with nightly rust are tagged `nightly`.
//!
//! To enable nightly support, add `features=["nightly"]` to your `Cargo.toml` entry for `tallystick`.
//!

#![warn(missing_docs)]
#![allow(clippy::redundant_field_names, clippy::cognitive_complexity)]
#![cfg_attr(feature = "nightly", feature(specialization))]

/// Utilities for parsing common vote formats
pub mod util;

/// Plurality voting is an electoral system in which each voter is allowed to vote for only one candidate
/// and the candidate who polls the most among their counterparts (a plurality) is elected. It may be called
/// first-past-the-post (FPTP), single-choice voting, simple plurality, or relative/simple majority.
///
/// # Example
/// ```
///    use tallystick::plurality::DefaultPluralityTally;
///
///    // Election between Alice, Bob, and Cir with two winners.
///    let mut tally = DefaultPluralityTally::new(2);
///    tally.add("Alice");
///    tally.add("Cir");
///    tally.add("Bob");
///    tally.add("Alice");
///    tally.add("Alice");
///    tally.add("Bob");
///
///    let winners = tally.winners().into_unranked();
///    println!("The winners are {:?}", winners);
/// ```
pub mod plurality;

/// Score voting or "range voting" is an electoral system in which voters give each candidate a score,
/// the scores are summed, and the candidate with the highest total is elected. It has been described
/// by various other names including "evaluative voting", "utilitarian voting", and "the point system".
pub mod score;

// Common Data Structures
// ----------------------
mod result;
pub use crate::result::RankedWinners;

// TODO: Remove dead code
#[cfg(feature = "nightly")]
#[allow(dead_code)]
mod votetree;

#[cfg(feature = "nightly")]
pub use votetree::Transfer;

#[cfg(feature = "nightly")]
pub(crate) use votetree::VoteTree;

/// Requires the `nightly` feature to be enabled `nightly`
#[cfg(feature = "nightly")]
mod quota;

/// Requires the `nightly` feature to be enabled
#[cfg(feature = "nightly")]
pub use crate::quota::Quota;

mod traits;
pub use crate::traits::Numeric;

mod errors;
pub use crate::errors::TallyError;

// Common Utility Functions
// ------------------------

// Check if a vector has a duplicate
// This is critical for transitive (ranked) votes
#[cfg(feature = "nightly")]
pub(crate) fn check_duplicate<T: PartialEq>(slice: &[T]) -> Result<(), TallyError> {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return Err(TallyError::VoteHasDuplicateCandidates);
        }
    }
    Ok(())
}

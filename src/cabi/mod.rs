//! C-compatible ABI wrappers for the RNGs.
//!
//! Handles are opaque: create and free them with the matching C functions.
//! Fill/free functions are unsafe in Rust. For a nonempty fill the handle and
//! output allocation must be valid, properly aligned, exclusive and disjoint.
//! Zero-length fills accept null pointers and leave state unchanged. Invalid
//! numeric ranges, null pointers and misaligned pointers cause a no-op.
//! SIMD constructors return null when the required CPU features are unavailable.
//! Fills form a continuous stream independent of output alignment or call sizes.
//! Pending SIMD words are retained in the opaque handle. Cached stores and a
//! sequential implementation are intentional: no parallel reseeding or NT-store
//! synchronization is involved.
//!
//! ```compile_fail
//! let p = urng::cabi::pcg32_new(0);
//! urng::cabi::pcg32_free(p); // requires an unsafe block
//! ```

#![allow(ambiguous_glob_reexports)]

mod stream;

pub mod cbrng;
pub mod prng;

pub use cbrng::*;
pub use prng::*;

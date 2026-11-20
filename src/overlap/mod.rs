#![allow(dead_code)]

//! Overlap (yes/no) tests between geometric shapes.
//!
//! Each function returns only a `bool`; no distance or intersection points
//! are computed, so these are the fast path for collision checks.
//! **Touching counts as overlap** (shapes are treated as closed sets).
//!
//! | Function | Shapes |
//! |----------|--------|
//! | [`overlap_orect_orect()`] | oriented rectangle vs oriented rectangle |
//! | [`overlap_circle_orect()`] | circle vs oriented rectangle |

pub mod overlap_circle_orect;
pub mod overlap_orect_orect;

pub use overlap_circle_orect::overlap_circle_orect;
pub use overlap_orect_orect::overlap_orect_orect;

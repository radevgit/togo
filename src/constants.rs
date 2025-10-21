//! Centralized epsilon and tolerance constants for numeric stability.
//!
//! This module defines all epsilon and tolerance values used throughout the library
//! to ensure consistent handling of floating-point precision across geometric algorithms.
//!
//! # Epsilon Hierarchy
//!
//! - `DIVISION_EPSILON` (1e-12): Guards near-zero denominators in degenerate geometry
//! - `GEOMETRIC_EPSILON` (1e-10): Threshold for geometric predicates and tolerance tests
//!
//! For reference, f64::EPSILON (~2.22e-16) is the machine precision limit but is not
//! directly used since geometric operations require much larger tolerances.

/// Geometric epsilon for tolerance in geometric predicates and comparisons.
/// Used for detecting collinearity, near-tangency, coincident points, and similar conditions.
/// This is larger than machine epsilon to account for accumulated floating-point errors
/// in geometric computations.
pub const GEOMETRIC_EPSILON: f64 = 1e-10;

/// Epsilon for guarding division operations to prevent division by near-zero values.
/// Used before dividing by computed values that might be near zero due to degenerate geometry
/// (e.g., line directions with very small magnitude, perpendicular distances near zero).
///
/// This is smaller than GEOMETRIC_EPSILON because we want to allow divisions by values
/// slightly smaller than geometric tolerance, but still guard against truly degenerate cases.
pub const DIVISION_EPSILON: f64 = 1e-12;

/// Tolerance for collinearity detection in segment-segment and point-line tests.
/// Points or lines that are collinear within this tolerance are treated as collinear.
pub const COLLINEARITY_TOLERANCE: f64 = GEOMETRIC_EPSILON; // 1e-10

/// Tolerance for detecting nearly identical circles (same center and radius within tolerance).
/// Used to handle degenerate cases where two circles are effectively the same.
pub const CIRCLE_TOLERANCE: f64 = GEOMETRIC_EPSILON; // 1e-10

/// Tolerance for detecting coincident or nearly identical points.
/// Used when checking if two points are effectively the same location.
pub const POINT_TOLERANCE: f64 = GEOMETRIC_EPSILON; // 1e-10

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epsilon_ordering() {
        // Verify that epsilon constants are ordered correctly:
        // DIVISION_EPSILON < GEOMETRIC_EPSILON
        assert!(DIVISION_EPSILON < GEOMETRIC_EPSILON);
    }

    #[test]
    fn test_tolerance_values_reasonable() {
        // Verify tolerances are positive and not too large
        assert!(GEOMETRIC_EPSILON > 0.0);
        assert!(GEOMETRIC_EPSILON < 1.0);
        assert!(DIVISION_EPSILON > 0.0);
        assert!(DIVISION_EPSILON < GEOMETRIC_EPSILON);
    }

    #[test]
    fn test_related_tolerances_consistent() {
        // Verify related tolerances use the same value where appropriate
        assert_eq!(COLLINEARITY_TOLERANCE, GEOMETRIC_EPSILON);
        assert_eq!(CIRCLE_TOLERANCE, GEOMETRIC_EPSILON);
        assert_eq!(POINT_TOLERANCE, GEOMETRIC_EPSILON);
    }
}

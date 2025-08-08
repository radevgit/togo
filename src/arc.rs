#![allow(dead_code)]

use robust::{Coord, orient2d};

use crate::{
    circle::{Circle, circle},
    point::{Point, point},
};

use std::{fmt::Display, sync::atomic::AtomicUsize};

pub type Arcline = Vec<Arc>;

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);

/// An arc segment defined by start point, end point, center, and radius.
///
/// Arcs are fundamental geometric primitives.
/// <div class="warning">NOTE: Arcs are always CCW (counter-clockwise) in this library.</div>
///
/// # Fields
///
/// * `a` - Start point of the arc
/// * `b` - End point of the arc  
/// * `c` - Center point of the arc
/// * `r` - Radius of the arc (f64::INFINITY indicates a line segment)
/// * `id` - Non-unique identifier used for debugging and tracking segments
///
/// # Examples
///
/// ```
/// use base_geom::prelude::*;
///
/// let start = point(0.0, 1.0);
/// let end = point(1.0, 0.0);
/// let center = point(0.0, 0.0);
/// let radius = 1.0;
///
/// let arc = Arc::new(start, end, center, radius);
/// ```
///
// #00001
#[derive(Debug, Copy, Clone)]
pub struct Arc {
    /// Start point of the arc.
    pub a: Point,
    /// End point of the arc.
    pub b: Point,
    /// Center point of the arc.
    pub c: Point,
    /// Radius of the arc.
    pub r: f64,
    /// non-unique id, used for debugging and
    /// checking parts coming from the same segment
    pub id: usize,
}

// Implemented because id is different in tests
impl PartialEq for Arc {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.r == other.r
        //??? self.r == other.r
    }
}

impl Display for Arc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}, {:.20}]", self.a, self.b, self.c, self.r)
    }
}

impl Arc {
    /// Creates a new arc with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `a` - Start point of the arc
    /// * `b` - End point of the arc
    /// * `c` - Center point of the arc
    /// * `r` - Radius of the arc (use linearc() for segments)
    ///
    /// # Returns
    ///
    /// A new Arc instance with a unique internal ID
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// // Create a quarter circle arc
    /// let arc = Arc::new(
    ///     point(1.0, 0.0),  // start
    ///     point(0.0, 1.0),  // end
    ///     point(0.0, 0.0),  // center
    ///     1.0               // radius
    /// );
    ///
    /// // Create a line segment
    /// let line = Arc::new(
    ///     point(0.0, 0.0),     // start
    ///     point(1.0, 1.0),     // end
    ///     point(0.0, 0.0),     // center (unused for lines)
    ///     f64::INFINITY        // infinite radius indicates line
    /// );
    /// ```
    #[inline]
    pub fn new(a: Point, b: Point, c: Point, r: f64) -> Self {
        let id = ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Arc { a, b, c, r, id }
    }

    /// Set the id of the arc.
    #[inline]
    pub fn id(&mut self, id: usize) {
        self.id = id;
    }

    /// Returns true if this arc represents a circular arc (finite radius).
    ///
    /// # Returns
    ///
    /// True if the radius is finite, false if it represents a line segment
    #[inline]
    pub fn is_arc(&self) -> bool {
        self.r != f64::INFINITY
    }

    /// Returns true if this arc represents a line segment (infinite radius).
    ///
    /// # Returns
    ///
    /// True if the radius is infinite, false if it represents a circular arc
    #[inline]
    pub fn is_line(&self) -> bool {
        self.r == f64::INFINITY
    }

    /// Translates this arc by the given vector.
    ///
    /// # Arguments
    ///
    /// * `point` - The translation vector to apply
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let mut arc = Arc::new(
    ///     point(0.0, 0.0),
    ///     point(1.0, 0.0),
    ///     point(0.5, 0.0),
    ///     1.0
    /// );
    /// arc.translate(point(10.0, 5.0));
    /// // All points are now shifted by (10, 5)
    /// ```
    #[inline]
    pub fn translate(&mut self, point: Point) {
        self.a = self.a + point;
        self.b = self.b + point;
        self.c = self.c + point;
    }

    /// Returns a reversed copy of this Arc.
    #[inline]
    pub fn reverse(&self) -> Arc {
        Arc::new(self.b, self.a, self.c, self.r)
    }

    #[doc(hidden)]
    #[inline]
    /// Checks if the arc contains the given point,
    /// where the point is a result of intersection.
    ///
    /// # Arguments
    ///
    /// * `p` - The point to check
    ///
    /// # Returns
    /// True if the point is contained within the arc, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    /// let arc0 = arc(point(0.0, 0.0), point(1.0, 1.0), point(0.5, 0.5), 1.0);
    /// assert!(arc0.contains(point(1.0, 0.0))); // Point on the arc
    /// assert!(!arc0.contains(point(0.0, 1.0))); // Point outside the arc
    /// ```
    pub fn contains(&self, p: Point) -> bool {
        let pa = Coord {
            x: self.a.x,
            y: self.a.y,
        };
        let pb = Coord {
            x: self.b.x,
            y: self.b.y,
        };
        let pp = Coord { x: p.x, y: p.y };
        let perp = orient2d(pa, pp, pb);
        //let perp = Arc::simple_orient2d(pa, pp, pb);
        perp >= 0f64
    }

    // fn simple_orient2d(p: Coord<f64>, q: Coord<f64>, r: Coord<f64>) -> f64 {
    //     (q.x - p.x) * (r.y - q.y) - (q.y - p.y) * (r.x - q.x)
    // }

    #[inline]
    /// a, b, p - points on arc using robust order
    pub(crate) fn contains_order2d(a: Point, b: Point, p: Point) -> f64 {
        let pa = Coord { x: a.x, y: a.y };
        let pb = Coord { x: b.x, y: b.y };
        let pp = Coord { x: p.x, y: p.y };
        orient2d(pa, pb, pp)
    }
}

/// Shorter version of `Arc::new`.
#[inline]
pub fn arc(a: Point, b: Point, c: Point, r: f64) -> Arc {
    Arc::new(a, b, c, r)
}

/// Create line segment as an arc.
#[inline]
pub fn arcline(a: Point, b: Point) -> Arc {
    Arc::new(a, b, point(f64::INFINITY, f64::INFINITY), f64::INFINITY)
}

#[cfg(test)]
mod test_arc {
    use super::*;

    #[test]
    fn test_new() {
        let arc0 = Arc::new(point(1.0, 1.0), point(1.0, 3.0), point(2.0, -1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(1.0, 3.0), point(2.0, -1.0), 1.0);
        assert_eq!(arc0, arc1);
    }

    #[test]
    fn test_display() {
        let arc = arc(point(1.0, 1.0), point(1.0, 3.0), point(2.0, -1.0), 1.0);
        //print!("{}", arc);
        assert_eq!(
            "[[1.00000000000000000000, 1.00000000000000000000], [1.00000000000000000000, 3.00000000000000000000], [2.00000000000000000000, -1.00000000000000000000], 1.00000000000000000000]",
            format!("{}", arc)
        );
    }

    #[test]
    fn test_id_set() {
        let mut arc = arc(point(1.0, 1.0), point(1.0, 3.0), point(2.0, -1.0), 1.0);
        arc.id(42);
        assert!(arc.id == 42);
    }

    #[test]
    fn test_is_arc() {
        let arc = arcline(point(1.0, 1.0), point(1.0, 3.0));
        assert!(arc.is_line());
        assert!(!arc.is_arc());
    }

    #[test]
    fn test_contains_orientation() {
        // CCW quarter-circle from (1,0) to (0,1) centered at (0,0)
        let a = Arc::new(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        // Point at 45 degrees should be contained
        assert!(a.contains(point(0.7071067811865476, 0.7071067811865476)));
        // Point outside arc span should not
        assert!(!a.contains(point(0.7071067811865476, -0.7071067811865476)));
        // Endpoints are considered contained (collinear => orient2d >= 0)
        assert!(a.contains(point(1.0, 0.0)));
        assert!(a.contains(point(0.0, 1.0)));
    }

    #[test]
    fn test_contains_order2d() {
        let a = point(0.0, 0.0);
        let b = point(1.0, 0.0);
        // Left of AB is positive orientation
        let p_left = point(0.5, 1.0);
        assert!(Arc::contains_order2d(a, b, p_left) > 0.0);
        // Right of AB is negative orientation
        let p_right = point(0.5, -1.0);
        assert!(Arc::contains_order2d(a, b, p_right) < 0.0);
        // Collinear is zero
        let p_col = point(0.5, 0.0);
        assert!(Arc::contains_order2d(a, b, p_col) == 0.0);
    }

    #[test]
    fn test_arcline_creation() {
        // Test that arcline creates a line segment (infinite radius)
        let line_arc = arcline(point(0.0, 0.0), point(5.0, 5.0));
        assert!(line_arc.is_line());
        assert!(!line_arc.is_arc());
        assert_eq!(line_arc.r, f64::INFINITY);
        assert_eq!(line_arc.a, point(0.0, 0.0));
        assert_eq!(line_arc.b, point(5.0, 5.0));
    }

    #[test]
    fn test_arc_reverse() {
        let original = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let reversed = original.reverse();
        
        // Check that endpoints are swapped
        assert_eq!(reversed.a, original.b);
        assert_eq!(reversed.b, original.a);
        // Center and radius should remain the same
        assert_eq!(reversed.c, original.c);
        assert_eq!(reversed.r, original.r);
    }

    #[test]
    fn test_arc_translate() {
        let mut arc = Arc::new(point(1.0, 1.0), point(2.0, 2.0), point(1.5, 1.5), 0.5);
        let translation = point(10.0, -5.0);
        
        arc.translate(translation);
        
        // All points should be translated
        assert_eq!(arc.a, point(11.0, -4.0));
        assert_eq!(arc.b, point(12.0, -3.0));
        assert_eq!(arc.c, point(11.5, -3.5));
        // Radius should remain unchanged
        assert_eq!(arc.r, 0.5);
    }

    #[test]
    fn test_copy() {
        let arc = arcline(point(1.0, 1.0), point(1.0, 3.0));
        let arc2 = arc;
        assert_eq!(arc, arc2);
    }

    #[test]
    fn test_reverse() {
        // Test reversing a circular arc
        let original = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let reversed = original.reverse();

        // Start and end points should be swapped
        assert_eq!(reversed.a, original.b);
        assert_eq!(reversed.b, original.a);

        // Center and radius should remain the same
        assert_eq!(reversed.c, original.c);
        assert_eq!(reversed.r, original.r);

        // Test the specific values
        assert_eq!(reversed.a, point(0.0, 1.0));
        assert_eq!(reversed.b, point(1.0, 0.0));
        assert_eq!(reversed.c, point(0.0, 0.0));
        assert_eq!(reversed.r, 1.0);
    }

    #[test]
    fn test_reverse_twice_returns_original() {
        // Test that reversing twice returns to the original arc
        let original = arc(point(3.0, 4.0), point(1.0, 2.0), point(2.0, 3.0), 2.5);
        let double_reversed = original.reverse().reverse();

        // Should be equal to the original (excluding ID which may differ)
        assert_eq!(double_reversed.a, original.a);
        assert_eq!(double_reversed.b, original.b);
        assert_eq!(double_reversed.c, original.c);
        assert_eq!(double_reversed.r, original.r);
    }
}

/// Check if the arc contains the point.
// #00003 #00004

// pub fn contains_ulps(self: Self, p: Point, ulps: i64) -> bool {
//     let length = (p - self.c).norm();
//     if almost_equal_as_int(length, self.r, ulps) {
//         let diff_pa = p - self.a;
//         let diff_ba = self.b - self.a;
//         let perp = diff_pa.perp(diff_ba);
//         return perp >= 0f64;
//     } else {
//         return false;
//     }
// }

// pub fn contains_eps(self: Self, p: Point, eps: f64) -> bool {
//     let length = (p - self.c).norm();
//     if (length - self.r).abs() <= eps {
//         let diff_pa = p - self.a;
//         let diff_ba = self.b - self.a;
//         let perp = diff_pa.perp(diff_ba);
//         return perp >= 0f64;
//     } else {
//         return false;
//     }
// }

// If the point is result of intersection,
// we already know the distance, from center is arc radius
// so no need to check the distance.
// pub fn contains(&self, p: Point) -> bool {
//     let diff_pa = p - self.a;
//     let diff_ba = self.b - self.a;
//     let perp = diff_pa.perp(diff_ba);
//     return perp >= 0f64;
// }

/// Check if the arc is with collapsed radius.
const EPS_COLLAPSED: f64 = 1E-8; // TODO: what should be the exact value.
pub fn arc_is_collapsed_radius(r: f64) -> bool {
    // no abs() since it can be negative
    if r < EPS_COLLAPSED || r.is_nan() {
        return true;
    }
    false
}

/// Check if the arc is with collapsed ends.
pub fn arc_is_collapsed_ends(a: Point, b: Point) -> bool {
    if a.close_enough(b, EPS_COLLAPSED) {
        return true;
    }
    false
}

/// Check if the line-arc segments are degenerate.
pub fn arc_check(seg: &Arc) -> bool {
    if arc_is_collapsed_radius(seg.r) || arc_is_collapsed_ends(seg.a, seg.b) {
        return false;
    }
    true
}

#[cfg(test)]
mod test_arc_contains {
    use super::*;

    #[test]
    fn test_arc_contains_01() {
        let arc1 = arc(point(2.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0);
        assert_eq!(arc1.contains(point(0.0, 0.0)), true);
        assert_eq!(arc1.contains(point(-1.0, 1.0)), true);
    }

    #[test]
    fn test_arc_contains_02() {
        let arc1 = arc(point(-1.0, 1.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        assert_eq!(arc1.contains(point(0.0, 0.0)), true);
    }

    // #[test]
    // fn test_point_on_arc() {
    //     let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
    //     assert_eq!(arc.contains_ulps(point(0.0, 1.0), 5), true);
    // }

    #[test]
    fn test_arc_contains_large_r() {
        let arc = arc_circle_parametrization(point(1e20, 30.0), point(10.0, 30.0), 1f64);
        assert_eq!(arc.contains(point(1e20 + 1000.0, 30.0)), true);
    }

    #[test]
    fn test_arc_contains_00() {
        let sgrt_2_2 = std::f64::consts::SQRT_2 / 2.0;
        let arc0 = arc(point(1.0, 1.0), point(0.0, 0.0), point(0.5, 0.5), sgrt_2_2);
        assert!(arc0.contains(point(0.0, 1.0)));
    }

    #[test]
    fn test_arc_contains_03() {
        let arc0 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        assert!(arc0.contains(point(0.0, 1.0)));
    }

    #[test]
    fn test_arc_not_contains() {
        let arc = arc(point(0.0, -1.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let p = point(-1.0, 0.0);
        assert_eq!(arc.contains(p), false);
    }
}

#[cfg(test)]
mod test_arc_validation {
    use super::*;

    #[test]
    fn test_arc_is_collapsed_radius_normal_values() {
        // Normal positive radius values should not be collapsed
        assert!(!arc_is_collapsed_radius(1.0));
        assert!(!arc_is_collapsed_radius(0.1));
        assert!(!arc_is_collapsed_radius(100.0));
        assert!(!arc_is_collapsed_radius(f64::INFINITY));
    }

    #[test]
    fn test_arc_is_collapsed_radius_small_values() {
        // Values smaller than EPS_COLLAPSED (1E-8) should be collapsed
        assert!(arc_is_collapsed_radius(1E-9));
        assert!(arc_is_collapsed_radius(1E-10));
        assert!(arc_is_collapsed_radius(0.0));
    }

    #[test]
    fn test_arc_is_collapsed_radius_boundary_values() {
        // Test values around the EPS_COLLAPSED boundary
        assert!(arc_is_collapsed_radius(EPS_COLLAPSED / 2.0));
        assert!(!arc_is_collapsed_radius(EPS_COLLAPSED * 2.0));
        
        // Exactly at the boundary should be collapsed
        assert!(arc_is_collapsed_radius(EPS_COLLAPSED - f64::EPSILON));
    }

    #[test]
    fn test_arc_is_collapsed_radius_negative_values() {
        // Negative radius values should be collapsed
        assert!(arc_is_collapsed_radius(-1.0));
        assert!(arc_is_collapsed_radius(-0.1));
        assert!(arc_is_collapsed_radius(-1E-10));
    }

    #[test]
    fn test_arc_is_collapsed_radius_nan() {
        // NaN values should be collapsed
        assert!(arc_is_collapsed_radius(f64::NAN));
    }

    #[test]
    fn test_arc_is_collapsed_ends_normal_points() {
        // Normal separated points should not be collapsed
        assert!(!arc_is_collapsed_ends(point(0.0, 0.0), point(1.0, 0.0)));
        assert!(!arc_is_collapsed_ends(point(0.0, 0.0), point(0.0, 1.0)));
        assert!(!arc_is_collapsed_ends(point(-1.0, -1.0), point(1.0, 1.0)));
        assert!(!arc_is_collapsed_ends(point(100.0, 200.0), point(300.0, 400.0)));
    }

    #[test]
    fn test_arc_is_collapsed_ends_identical_points() {
        // Identical points should be collapsed
        assert!(arc_is_collapsed_ends(point(0.0, 0.0), point(0.0, 0.0)));
        assert!(arc_is_collapsed_ends(point(1.0, 1.0), point(1.0, 1.0)));
        assert!(arc_is_collapsed_ends(point(-5.0, 10.0), point(-5.0, 10.0)));
    }

    #[test]
    fn test_arc_is_collapsed_ends_very_close_points() {
        // Points closer than EPS_COLLAPSED should be collapsed
        let p1 = point(0.0, 0.0);
        let p2 = point(EPS_COLLAPSED / 2.0, 0.0);
        assert!(arc_is_collapsed_ends(p1, p2));

        let p3 = point(100.0, 100.0);
        let p4 = point(100.0 + EPS_COLLAPSED / 3.0, 100.0 + EPS_COLLAPSED / 3.0);
        assert!(arc_is_collapsed_ends(p3, p4));
    }

    #[test]
    fn test_arc_is_collapsed_ends_boundary_distance() {
        // Points at exactly EPS_COLLAPSED distance
        let p1 = point(0.0, 0.0);
        let p2 = point(EPS_COLLAPSED, 0.0);
        // This should not be collapsed (distance equals tolerance)
        assert!(!arc_is_collapsed_ends(p1, p2));

        // Points slightly farther than EPS_COLLAPSED
        let p3 = point(0.0, 0.0);
        let p4 = point(EPS_COLLAPSED * 2.0, 0.0);
        assert!(!arc_is_collapsed_ends(p3, p4));
    }

    #[test]
    fn test_arc_check_valid_arcs() {
        // Valid arcs should pass the check
        let valid_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1.0);
        assert!(arc_check(&valid_arc1));

        let valid_arc2 = arc(point(-1.0, -1.0), point(1.0, 1.0), point(0.0, 0.0), 2.0);
        assert!(arc_check(&valid_arc2));

        // Line segments (infinite radius) should also be valid if endpoints are separated
        let valid_line = arcline(point(0.0, 0.0), point(10.0, 0.0));
        assert!(arc_check(&valid_line));
    }

    #[test]
    fn test_arc_check_collapsed_radius() {
        // Arcs with collapsed radius should fail the check
        let collapsed_radius_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1E-10);
        assert!(!arc_check(&collapsed_radius_arc1));

        let collapsed_radius_arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -1.0);
        assert!(!arc_check(&collapsed_radius_arc2));

        let nan_radius_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::NAN);
        assert!(!arc_check(&nan_radius_arc));
    }

    #[test]
    fn test_arc_check_collapsed_ends() {
        // Arcs with collapsed endpoints should fail the check
        let collapsed_ends_arc1 = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1.0);
        assert!(!arc_check(&collapsed_ends_arc1));

        let close_points = point(0.0, 0.0);
        let very_close_points = point(EPS_COLLAPSED / 2.0, 0.0);
        let collapsed_ends_arc2 = arc(close_points, very_close_points, point(0.0, 1.0), 1.0);
        assert!(!arc_check(&collapsed_ends_arc2));

        // Line segments with collapsed endpoints should also fail
        let collapsed_line = arcline(point(1.0, 1.0), point(1.0, 1.0));
        assert!(!arc_check(&collapsed_line));
    }

    #[test]
    fn test_arc_check_both_collapsed() {
        // Arcs with both collapsed radius and collapsed endpoints should fail
        let both_collapsed = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1E-10);
        assert!(!arc_check(&both_collapsed));

        let both_collapsed2 = arc(point(5.0, 5.0), point(5.0, 5.0), point(0.0, 0.0), f64::NAN);
        assert!(!arc_check(&both_collapsed2));
    }

    #[test]
    fn test_arc_check_edge_cases() {
        // Test with very large coordinates
        let large_coord_arc = arc(
            point(1E10, 1E10), 
            point(1E10 + 1.0, 1E10), 
            point(1E10, 1E10 + 1.0), 
            1.0
        );
        assert!(arc_check(&large_coord_arc));

        // Test with very small but valid radius
        let small_radius_arc = arc(
            point(0.0, 0.0), 
            point(1.0, 0.0), 
            point(0.5, 0.0), 
            EPS_COLLAPSED * 10.0
        );
        assert!(arc_check(&small_radius_arc));

        // Test with large radius
        let large_radius_arc = arc(
            point(0.0, 0.0), 
            point(1E-6, 0.0), 
            point(0.0, 1E6), 
            1E6
        );
        assert!(arc_check(&large_radius_arc));
    }
}

/// Returns the circle parameterization of the Arc. Without thetas.
/// Much faster, avoids arctan()
/// Important: There are two arcs. Always return CCW oriented one.
const ZERO: f64 = 0f64;
const MIN_BULGE: f64 = 1E-8;
pub fn arc_circle_parametrization(pp1: Point, pp2: Point, bulge: f64) -> Arc {
    let mut p1 = pp1;
    let mut p2 = pp2;
    let mut bulge = bulge;
    if bulge.abs() < MIN_BULGE || p1.close_enough(p2, EPS_COLLAPSED) {
        // create line
        return arcline(pp1, pp2);
    }
    if bulge < 0f64 {
        // make arc CCW
        p1 = pp2;
        p2 = pp1;
        bulge = -bulge;
    }

    // TODO: check for numerical issues
    let t2 = (p2 - p1).norm();
    let dt2 = (1.0 + bulge) * (1.0 - bulge) / (4.0 * bulge);
    let cx = (0.5 * p1.x + 0.5 * p2.x) + dt2 * (p1.y - p2.y);
    let cy = (0.5 * p1.y + 0.5 * p2.y) + dt2 * (p2.x - p1.x);
    let r = 0.25 * t2 * (1.0 / bulge + bulge).abs();
    arc(p1, p2, point(cx, cy), r)
}

#[cfg(test)]
mod test_arc_circle_parametrization {
    use std::f64::consts::SQRT_2;

    use crate::svg::svg;

    use super::*;

    const _0: f64 = 0f64;
    const _1: f64 = 0f64;
    const _2: f64 = 0f64;

    #[test]
    fn test_arc_circle_parametrization_01() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), 0.5);
        assert_eq!(
            arc0,
            arc(
                point(100.0, 100.0),
                point(200.0, 200.0),
                point(112.5, 187.5),
                88.38834764831844,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_01_svg() {
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), 0.5);
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 3.0);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_arc_circle_parametrization_02() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), 1.5);
        assert_eq!(
            arc0,
            arc(
                point(100.0, 100.0),
                point(200.0, 200.0),
                point(170.83333333333334, 129.16666666666666),
                76.60323462854265,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_02_svg() {
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), 1.5);
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 3.0);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_arc_circle_parametrization_03() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), -0.5);
        assert_eq!(
            arc0,
            arc(
                point(200.0, 200.0),
                point(100.0, 100.0),
                point(187.5, 112.5),
                88.38834764831844,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_03_svg() {
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), -0.5);
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 3.0);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_arc_circle_parametrization_04() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), -1.5);
        assert_eq!(
            arc0,
            arc(
                point(200.0, 200.0),
                point(100.0, 100.0),
                point(129.16666666666666, 170.83333333333334),
                76.60323462854265,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_04_svg() {
        let arc0 = arc_circle_parametrization(point(100.0, 100.0), point(200.0, 200.0), -1.5);
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 3.0);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_arc_circle_parametrization_05() {
        // the function should return CCW arc
        let arc0 =
            arc_circle_parametrization(point(1.0, 0.0), point(2.0, 1.0), -1.0 + f64::EPSILON);
        assert_eq!(
            arc0,
            arc(
                point(2.0, 1.0),
                point(1.0, 0.0),
                point(1.5000000000000002, 0.4999999999999999),
                SQRT_2 / 2.0,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_05_svg() {
        let arc0 =
            arc_circle_parametrization(point(1.0, 0.0), point(2.0, 1.0), -1.0 + f64::EPSILON);
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 0.1);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_display_01() {
        let arc0 = arc_circle_parametrization(point(1.0, 2.0), point(3.0, 4.0), 3.3);
        assert_eq!(
            "[[1.00000000000000000000, 2.00000000000000000000], [3.00000000000000000000, 4.00000000000000000000], [3.49848484848484808651, 1.50151515151515169144], 2.54772716009334887488]",
            format!("{}", arc0)
        );
    }

    #[test]
    fn test_arc_circle_parametrization_bulge_zero() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(1.0, 0.0), point(2.0, 1.0), 0.0);
        assert_eq!(arc0, arcline(point(1.0, 0.0), point(2.0, 1.0),),);
    }

    #[test]
    fn test_arc_circle_parametrization_the_same_points() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(point(2.0, 1.0), point(2.0, 1.0), 1.0);
        assert_eq!(arc0, arcline(point(2.0, 1.0), point(2.0, 1.0),),);
    }

    #[test]
    fn test_arc_circle_parametrization_06() {
        // the function should return CCW arc
        let arc0 = arc_circle_parametrization(
            point(200.0, -200.0),
            point(-200.0, 200.0),
            -1.0 + f64::EPSILON,
        );
        assert_eq!(
            arc0,
            arc(
                point(-200.0, 200.0),
                point(200.0, -200.0),
                point(4.4408920985006274e-14, 4.4408920985006274e-14),
                SQRT_2 * 200.0
            ),
        );
    }

    #[test]
    fn test_arc_circle_parametrization_07() {
        let mut arc0 = arc_circle_parametrization(
            point(200.0, -200.0),
            point(-200.0, 200.0),
            -1.0 + f64::EPSILON,
        );
        arc0.translate(point(200.0, 200.0));
        assert_eq!(
            arc0,
            arc(
                point(0.0, 400.0),
                point(400.0, 0.0),
                point(200.00000000000006, 200.00000000000006),
                SQRT_2 * 200.0,
            ),
        );
    }

    #[test]
    #[ignore = "svg output"]
    fn test_arc_circle_parametrization_07_svg() {
        let mut arc0 = arc_circle_parametrization(point(200.0, -200.0), point(-200.0, 200.0), -1.0);
        arc0.translate(point(200.0, 200.0));
        let mut svg = svg(400.0, 600.0);
        svg.arc(&arc0, "red");
        let circle = circle(point(arc0.c.x, arc0.c.y), 2.0);
        svg.circle(&circle, "blue");
        svg.write();
    }

    #[test]
    fn test_arc_circle_parametrization_line() {
        // should return line
        let line0 = arc_circle_parametrization(point(100.0, 100.0), point(300.0, 100.0), 0.0);
        assert_eq!(line0, arcline(point(100.0, 100.0), point(300.0, 100.0)));
    }
}

/// Given start end points of arc and radius, calculate bulge
/// TODO: not tested
// #00006
// pub fn arc_g_from_points2(a: Point, b: Point, r: f64) -> f64 {
//     let d = b - a;
//     let dist = d.x.hypot(d.y);
//     let seg = r - (0.5 * ((4.0 * r * r) - dist * dist).sqrt());
//     let mut g = 2.0 * seg / dist;
//     if g.is_nan() {
//         g = 0f64;
//     }
//     g
// }

/// Given start end points of arc and radius, calculate bulge
/// TODO: not tested
// pub fn arc_g_from_points3(a: Point, b: Point, c: Point) -> f64 {
//     let d = b - a;
//     let dist = d.x.hypot(d.y);
//     if dist < 1E-8 {
//         // close points
//         return 0f64;
//     }
//     let theta0 = (a.y - c.y).atan2(a.x - c.x);
//     let theta1 = (b.y - c.y).atan2(b.x - c.x);
//     let mut angle = (theta1 - theta0).abs();
//     if angle < 0f64 {
//         angle += 2.0 * std::f64::consts::PI;
//     }
//     angle * 0.25
// }

// Given start end points of arc and radius, calculate bulge
// TODO: looks here we need to solve quadratic equation
// https://stackoverflow.com/questions/48979861/numerically-stable-method-for-solving-quadratic-equations/50065711#50065711
pub fn arc_g_from_points(a: Point, b: Point, c: Point, r: f64) -> f64 {
    let dist = (b - a).norm();
    if dist < 1E-10 {
        // close points
        // TODO
        return 0f64;
    }
    // Side of line test
    // let diff_pa = c - a;
    // let diff_ba = b - a;
    // let perp = diff_pa.perp(diff_ba); // maybe use orient2d
    let pa = Coord { x: a.x, y: a.y };
    let pb = Coord { x: b.x, y: b.y };
    let pc = Coord { x: c.x, y: c.y };
    let perp = orient2d(pa, pb, pc);
    let ddd = (4.0 * r * r) - dist * dist;
    if ddd < 0.0 {
        // Invalid case - radius too small for the chord length
        return 0f64;
    }
    
    let seg = if perp <= 0f64 {
        // small arc
        r - (0.5 * ddd.sqrt())
    } else {
        // large arc
        r + (0.5 * ddd.sqrt())
    };
    
    // The original formula was returning 2.0 * seg / dist
    // But to match arc_circle_parametrization, we need to return the actual bulge
    // The relationship is: bulge = dist / (2 * seg)
    // This comes from the inverse of the parametrization formula
    if seg.abs() < 1E-10 {
        return 0f64;
    }
    
    return dist / (2.0 * seg);
}

#[cfg(test)]
mod test_arc_g_from_points {
    use crate::prelude::*;

    use super::*;

    const TEST_EPS: f64 = 1E-10;

    #[test]
    fn test_a_b_are_close() {
        let a = point(114.31083505599867, 152.84458247200070);
        let b = point(114.31083505599865, 152.84458247200067);
        let arc = arc_circle_parametrization(a, b, 16.0);
        assert_eq!(arc_g_from_points(a, b, arc.c, arc.r), 0.0);
    }

    #[test]
    fn test_a_b_are_the_same() {
        let a = point(114.31083505599865, 152.84458247200067);
        let b = point(114.31083505599865, 152.84458247200067);
        let arc = arc_circle_parametrization(a, b, 16.0);
        assert_eq!(arc_g_from_points(a, b, arc.c, arc.r), 0.0);
    }

    #[test]
    fn test_small_arc_perp_negative() {
        // Test small arc case with positive bulge
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let bulge = 0.3; // Small positive bulge
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should be reasonably close to original bulge
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_large_arc_perp_positive() {
        // Test large arc case with larger bulge
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let bulge = 1.5; // Large positive bulge
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should be reasonably close to original bulge
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_semicircle() {
        // Test semicircle case (bulge = 1.0)
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let bulge = 1.0; // Semicircle
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // For a semicircle, the function should return a finite positive value
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_quarter_circle() {
        // Test quarter circle case
        let a = point(0.0, 0.0);
        let b = point(1.0, 1.0);
        let bulge = 0.41421356; // Approximates tan(π/8) for quarter circle
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should be reasonably close to original bulge
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_very_small_distance() {
        // Test edge case with very small bulge (near line segment)
        let a = point(0.0, 0.0);
        let b = point(1.0, 0.0);
        let bulge = 1e-9; // Very small bulge, should create near-line
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // For very small bulge, arc_circle_parametrization returns a line segment
        if arc.r == f64::INFINITY {
            // Line segment case - arc_g_from_points should handle this gracefully
            let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            assert!(close_enough(bulge, result, TEST_EPS));
            assert!(result.is_finite());
        }
    }

    #[test]
    fn test_collinear_points() {
        // Test case with zero bulge (creates a line segment)
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let bulge = 0.0; // Zero bulge creates line segment
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // For zero bulge, arc_circle_parametrization returns a line segment
        if arc.r == f64::INFINITY {
            // Line segment case - arc_g_from_points should handle this gracefully
            let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            assert!(close_enough(bulge, result, TEST_EPS));
            assert!(result.is_finite());
        }
    }

    #[test]
    fn test_large_radius() {
        // Test with a small bulge that creates large radius (nearly straight line)
        let a = point(0.0, 0.0);
        let b = point(100.0, 0.0);
        let bulge = 0.01; // Small bulge creates large radius
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should be positive and finite
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_minimal_radius() {
        // Test with semicircle bulge (maximum curvature for single arc)
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let bulge = 1.0; // Semicircle
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let result = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should be positive and finite
        assert!(close_enough(bulge, result, TEST_EPS));
        assert!(result.is_finite());
    }

    #[test]
    fn test_consistency_with_parametrization() {
        // Test that arc_g_from_points is consistent with arc_circle_parametrization
        let a = point(1.0, 2.0);
        let b = point(3.0, 4.0);
        let bulge = 0.5;
        
        // Create arc from parametrization
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let calculated_bulge = arc_g_from_points(a, b, arc.c, arc.r);
        
        // Debug: print both values
        println!("Original bulge: {}, Calculated bulge: {}, Ratio: {}", bulge, calculated_bulge, calculated_bulge / bulge);
        
        // Should match the original bulge within numerical precision
        assert!((calculated_bulge - bulge).abs() < 1e-10, 
                "Expected {}, got {}", bulge, calculated_bulge);
    }

    #[test]
    fn test_negative_bulge_consistency() {
        // Test with negative bulge (clockwise arc converted to CCW)
        let a = point(0.0, 0.0);
        let b = point(2.0, 2.0);
        let bulge = -0.8;
        
        // Create arc from parametrization (should convert to CCW)
        let arc = arc_circle_parametrization(a, b, bulge);
        
        // Calculate bulge back from points
        let calculated_bulge = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
        
        // Should return positive value (CCW orientation)
        assert!(close_enough(-bulge, calculated_bulge, TEST_EPS));
        assert!(calculated_bulge.is_finite());
    }

    #[test]
    fn test_various_bulge_values() {
        // Test with various bulge values to verify round-trip consistency
        let test_bulges = [0.1, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0];
        let a = point(0.0, 0.0);
        let b = point(1.0, 0.0);
        
        for &bulge in &test_bulges {
            // Create arc from parametrization
            let arc = arc_circle_parametrization(a, b, bulge);
            
            // Skip line segments (bulge = 0 case)
            if arc.r == f64::INFINITY {
                continue;
            }
            
            // Calculate bulge back from points
            let calculated_bulge = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            
            // Should match the original bulge within numerical precision
            assert!((calculated_bulge - bulge).abs() < 1e-10, 
                    "Bulge {} resulted in {} (difference: {})", 
                    bulge, calculated_bulge, (calculated_bulge - bulge).abs());
        }
    }

    #[test]
    fn test_different_point_positions() {
        // Test with different start/end point configurations
        let test_cases = [
            (point(0.0, 0.0), point(1.0, 0.0)),   // Horizontal
            (point(0.0, 0.0), point(0.0, 1.0)),   // Vertical
            (point(0.0, 0.0), point(1.0, 1.0)),   // Diagonal
            (point(-1.0, -1.0), point(1.0, 1.0)), // Diagonal through origin
            (point(10.0, 20.0), point(30.0, 40.0)), // Larger coordinates
        ];
        
        let bulge = 0.5;
        
        for (a, b) in test_cases.iter() {
            // Create arc from parametrization
            let arc = arc_circle_parametrization(*a, *b, bulge);
            
            // Calculate bulge back from points
            let calculated_bulge = arc_g_from_points(arc.a, arc.b, arc.c, arc.r);
            
            // Should match the original bulge within numerical precision
            assert!((calculated_bulge - bulge).abs() < 1e-10, 
                    "Points {:?} -> {:?}: expected {}, got {}", 
                    a, b, bulge, calculated_bulge);
        }
    }
}

#[cfg(test)]
mod test_geom_arc_g_from_pt {
    const _0: f64 = 0f64;
    const _1: f64 = 0f64;
    const _2: f64 = 0f64;

    #[test]
    fn test_arc_g_from_pt_colinear() {
        // a,b,c points on a line
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, 0.5)),
        //     0.0
        // );
        // // a,b,c on a line, c outside ab
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, 2.0)),
        //     0.0
        // );
        // // a,b,c on a line, c outside ab
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, -2.0)),
        //     0.0
        // );

        // // a=c
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, 0.0)),
        //     0.0
        // );
        // // b=c
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, 1.0)),
        //     0.0
        // );

        // // a=b !=c
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 1.0), point(0.0, 1.0), point(0.5, 1.0)),
        //     0.0
        // );
        // // a=b=c
        // assert_eq!(
        //     geom_arc_g_from_pt(point(0.0, 1.0), point(0.0, 1.0), point(0.0, 1.0)),
        //     0.0
        // );
    }

    // #[test]
    // #[ignore]
    // fn test_arc_g_from_pt_half_circle() {
    //     let g = geom_arc_g_from_pt(point(0.0, 0.0), point(0.0, 1.0), point(0.0, 0.5));
    //     assert_eq!(g, 1.0);
    // }
}

/// Computes a tight bounding circle for the arc
// #00007
// TODO: Check the correctness
pub fn arc_bound_circle(a: Point, b: Point, g: f64) -> Circle {
    // set c to midpoint m for now
    let cx = 0.5 * a.x + 0.5 * b.x;
    let cy = 0.5 * a.y + 0.5 * b.y;
    if g.abs() <= 1f64 {
        // c should just be m
        let r = 0.5 * (b - a).norm();
        circle(point(cx, cy), r)
    } else {
        let t2 = (b - a).norm();
        let dt2 = (1f64 + g) * (1f64 - g) / (4f64 * g);
        let cx = cx + dt2 * (a.y - b.y);
        let cy = cy + dt2 * (b.x - a.x);
        // r = t * (1+g*g)/(2*g);
        // Since g > 1, we can do better:
        let r = 0.25 * t2 * (1. / g + g);
        circle(point(cx, cy), r)
    }
}

#[cfg(test)]
mod test_arc_bound_circle {
    use super::*;
    const ONE: f64 = 1f64;
    const ZERO: f64 = 0f64;

    #[test]
    fn test_g_less_1() {
        // horizontal line segment
        let v0 = point(-2.0, 1.0);
        let v1 = point(2.0, 1.0);
        let res = circle(point(0f64, 1f64), 2f64);
        assert_eq!(arc_bound_circle(v0, v1, 0.0), res);

        // half circle
        let v0 = point(1.0, 1.0);
        let v1 = point(1.0, 3.0);
        let res = circle(point(1f64, 2f64), 1f64);
        assert_eq!(arc_bound_circle(v0, v1, 1.0), res);
    }

    #[test]
    fn test_g_greater_1() {
        // horizontal line segment
        let res = circle(point(0.0, -0.5), 2.5);
        assert_eq!(
            arc_bound_circle(point(-2.0, 1.0), point(2.0, 1.0), 2.0),
            res
        );

        // half circle
        let res = circle(point(5.999999969612645, 1.000000005), 4.999999969612645);
        assert_eq!(
            arc_bound_circle(point(1.0, 1.0), point(1.0, 1.00000001), 2000000000.0),
            res
        );
    }
}

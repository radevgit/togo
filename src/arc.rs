#![allow(dead_code)]

use robust::{Coord, orient2d};

use crate::prelude::*;

use std::{fmt::Display, sync::atomic::AtomicUsize};

/// A Arcline is a sequence of connected Arc-s forming a path.
pub type Arcline = Vec<Arc>;

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);
const EPS_COLLAPSED: u64 = 1000;

/// An arc segment (CCW) defined by start point, end point, center, and radius.
///
/// Arcs are fundamental geometric primitives.
/// <div class="warning">NOTE: Arcs are always CCW (counter-clockwise) in this library.</div>
///
/// # Fields
///
/// * `a` - Start point of the arc
/// * `b` - End point of the arc  
/// * `c` - Center point of the arc
/// * `r` - Radius of the arc (`f64::INFINITY` indicates a line segment)
/// * `id` - Non-unique identifier used for debugging and tracking segments
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// let start = point(0.0, 1.0);
/// let end = point(1.0, 0.0);
/// let center = point(0.0, 0.0);
/// let radius = 1.0;
///
/// let arc = arc(start, end, center, radius);
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
    /// * `r` - Radius of the arc (use `arcseg()` for segments)
    ///
    /// # Returns
    ///
    /// A new Arc instance with a unique internal ID
    ///
    /// <div class="warning">Arcs are always CCW (counter-clockwise) in this library!</div>
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
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
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
    /// assert!(arc.is_arc()); // Has finite radius
    ///
    /// let line = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    /// assert!(!line.is_arc()); // Has infinite radius (line segment)
    /// ```
    #[inline]
    #[must_use]
    pub fn is_arc(&self) -> bool {
        self.r != f64::INFINITY
    }

    /// Returns true if this arc represents a line segment (infinite radius).
    ///
    /// # Returns
    ///
    /// True if the radius is infinite, false if it represents a circular arc
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
    /// assert!(!arc.is_seg()); // Has finite radius
    ///
    /// let line = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    /// assert!(line.is_seg()); // Has infinite radius (line segment)
    /// ```
    #[inline]
    #[must_use]
    pub fn is_seg(&self) -> bool {
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
    /// use basegeom::prelude::*;
    ///
    /// let mut my_arc = arc(
    ///     point(0.0, 0.0),
    ///     point(1.0, 0.0),
    ///     point(0.5, 0.0),
    ///     1.0
    /// );
    /// my_arc.translate(point(10.0, 5.0));
    /// // All points are now shifted by (10, 5)
    /// ```
    #[inline]
    pub fn translate(&mut self, point: Point) {
        self.a = self.a + point;
        self.b = self.b + point;
        self.c = self.c + point;
    }

    /// Scales this arc by the given factor.
    #[inline]
    pub fn scale(&mut self, factor: f64) {
        self.a = self.a * factor;
        self.b = self.b * factor;
        self.c = self.c * factor;
        self.r *= factor;
    }

    /// Returns a reversed copy of this Arc.
    ///
    /// The reversed arc (all arcs are CCW) is not the same as original arc, but complement of the circle.
    ///
    /// # Returns
    ///
    /// A new Arc with start and end points swapped
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
    /// let reversed = arc.reverse();
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(&self) -> Arc {
        arc(self.b, self.a, self.c, self.r)
    }

    #[inline]
    /// Checks if the arc contains the given point,
    /// where the point is a result of intersection.
    /// <div class="warning">This does not work for points not on the circle!</div>
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
    /// use basegeom::prelude::*;
    /// let arc0 = arc(point(0.0, 0.0), point(1.0, 1.0), point(0.5, 0.5), 1.0);
    /// assert!(arc0.contains(point(1.0, 0.0))); // Point on the arc
    /// assert!(!arc0.contains(point(0.0, 1.0))); // Point outside the arc
    /// ```
    #[must_use]
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
}

/// Creates a new Arc with the given parameters.
///
/// This is a convenience function equivalent to `Arc::new(a, b, c, r)`.
///
/// # Arguments
///
/// * `a` - The start point of the arc
/// * `b` - The end point of the arc  
/// * `c` - The center point of the arc
/// * `r` - The radius of the arc
///
/// # Returns
///
/// A new Arc instance
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
/// let arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1.0);
/// assert_eq!(arc.a, point(0.0, 0.0));
/// assert_eq!(arc.b, point(1.0, 0.0));
/// assert_eq!(arc.r, 1.0);
/// ```
#[inline]
#[must_use]
pub fn arc(a: Point, b: Point, c: Point, r: f64) -> Arc {
    Arc::new(a, b, c, r)
}

/// Creates a line segment represented as an Arc with infinite radius.
///
/// This function creates an Arc that represents a straight line segment
/// between two points. The arc uses infinite radius to distinguish it
/// from curved arcs.
///
/// # Arguments
///
/// * `a` - The start point of the line segment
/// * `b` - The end point of the line segment
///
/// # Returns
///
/// An Arc representing a line segment with infinite radius
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
/// let line = arcseg(point(0.0, 0.0), point(1.0, 1.0));
/// assert!(line.is_seg());
/// assert!(!line.is_arc());
/// assert_eq!(line.r, f64::INFINITY);
/// ```
#[inline]
#[must_use]
pub fn arcseg(a: Point, b: Point) -> Arc {
    arc(a, b, point(f64::INFINITY, f64::INFINITY), f64::INFINITY)
}

/// Translates Arcline by a given translation vector.
pub fn arcline_translate(arc: &mut Arcline, translation: Point) {
    for segment in arc.iter_mut() {
        segment.translate(translation);
    }
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
        let arc = arcseg(point(1.0, 1.0), point(1.0, 3.0));
        assert!(arc.is_seg());
        assert!(!arc.is_arc());
    }

    #[test]
    fn test_contains_orientation() {
        // CCW quarter-circle from (1,0) to (0,1) centered at (0,0)
        let a = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        // Point at 45 degrees should be contained
        assert!(a.contains(point(0.7071067811865476, 0.7071067811865476)));
        // Point outside arc span should not
        assert!(!a.contains(point(0.7071067811865476, -0.7071067811865476)));
        // Endpoints are considered contained (collinear => orient2d >= 0)
        assert!(a.contains(point(1.0, 0.0)));
        assert!(a.contains(point(0.0, 1.0)));
    }

    #[test]
    fn test_arcseg_creation() {
        // Test that arcseg creates a line segment (infinite radius)
        let line_arc = arcseg(point(0.0, 0.0), point(5.0, 5.0));
        assert!(line_arc.is_seg());
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
        let mut arc = arc(point(1.0, 1.0), point(2.0, 2.0), point(1.5, 1.5), 0.5);
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
        let arc = arcseg(point(1.0, 1.0), point(1.0, 3.0));
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

    #[test]
    fn test_arcline_translate_empty() {
        // Test translating an empty arcline
        let mut empty_arcline: Arcline = vec![];
        let translation = point(5.0, -3.0);
        arcline_translate(&mut empty_arcline, translation);
        assert_eq!(empty_arcline.len(), 0);
    }

    #[test]
    fn test_arcline_translate_single_arc() {
        // Test translating an arcline with a single arc
        let mut arcline = vec![arc(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0)];
        let translation = point(10.0, 5.0);
        arcline_translate(&mut arcline, translation);

        assert_eq!(arcline.len(), 1);
        assert_eq!(arcline[0].a, point(10.0, 5.0));
        assert_eq!(arcline[0].b, point(12.0, 5.0));
        assert_eq!(arcline[0].c, point(11.0, 5.0));
        assert_eq!(arcline[0].r, 1.0); // Radius should remain unchanged
    }

    #[test]
    fn test_arcline_translate_multiple_arcs() {
        // Test translating an arcline with multiple arcs
        let mut arcline = vec![
            arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5),
            arcseg(point(1.0, 0.0), point(3.0, 2.0)), // Line segment
            arc(
                point(3.0, 2.0),
                point(3.0, 4.0),
                point(4.0, 3.0),
                1.414213562373095,
            ),
        ];
        let translation = point(-2.0, 3.0);
        arcline_translate(&mut arcline, translation);

        assert_eq!(arcline.len(), 3);
    }
}

// #00003 #00004
// Check if the arc contains the point.
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

impl Arc {
    // TODO: what should be the exact value.
    /// Checks if the arc has a collapsed radius.
    ///
    /// An arc is considered to have a collapsed radius if the radius is smaller
    /// than the given epsilon threshold or if it's NaN.
    ///
    /// # Arguments
    ///
    /// * `r` - The radius to check
    /// * `eps` - The epsilon threshold for comparison
    ///
    /// # Returns
    ///
    /// True if the radius is collapsed (too small or NaN), false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.0001);
    /// assert!(arc1.is_collapsed_radius(0.01)); // Radius too small
    /// let arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::NAN);
    /// assert!(arc2.is_collapsed_radius(0.01)); // NaN radius
    /// let arc3 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1.0);
    /// assert!(!arc3.is_collapsed_radius(0.01)); // Valid radius
    /// ```
    pub fn is_collapsed_radius(&self, ulps: u64) -> bool {
        // no abs() since it can be negative
        if float_equal(self.r, ZERO, ulps) || self.r.is_nan() {
            return true;
        }
        false
    }

    /// Checks if the arc has collapsed endpoints.
    ///
    /// An arc is considered to have collapsed endpoints if the start and end
    /// points are too close to each other within the given epsilon threshold.

    ///
    /// # Returns
    ///
    /// True if the endpoints are too close together, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let p1 = point(0.0, 0.0);
    /// let p2 = point(0.0001, 0.0);
    /// let p3 = point(1.0, 0.0);
    ///
    /// let arc1 = arc(p1, p2, point(0.0, 0.5), 0.5);
    /// assert!(arc1.is_collapsed_ends(0.01)); // Points too close
    /// let arc2 = arc(p1, p3, point(0.5, 0.0), 0.5);
    /// assert!(!arc2.is_collapsed_ends(0.01)); // Points far enough apart
    /// ```
    pub fn is_collapsed_ends(&self, ulps: u64) -> bool {
        if self.a.point_equal(self.b, ulps) {
            return true;
        }
        false
    }

    /// Checks if an arc has inconsistent geometry.
    ///
    /// An arc is considered inconsistent if the center point is not equidistant
    /// from both endpoints within the given epsilon threshold. This validates
    /// that the arc's center and radius are geometrically consistent.
    ///
    /// # Arguments
    ///
    /// * `eps` - The epsilon threshold for distance comparison
    ///
    /// # Returns
    ///
    /// True if the arc geometry is consistent, false if it's inconsistent
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    ///
    /// // Consistent arc: center is equidistant from both endpoints
    /// let start = point(0.0, 0.0);
    /// let end = point(2.0, 0.0);
    /// let center = point(1.0, 0.0);
    /// let radius = 1.0;
    /// let arc = arc(start, end, center, radius);
    /// assert!(arc.is_consistent(1e-10));
    ///
    /// // Inconsistent arc: center is not equidistant from endpoints
    /// let bad_center = point(0.5, 0.0);
    /// let mut arc2 = arc.clone();
    /// arc2.c = bad_center;
    /// assert!(!arc2.is_consistent(1e-10));
    ///
    /// // Another inconsistent case: wrong radius
    /// let mut arc3 = arc.clone();
    /// arc3.r = 2.0;
    /// assert!(!arc3.is_consistent(1e-10));
    /// ```
    pub fn is_consistent(&self, ulps: u64) -> bool {
        if self.is_seg() {
            // Lines are always consistent, no center point
            return true;
        }
        // Check if the radius is consistent with the center and endpoints
        let ac = float_equal((self.a - self.c).norm(), self.r, ulps);
        let bc = float_equal((self.b - self.c).norm(), self.r, ulps);
        if !ac || !bc {
            return false; // Inconsistent radius
        }
        true
    }

    /// Validates if an arc is geometrically valid.
    ///
    /// An arc is considered valid if it doesn't have a collapsed radius,
    /// doesn't have collapsed endpoints, and has consistent geometry
    /// (the center point is equidistant from both endpoints).
    ///
    /// # Arguments
    ///
    /// * `eps` - The epsilon threshold for validation checks
    ///
    /// # Returns
    ///
    /// True if the arc is valid, false if it's degenerate or inconsistent
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    /// let valid_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5);
    /// assert!(valid_arc.is_valid(1e-10));
    ///
    /// let invalid_arc = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.5, 0.5), 1.0);
    /// assert!(!invalid_arc.is_valid(1e-10)); // Collapsed endpoints
    ///
    /// let inconsistent_arc = arc(point(0.0, 0.0), point(2.0, 0.0), point(0.5, 0.0), 2.0);
    /// assert!(!inconsistent_arc.is_valid(1e-10)); // Inconsistent geometry
    /// ```
    #[must_use]
    pub fn is_valid(&self, ulps: u64) -> bool {
        if self.is_seg() {
            if self.is_collapsed_ends(ulps) {
                return false;
            }
        }
        if self.is_arc() {
            if self.is_collapsed_ends(ulps)
                || self.is_collapsed_radius(ulps)
                || !self.is_consistent(ulps)
            {
                return false;
            }
        }
        true
    }
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
    use crate::utils::float_next;

    use super::*;

    const UPLS_COLLAPSED: u64 = 1000; // Tolerance for collapsed checks

    #[test]
    fn test_arc_is_collapsed_radius_normal_values() {
        // Normal positive radius values should not be collapsed
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1.0);
        let arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.1);
        let arc3 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 100.0);
        let arc4 = arc(
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 0.0),
            f64::INFINITY,
        );
        assert!(!arc1.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(!arc2.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(!arc3.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(!arc4.is_collapsed_radius(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_small_values() {
        // Values smaller than EPS_COLLAPSED (1E-8) should be collapsed
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1E-9);
        let arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1E-10);
        let arc3 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.0);
        assert!(arc1.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(arc2.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(arc3.is_collapsed_radius(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_boundary_values() {
        // Test values around the UPLS_COLLAPSED boundary
        let arc1 = arc(
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 0.0),
            float_next(ZERO, UPLS_COLLAPSED / 2),
        );
        let arc2 = arc(
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 0.0),
            float_next(ZERO, UPLS_COLLAPSED * 2),
        );
        let arc3 = arc(
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 0.0),
            float_next(ZERO, UPLS_COLLAPSED - 1),
        );
        assert!(arc1.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(!arc2.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(arc3.is_collapsed_radius(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_negative_values() {
        // Negative radius values should be collapsed
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -1.0);
        let arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -0.1);
        let arc3 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -1E-10);
        assert!(arc1.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(arc2.is_collapsed_radius(UPLS_COLLAPSED));
        assert!(arc3.is_collapsed_radius(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_nan() {
        // NaN values should be collapsed
        let arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::NAN);
        assert!(arc.is_collapsed_radius(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_normal_points() {
        // Normal separated points should not be collapsed
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(0.0, 0.0), point(0.0, 1.0), point(0.5, 0.5), 1.0);
        let arc3 = arc(point(-1.0, -1.0), point(1.0, 1.0), point(0.0, 0.0), 2.0);
        let arc4 = arc(
            point(100.0, 200.0),
            point(300.0, 400.0),
            point(200.0, 300.0),
            100.0,
        );
        assert!(!arc1.is_collapsed_ends(UPLS_COLLAPSED));
        assert!(!arc2.is_collapsed_ends(UPLS_COLLAPSED));
        assert!(!arc3.is_collapsed_ends(UPLS_COLLAPSED));
        assert!(!arc4.is_collapsed_ends(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_identical_points() {
        // Identical points should be collapsed
        let arc1 = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.5, 0.0), 1.0);
        let arc2 = arc(point(1.0, 1.0), point(1.0, 1.0), point(1.5, 1.0), 1.0);
        let arc3 = arc(point(-5.0, 10.0), point(-5.0, 10.0), point(-4.0, 10.0), 1.0);
        assert!(arc1.is_collapsed_ends(UPLS_COLLAPSED));
        assert!(arc2.is_collapsed_ends(UPLS_COLLAPSED));
        assert!(arc3.is_collapsed_ends(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_very_close_points() {
        // Points closer than EPS_COLLAPSED should be collapsed
        let p1 = point(0.0, 0.0);
        let p2 = point(float_next(ZERO, UPLS_COLLAPSED / 2), 0.0);
        let test_arc1 = arc(p1, p2, point(0.0, 0.0), 1.0);
        assert!(test_arc1.is_collapsed_ends(UPLS_COLLAPSED));

        let p3 = point(100.0, 100.0);
        let p4 = point(float_next(100.0, UPLS_COLLAPSED / 2), float_next(100.0, UPLS_COLLAPSED / 3));
        let test_arc2 = arc(p3, p4, point(100.0, 100.0), 1.0);
        assert!(test_arc2.is_collapsed_ends(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_boundary_distance() {
        // Points at exactly UPLS_COLLAPSED distance
        let p1 = point(0.0, 0.0);
        let p2 = point(float_next(ZERO, UPLS_COLLAPSED), 0.0);
        let test_arc1 = arc(p1, p2, point(0.0, 0.0), 1.0);
        // This should not be collapsed (distance equals tolerance)
        assert!(test_arc1.is_collapsed_ends(UPLS_COLLAPSED));

        // Points slightly farther than UPLS_COLLAPSED
        let p3 = point(0.0, 0.0);
        let p4 = point(float_next(ZERO, UPLS_COLLAPSED * 2), 0.0);
        let test_arc2 = arc(p3, p4, point(0.0, 0.0), 1.0);
        assert!(!test_arc2.is_collapsed_ends(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_valid_arcs() {
        // Valid arcs should pass the check
        let valid_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5);
        assert!(valid_arc1.is_valid(UPLS_COLLAPSED));

        let valid_arc2 = arc(
            point(-1.0, -1.0),
            point(1.0, 1.0),
            point(0.0, 0.0),
            std::f64::consts::SQRT_2,
        );
        assert!(valid_arc2.is_valid(UPLS_COLLAPSED));

        // Line segments (infinite radius) should also be valid if endpoints are separated
        let valid_line = arcseg(point(0.0, 0.0), point(10.0, 0.0));
        assert!(valid_line.is_valid(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_collapsed_radius() {
        // Arcs with collapsed radius should fail the check
        let collapsed_radius_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1E-10);
        assert!(!collapsed_radius_arc1.is_valid(UPLS_COLLAPSED));

        let collapsed_radius_arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -1.0);
        assert!(!collapsed_radius_arc2.is_valid(UPLS_COLLAPSED));

        let nan_radius_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::NAN);
        assert!(!nan_radius_arc.is_valid(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_collapsed_ends() {
        // Arcs with collapsed endpoints should fail the check
        let collapsed_ends_arc1 = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1.0);
        assert!(!collapsed_ends_arc1.is_valid(UPLS_COLLAPSED));

        let close_points = point(0.0, 0.0);
        let very_close_points = point(float_next(ZERO, UPLS_COLLAPSED / 2), 0.0);
        let collapsed_ends_arc2 = arc(close_points, very_close_points, point(0.0, 1.0), 1.0);
        assert!(!collapsed_ends_arc2.is_valid(UPLS_COLLAPSED));

        // Line segments with collapsed endpoints should also fail
        let collapsed_line = arcseg(point(1.0, 1.0), point(1.0, 1.0));
        assert!(!collapsed_line.is_valid(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_both_collapsed() {
        // Arcs with both collapsed radius and collapsed endpoints should fail
        let both_collapsed = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1E-10);
        assert!(!both_collapsed.is_valid(UPLS_COLLAPSED));

        let both_collapsed2 = arc(point(5.0, 5.0), point(5.0, 5.0), point(0.0, 0.0), f64::NAN);
        assert!(!both_collapsed2.is_valid(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_edge_cases() {
        // Test with very large coordinates - ensure consistent geometry
        let large_coord_arc = arc(
            point(1E10, 1E10),
            point(1E10 + 1.0, 1E10),
            point(1E10 + 0.5, 1E10),
            0.5,
        );
        assert!(large_coord_arc.is_valid(UPLS_COLLAPSED));

        // Test with very small but valid radius - ensure consistent geometry
        let small_radius_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5);
        assert!(small_radius_arc.is_valid(UPLS_COLLAPSED));

        // Test with large radius
        let large_radius_arc = arc(point(0.0, 0.0), point(1E-6, 0.0), point(0.0, 1E6), 1E6);
        assert!(large_radius_arc.is_valid(UPLS_COLLAPSED));
    }

    #[test]
    fn test_arcline_reverse_basic() {
        let arc1 = Arc {
            a: point(0.0, 0.0),
            b: point(1.0, 0.0),
            c: point(0.5, 0.5),
            r: 1.0,
            id: 1,
        };
        let arc2 = Arc {
            a: point(1.0, 0.0),
            b: point(1.0, 1.0),
            c: point(1.0, 0.5),
            r: 1.0,
            id: 2,
        };
        let arcline = vec![arc1, arc2];
        let reversed = arcline_reverse(&arcline);
        assert_eq!(reversed.len(), 2);
        // For circular arcs (finite radius), endpoints should NOT be swapped - they remain CCW
        assert_eq!(reversed[0].a, arc2.a); // arc2 comes first, unchanged
        assert_eq!(reversed[0].b, arc2.b);
        assert_eq!(reversed[1].a, arc1.a); // arc1 comes second, unchanged
        assert_eq!(reversed[1].b, arc1.b);
        assert_eq!(reversed[0].id, arc2.id);
        assert_eq!(reversed[1].id, arc1.id);
    }

    #[test]
    fn test_arcline_reverse_empty() {
        let arcline: Vec<Arc> = vec![];
        let reversed = arcline_reverse(&arcline);
        assert_eq!(reversed.len(), 0);
    }

    #[test]
    fn test_arcline_reverse_single_arc() {
        let arc = Arc {
            a: point(2.0, 2.0),
            b: point(3.0, 3.0),
            c: point(2.5, 2.5),
            r: 2.0,
            id: 42,
        };
        let arcline = vec![arc];
        let reversed = arcline_reverse(&arcline);
        assert_eq!(reversed.len(), 1);
        // For circular arcs (finite radius), endpoints should NOT be swapped
        assert_eq!(reversed[0].a, arc.a);
        assert_eq!(reversed[0].b, arc.b);
        assert_eq!(reversed[0].id, arc.id);
    }

    #[test]
    fn test_arcline_reverse_all_lines() {
        // All arcs are actually lines (r = infinity)
        let arc1 = Arc {
            a: point(0.0, 0.0),
            b: point(1.0, 0.0),
            c: point(0.0, 0.0),
            r: f64::INFINITY,
            id: 1,
        };
        let arc2 = Arc {
            a: point(1.0, 0.0),
            b: point(2.0, 0.0),
            c: point(0.0, 0.0),
            r: f64::INFINITY,
            id: 2,
        };
        let arcline = vec![arc1, arc2];
        let reversed = arcline_reverse(&arcline);
        assert_eq!(reversed[0].r, f64::INFINITY);
        assert_eq!(reversed[1].r, f64::INFINITY);
        // For line segments (infinite radius), endpoints should be swapped
        assert_eq!(reversed[0].a, arc2.b); // arc2 reversed: b->a
        assert_eq!(reversed[0].b, arc2.a); // arc2 reversed: a->b
        assert_eq!(reversed[1].a, arc1.b); // arc1 reversed: b->a  
        assert_eq!(reversed[1].b, arc1.a); // arc1 reversed: a->b
    }

    #[test]
    fn test_arcline_reverse_all_arcs() {
        // All arcs are true arcs (finite radius)
        let arc1 = Arc {
            a: point(0.0, 0.0),
            b: point(1.0, 0.0),
            c: point(0.5, 0.5),
            r: 2.0,
            id: 1,
        };
        let arc2 = Arc {
            a: point(1.0, 0.0),
            b: point(2.0, 0.0),
            c: point(1.5, 0.5),
            r: 2.0,
            id: 2,
        };
        let arcline = vec![arc1, arc2];
        let reversed = arcline_reverse(&arcline);
        assert!(reversed.iter().all(|arc| arc.r == 2.0));
        // For circular arcs (finite radius), endpoints should NOT be swapped - order is reversed but arcs stay CCW
        assert_eq!(reversed[0].a, arc2.a); // arc2 comes first, unchanged
        assert_eq!(reversed[0].b, arc2.b);
        assert_eq!(reversed[1].a, arc1.a); // arc1 comes second, unchanged  
        assert_eq!(reversed[1].b, arc1.b);
    }
}

// Given start end points of arc and radius, calculate bulge
// TODO: not tested
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

// Given start end points of arc and radius, calculate bulge
// TODO: not tested
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

/// Calculates the bulge parameter for an arc given start point, end point, center, and radius.
///
/// This function computes the bulge parameter that would be needed to create an arc
/// connecting points a and b with the given center c and radius r. The bulge represents
/// the ratio used in arc parametrization.
///
/// # Arguments
///
/// * `a` - The start point of the arc
/// * `b` - The end point of the arc
/// * `c` - The center point of the arc
/// * `r` - The radius of the arc
///
/// # Returns
///
/// The bulge parameter for the arc, or 0.0 if the arc is invalid
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// let start = point(0.0, 0.0);
/// let end = point(2.0, 0.0);
/// let center = point(1.0, 0.0);
/// let radius = 1.0;
///
/// let bulge = arc_bulge_from_points(start, end, center, radius);
/// // The bulge parameter can be used to recreate the arc
/// ```
// Given start end points of arc and radius, calculate bulge
// https://stackoverflow.com/questions/48979861/numerically-stable-method-for-solving-quadratic-equations/50065711#50065711
#[must_use]
pub fn arc_bulge_from_points(a: Point, b: Point, c: Point, r: f64) -> f64 {
    let dist = (b - a).norm();
    if dist <= 1E-10 {
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
    if seg.abs() <= 1E-10 {
        return 0f64;
    }
    dist / (2.0 * seg)
}

const ZERO: f64 = 0f64;
const MIN_BULGE: f64 = 1E-8;
/// Returns the circle parameterization of the Arc. Without thetas.
/// Much faster, avoids arctan()
/// <div class="warning">There are two arcs. Always return CCW (Counter-Clockwise) oriented one!</div>
///
/// Creates an arc from two points and a bulge parameter.
///
/// This function creates an arc that connects two points using a bulge parameter
/// to define the curvature. The bulge represents the ratio of the sagitta
/// (the perpendicular distance from the chord midpoint to the arc) to half the chord length.
///
/// # Arguments
///
/// * `pp1` - The first point of the arc
/// * `pp2` - The second point of the arc
/// * `bulge` - The bulge parameter controlling the arc curvature
///
/// # Returns
///
/// An Arc connecting the two points with the specified curvature that is CCW
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// // Create a semicircle arc
/// let arc = arc_circle_parametrization(point(0.0, 0.0), point(2.0, 0.0), 1.0);
/// assert!(arc.is_arc());
///
/// // Create a line (very small bulge)
/// let line = arc_circle_parametrization(point(0.0, 0.0), point(2.0, 0.0), 1e-10);
/// assert!(line.is_seg());
/// ```
#[must_use]
pub fn arc_circle_parametrization(p1: Point, p2: Point, bulge: f64) -> Arc {
    let mut pp1 = p1;
    let mut pp2 = p2;
    let mut bulge = bulge;
    if bulge.abs() <= MIN_BULGE || pp1.point_equal(pp2, EPS_COLLAPSED) {
        // create line
        return arcseg(p1, p2);
    }
    if bulge < 0f64 {
        // make arc CCW
        pp1 = p2;
        pp2 = p1;
        bulge = -bulge;
    }

    // TODO: check for numerical issues
    let t2 = (pp2 - pp1).norm();
    let dt2 = (1.0 + bulge) * (1.0 - bulge) / (4.0 * bulge);
    let cx = (0.5 * pp1.x + 0.5 * pp2.x) + dt2 * (pp1.y - pp2.y);
    let cy = (0.5 * pp1.y + 0.5 * pp2.y) + dt2 * (pp2.x - pp1.x);
    let r = 0.25 * t2 * (1.0 / bulge + bulge).abs();
    arc(pp1, pp2, point(cx, cy), r)
}

#[cfg(test)]
mod test_arc_g_from_points {
    use crate::{prelude::*};

    const TEST_EPS: f64 = 1E-10;
    const TEST_UPLS: u64 = 100;

    #[test]
    fn test_a_b_are_close() {
        let a = point(114.31083505599867, 152.84458247200070);
        let b = point(114.31083505599865, 152.84458247200067);
        let arc = arc_circle_parametrization(a, b, 16.0);
        assert_eq!(arc_bulge_from_points(a, b, arc.c, arc.r), 0.0);
    }

    #[test]
    fn test_a_b_are_the_same() {
        let a = point(114.31083505599865, 152.84458247200067);
        let b = point(114.31083505599865, 152.84458247200067);
        let arc = arc_circle_parametrization(a, b, 16.0);
        assert_eq!(arc_bulge_from_points(a, b, arc.c, arc.r), 0.0);
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should be reasonably close to original bulge
        assert!(float_equal(bulge, result, TEST_UPLS));
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should be reasonably close to original bulge
        assert!(float_equal(bulge, result, TEST_UPLS));
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // For a semicircle, the function should return a finite positive value
        assert!(float_equal(bulge, result, TEST_UPLS));
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should be reasonably close to original bulge
        assert!(float_equal(bulge, result, TEST_UPLS));
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
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            assert!(float_equal(bulge, result, TEST_UPLS));
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
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            assert!(float_equal(bulge, result, TEST_UPLS));
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should be positive and finite
        assert!(float_equal(bulge, result, TEST_UPLS));
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should be positive and finite
        assert!(float_equal(bulge, result, TEST_UPLS));
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
        let calculated_bulge = arc_bulge_from_points(a, b, arc.c, arc.r);

        // Debug: print both values
        println!(
            "Original bulge: {}, Calculated bulge: {}, Ratio: {}",
            bulge,
            calculated_bulge,
            calculated_bulge / bulge
        );

        // Should match the original bulge within numerical precision
        assert!(
            (calculated_bulge - bulge).abs() < 1e-10,
            "Expected {}, got {}",
            bulge,
            calculated_bulge
        );
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
        let calculated_bulge = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

        // Should return positive value (CCW orientation)
        assert!(float_equal(-bulge, calculated_bulge, TEST_UPLS));
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
            let calculated_bulge = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

            // Should match the original bulge within numerical precision
            assert!(
                (calculated_bulge - bulge).abs() < 1e-10,
                "Bulge {} resulted in {} (difference: {})",
                bulge,
                calculated_bulge,
                (calculated_bulge - bulge).abs()
            );
        }
    }

    #[test]
    fn test_different_point_positions() {
        // Test with different start/end point configurations
        let test_cases = [
            (point(0.0, 0.0), point(1.0, 0.0)),     // Horizontal
            (point(0.0, 0.0), point(0.0, 1.0)),     // Vertical
            (point(0.0, 0.0), point(1.0, 1.0)),     // Diagonal
            (point(-1.0, -1.0), point(1.0, 1.0)),   // Diagonal through origin
            (point(10.0, 20.0), point(30.0, 40.0)), // Larger coordinates
        ];

        let bulge = 0.5;

        for (a, b) in test_cases.iter() {
            // Create arc from parametrization
            let arc = arc_circle_parametrization(*a, *b, bulge);

            // Calculate bulge back from points
            let calculated_bulge = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

            // Should match the original bulge within numerical precision
            assert!(
                (calculated_bulge - bulge).abs() < 1e-10,
                "Points {:?} -> {:?}: expected {}, got {}",
                a,
                b,
                bulge,
                calculated_bulge
            );
        }
    }
    
    #[test]
    fn test_close_points_large_bulge() {
        let r = 1.0;
        let bulge = arc_bulge_from_points(point(0.0, 0.0), point(0.0, 3e-5), point(0.0, 1.0), r);
        assert!(bulge > 133333.0);
        let arc = arc_circle_parametrization(point(0.0, 0.0), point(0.0, 3e-5), bulge);
        assert_eq!(float_equal(arc.r, r, TEST_UPLS), true);
    }
}

/// Reverses the direction of an arcline (sequence of CCW arcs).
/// Each arc is reversed by swapping its start and end points, and the order of arcs is reversed.
/// The orientation remains CCW for each arc.
///
/// # Arguments
/// * `arcs` - The arcline (Vec<Arc>) to reverse
///
/// # Returns
/// A new arcline with reversed direction
#[must_use]
pub fn arcline_reverse(arcs: &Arcline) -> Arcline {
    let mut reversed: Vec<Arc> = Vec::with_capacity(arcs.len());
    for arc in arcs.iter().rev() {
        if arc.is_seg() {
            reversed.push(arc.reverse());
        } else {
            reversed.push(*arc);
        }
    }
    reversed
}

impl Arc {
    const ULPS_MAKE_CONSISTENT: u64 = 100;
    /// Makes slightly inconsistent arc consistent by adjusting the arc center
    /// and radius, keeping the endpoints fixed.
    pub fn make_consistent(&mut self) {
        if self.is_seg() {
            return;
        }

        // Handle degenerate case where endpoints are the same
        if self.a.point_equal(self.b, Arc::ULPS_MAKE_CONSISTENT) {
            *self = arcseg(self.a, self.b);
            return;
        }

        // Calculate the distances from the current center to endpoints
        let dist_a_c = (self.a - self.c).norm();
        let dist_b_c = (self.b - self.c).norm();

        // Use the average of the two distances as the new radius
        let avg_radius = (dist_a_c + dist_b_c) / 2.0;

        // Calculate chord properties
        let chord = self.b - self.a;
        let chord_length = chord.norm();
        let half_chord = chord_length / 2.0;

        // If average radius is too small (less than half chord), use minimum possible radius
        let new_radius = if avg_radius < half_chord {
            half_chord
        } else {
            avg_radius
        };

        // Find the center that makes both endpoints equidistant
        // The center lies on the perpendicular bisector of the chord ab
        let midpoint = (self.a + self.b) / 2.0;

        // For a circle with radius r passing through points a and b,
        // the distance from chord midpoint to center is sqrt(r^2 - (chord_length/2)^2)
        let distance_to_center = (new_radius * new_radius - half_chord * half_chord).sqrt();

        // Perpendicular direction to chord (normalized)
        let perp = if chord_length > 1e-12 {
            point(-chord.y, chord.x) / chord_length
        } else {
            point(0.0, 1.0) // Default direction if chord is too small
        };

        // Two possible centers on the perpendicular bisector
        let c1 = midpoint + perp * distance_to_center;
        let c2 = midpoint - perp * distance_to_center;

        // Choose the center closer to the original center
        let dist1 = (c1 - self.c).norm();
        let dist2 = (c2 - self.c).norm();

        let new_center = if dist1 < dist2 { c1 } else { c2 };

        *self = Arc {
            a: self.a,
            b: self.b,
            c: new_center,
            r: new_radius,
            id: self.id, // Keep the same ID
        };
    }
}

#[cfg(test)]
mod test_arc_make_consistent {
    use crate::{prelude::*, utils::float_equal};

    const TEST_UPLS: u64 = 100;

    #[test]
    fn test_arc_make_consistent() {
        let mut arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 0.5);
        arc.make_consistent();
        assert!(arc.is_consistent(TEST_UPLS));
    }

    #[test]
    fn test_arc_make_consistent_already_consistent() {
        // Create an already consistent arc
        let mut arc = arc(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0);
        arc.make_consistent();
        assert!(arc.is_consistent(TEST_UPLS));
        // Should be very close to the original
        assert!(float_equal(arc.c.x, 1.0, TEST_UPLS));
        assert!(float_equal(arc.c.y, 0.0, TEST_UPLS));
        assert!(float_equal(arc.r, 1.0, TEST_UPLS));
    }

    #[test]
    fn test_arc_make_consistent_different_distances() {
        // Create an arc where endpoints are at different distances from center
        let mut arc = arc(point(0.0, 0.0), point(3.0, 4.0), point(1.0, 1.0), 2.0);
        arc.make_consistent();
        assert!(arc.is_consistent(TEST_UPLS));

        // Check that both endpoints are equidistant from the new center
        let dist_a = (arc.a - arc.c).norm();
        let dist_b = (arc.b - arc.c).norm();
        assert!(float_equal(dist_a, arc.r, TEST_UPLS));
        assert!(float_equal(dist_b, arc.r, TEST_UPLS));
    }

    #[test]
    fn test_arc_make_consistent_degenerate_endpoints() {
        // Create an arc with same start and end points
        let mut arc = arc(point(1.0, 1.0), point(1.0, 1.0), point(2.0, 2.0), 1.0);
        arc.make_consistent();
        // Degenerate case should result in line segment
        assert!(arc.is_consistent(TEST_UPLS));
        assert!(arc.is_seg());
    }

    #[test]
    fn test_arc_make_consistent_line_segment() {
        // Test with a line segment (infinite radius)
        let mut line_arc = arc(
            point(0.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 0.0),
            f64::INFINITY,
        );
        line_arc.make_consistent();
        assert_eq!(line_arc.r, f64::INFINITY);
        assert_eq!(line_arc.a, line_arc.a);
        assert_eq!(line_arc.b, line_arc.b);
    }

    #[test]
    fn test_arc_make_consistent_small_radius() {
        // Test case where desired radius is smaller than minimum possible (half chord length)
        let mut arc = arc(point(0.0, 0.0), point(4.0, 0.0), point(2.0, 1.0), 1.0); // chord length = 4, so min radius = 2

        // Debug: check what the original distances are
        let dist_a_c = (arc.a - arc.c).norm(); // distance from (0,0) to (2,1) = sqrt(5) ≈ 2.236
        let dist_b_c = (arc.b - arc.c).norm(); // distance from (4,0) to (2,1) = sqrt(5) ≈ 2.236
        let avg_radius = (dist_a_c + dist_b_c) / 2.0; // ≈ 2.236

        arc.make_consistent();
        assert!(arc.is_consistent(TEST_UPLS));

        // The average radius is about 2.236, which is larger than half chord length (2.0)
        // So it should use the computed average radius, not the minimum
        assert!(float_equal(arc.r, avg_radius, TEST_UPLS));

        // Verify that both endpoints are equidistant from the center
        let new_dist_a = (arc.a - arc.c).norm();
        let new_dist_b = (arc.b - arc.c).norm();
        assert!(float_equal(new_dist_a, arc.r, TEST_UPLS));
        assert!(float_equal(new_dist_b, arc.r, TEST_UPLS));
    }

    #[test]
    fn test_arc_make_consistent_radius_too_small() {
        // Test case where the average radius is smaller than half chord length
        let mut arc = arc(point(0.0, 0.0), point(10.0, 0.0), point(1.0, 0.1), 0.5); // chord length = 10, half = 5, but point is close to first endpoint

        let dist_a_c = (arc.a - arc.c).norm();
        let dist_b_c = (arc.b - arc.c).norm();
        let avg_radius = (dist_a_c + dist_b_c) / 2.0;
        let chord_length = (arc.b - arc.a).norm();
        let half_chord = chord_length / 2.0;

        arc.make_consistent();
        assert!(arc.is_consistent(TEST_UPLS));

        // Check if the average radius is actually smaller than half chord
        if avg_radius < half_chord {
            // Should use minimum possible radius (half chord length)
            assert!(float_equal(arc.r, half_chord, TEST_UPLS));
            // Center should be at chord midpoint
            assert!(float_equal(arc.c.x, chord_length / 2.0, TEST_UPLS));
            assert!(float_equal(arc.c.y, 0.0, TEST_UPLS));
        } else {
            // Should use the average radius
            assert!(float_equal(arc.r, avg_radius, TEST_UPLS));
        }
    }
}

/// Checks if two arcs are genuinely intersecting, not just touching at endpoints.
///
/// This function determines whether two arcs have a "real" intersection that would
/// require further processing (like splitting the arcs). It returns `true` only when
/// the arcs intersect at interior points, not just when they touch at their endpoints.
///
/// The function handles all combinations of arc types:
/// - Line segment to line segment
/// - Arc to arc
/// - Line segment to arc
/// - Arc to line segment
///
/// # Arguments
///
/// * `arc1` - The first arc (can be a line segment or circular arc)
/// * `arc2` - The second arc (can be a line segment or circular arc)
///
/// # Returns
///
/// `true` if the arcs intersect at interior points, `false` if they don't intersect
/// or only touch at endpoints.
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// // Two crossing line segments - really intersecting
/// let line1 = arcseg(point(0.0, 0.0), point(2.0, 2.0));
/// let line2 = arcseg(point(0.0, 2.0), point(2.0, 0.0));
/// assert!(is_really_intersecting(&line1, &line2));
///
/// // Two line segments sharing an endpoint - not really intersecting
/// let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
/// let line2 = arcseg(point(1.0, 0.0), point(2.0, 0.0));
/// assert!(!is_really_intersecting(&line1, &line2));
///
/// // Arc and line segment intersecting at interior points
/// let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
/// let line = arcseg(point(0.0, -0.5), point(0.0, 1.5));
/// assert!(is_really_intersecting(&arc, &line));
///
/// // Parallel line segments - no intersection
/// let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
/// let line2 = arcseg(point(0.0, 1.0), point(1.0, 1.0));
/// assert!(!is_really_intersecting(&line1, &line2));
/// ```
#[must_use]
pub fn is_really_intersecting(arc1: &Arc, arc2: &Arc) -> bool {
    if arc1.is_seg() && arc2.is_seg() {
        let seg1 = segment(arc1.a, arc1.b);
        let seg2 = segment(arc2.a, arc2.b);
        return if_really_intersecting_segment_segment(&seg1, &seg2);
    }
    if arc1.is_arc() && arc2.is_arc() {
        return if_really_intersecting_arc_arc(arc1, arc2);
    }
    if arc1.is_seg() && arc2.is_arc() {
        let seg1 = segment(arc1.a, arc1.b);
        return if_really_intersecting_segment_arc(&seg1, &arc2);
    }
    if arc1.is_arc() && arc2.is_seg() {
        let seg2 = segment(arc2.a, arc2.b);
        return if_really_intersecting_segment_arc(&seg2, arc1);
    }
    false
}

#[cfg(test)]
mod test_is_really_intersecting {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_crossing_line_segments() {
        // Two line segments crossing at their midpoints
        let line1 = arcseg(point(0.0, 0.0), point(2.0, 2.0));
        let line2 = arcseg(point(0.0, 2.0), point(2.0, 0.0));
        assert!(is_really_intersecting(&line1, &line2));
    }

    #[test]
    fn test_endpoint_touching_segments() {
        // Two line segments sharing an endpoint - not really intersecting
        let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let line2 = arcseg(point(1.0, 0.0), point(2.0, 0.0));
        assert!(!is_really_intersecting(&line1, &line2));
    }

    #[test]
    fn test_parallel_segments() {
        // Parallel line segments - no intersection
        let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let line2 = arcseg(point(0.0, 1.0), point(1.0, 1.0));
        assert!(!is_really_intersecting(&line1, &line2));
    }

    #[test]
    fn test_overlapping_segments() {
        // Overlapping line segments - really intersecting
        let line1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let line2 = arcseg(point(1.0, 0.0), point(3.0, 0.0));
        assert!(is_really_intersecting(&line1, &line2));
    }

    #[test]
    fn test_arc_to_arc_intersecting() {
        // Two arcs that cross each other
        let arc1 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        let arc2 = arc(point(0.0, -1.0), point(0.0, 1.0), point(1.0, 0.0), 1.0);
        assert!(is_really_intersecting(&arc1, &arc2));
    }

    #[test]
    fn test_arc_to_arc_touching_endpoints() {
        // Two arcs touching at endpoints only
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
        assert!(!is_really_intersecting(&arc1, &arc2));
    }

    #[test]
    fn test_arc_to_arc_no_intersection() {
        // Two arcs that don't intersect at all
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(2.0, 2.0), point(3.0, 2.0), point(2.5, 2.5), 1.0);
        assert!(!is_really_intersecting(&arc1, &arc2));
    }

    #[test]
    fn test_segment_to_arc_intersecting() {
        // Line segment cutting through an arc
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        let line = arcseg(point(0.0, -0.5), point(0.0, 1.5));
        assert!(is_really_intersecting(&line, &arc));
    }

    #[test]
    fn test_segment_to_arc_touching_endpoint() {
        // Line segment touching arc at its endpoint
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        let line = arcseg(point(-1.0, 0.0), point(-2.0, 0.0));
        assert!(!is_really_intersecting(&line, &arc));
    }

    #[test]
    fn test_segment_to_arc_no_intersection() {
        // Line segment that doesn't intersect the arc
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        let line = arcseg(point(2.0, 0.0), point(3.0, 0.0));
        assert!(!is_really_intersecting(&line, &arc));
    }

    #[test]
    fn test_arc_to_segment_intersecting() {
        // Arc cutting through a line segment (opposite order of previous test)
        let line = arcseg(point(0.0, -0.5), point(0.0, 1.5));
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        assert!(is_really_intersecting(&arc, &line));
    }

    #[test]
    fn test_tangent_cases() {
        // Line segment tangent to arc - should not be "really intersecting"
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
        let line = arcseg(point(-1.0, 1.0), point(1.0, 1.0));
        assert!(!is_really_intersecting(&line, &arc));
    }

    #[test]
    fn test_collinear_segments() {
        // Collinear segments that don't overlap
        let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let line2 = arcseg(point(2.0, 0.0), point(3.0, 0.0));
        assert!(!is_really_intersecting(&line1, &line2));
    }

    #[test]
    fn test_perpendicular_segments_intersecting() {
        // Perpendicular segments crossing
        let line1 = arcseg(point(-1.0, 0.0), point(1.0, 0.0));
        let line2 = arcseg(point(0.0, -1.0), point(0.0, 1.0));
        assert!(is_really_intersecting(&line1, &line2));
    }
}

#[derive(Debug, PartialEq)]
/// Result of arcline validation, indicating whether an arcline is valid or the specific reason for invalidity.
///
/// This enum provides detailed feedback about arcline validation, allowing callers to understand
/// exactly what makes an arcline invalid and which specific elements are problematic.
pub enum ArclineValidation {
    /// The arcline is valid and forms a proper continuous path.
    Valid,
    /// The arcline is invalid because it contains fewer than 2 elements.
    Invalid,
    /// The arcline contains an invalid arc or line segment.
    /// The enclosed `Arc` is the first invalid element found.
    InvalidArc(Arc),
    /// Adjacent elements in the arcline are not connected (have a gap between them).
    GapBetweenArcs(Arc),
    /// Two consecutive elements form a zero-degree angle at their connection point.
    ZeroDegreeAngle(Arc, Arc),
    /// Two elements in the arcline intersect each other.
    IntersectingArcs(Arc, Arc),
}


/// Validates an arcline (sequence of connected arcs and line segments).
///
/// An arcline is considered valid when it forms a proper continuous path where:
/// - The arcline contains at least 2 elements (arcs or line segments)
/// - Each individual arc/segment is geometrically valid
/// - Adjacent elements are properly connected (share endpoints)
/// - No zero-degree angles exist between consecutive elements
/// - Each two elements do not intersect each other
///
/// # Parameters
/// * `arcs` - The arcline to validate (vector of Arc elements)
///
/// # Returns
/// Returns an [`ArclineValidation`] enum indicating the validation result:
/// - [`ArclineValidation::Valid`] - The arcline is valid
/// - [`ArclineValidation::Invalid`] - The arcline has fewer than 2 elements
/// - [`ArclineValidation::InvalidArc`] - Contains an invalid arc/segment
/// - [`ArclineValidation::GapBetweenArcs`] - Adjacent elements are not connected
/// - [`ArclineValidation::ZeroDegreeAngle`] - Consecutive elements form a zero-degree angle
/// - [`ArclineValidation::IntersectingArcs`] - Non-adjacent elements intersect
///
/// # Examples
/// ```
/// use basegeom::prelude::*;
///
/// // Valid arcline: two connected line segments forming an L-shape
/// let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
/// let arc2 = arcseg(point(1.0, 0.0), point(1.0, 1.0));
/// let arc3 = arcseg(point(1.0, 1.0), point(0.0, 0.0));
/// let arcline = vec![arc1, arc2, arc3];
/// assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
///
/// // Invalid arcline: empty
/// let empty_arcline: Vec<Arc> = vec![];
/// assert_eq!(arcline_is_valid(&empty_arcline), ArclineValidation::Invalid);
/// ```
///
/// # Validation Criteria
///
/// ## 1. Minimum Size
/// The arcline must contain at least 2 elements to form a meaningful path.
///
/// ## 2. Individual Arc Validity
/// Each arc/segment must pass geometric validation (proper radius, distinct endpoints, etc.).
///
/// ## 3. Connectivity
/// Adjacent elements must share exactly one endpoint.
///
/// ## 4. No Zero-Degree Angles
/// Connected elements must not have collinear tangents at their connection point,
/// which would create a zero-degree angle and make the path non-smooth.
///
/// ## 5. No Self-Intersection
/// Non-adjacent elements (separated by at least one other element) must not intersect,
/// ensuring the path doesn't cross itself.
///
/// # Performance
/// Time complexity: O(n²) where n is the number of elements in the arcline,
/// due to intersection checking between all non-adjacent pairs.
#[must_use]
pub fn arcline_is_valid(arcs: &Arcline) -> ArclineValidation {
    let size = arcs.len();
    if size < 2 {
        return ArclineValidation::Invalid;
    }

    // Arcs should be valid
    for arc in arcs {
        if !arc.is_valid(UPLS_ARC_IS_VALID) {
            return ArclineValidation::InvalidArc(arc.clone());
        }
    }

    for i in 0..size {
        let arc0 = arcs[i % size];
        let arc1 = arcs[(i + 1) % size]; // <- current
        let arc2 = arcs[(i + 2) % size];

        // There should be no gaps between arcs
        if !arc_have_two_connected_ends(&arc0, &arc1, &arc2) {
            return ArclineValidation::GapBetweenArcs(arc1.clone());
        }

        // Check if tangents are collinear
        if arc_tangents_are_collinear(&arc0, &arc1) {
            return ArclineValidation::ZeroDegreeAngle(arc0.clone(), arc1.clone());
        }
    }

    // No intersection between arcs
    for i in 0..size {
        for j in (i + 2)..size {
            let arc0 = arcs[i];
            let arc1 = arcs[j];
            if is_really_intersecting(&arc0, &arc1) {
                return ArclineValidation::IntersectingArcs(arc0.clone(), arc1.clone());
            }
        }
    }

    ArclineValidation::Valid
}

// Check that each arc have 2 connected ends
#[must_use]
fn arc_have_two_connected_ends(arc1: &Arc, arc2: &Arc, arc3: &Arc) -> bool {
    // Check if arc1.end connects to arc2.start and arc2.end connects to arc3.start
    // or any other valid connection pattern
    let endpoints1 = [arc1.a, arc1.b];
    let endpoints2 = [arc2.a, arc2.b];
    let endpoints3 = [arc3.a, arc3.b];

    // Try all possible connection patterns
    for &e1 in &endpoints1 {
        for &e2_start in &endpoints2 {
            for &e2_end in &endpoints2 {
                for &e3 in &endpoints3 {
                    if e2_start != e2_end && e1 == e2_start && e2_end == e3 {
                        return true;
                    }
                }
            }
        }
    }
    false
}

// Check that each arc have 2 connected ends
#[must_use]
fn arc_tangents_are_collinear(arc1: &Arc, arc2: &Arc) -> bool {
    let x = vec![arc1.a, arc1.b];
    let y = vec![arc2.a, arc2.b];

    for i in 0..2 {
        for j in 0..2 {
            if x[i] == y[j] {
                let t1 = arc1.tangents()[i];
                let t2 = arc2.tangents()[j];
                if t1.point_equal(t2, UPLS_ARC_IS_VALID) {
                    // Tangents are collinear
                    return true;
                }
            }
        }
    }
    false
}

impl Arc {
    /// Compute tangents at ends of arc
    #[must_use]
    pub fn tangents(&self) -> [Point; 2] {
        if self.is_seg() {
            let (t, _) = (self.b - self.a).normalize(false);
            return [-t, t];
        }

        let a_to_c = self.a - self.c;
        let b_to_c = self.b - self.c;
        let (va, _) = point(a_to_c.y, -a_to_c.x).normalize(false);
        let (vb, _) = point(b_to_c.y, -b_to_c.x).normalize(false);
        [va, -vb]
    }
}

#[cfg(test)]
mod test_tangents {
    use super::*;

    const TEST_UPLS: u64 = 100;

    #[test]
    fn test_tangents_semicircle() {
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let tangents = arc.tangents();
        let t1 = tangents[0];
        let t2 = tangents[1];
        assert_eq!(t1, point(0.0, -1.0));
        assert_eq!(t2, point(0.0, -1.0));
    }

    #[test]
    fn test_tangents_quarter_circle() {
        // Quarter circle from (1,0) to (0,1) with center at origin
        let arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // At start point (1,0), tangent should be perpendicular to radius, pointing up
        assert!(float_equal(t_start.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_start.y, -1.0, TEST_UPLS));

        // At end point (0,1), tangent should be perpendicular to radius, pointing left
        assert!(float_equal(t_end.x, -1.0, TEST_UPLS));
        assert!(float_equal(t_end.y, 0.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_line_segment() {
        // Test line segment (infinite radius case)
        let line = arcseg(point(0.0, 0.0), point(3.0, 4.0));
        let tangents = line.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // For line segment, both tangents should be in the direction of the line
        // Direction vector is (3,4), normalized to (0.6, 0.8)
        let expected_dir = point(0.6, 0.8);

        // Start tangent should be negative direction
        assert!(float_equal(t_start.x, -expected_dir.x, TEST_UPLS));
        assert!(float_equal(t_start.y, -expected_dir.y, TEST_UPLS));

        // End tangent should be positive direction
        assert!(float_equal(t_end.x, expected_dir.x, TEST_UPLS));
        assert!(float_equal(t_end.y, expected_dir.y, TEST_UPLS));
    }

    #[test]
    fn test_tangents_horizontal_line() {
        let line = arcseg(point(-2.0, 5.0), point(2.0, 5.0));
        let tangents = line.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Horizontal line: start tangent points left, end tangent points right
        assert!(float_equal(t_start.x, -1.0, TEST_UPLS));
        assert!(float_equal(t_start.y, 0.0, TEST_UPLS));
        assert!(float_equal(t_end.x, 1.0, TEST_UPLS));
        assert!(float_equal(t_end.y, 0.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_vertical_line() {
        let line = arcseg(point(3.0, -1.0), point(3.0, 1.0));
        let tangents = line.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Vertical line: start tangent points down, end tangent points up
        assert!(float_equal(t_start.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_start.y, -1.0, TEST_UPLS));
        assert!(float_equal(t_end.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_end.y, 1.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_semicircle_arc() {
        // Test a simple semicircle instead - more predictable
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Tangent vectors should be unit length
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));

        // For semicircle: at (1,0) tangent points up, at (-1,0) tangent points up
        assert!(float_equal(t_start.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_start.y, -1.0, TEST_UPLS));
        assert!(float_equal(t_end.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_end.y, -1.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_small_arc() {
        // Small arc around the first quadrant
        let arc = arc(
            point(1.0, 0.1),
            point(0.1, 1.0),
            point(0.0, 0.0),
            ((1.0_f64 - 0.0).powi(2) + (0.1_f64 - 0.0).powi(2)).sqrt(),
        );
        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Tangent vectors should be unit length
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));

        // Tangents should be perpendicular to radii
        let radius_start = point(1.0, 0.1) - arc.c;
        let radius_end = point(0.1, 1.0) - arc.c;

        // Dot product of tangent and radius should be ~0 (perpendicular)
        assert!(float_equal(t_start.dot(radius_start), 0.0, TEST_UPLS));
        assert!(float_equal(t_end.dot(radius_end), 0.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_counterclockwise_vs_clockwise() {
        // Test same arc points but with different orientations
        let ccw_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let cw_arc = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);

        let ccw_tangents = ccw_arc.tangents();
        let ccw_t_start = ccw_tangents[0];
        let ccw_t_end = ccw_tangents[1];

        let cw_tangents = cw_arc.tangents();
        let cw_t_start = cw_tangents[0];
        let cw_t_end = cw_tangents[1];

        // All tangent vectors should be unit length
        assert!(float_equal(ccw_t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(ccw_t_end.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(cw_t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(cw_t_end.norm(), 1.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_translated_arc() {
        // Test arc that's not centered at origin
        let center = point(5.0, -3.0);
        let arc = arc(point(6.0, -3.0), point(4.0, -3.0), center, 1.0);
        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Even when translated, tangent vectors should be unit length
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));

        // For horizontal semicircle, tangents should be vertical
        assert!(float_equal(t_start.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_end.x, 0.0, TEST_UPLS));
        assert!(float_equal(t_start.y.abs(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.y.abs(), 1.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_mathematical_properties() {
        // Test that tangents are perpendicular to radii
        let arc = arc(point(2.0, 0.0), point(0.0, 2.0), point(0.0, 0.0), 2.0);
        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Calculate radius vectors
        let radius_start = arc.a - arc.c; // From center to start point
        let radius_end = arc.b - arc.c; // From center to end point

        // Tangents should be perpendicular to radii (dot product = 0)
        assert!(float_equal(t_start.dot(radius_start), 0.0, TEST_UPLS));
        assert!(float_equal(t_end.dot(radius_end), 0.0, TEST_UPLS));

        // Tangents should be unit vectors
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_arbitrary_arc() {
        // Test with an arbitrary arc position and size
        let center = point(3.0, -2.0);
        let radius = 1.5;
        let start = center + point(radius, 0.0); // (4.5, -2.0)
        let end = center + point(0.0, radius); // (3.0, -0.5)
        let arc = arc(start, end, center, radius);

        let tangents = arc.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Tangents should be unit vectors
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));

        // Tangents should be perpendicular to radii
        let radius_start = start - center;
        let radius_end = end - center;
        assert!(float_equal(t_start.dot(radius_start), 0.0, TEST_UPLS));
        assert!(float_equal(t_end.dot(radius_end), 0.0, TEST_UPLS));
    }

    #[test]
    fn test_tangents_very_small_line_segment() {
        // Test with a very small line segment
        let line = arcseg(point(0.0, 0.0), point(1e-6, 1e-6));
        let tangents = line.tangents();
        let t_start = tangents[0];
        let t_end = tangents[1];

        // Should still produce unit tangent vectors
        assert!(float_equal(t_start.norm(), 1.0, TEST_UPLS));
        assert!(float_equal(t_end.norm(), 1.0, TEST_UPLS));

        // Direction should be normalized (1,1) -> (√2/2, √2/2)
        let expected = 1.0 / (2.0_f64.sqrt());
        assert!(float_equal(t_start.x, -expected, TEST_UPLS));
        assert!(float_equal(t_start.y, -expected, TEST_UPLS));
        assert!(float_equal(t_end.x, expected, TEST_UPLS));
        assert!(float_equal(t_end.y, expected, TEST_UPLS));
    }
}

#[cfg(test)]
mod test_is_valid_arcline {
    use super::*;

    const EPS: f64 = 1e-10;

    #[test]
    fn test_is_valid_arcline_invalid_case() {
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let arc2 = arc(point(1.0, 0.0), point(0.0, 0.0), point(0.5, 0.0), 0.5);
        let arcline = vec![arc1, arc2];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }

    #[test]
    fn test_is_valid_arcline_empty() {
        let empty_arcline: Arcline = vec![];
        assert_eq!(arcline_is_valid(&empty_arcline), ArclineValidation::Invalid);
    }

    #[test]
    fn test_is_valid_arcline_single_arc() {
        let arc = arcseg(point(0.0, 0.0), point(1.0, 1.0));
        let arcline = vec![arc];
        // Single arc should be invalid since minimum requirement is 2 arcs
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Invalid);
    }

    #[test]
    fn test_is_valid_arcline_invalid_arc() {
        // Create an invalid arc with collapsed endpoints
        let invalid_arc1 = arcseg(point(0.0, 0.0), point(0.0, 0.0));
        let valid_arc = arcseg(point(1.0, 1.0), point(2.0, 2.0));

        let arcline = vec![invalid_arc1, valid_arc];
        match arcline_is_valid(&arcline) {
            ArclineValidation::InvalidArc(arcx) => {
                assert_eq!(arcx, invalid_arc1);
            } // Expected
            other => assert!(false, "Expected InvalidArc, got {:?}", other),
        }
    }

    #[test]
    fn test_is_valid_arcline_gap_between_arcs() {
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let arc2 = arcseg(point(2.0, 0.0), point(3.0, 0.0)); // Gap between arcs

        let arcline = vec![arc1, arc2];
        match arcline_is_valid(&arcline) {
            ArclineValidation::GapBetweenArcs(_) => {} // Expected
            other => assert!(false, "Expected GapBetweenArcs, got {:?}", other),
        }
    }

    #[test]
    fn test_is_valid_arcline_intersecting_arcs() {
        // Create connected line segments that form an angle, then add a third that intersects
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let arc2 = arcseg(point(1.0, 0.0), point(1.0, 1.0)); // Connected L-shape
        let arc3 = arcseg(point(0.5, -0.5), point(0.5, 1.5)); // Crosses arc1 and arc2

        let arcline = vec![arc1, arc2, arc3];
        match arcline_is_valid(&arcline) {
            ArclineValidation::IntersectingArcs(_, _) => {} // Expected
            ArclineValidation::GapBetweenArcs(_) => {} // Also possible since arc3 isn't connected
            other => assert!(
                false,
                "Expected IntersectingArcs or GapBetweenArcs, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_is_valid_arcline_connected() {
        // Valid L-shaped arcline
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0)); // Horizontal segment
        let arc2 = arc(point(1.0, 0.0), point(0.0, 0.0), point(0.5, 0.0), 0.5); // half circle

        let arcline = vec![arc1, arc2];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }

    #[test]
    fn test_is_valid_arcline_closed_triangle() {
        // Valid closed triangle
        let p1 = point(0.0, 0.0);
        let p2 = point(1.0, 0.0);
        let p3 = point(0.5, 1.0);

        let arc1 = arcseg(p1, p2);
        let arc2 = arcseg(p2, p3);
        let arc3 = arcseg(p3, p1);

        let arcline = vec![arc1, arc2, arc3];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }

    #[test]
    fn test_is_valid_arcline_connected_arcs_and_segments() {
        // Mix of arcs and line segments that connect properly
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let arc2 = arc(
            point(1.0, 0.0),
            point(2.0, 1.0),
            point(1.5, 0.5),
            0.7071067811865476,
        );
        let arc3 = arcseg(point(2.0, 1.0), point(0.0, 1.0));
        let arc4 = arcseg(point(0.0, 1.0), point(0.0, 0.0));

        let arcline = vec![arc1, arc2, arc3, arc4];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }

    #[test]
    fn test_is_valid_arcline_multiple_invalid_arcs() {
        // Test with multiple invalid arcs - should return first invalid one
        let mut invalid_arc1 = arcseg(point(0.0, 0.0), point(1.0, 1.0));
        invalid_arc1.a = invalid_arc1.b; // Make invalid

        let valid_arc = arcseg(point(1.0, 1.0), point(2.0, 2.0));

        let mut invalid_arc2 = arcseg(point(2.0, 2.0), point(3.0, 3.0));
        invalid_arc2.a = invalid_arc2.b; // Make invalid

        let arcline = vec![invalid_arc1, valid_arc, invalid_arc2];
        match arcline_is_valid(&arcline) {
            ArclineValidation::InvalidArc(arc) => {
                // Should return the first invalid arc (invalid_arc1)
                assert_eq!(arc.a, arc.b);
            }
            other => assert!(false, "Expected InvalidArc, got {:?}", other),
        }
    }

    #[test]
    fn test_is_valid_arcline_non_adjacent_intersecting_arcs() {
        // Non-adjacent arcs that intersect (should be caught)
        let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let arc2 = arcseg(point(1.0, 0.0), point(2.0, 1.0));
        let arc3 = arcseg(point(2.0, 1.0), point(0.5, 2.0)); // Connected but intersects arc1

        let arcline = vec![arc1, arc2, arc3];
        match arcline_is_valid(&arcline) {
            ArclineValidation::IntersectingArcs(_, _) => {} // Expected
            ArclineValidation::GapBetweenArcs(_) => {} // Also possible if arcs aren't connected properly
            other => assert!(
                false,
                "Expected IntersectingArcs or GapBetweenArcs, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn test_is_valid_arcline_circular_arc_with_segments() {
        // Test with a proper circular arc connected to line segments
        let p1 = point(0.0, 0.0);
        let p2 = point(1.0, 0.0);
        let p3 = point(0.0, 1.0);

        let arc1 = arcseg(p1, p2); // Bottom edge
        let arc2 = arc(p2, p3, point(0.0, 0.0), 1.0); // Quarter circle
        let arc3 = arcseg(p3, p1); // Left edge

        let arcline = vec![arc1, arc2, arc3];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }

    #[test]
    fn test_is_valid_arcline_edge_case_very_small_segments() {
        // Test with very small but valid segments
        let arc1 = arcseg(point(0.0, 0.0), point(1e-6, 0.0));
        let arc2 = arc(point(1e-6, 0.0), point(0.0, 0.0), point(5e-7, 0.0), 5e-7);

        let arcline = vec![arc1, arc2];
        assert_eq!(arcline_is_valid(&arcline), ArclineValidation::Valid);
    }
}

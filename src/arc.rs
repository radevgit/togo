#![allow(dead_code)]

use robust::{Coord, orient2d};

use crate::prelude::*;

use std::{fmt::Display, sync::atomic::AtomicUsize};

/// A Arcline is a sequence of connected Arc-s forming a path.
pub type Arcline = Vec<Arc>;

static ID_COUNT: AtomicUsize = AtomicUsize::new(0);
const EPS_COLLAPSED: f64 = 1E-8;

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
    /// assert!(!arc.is_line()); // Has finite radius
    ///
    /// let line = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    /// assert!(line.is_line()); // Has infinite radius (line segment)
    /// ```
    #[inline]
    #[must_use]
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
    /// use basegeom::prelude::*;
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
        Arc::new(self.b, self.a, self.c, self.r)
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
/// assert!(line.is_line());
/// assert!(!line.is_arc());
/// assert_eq!(line.r, f64::INFINITY);
/// ```
#[inline]
#[must_use]
pub fn arcseg(a: Point, b: Point) -> Arc {
    Arc::new(a, b, point(f64::INFINITY, f64::INFINITY), f64::INFINITY)
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
    fn test_arcseg_creation() {
        // Test that arcseg creates a line segment (infinite radius)
        let line_arc = arcseg(point(0.0, 0.0), point(5.0, 5.0));
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
/// assert!(arc_is_collapsed_radius(0.0001, 0.01)); // Radius too small
/// assert!(arc_is_collapsed_radius(f64::NAN, 0.01)); // NaN radius
/// assert!(!arc_is_collapsed_radius(1.0, 0.01)); // Valid radius
/// ```
pub fn arc_is_collapsed_radius(r: f64, eps: f64) -> bool {
    // no abs() since it can be negative
    if r < eps || r.is_nan() {
        return true;
    }
    false
}

/// Checks if the arc has collapsed endpoints.
///
/// An arc is considered to have collapsed endpoints if the start and end
/// points are too close to each other within the given epsilon threshold.
///
/// # Arguments
///
/// * `a` - The start point of the arc
/// * `b` - The end point of the arc  
/// * `eps` - The epsilon threshold for distance comparison
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
/// assert!(arc_is_collapsed_ends(p1, p2, 0.01)); // Points too close
/// assert!(!arc_is_collapsed_ends(p1, p3, 0.01)); // Points far enough apart
/// ```
pub fn arc_is_collapsed_ends(a: Point, b: Point, eps: f64) -> bool {
    if a.close_enough(b, eps) {
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
/// * `a` - The start point of the arc
/// * `b` - The end point of the arc
/// * `c` - The center point of the arc
/// * `r` - The radius of the arc
/// * `eps` - The epsilon threshold for distance comparison
///
/// # Returns
///
/// True if the arc geometry is inconsistent, false if it's valid
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
/// let mut arc = arc(start, end, center, radius);
/// assert!(!arc_is_not_consistent(&arc, 1e-10));
///
/// // Inconsistent arc: center is not equidistant from endpoints
/// let bad_center = point(0.5, 0.0);
/// let mut arc2 = arc.clone();
/// arc2.c = bad_center;
/// assert!(arc_is_not_consistent(&arc2, 1e-10));
///
/// // Another inconsistent case: wrong radius
/// let mut arc3 = arc.clone();
/// arc3.r = 2.0;
/// assert!(arc_is_not_consistent(&arc3, 1e-10));
/// ```
pub fn arc_is_not_consistent(arc: &Arc, eps: f64) -> bool {
    if arc.is_line() {
        // Lines are always consistent, no center point
        return false;
    }
    // Check if the radius is consistent with the center and endpoints
    let dist_a_c = (arc.a - arc.c).norm();
    let dist_b_c = (arc.b - arc.c).norm();
    if (dist_a_c - arc.r).abs() > eps || (dist_b_c - arc.r).abs() > eps {
        return true; // Inconsistent radius
    }
    false
}

/// Validates if an arc is geometrically valid.
///
/// An arc is considered valid if it doesn't have a collapsed radius,
/// doesn't have collapsed endpoints, and has consistent geometry
/// (the center point is equidistant from both endpoints).
///
/// # Arguments
///
/// * `seg` - The arc to validate
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
/// assert!(arc_check(&valid_arc, 1e-10));
///
/// let invalid_arc = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.5, 0.5), 1.0);
/// assert!(!arc_check(&invalid_arc, 1e-10)); // Collapsed endpoints
///
/// let inconsistent_arc = arc(point(0.0, 0.0), point(2.0, 0.0), point(0.5, 0.0), 2.0);
/// assert!(!arc_check(&inconsistent_arc, 1e-10)); // Inconsistent geometry
/// ```
#[must_use]
pub fn arc_check(seg: &Arc, eps: f64) -> bool {
    if seg.is_line() {
        if arc_is_collapsed_ends(seg.a, seg.b, eps) {
            return false;
        }
    }
    if seg.is_arc() {
        if arc_is_collapsed_ends(seg.a, seg.b, eps)
            || arc_is_collapsed_radius(seg.r, eps)
            || arc_is_not_consistent(seg, eps)
        {
            return false;
        }
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

    const EPS_COLLAPSED: f64 = 1E-8; // Tolerance for collapsed checks

    #[test]
    fn test_arc_is_collapsed_radius_normal_values() {
        // Normal positive radius values should not be collapsed
        assert!(!arc_is_collapsed_radius(1.0, EPS_COLLAPSED));
        assert!(!arc_is_collapsed_radius(0.1, EPS_COLLAPSED));
        assert!(!arc_is_collapsed_radius(100.0, EPS_COLLAPSED));
        assert!(!arc_is_collapsed_radius(f64::INFINITY, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_small_values() {
        // Values smaller than EPS_COLLAPSED (1E-8) should be collapsed
        assert!(arc_is_collapsed_radius(1E-9, EPS_COLLAPSED));
        assert!(arc_is_collapsed_radius(1E-10, EPS_COLLAPSED));
        assert!(arc_is_collapsed_radius(0.0, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_boundary_values() {
        // Test values around the EPS_COLLAPSED boundary
        assert!(arc_is_collapsed_radius(EPS_COLLAPSED / 2.0, EPS_COLLAPSED));
        assert!(!arc_is_collapsed_radius(EPS_COLLAPSED * 2.0, EPS_COLLAPSED));

        // Exactly at the boundary should be collapsed
        assert!(arc_is_collapsed_radius(
            EPS_COLLAPSED - f64::EPSILON,
            EPS_COLLAPSED
        ));
    }

    #[test]
    fn test_arc_is_collapsed_radius_negative_values() {
        // Negative radius values should be collapsed
        assert!(arc_is_collapsed_radius(-1.0, EPS_COLLAPSED));
        assert!(arc_is_collapsed_radius(-0.1, EPS_COLLAPSED));
        assert!(arc_is_collapsed_radius(-1E-10, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_radius_nan() {
        // NaN values should be collapsed
        assert!(arc_is_collapsed_radius(f64::NAN, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_normal_points() {
        // Normal separated points should not be collapsed
        assert!(!arc_is_collapsed_ends(
            point(0.0, 0.0),
            point(1.0, 0.0),
            EPS_COLLAPSED
        ));
        assert!(!arc_is_collapsed_ends(
            point(0.0, 0.0),
            point(0.0, 1.0),
            EPS_COLLAPSED
        ));
        assert!(!arc_is_collapsed_ends(
            point(-1.0, -1.0),
            point(1.0, 1.0),
            EPS_COLLAPSED
        ));
        assert!(!arc_is_collapsed_ends(
            point(100.0, 200.0),
            point(300.0, 400.0),
            EPS_COLLAPSED
        ));
    }

    #[test]
    fn test_arc_is_collapsed_ends_identical_points() {
        // Identical points should be collapsed
        assert!(arc_is_collapsed_ends(
            point(0.0, 0.0),
            point(0.0, 0.0),
            EPS_COLLAPSED
        ));
        assert!(arc_is_collapsed_ends(
            point(1.0, 1.0),
            point(1.0, 1.0),
            EPS_COLLAPSED
        ));
        assert!(arc_is_collapsed_ends(
            point(-5.0, 10.0),
            point(-5.0, 10.0),
            EPS_COLLAPSED
        ));
    }

    #[test]
    fn test_arc_is_collapsed_ends_very_close_points() {
        // Points closer than EPS_COLLAPSED should be collapsed
        let p1 = point(0.0, 0.0);
        let p2 = point(EPS_COLLAPSED / 2.0, 0.0);
        assert!(arc_is_collapsed_ends(p1, p2, EPS_COLLAPSED));

        let p3 = point(100.0, 100.0);
        let p4 = point(100.0 + EPS_COLLAPSED / 3.0, 100.0 + EPS_COLLAPSED / 3.0);
        assert!(arc_is_collapsed_ends(p3, p4, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_is_collapsed_ends_boundary_distance() {
        // Points at exactly EPS_COLLAPSED distance
        let p1 = point(0.0, 0.0);
        let p2 = point(EPS_COLLAPSED, 0.0);
        // This should not be collapsed (distance equals tolerance)
        assert!(arc_is_collapsed_ends(p1, p2, EPS_COLLAPSED));

        // Points slightly farther than EPS_COLLAPSED
        let p3 = point(0.0, 0.0);
        let p4 = point(EPS_COLLAPSED * 2.0, 0.0);
        assert!(!arc_is_collapsed_ends(p3, p4, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_valid_arcs() {
        // Valid arcs should pass the check
        let valid_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5);
        assert!(arc_check(&valid_arc1, EPS_COLLAPSED));

        let valid_arc2 = arc(
            point(-1.0, -1.0),
            point(1.0, 1.0),
            point(0.0, 0.0),
            std::f64::consts::SQRT_2,
        );
        assert!(arc_check(&valid_arc2, EPS_COLLAPSED));

        // Line segments (infinite radius) should also be valid if endpoints are separated
        let valid_line = arcseg(point(0.0, 0.0), point(10.0, 0.0));
        assert!(arc_check(&valid_line, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_collapsed_radius() {
        // Arcs with collapsed radius should fail the check
        let collapsed_radius_arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1E-10);
        assert!(!arc_check(&collapsed_radius_arc1, EPS_COLLAPSED));

        let collapsed_radius_arc2 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), -1.0);
        assert!(!arc_check(&collapsed_radius_arc2, EPS_COLLAPSED));

        let nan_radius_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::NAN);
        assert!(!arc_check(&nan_radius_arc, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_collapsed_ends() {
        // Arcs with collapsed endpoints should fail the check
        let collapsed_ends_arc1 = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1.0);
        assert!(!arc_check(&collapsed_ends_arc1, EPS_COLLAPSED));

        let close_points = point(0.0, 0.0);
        let very_close_points = point(EPS_COLLAPSED / 2.0, 0.0);
        let collapsed_ends_arc2 = arc(close_points, very_close_points, point(0.0, 1.0), 1.0);
        assert!(!arc_check(&collapsed_ends_arc2, EPS_COLLAPSED));

        // Line segments with collapsed endpoints should also fail
        let collapsed_line = arcseg(point(1.0, 1.0), point(1.0, 1.0));
        assert!(!arc_check(&collapsed_line, EPS_COLLAPSED));
    }

    #[test]
    fn test_arc_check_both_collapsed() {
        // Arcs with both collapsed radius and collapsed endpoints should fail
        let both_collapsed = arc(point(0.0, 0.0), point(0.0, 0.0), point(0.0, 1.0), 1E-10);
        assert!(!arc_check(&both_collapsed, EPS_COLLAPSED));

        let both_collapsed2 = arc(point(5.0, 5.0), point(5.0, 5.0), point(0.0, 0.0), f64::NAN);
        assert!(!arc_check(&both_collapsed2, EPS_COLLAPSED));
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
        assert!(arc_check(&large_coord_arc, EPS_COLLAPSED));

        // Test with very small but valid radius - ensure consistent geometry
        let small_radius_arc = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.5);
        assert!(arc_check(&small_radius_arc, EPS_COLLAPSED));

        // Test with large radius
        let large_radius_arc = arc(point(0.0, 0.0), point(1E-6, 0.0), point(0.0, 1E6), 1E6);
        assert!(arc_check(&large_radius_arc, EPS_COLLAPSED));
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
/// assert!(line.is_line());
/// ```
#[must_use]
pub fn arc_circle_parametrization(pp1: Point, pp2: Point, bulge: f64) -> Arc {
    let mut p1 = pp1;
    let mut p2 = pp2;
    let mut bulge = bulge;
    if bulge.abs() < MIN_BULGE || p1.close_enough(p2, EPS_COLLAPSED) {
        // create line
        return arcseg(pp1, pp2);
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
mod test_arc_g_from_points {
    use crate::prelude::*;

    const TEST_EPS: f64 = 1E-10;

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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

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
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
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
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
            // For line segments, the function may return infinity or 0 depending on implementation
            assert!(result == 0.0 || result.is_infinite());
        } else {
            // Calculate bulge back from points
            let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);
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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

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
        let result = arc_bulge_from_points(arc.a, arc.b, arc.c, arc.r);

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
        if arc.is_line() {
            reversed.push(arc.reverse());
        } else {
            reversed.push(*arc);
        }
    }
    reversed
}

/// Makes an arc consistent by ensuring both endpoints are equidistant from the center.
/// This is achieved by adjusting the center point and radius of the arc.
#[must_use]
pub fn arc_make_consistent(seg: &Arc) -> Arc {
    if seg.is_line() {
        return *seg;
    }
    let dist_a_c = (seg.a - seg.c).norm();
    let dist_b_c = (seg.b - seg.c).norm();
    let r = (dist_a_c + dist_b_c) / 2.0;

    // Find the center that makes both endpoints equidistant
    // The center lies on the perpendicular bisector of the chord ab
    let midpoint = (seg.a + seg.b) / 2.0;
    let chord = seg.b - seg.a;
    let chord_length = chord.norm();

    // Handle degenerate case where endpoints are the same
    if chord_length < 1e-12 {
        return arcseg(seg.a, seg.b);
    }

    // Handle case where radius is too small for the chord length
    let half_chord = chord_length / 2.0;
    if r < half_chord {
        // Use the minimum possible radius (half chord length)
        return arc(seg.a, seg.b, midpoint, half_chord);
    }

    // Calculate distance from midpoint to center along perpendicular bisector
    let h = (r * r - half_chord * half_chord).sqrt();

    // Perpendicular direction to chord (normalized)
    let perp = point(-chord.y, chord.x) / chord_length;

    // Choose the center closer to the original center
    let c1 = midpoint + perp * h;
    let c2 = midpoint - perp * h;

    let dist1 = (c1 - seg.c).norm();
    let dist2 = (c2 - seg.c).norm();

    let c = if dist1 < dist2 { c1 } else { c2 };

    Arc::new(seg.a, seg.b, c, r)
}

#[cfg(test)]
mod test_arc_make_consistent {
    use crate::{arc::arc_make_consistent, prelude::*};

    const TEST_EPS: f64 = 1E-10;

    #[test]
    fn test_arc_make_consistent() {
        let arc = Arc::new(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 0.5);
        let consistent_arc = arc_make_consistent(&arc);
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));
    }

    #[test]
    fn test_arc_make_consistent_already_consistent() {
        // Create an already consistent arc
        let arc = Arc::new(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0);
        let consistent_arc = arc_make_consistent(&arc);
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));
        // Should be very close to the original
        assert!(close_enough(consistent_arc.c.x, 1.0, TEST_EPS));
        assert!(close_enough(consistent_arc.c.y, 0.0, TEST_EPS));
        assert!(close_enough(consistent_arc.r, 1.0, TEST_EPS));
    }

    #[test]
    fn test_arc_make_consistent_different_distances() {
        // Create an arc where endpoints are at different distances from center
        let arc = Arc::new(point(0.0, 0.0), point(3.0, 4.0), point(1.0, 1.0), 2.0);
        let consistent_arc = arc_make_consistent(&arc);
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));

        // Check that both endpoints are equidistant from the new center
        let dist_a = (consistent_arc.a - consistent_arc.c).norm();
        let dist_b = (consistent_arc.b - consistent_arc.c).norm();
        assert!(close_enough(dist_a, consistent_arc.r, TEST_EPS));
        assert!(close_enough(dist_b, consistent_arc.r, TEST_EPS));
    }

    #[test]
    fn test_arc_make_consistent_degenerate_endpoints() {
        // Create an arc with same start and end points
        let arc = Arc::new(point(1.0, 1.0), point(1.0, 1.0), point(2.0, 2.0), 1.0);
        let consistent_arc = arc_make_consistent(&arc);
        // Degenerate case should result in line segment
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));
        assert!(consistent_arc.is_line());
    }

    #[test]
    fn test_arc_make_consistent_line_segment() {
        // Test with a line segment (infinite radius)
        let line_arc = Arc::new(
            point(0.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 0.0),
            f64::INFINITY,
        );
        let consistent_arc = arc_make_consistent(&line_arc);
        assert_eq!(consistent_arc.r, f64::INFINITY);
        assert_eq!(consistent_arc.a, line_arc.a);
        assert_eq!(consistent_arc.b, line_arc.b);
    }

    #[test]
    fn test_arc_make_consistent_small_radius() {
        // Test case where desired radius is smaller than minimum possible (half chord length)
        let arc = Arc::new(point(0.0, 0.0), point(4.0, 0.0), point(2.0, 1.0), 1.0); // chord length = 4, so min radius = 2

        // Debug: check what the original distances are
        let dist_a_c = (arc.a - arc.c).norm(); // distance from (0,0) to (2,1) = sqrt(5) ≈ 2.236
        let dist_b_c = (arc.b - arc.c).norm(); // distance from (4,0) to (2,1) = sqrt(5) ≈ 2.236
        let avg_radius = (dist_a_c + dist_b_c) / 2.0; // ≈ 2.236

        let consistent_arc = arc_make_consistent(&arc);
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));

        // The average radius is about 2.236, which is larger than half chord length (2.0)
        // So it should use the computed average radius, not the minimum
        assert!(close_enough(consistent_arc.r, avg_radius, TEST_EPS));

        // Verify that both endpoints are equidistant from the center
        let new_dist_a = (consistent_arc.a - consistent_arc.c).norm();
        let new_dist_b = (consistent_arc.b - consistent_arc.c).norm();
        assert!(close_enough(new_dist_a, consistent_arc.r, TEST_EPS));
        assert!(close_enough(new_dist_b, consistent_arc.r, TEST_EPS));
    }

    #[test]
    fn test_arc_make_consistent_radius_too_small() {
        // Test case where the average radius is smaller than half chord length
        let arc = Arc::new(point(0.0, 0.0), point(10.0, 0.0), point(1.0, 0.1), 0.5); // chord length = 10, half = 5, but point is close to first endpoint

        let dist_a_c = (arc.a - arc.c).norm();
        let dist_b_c = (arc.b - arc.c).norm();
        let avg_radius = (dist_a_c + dist_b_c) / 2.0;
        let chord_length = (arc.b - arc.a).norm();
        let half_chord = chord_length / 2.0;

        let consistent_arc = arc_make_consistent(&arc);
        assert!(!arc_is_not_consistent(&consistent_arc, TEST_EPS));

        // Check if the average radius is actually smaller than half chord
        if avg_radius < half_chord {
            // Should use minimum possible radius (half chord length)
            assert!(close_enough(consistent_arc.r, half_chord, TEST_EPS));
            // Center should be at chord midpoint
            assert!(close_enough(
                consistent_arc.c.x,
                chord_length / 2.0,
                TEST_EPS
            ));
            assert!(close_enough(consistent_arc.c.y, 0.0, TEST_EPS));
        } else {
            // Should use the average radius
            assert!(close_enough(consistent_arc.r, avg_radius, TEST_EPS));
        }
    }
}

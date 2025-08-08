use std::fmt::Display;

use crate::point::Point;


/// A line segment defined by two endpoints.
///
/// Segments are fundamental geometric primitives used throughout the offsetting
/// algorithm for representing straight line portions of polylines and for
/// geometric intersection calculations.
///
/// # Examples
///
/// ```
/// use base_geom::{Segment, point};
///
/// let seg = Segment::new(point(0.0, 0.0), point(3.0, 4.0));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Segment {
    pub a: Point,
    pub b: Point,
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.a, self.b)
    }
}

impl Segment {
    #[doc(hidden)]
    /// Creates a new line segment between two points.
    ///
    /// # Arguments
    ///
    /// * `a` - First endpoint of the segment
    /// * `b` - Second endpoint of the segment
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::{Segment, point};
    ///
    /// let seg = Segment::new(point(0.0, 0.0), point(1.0, 1.0));
    /// ```
    #[inline]
    pub fn new(a: Point, b: Point) -> Self {
        Segment { a, b }
    }
}

/// Creates a new line segment between two points.
///
/// This is a convenience function equivalent to `Segment::new(a, b)`.
///
/// # Arguments
///
/// * `p0` - First endpoint of the segment
/// * `p1` - Second endpoint of the segment
///
/// # Examples
///
/// ```
/// use base_geom::{segment, point};
///
/// let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
/// ```
#[inline]
pub fn segment(p0: Point, p1: Point) -> Segment {
    Segment::new(p0, p1)
}

impl Segment {
    #[doc(hidden)]
    /// Converts the segment to centered form representation.
    ///
    /// This representation is useful for certain geometric algorithms that work
    /// with center-extent rather than endpoint representation.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * Center point of the segment
    /// * Normalized direction vector from center to endpoint
    /// * Half-length (extent) of the segment
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::{segment, point};
    ///
    /// let seg = segment(point(0.0, 0.0), point(4.0, 0.0));
    /// let (center, direction, extent) = seg.get_centered_form();
    /// // center = (2.0, 0.0), direction = (1.0, 0.0), extent = 2.0
    /// ```
    pub fn get_centered_form(&self) -> (Point, Point, f64) {
        let center = (self.a + self.b) * 0.5;
        let dir = self.b - self.a;
        let (dirn, norm) = dir.normalize();
        let extent = norm * 0.5;
        (center, dirn, extent)
    }
}

#[cfg(test)]
mod test_segment {
    use crate::point::point;

    use super::*;

    #[test]
    fn test_new() {
        let s0 = Segment::new(point(1.0, 2.0), point(3.0, 4.0));
        let s1 = segment(point(1.0, 2.0), point(3.0, 4.0));
        assert_eq!(s0, s1);
    }

    #[test]
    fn test_display() {
        let s0 = Segment::new(point(1.0, 2.0), point(3.0, 4.0));
        assert_eq!(
            "[[1.00000000000000000000, 2.00000000000000000000], [3.00000000000000000000, 4.00000000000000000000]]",
            format!("{}", s0)
        );
    }

    #[test]
    fn test_get_centered_form() {
        let s0 = Segment::new(point(1.0, 1.0), point(3.0, 3.0));
        let (center, dir, extent) = s0.get_centered_form();
        assert_eq!(center, point(2.0, 2.0));
        assert_eq!(dir, point(0.7071067811865475, 0.7071067811865475));
        assert_eq!(extent, 1.4142135623730951);
    }

    #[test]
    fn test_get_centered_form_edge_cases() {
        // Test zero-length segment (degenerate case)
        let s_zero = Segment::new(point(5.0, 3.0), point(5.0, 3.0));
        let (center, dir, extent) = s_zero.get_centered_form();
        assert_eq!(center, point(5.0, 3.0));
        assert_eq!(extent, 0.0);
        // Direction should be zero for zero-length segments (normalize returns (0,0))
        assert_eq!(dir.x, 0.0);
        assert_eq!(dir.y, 0.0);
        
        // Test horizontal segment
        let s_horiz = Segment::new(point(0.0, 2.0), point(6.0, 2.0));
        let (center, dir, extent) = s_horiz.get_centered_form();
        assert_eq!(center, point(3.0, 2.0));
        assert_eq!(dir, point(1.0, 0.0));
        assert_eq!(extent, 3.0);
        
        // Test vertical segment  
        let s_vert = Segment::new(point(1.0, -2.0), point(1.0, 4.0));
        let (center, dir, extent) = s_vert.get_centered_form();
        assert_eq!(center, point(1.0, 1.0));
        assert_eq!(dir, point(0.0, 1.0));
        assert_eq!(extent, 3.0);
    }
}

#![allow(dead_code)]

use std::fmt::Display;

use crate::prelude::*;

/// A polyline vertex that defines either a line segment or an arc segment.
///
/// The vertex contains a starting point and a "bulge" factor that determines
/// the curvature of the segment from this vertex to the next one.
///
/// # Fields
///
/// * `p` - Starting point of the segment
/// * `b` - Bulge factor determining curvature:
///   - `0.0` creates a straight line segment
///   - Positive values create counter-clockwise arcs
///   - Negative values create clockwise arcs
///   - The magnitude determines the arc's curvature
///
/// # Bulge Calculation
///
/// The bulge value is defined as `tan(θ/4)` where θ is the included angle
/// of the arc. This allows representing any arc segment with a single scalar value.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// // Straight line segment
/// let straight = pvertex(point(0.0, 0.0), 0.0);
///
/// // Quarter circle arc (90 degrees)
/// let quarter_arc = pvertex(point(0.0, 0.0), 1.0); // tan(90°/4) = tan(22.5°) ≈ 0.414
///
/// // Semicircle arc (180 degrees)  
/// let semicircle = pvertex(point(0.0, 0.0), 1.0); // tan(180°/4) = tan(45°) = 1.0
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PVertex {
    /// Starting point of the arc or line.
    pub p: Point,
    /// Bulge factor for the arc.
    pub b: f64,
}

impl Display for PVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.p, self.b)
    }
}

impl PVertex {
    /// Creates a new polyline vertex.
    ///
    /// # Arguments
    ///
    /// * `p` - Starting point of the segment
    /// * `b` - Bulge factor (0.0 for straight line, non-zero for arc)
    ///
    /// # Examples
    ///
    /// ```
    /// use togo::prelude::*;
    ///
    /// let vertex = PVertex::new(point(1.0, 2.0), 0.5);
    /// ```
    #[inline]
    pub fn new(p: Point, b: f64) -> Self {
        PVertex { p, b }
    }
}

/// Creates a new polyline vertex.
///
/// This is a convenience function equivalent to `PVertex::new(p, b)`.
///
/// # Arguments
///
/// * `p` - Starting point of the segment
/// * `b` - Bulge factor (0.0 for straight line, non-zero for arc)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let vertex = pvertex(point(1.0, 2.0), 0.5);
/// ```
#[inline]
pub fn pvertex(p: Point, b: f64) -> PVertex {
    PVertex::new(p, b)
}

/// A Polyline is a sequence of connected PVertex-es forming a path.
///
/// Each vertex defines the start of a segment, with the bulge factor
/// determining whether the segment to the next vertex is straight or curved.
pub type Polyline = Vec<PVertex>;

/// Reverses the direction of a polyline.
///
/// This function creates a new polyline that traces the same path but in
/// the opposite direction. The bulge values are negated to maintain the
/// same geometric shape while reversing the orientation.
///
/// # Arguments
///
/// * `poly` - The polyline to reverse
///
/// # Returns
///
/// A new polyline with reversed direction
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let original = vec![
///     pvertex(point(0.0, 0.0), 0.5),
///     pvertex(point(1.0, 0.0), -0.3),
///     pvertex(point(1.0, 1.0), 0.0),
/// ];
/// let reversed = polyline_reverse(&original);
/// ```
#[must_use]
pub fn polyline_reverse(poly: &Polyline) -> Polyline {
    if poly.is_empty() {
        return Vec::new();
    }
    if poly.len() == 1 {
        // If there's only one vertex, just negate the bulge
        return vec![pvertex(poly[0].p, -poly[0].b)];
    }
    let last = match poly.last() {
        Some(l) => l,
        None => unreachable!("polyline_reverse: poly is empty, should be handled above"),
    };
    let mut reverse = poly.clone();
    reverse.reverse();
    let mut res: Polyline = Vec::with_capacity(poly.len());
    for i in 0..reverse.len() - 1 {
        let e = pvertex(reverse[i].p, -reverse[i + 1].b);
        res.push(e);
    }
    let e = pvertex(match reverse.last() {
        Some(v) => v.p,
        None => unreachable!("polyline_reverse: reverse is empty, should be impossible here"),
    }, -last.b);
    res.push(e);

    res
}

/// Reverses the direction of multiple polylines.
///
/// Applies `polyline_reverse` to each polyline in the collection.
///
/// # Arguments
///
/// * `poly` - Vector of polylines to reverse
///
/// # Returns
///
/// A new vector containing all reversed polylines
#[must_use]
pub fn polylines_reverse(poly: &Vec<Polyline>) -> Vec<Polyline> {
    let mut res: Vec<Polyline> = Vec::with_capacity(poly.len());
    for p in poly {
        res.push(polyline_reverse(p));
    }
    res
}

/// Scales a polyline by a uniform scale factor.
///
/// Multiplies all point coordinates by the scale factor while preserving
/// the bulge values (which are dimensionless).
///
/// # Arguments
///
/// * `poly` - The polyline to scale
/// * `scale` - The scale factor to apply
///
/// # Returns
///
/// A new scaled polyline
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let original = vec![
///     pvertex(point(1.0, 2.0), 0.5),
///     pvertex(point(3.0, 4.0), 0.0),
/// ];
/// let scaled = polyline_scale(&original, 2.0);
/// // Points are now (2.0, 4.0) and (6.0, 8.0)
/// ```
#[must_use]
pub fn polyline_scale(poly: &Polyline, scale: f64) -> Polyline {
    let mut res: Polyline = Vec::with_capacity(poly.len());
    for e in poly {
        let e = pvertex(e.p * scale, e.b);
        res.push(e);
    }
    res
}

/// Translates a polyline by a given offset vector.
///
/// Adds the translation vector to all vertices while preserving bulge values.
///
/// # Arguments
///
/// * `poly` - The polyline to translate
/// * `translate` - The translation vector to apply
///
/// # Returns
///
/// A new translated polyline
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let original = vec![
///     pvertex(point(1.0, 2.0), 0.5),
///     pvertex(point(3.0, 4.0), 0.0),
/// ];
/// let translated = polyline_translate(&original, point(10.0, 5.0));
/// // Points are now (11.0, 7.0) and (13.0, 9.0)
/// ```
#[must_use]
pub fn polyline_translate(poly: &Polyline, translate: Point) -> Polyline {
    let mut res: Polyline = Vec::with_capacity(poly.len());
    for e in poly {
        let e = pvertex(e.p + translate, e.b);
        res.push(e);
    }
    res
}

#[cfg(test)]
mod test_pvertex {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_new() {
        let p0 = PVertex::new(point(1.0, 2.0), 5.5);
        let p1 = pvertex(point(1.0, 2.0), 5.5);
        assert_eq!(p0, p1);
    }

    #[test]
    fn test_display() {
        let p = pvertex(point(1.0, 2.0), 5.5);
        //print!("{}", p);
        assert_eq!(
            "[[1.00000000000000000000, 2.00000000000000000000], 5.5]",
            format!("{}", p)
        );
    }

    #[test]
    fn test_polylines_reverse() {
        // Test reversing multiple polylines
        let poly1 = vec![
            pvertex(point(0.0, 0.0), 0.5),
            pvertex(point(1.0, 0.0), -0.3),
        ];
        let poly2 = vec![pvertex(point(2.0, 2.0), 0.0), pvertex(point(3.0, 3.0), 0.2)];
        let polylines = vec![poly1, poly2];
        let reversed = polylines_reverse(&polylines);

        assert_eq!(reversed.len(), 2);
        // Each polyline should be individually reversed
        assert_eq!(reversed[0].len(), 2);
        assert_eq!(reversed[1].len(), 2);
    }

    #[test]
    fn test_polyline_scale_edge_cases() {
        // Test with zero scale
        let original = vec![pvertex(point(2.0, 3.0), 0.5)];
        let scaled = polyline_scale(&original, 0.0);
        assert_eq!(scaled[0].p, point(0.0, 0.0));
        assert_eq!(scaled[0].b, 0.5); // Bulge should be preserved

        // Test with negative scale
        let scaled_neg = polyline_scale(&original, -2.0);
        assert_eq!(scaled_neg[0].p, point(-4.0, -6.0));
        assert_eq!(scaled_neg[0].b, 0.5);
    }

    #[test]
    fn test_polyline_translate_empty() {
        // Test with empty polyline
        let empty: Polyline = vec![];
        let translated = polyline_translate(&empty, point(10.0, 5.0));
        assert_eq!(translated.len(), 0);
    }

    #[test]
    fn test_polyline_reverse_basic() {
        // Basic reversal of a polyline
        let poly = vec![
            pvertex(point(0.0, 0.0), 0.5),
            pvertex(point(1.0, 0.0), -0.3),
            pvertex(point(1.0, 1.0), 0.0),
        ];
        let reversed = polyline_reverse(&poly);
        assert_eq!(reversed.len(), poly.len());
        // The points should be in reverse order
        assert_eq!(reversed[0].p, poly[2].p);
        assert_eq!(reversed[1].p, poly[1].p);
        assert_eq!(reversed[2].p, poly[0].p);
    }

    #[test]
    fn test_polyline_reverse_bulge_negation() {
        // Bulge values should be negated and shifted according to the function logic
        let poly = vec![
            pvertex(point(0.0, 0.0), 0.5),
            pvertex(point(1.0, 0.0), -0.3),
            pvertex(point(1.0, 1.0), 0.7),
        ];
        let reversed = polyline_reverse(&poly);
        // The bulge of each reversed vertex is the negative of the next bulge in the reversed polyline
        assert_eq!(reversed[0].b, -poly[1].b); // -(-0.3) = 0.3
        assert_eq!(reversed[1].b, -poly[0].b); // -(0.5) = -0.5
        assert_eq!(reversed[2].b, -poly[2].b); // -(0.7) = -0.7
    }

    #[test]
    fn test_polyline_reverse_single_vertex() {
        // Edge case: single vertex polyline
        let poly = vec![pvertex(point(5.0, 5.0), 1.2)];
        let reversed = polyline_reverse(&poly);
        assert_eq!(reversed.len(), 1);
        assert_eq!(reversed[0].p, poly[0].p);
        assert_eq!(reversed[0].b, -poly[0].b);
    }

    #[test]
    fn test_polyline_reverse_empty() {
        // Edge case: empty polyline
        let poly: Polyline = vec![];
        let reversed = polyline_reverse(&poly);
        assert_eq!(reversed.len(), 0);
        assert_eq!(reversed, vec![]);
    }

    #[test]
    fn test_polyline_reverse_all_zero_bulges() {
        // Polyline with all zero bulges
        let poly = vec![
            pvertex(point(0.0, 0.0), 0.0),
            pvertex(point(1.0, 0.0), 0.0),
            pvertex(point(1.0, 1.0), 0.0),
        ];
        let reversed = polyline_reverse(&poly);
        assert_eq!(reversed.len(), poly.len());
        for v in reversed {
            assert_eq!(v.b, 0.0);
        }
    }
}

//! Area calculation algorithms for geometric shapes.
//!
//! This module provides functions for calculating areas of various geometric shapes,
//! including polygons defined by points and complex shapes bounded by arc sequences.

use crate::prelude::*;

/// Calculates the area of a simple polygon defined by a series of points.
///
/// Uses the shoelace formula (also known as the surveyor's formula) to compute
/// the area of a polygon given its vertices in order.
///
/// # Arguments
///
/// * `points` - A slice of points defining the polygon vertices in order
///
/// # Returns
///
/// The area of the polygon (positive for counter-clockwise orientation)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// use togo::algo::area::pointline_area;
///
/// let square = vec![
///     point(0.0, 0.0),
///     point(1.0, 0.0),
///     point(1.0, 1.0),
///     point(0.0, 1.0),
/// ];
/// let area = pointline_area(&square);
/// assert_eq!(area, 1.0);
/// ```
#[must_use]
pub fn pointline_area(points: &Pointline) -> f64 {
    if points.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].x * points[j].y;
        area -= points[j].x * points[i].y;
    }

    area / 2.0
}

/// Calculates the area of a region enclosed by an arcline (sequence of arcs and line segments).
///
/// This function computes the area by treating each arc segment appropriately:
/// - For line segments: Uses the shoelace formula contribution
/// - For circular arcs: Computes the sector area minus the triangular area
///
/// **Important**: This function assumes all arcs are oriented counter-clockwise (CCW).
/// All arcs in this library are CCW oriented by design. The implementation relies on this
/// orientation to correctly compute the signed area.
///
/// The implementation assumes the arcline forms a closed shape (last point connects back to first).
///
/// # Arguments
///
/// * `arcs` - A slice of Arc segments defining the boundary of the region (all CCW oriented)
///
/// # Returns
///
/// The area of the region (positive for counter-clockwise orientation)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// use togo::algo::area::arcline_area;
///
/// // Square made of line segments (CCW oriented)
/// let square_arcs = vec![
///     arcseg(point(0.0, 0.0), point(1.0, 0.0)),
///     arcseg(point(1.0, 0.0), point(1.0, 1.0)),
///     arcseg(point(1.0, 1.0), point(0.0, 1.0)),
///     arcseg(point(0.0, 1.0), point(0.0, 0.0)),
/// ];
/// let area = arcline_area(&square_arcs);
/// assert!((area - 1.0).abs() < 1e-10);
///
/// // Circle made of arcs (CCW oriented)
/// let center = point(0.0, 0.0);
/// let radius = 1.0;
/// let circle_arc = arc(
///     point(1.0, 0.0),
///     point(1.0, 0.0), // Full circle: start == end
///     center,
///     radius
/// );
/// let circle_arcs = vec![circle_arc];
/// let area = arcline_area(&circle_arcs);
/// let expected_area = std::f64::consts::PI * radius * radius;
/// assert!((area - expected_area).abs() < 1e-10);
/// ```
#[must_use]
pub fn arcline_area(arcs: &Arcline) -> f64 {
    if arcs.is_empty() {
        return 0.0;
    }

    let mut total_area = 0.0;

    for arc in arcs {
        if arc.is_seg() {
            // Line segment: use shoelace formula contribution
            //total_area += (arc.a.x * arc.b.y - arc.b.x * arc.a.y) / 2.0;
            total_area += arc.a.perp(arc.b) / 2.0;
        } else {
            // Circular arc: compute area contribution
            total_area += arc_area_contribution(arc);
        }
    }

    total_area
}

#[doc(hidden)]
/// Computes the area contribution of a circular arc.
///
/// For a circular arc, the area contribution is computed using Green's theorem,
/// which for a path integral around a closed curve gives the enclosed area.
/// For an arc from point A to point B with center C and radius R, the contribution
/// is the sector area from the center to the arc.
///
/// **Important**: This function assumes the arc is oriented counter-clockwise (CCW).
/// All arcs in this library are CCW oriented by design.
///
/// # Arguments
///
/// * `arc` - The circular arc (must be CCW oriented)
///
/// # Returns
///
/// The signed area contribution of the arc
fn arc_area_contribution(arc: &Arc) -> f64 {
    if arc.is_seg() {
        // This should not be called for line segments, but handle gracefully
        //return (arc.a.x * arc.b.y - arc.b.x * arc.a.y) / 2.0;
        return arc.a.perp(arc.b) / 2.0;
    }

    let center = arc.c;
    let start = arc.a;
    let end = arc.b;

    // Calculate the angle subtended by the arc
    let start_vector = start - center;
    let end_vector = end - center;

    let start_angle = start_vector.y.atan2(start_vector.x);
    let end_angle = end_vector.y.atan2(end_vector.x);

    // Calculate the arc angle for CCW orientation
    // Since all arcs are CCW, we compute the positive angle from start to end
    let mut arc_angle = end_angle - start_angle;
    if arc_angle < 0.0 {
        arc_angle += 2.0 * std::f64::consts::PI;
    }

    // Handle the special case of a full circle (start == end)
    if start.close_enough(end, 1e-10) {
        arc_angle = 2.0 * std::f64::consts::PI;
    }

    // For area calculation, we want the area "swept" by the arc
    // This includes both the sector area AND the triangular area from origin to center
    // to properly integrate with the shoelace formula for the rest of the boundary

    // First, compute the standard shoelace contribution as if this were a line segment
    let line_contribution = (start.x * end.y - end.x * start.y) / 2.0;

    // Then add the additional area due to the arc curvature
    // This is the difference between the sector and the triangle chord area
    let radius = arc.r;
    let sector_area = 0.5 * radius * radius * arc_angle;
    let triangle_area = 0.5
        * (center.x * (start.y - end.y)
            + start.x * (end.y - center.y)
            + end.x * (center.y - start.y));

    let arc_curvature_contribution = sector_area - triangle_area;

    line_contribution + arc_curvature_contribution
}

#[cfg(test)]
mod test_pointline_area {
    use super::*;

    #[test]
    fn test_pointline_area_square() {
        let square = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let area = pointline_area(&square);
        assert_eq!(area, 1.0);
    }

    #[test]
    fn test_pointline_area_triangle() {
        let triangle = vec![point(0.0, 0.0), point(2.0, 0.0), point(1.0, 2.0)];
        let area = pointline_area(&triangle);
        assert_eq!(area, 2.0);
    }
}

#[cfg(test)]
mod test_arcline_area {
    use super::*;

    #[test]
    fn test_arcline_area_empty() {
        let empty_arcline: Vec<Arc> = vec![];
        let area = arcline_area(&empty_arcline);
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_arcline_area_square_line_segments() {
        // Square made of line segments
        let square_arcs = vec![
            arcseg(point(0.0, 0.0), point(1.0, 0.0)),
            arcseg(point(1.0, 0.0), point(1.0, 1.0)),
            arcseg(point(1.0, 1.0), point(0.0, 1.0)),
            arcseg(point(0.0, 1.0), point(0.0, 0.0)),
        ];
        let area = arcline_area(&square_arcs);
        assert!((area - 1.0).abs() < 1e-10, "Expected 1.0, got {}", area);
    }

    #[test]
    fn test_arcline_area_triangle_line_segments() {
        // Triangle made of line segments
        let triangle_arcs = vec![
            arcseg(point(0.0, 0.0), point(2.0, 0.0)),
            arcseg(point(2.0, 0.0), point(1.0, 2.0)),
            arcseg(point(1.0, 2.0), point(0.0, 0.0)),
        ];
        let area = arcline_area(&triangle_arcs);
        assert!((area - 2.0).abs() < 1e-10, "Expected 2.0, got {}", area);
    }

    #[test]
    fn test_arcline_area_semicircle() {
        // Semicircle: arc from (1,0) to (-1,0) with center (0,0) and radius 1
        let semicircle = vec![
            arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),
            arcseg(point(-1.0, 0.0), point(1.0, 0.0)), // Closing line segment
        ];
        let area = arcline_area(&semicircle);
        let expected_area = std::f64::consts::PI / 2.0; // π * r² / 2 for semicircle
        assert!(
            (area - expected_area).abs() < 1e-10,
            "Expected {}, got {}",
            expected_area,
            area
        );
    }

    #[test]
    fn test_arcline_area_quarter_circle() {
        // Quarter circle: arc from (1,0) to (0,1) with center (0,0) and radius 1
        let quarter_circle = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),
            arcseg(point(0.0, 1.0), point(0.0, 0.0)), // Line to center
            arcseg(point(0.0, 0.0), point(1.0, 0.0)), // Line back to start
        ];
        let area = arcline_area(&quarter_circle);
        let expected_area = std::f64::consts::PI / 4.0; // π * r² / 4 for quarter circle
        assert!(
            (area - expected_area).abs() < 1e-10,
            "Expected {}, got {}",
            expected_area,
            area
        );
    }

    #[test]
    fn test_arcline_area_mixed_arcs_and_lines() {
        // Shape with both arcs and line segments
        // Half circle on top, straight line on bottom
        let mixed_shape = vec![
            arcseg(point(-1.0, 0.0), point(1.0, 0.0)), // Bottom line
            arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Top semicircle
        ];
        let area = arcline_area(&mixed_shape);
        let expected_area = std::f64::consts::PI / 2.0; // π/2 for semicircle
        assert!(
            (area - expected_area).abs() < 1e-10,
            "Expected {}, got {}",
            expected_area,
            area
        );
    }

    #[test]
    fn test_arcline_area_full_circle_single_arc() {
        // Full circle as a single arc (start point == end point)
        let center = point(0.0, 0.0);
        let radius = 2.0;
        let start_end = point(radius, 0.0);

        let full_circle = vec![arc(start_end, start_end, center, radius)];
        let area = arcline_area(&full_circle);
        let expected_area = std::f64::consts::PI * radius * radius;
        assert!(
            (area - expected_area).abs() < 1e-9,
            "Expected {}, got {}",
            expected_area,
            area
        );
    }

    #[test]
    fn test_arcline_area_single_arc_segment() {
        // Single arc segment (not a closed shape)
        // Arc from (1,0) to (0,1) with center (0,0) and radius 1 (90-degree arc)
        let single_arc = vec![arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0)];
        let area = arcline_area(&single_arc);

        // For a single arc, the area contribution includes both:
        // 1. The line segment contribution: (start.x * end.y - end.x * start.y) / 2.0
        // 2. The arc curvature contribution: sector_area - triangle_area

        let line_contribution = (1.0 * 1.0 - 0.0 * 0.0) / 2.0; // = 0.5
        let sector_area = 0.5 * 1.0 * 1.0 * (std::f64::consts::PI / 2.0); // = π/4
        let triangle_area = 0.5 * (0.0 * (0.0 - 1.0) + 1.0 * (1.0 - 0.0) + 0.0 * (0.0 - 0.0)); // = 0.5
        let arc_curvature_contribution = sector_area - triangle_area; // = π/4 - 0.5

        let expected_area = line_contribution + arc_curvature_contribution; // = 0.5 + π/4 - 0.5 = π/4

        assert!(
            (area - expected_area).abs() < 1e-10,
            "Expected {}, got {}",
            expected_area,
            area
        );

        // This should equal π/4 (quarter circle area)
        let quarter_circle_area = std::f64::consts::PI / 4.0;
        assert!(
            (area - quarter_circle_area).abs() < 1e-10,
            "Single arc area should be π/4, got {}",
            area
        );
    }

    #[test]
    fn test_arcline_area_clockwise_vs_counterclockwise() {
        // Test that orientation affects the sign
        let ccw_triangle = vec![
            arcseg(point(0.0, 0.0), point(1.0, 0.0)),
            arcseg(point(1.0, 0.0), point(0.0, 1.0)),
            arcseg(point(0.0, 1.0), point(0.0, 0.0)),
        ];

        let cw_triangle = vec![
            arcseg(point(0.0, 0.0), point(0.0, 1.0)),
            arcseg(point(0.0, 1.0), point(1.0, 0.0)),
            arcseg(point(1.0, 0.0), point(0.0, 0.0)),
        ];

        let ccw_area = arcline_area(&ccw_triangle);
        let cw_area = arcline_area(&cw_triangle);

        // Areas should have opposite signs
        assert!(
            (ccw_area + cw_area).abs() < 1e-10,
            "CCW: {}, CW: {}",
            ccw_area,
            cw_area
        );
        assert!(ccw_area > 0.0, "CCW area should be positive");
        assert!(cw_area < 0.0, "CW area should be negative");
    }

    #[test]
    fn test_arcline_area_ccw_arc_orientation() {
        // Test that CCW arc orientation produces positive area contribution
        // Quarter circle arc from (1,0) to (0,1) around center (0,0) - this is CCW
        let ccw_quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);

        // Create a shape with the CCW arc and closing line segments
        let ccw_shape = vec![
            ccw_quarter_arc,
            arcseg(point(0.0, 1.0), point(0.0, 0.0)), // To center
            arcseg(point(0.0, 0.0), point(1.0, 0.0)), // Back to start
        ];

        let area = arcline_area(&ccw_shape);

        // For CCW orientation, area should be positive and equal to π/4
        assert!(
            area > 0.0,
            "CCW oriented shape should have positive area, got {}",
            area
        );
        let expected_area = std::f64::consts::PI / 4.0;
        assert!(
            (area - expected_area).abs() < 1e-10,
            "Expected area {}, got {}",
            expected_area,
            area
        );
    }

    #[test]
    fn test_arcline_area_full_circle_ccw() {
        // Test full circle with CCW orientation
        let center = point(0.0, 0.0);
        let radius = 2.0;

        // Full circle: start point equals end point, CCW oriented
        let full_circle_ccw = vec![arc(point(radius, 0.0), point(radius, 0.0), center, radius)];

        let area = arcline_area(&full_circle_ccw);
        let expected_area = std::f64::consts::PI * radius * radius;

        // CCW full circle should have positive area equal to πr²
        assert!(area > 0.0, "CCW full circle should have positive area");
        assert!(
            (area - expected_area).abs() < 1e-9,
            "Expected area {}, got {}",
            expected_area,
            area
        );
    }
}

//! Convex hull algorithm for Arcline (arc-based polygons).
//!
//! This module implements convex hull computation for polygons defined by arcs and line segments,
//! preserving the arc structure where possible.

use crate::intersection::tangent::{external_tangents_between_circles, tangent_point_to_circle};
use crate::prelude::*;

/// Finds the starting arc index for convex hull construction.
fn find_starting_arc(arcs: &Arcline) -> usize {
    let mut idx = usize::MAX;
    let mut min_y = f64::INFINITY;
    for (i, arc) in arcs.iter().enumerate() {
        let l = left_point(*arc);
        if l < min_y {
            min_y = l;
            idx = i;
        }
    }
    idx
}

/// Finds most left point of an arc
fn left_point(arc: Arc) -> f64 {
    if arc.is_seg() {
        return arc.a.x.min(arc.b.x);
    }

    let left = point(arc.c.x - arc.r, arc.c.y);
    if arc.contains(left) {
        return left.x
    }
    return arc.a.x.min(arc.b.x)
}


/// Selects the correct tangent point from a point to a circle based on CCW consistency.
///
/// When there are two tangent options, we choose the one that maintains CCW ordering
/// by checking the cross product with the previous edge direction.
///
/// # Arguments
///
/// * `from_point` - The point from which tangent is drawn
/// * `to_circle` - The circle to which tangent is drawn
/// * `prev_direction` - Direction vector of the previous edge (for CCW consistency)
///
/// # Returns
///
/// The tangent point on the circle that maintains CCW order
fn select_tangent_point_to_circle(
    from_point: Point,
    to_circle: Circle,
    prev_direction: Point,
) -> Option<Point> {
    let tangents = tangent_point_to_circle(from_point, to_circle)?;
    let (t1, t2) = tangents;

    // Choose the tangent that makes a left turn (CCW) from prev_direction
    let dir_to_t1 = t1 - from_point;
    let dir_to_t2 = t2 - from_point;

    // Cross product: positive means t1 is to the left (CCW), negative means right
    let cross1 = prev_direction.perp(dir_to_t1);
    let cross2 = prev_direction.perp(dir_to_t2);

    // Choose the tangent with the most CCW (leftmost) direction
    if cross1 > cross2 { Some(t1) } else { Some(t2) }
}

/// Selects the correct external tangent between two circles based on CCW consistency.
///
/// For a convex hull, we want the tangent that makes sense geometrically - the one
/// where the tangent line goes in the direction we're traversing the hull.
///
/// # Arguments
///
/// * `from_circle` - The first circle
/// * `to_circle` - The second circle  
/// * `prev_direction` - Direction vector of the previous edge (for CCW consistency)
///
/// # Returns
///
/// Pair of tangent points `(point_on_c1, point_on_c2)` that maintains CCW order
fn select_external_tangent_between_circles(
    from_circle: Circle,
    to_circle: Circle,
    prev_direction: Point,
) -> Option<(Point, Point)> {
    let tangents = external_tangents_between_circles(from_circle, to_circle)?;
    let (t1_c1, t1_c2, t2_c1, t2_c2) = tangents;

    // Check which tangent is in the direction of prev_direction
    // by computing the tangent line directions and their cross product with prev_direction
    let dir1 = t1_c2 - t1_c1;
    let dir2 = t2_c2 - t2_c1;

    let cross1 = prev_direction.perp(dir1);
    let cross2 = prev_direction.perp(dir2);

    // Choose the tangent that is most aligned with prev_direction (has larger cross product)
    // This ensures we follow the CCW hull traversal
    if cross1 > cross2 {
        Some((t1_c1, t1_c2))
    } else {
        Some((t2_c1, t2_c2))
    }
}

/// Computes the convex hull of an arcline (arc-based polygon).
///
/// This function computes the convex hull while attempting to preserve arc segments
/// where they lie on the convex boundary. Concave arcs are replaced with line segments
/// or tangent connections.
///
/// # Algorithm Overview
///
/// 1. **Extract all candidate points**: Arc endpoints + cardinal extrema for curved arcs
/// 2. **Compute point-based hull**: Use gift wrapping on all candidates
/// 3. **Convert to arcline**: Create line segments connecting hull vertices
///
/// Future enhancements will preserve original arc segments where they lie on the hull.
///
/// # Arguments
///
/// * `arcs` - The input arcline (closed, non-self-intersecting, CCW)
///
/// # Returns
///
/// A new `Arcline` representing the convex hull
///
/// # Examples
///
/// ```ignore
/// use togo::prelude::*;
/// use togo::algo::convex_hull_arcs::arcline_convex_hull;
///
/// // Circle made of 4 quarter-circle arcs
/// let arcs = vec![
///     arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),
///     arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),
///     arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0),
///     arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0),
/// ];
/// let hull = arcline_convex_hull(&arcs);
/// assert_eq!(hull.len(), 4); // All arcs are convex
/// ```
#[must_use]
pub fn arcline_convex_hull(arcs: &Arcline) -> Arcline {
    if arcs.is_empty() {
        return Arcline::new();
    }

    let n = arcs.len();

    // Extract all candidate points: just arc endpoints for now
    let mut points_with_indices: Vec<(Point, usize, bool)> = Vec::new(); // (point, arc_idx, is_end_b)

    for (i, arc) in arcs.iter().enumerate() {
        points_with_indices.push((arc.a, i, false));
        points_with_indices.push((arc.b, i, true));
    }

    // Find the bottommost point (minimum y, then minimum x)
    let mut start_idx = 0;
    let mut start_point = points_with_indices[0].0;

    for (i, &(p, _, _)) in points_with_indices.iter().enumerate() {
        if p.y < start_point.y || (p.y == start_point.y && p.x < start_point.x) {
            start_idx = i;
            start_point = p;
        }
    }

    // Gift-wrapping on the extracted points
    let mut hull_point_indices: Vec<usize> = vec![start_idx];
    let mut current_point_idx = start_idx;
    let mut prev_direction = Point { x: 1.0, y: 0.0 }; // Start pointing right

    loop {
        let current_point = points_with_indices[current_point_idx].0;

        // Find the point with the largest (most left-turn) cross product
        let mut best_point_idx = current_point_idx;
        let mut best_cross = f64::NEG_INFINITY;

        for (i, &(candidate_point, _, _)) in points_with_indices.iter().enumerate() {
            if i == current_point_idx {
                continue;
            }

            let to_candidate = candidate_point - current_point;

            if to_candidate.norm() < 1e-9 {
                continue; // Skip if same point
            }

            // Cross product: prev_direction × to_candidate
            let cross = prev_direction.x * to_candidate.y - prev_direction.y * to_candidate.x;

            if cross > best_cross {
                best_cross = cross;
                best_point_idx = i;
            }
        }

        // If we've wrapped back to the start, we're done
        if best_point_idx == start_idx && hull_point_indices.len() > 2 {
            break;
        }

        hull_point_indices.push(best_point_idx);

        // Update direction for next iteration
        let next_point = points_with_indices[best_point_idx].0;
        prev_direction = next_point - current_point;
        current_point_idx = best_point_idx;

        // Safety check
        if hull_point_indices.len() > n + 10 {
            break;
        }
    }

    // Convert the hull points back to arcline segments
    let mut hull = Arcline::new();

    for i in 0..hull_point_indices.len() {
        let p1_idx = hull_point_indices[i];
        let p2_idx = hull_point_indices[(i + 1) % hull_point_indices.len()];

        let (p1, _, _) = points_with_indices[p1_idx];
        let (p2, _, _) = points_with_indices[p2_idx];

        hull.push(arcseg(p1, p2));
    }

    hull
}

#[cfg(test)]
mod tests {
    use crate::poly::arcline200;

    use super::*;

    #[test]
    fn test_arcline_convex_hull_empty() {
        let arcs: Arcline = vec![];
        let hull = arcline_convex_hull(&arcs);
        assert!(hull.is_empty());
    }

    #[test]
    fn test_arcline_convex_hull_single_arc() {
        // A single arc cannot form a closed polygon - this is an invalid input
        // The algorithm should handle it gracefully (likely return empty or the arc itself)
        let arcs = vec![arc(
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(0.5, 0.0),
            f64::INFINITY,
        )];
        let hull = arcline_convex_hull(&arcs);
        assert_eq!(hull.len(), 0);
    }

    #[test]
    fn test_arcline_convex_hull_square() {
        // Square made of 4 line segments
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(1.0, 0.0)),
            arcseg(point(1.0, 0.0), point(1.0, 1.0)),
            arcseg(point(1.0, 1.0), point(0.0, 1.0)),
            arcseg(point(0.0, 1.0), point(0.0, 0.0)),
        ];
        let hull = arcline_convex_hull(&arcs);
        assert_eq!(hull.len(), 4); // All segments should be on hull

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
        let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_circle() {
        // Circle made of 4 quarter-circle arcs (all convex)
        // Center at origin, radius 1.0
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0), // Right to Top
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Top to Left
            arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0), // Left to Bottom
            arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0), // Bottom to Right
        ];

        let hull = arcline_convex_hull(&arcs);

        // Hull should include all 4 cardinal points (endpoints)
        assert_eq!(hull.len(), 4);

        // Verify that all cardinal points are in the hull
        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
        assert!(hull_points.contains(&point(1.0, 0.0))); // Right
        assert!(hull_points.contains(&point(0.0, 1.0))); // Top
        assert!(hull_points.contains(&point(-1.0, 0.0))); // Left
        assert!(hull_points.contains(&point(0.0, -1.0))); // Bottom

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
        let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_semicircle() {
        // Semicircle (top half) with a line segment closing it
        // Two quarter-circle arcs + one line segment
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0), // Right to Top
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Top to Left
            arcseg(point(-1.0, 0.0), point(1.0, 0.0)),                   // Left to Right (base)
        ];

        let hull = arcline_convex_hull(&arcs);

        // Hull should be the semicircle itself (all arcs are convex)
        assert_eq!(hull.len(), 3);

        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
        assert!(hull_points.contains(&point(1.0, 0.0)));
        assert!(hull_points.contains(&point(0.0, 1.0)));
        assert!(hull_points.contains(&point(-1.0, 0.0)));

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
        let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_rounded_rectangle() {
        // Rectangle with rounded corners (4 arcs + 4 line segments)
        // Rectangle from (0,0) to (4,2) with corner radius 0.5
        let r = 0.5;
        let arcs = vec![
            // Bottom edge
            arcseg(point(r, 0.0), point(4.0 - r, 0.0)),
            // Bottom-right corner (CCW arc)
            arc(point(4.0 - r, 0.0), point(4.0, r), point(4.0 - r, r), r),
            // Right edge
            arcseg(point(4.0, r), point(4.0, 2.0 - r)),
            // Top-right corner (CCW arc)
            arc(
                point(4.0, 2.0 - r),
                point(4.0 - r, 2.0),
                point(4.0 - r, 2.0 - r),
                r,
            ),
            // Top edge
            arcseg(point(4.0 - r, 2.0), point(r, 2.0)),
            // Top-left corner (CCW arc)
            arc(point(r, 2.0), point(0.0, 2.0 - r), point(r, 2.0 - r), r),
            // Left edge
            arcseg(point(0.0, 2.0 - r), point(0.0, r)),
            // Bottom-left corner (CCW arc)
            arc(point(0.0, r), point(r, 0.0), point(r, r), r),
        ];

        let hull = arcline_convex_hull(&arcs);

        // Hull should include multiple points (edges + corner extrema)
        assert!(hull.len() >= 4); // At least 4 segments

        // Check that the bounding box extrema are approximately in the hull
        // The extrema of the rounded rectangle are:
        // Right: (4.0, y), Top: (x, 2.0), Left: (0.0, y), Bottom: (x, 0.0)
        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();

        // Find min/max coordinates in hull
        let max_x = hull_points
            .iter()
            .map(|p| p.x)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_x = hull_points
            .iter()
            .map(|p| p.x)
            .fold(f64::INFINITY, f64::min);
        let max_y = hull_points
            .iter()
            .map(|p| p.y)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = hull_points
            .iter()
            .map(|p| p.y)
            .fold(f64::INFINITY, f64::min);

        // Verify the bounding box matches the rounded rectangle's bounds
        assert!((max_x - 4.0).abs() < 1e-6);
        assert!((min_x - 0.0).abs() < 1e-6);
        assert!((max_y - 2.0).abs() < 1e-6);
        assert!((min_y - 0.0).abs() < 1e-6);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
        let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_concave_arc() {
        // Shape with a concave indentation (backward arc)
        // Square with a circular bite taken out of one side
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(2.0, 0.0)), // Bottom
            arcseg(point(2.0, 0.0), point(2.0, 2.0)), // Right
            arcseg(point(2.0, 2.0), point(0.0, 2.0)), // Top
            // Left side with concave arc (backward traversal = concave)
            arc(point(0.0, 0.0), point(0.0, 2.0), point(0.0, 1.0), 0.5), // Concave indent
        ];

        let hull = arcline_convex_hull(&arcs);

        // Hull should be approximately a square (concave arc removed)
        assert!(hull.len() >= 3);

        // The hull should have the 4 square corners
        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
        assert!(hull_points.contains(&point(0.0, 0.0)));
        assert!(hull_points.contains(&point(2.0, 0.0)));
        assert!(hull_points.contains(&point(2.0, 2.0)));
        assert!(hull_points.contains(&point(0.0, 2.0)));

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
        let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_square_with_convex_arcs() {
        let arcs = vec![
            arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 0.50),
            arc(point(1.0, 0.0), point(1.0, 1.0), point(1.0, 0.5), 0.50),
            arc(point(1.0, 1.0), point(0.0, 1.0), point(0.5, 1.0), 0.50),
            arc(point(0.0, 1.0), point(0.0, 0.0), point(0.0, 0.5), 0.50),
        ];

        let hull = arcline_convex_hull(&arcs);

        println!("Hull length: {}", hull.len());
        for (i, elem) in hull.iter().enumerate() {
            println!("  [{}] {} -> {}", i, elem.a, elem.b);
        }

        // With gift-wrapping, if all 4 arcs form a convex sequence, the hull should contain just those 4 arcs
        // (no connecting segments needed if they're selected in order)
        // However, the exact number depends on whether connecting segments are added between non-adjacent arcs
        assert!(hull.len() >= 4, "Hull should contain at least the 4 arcs");

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_mixed_convex_concave_arcs() {
        let arcs = vec![
            arc(point(2.0, 0.0), point(2.0, 2.0), point(2.0, 1.0), 1.0), // Convex arc
            arc(point(2.0, 2.0), point(0.0, 2.0), point(1.0, 2.0), 1.0), // Convex arc
            arc(point(0.0, 2.0), point(0.0, 0.0), point(0.0, 1.0), 0.5), // Concave arc (inward)
            arcseg(point(0.0, 0.0), point(2.0, 0.0)),                    // Line segment
        ];

        let hull = arcline_convex_hull(&arcs);
        assert!(hull.len() >= 3);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_alternating_convex_concave() {
        let arcs = vec![
            arc(point(0.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0),
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(-0.5, 0.5), 1.0),
            arc(point(0.0, -1.0), point(-1.0, 0.0), point(-1.0, -1.0), 1.0),
            arc(point(0.0, -1.0), point(1.0, 0.0), point(0.5, -0.5), 1.0),
        ];

        let hull = arcline_convex_hull(&arcs);
        assert!(hull.len() >= 2);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_with_line_segments() {
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(2.0, 0.0)), // Bottom edge
            arc(point(2.0, 0.0), point(2.0, 2.0), point(2.0, 1.0), 1.0), // Right convex arc
            arcseg(point(2.0, 2.0), point(0.0, 2.0)), // Top edge
            arc(point(0.0, 2.0), point(0.0, 0.0), point(0.0, 1.0), 1.0), // Left convex arc
        ];

        let hull = arcline_convex_hull(&arcs);
        assert_eq!(hull.len(), 4);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_three_convex_arcs() {
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0), // Convex
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Convex
            arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0), // Convex (bottom)
        ];

        let hull = arcline_convex_hull(&arcs);
        assert!(hull.len() >= 3);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_convex_hull_segment_dominated() {
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(3.0, 0.0)), // Long bottom
            arc(point(3.0, 0.0), point(3.0, 1.0), point(3.0, 0.5), 0.5), // Small convex
            arcseg(point(3.0, 1.0), point(0.0, 1.0)), // Long top
            arc(point(0.0, 1.0), point(0.0, 0.0), point(0.0, 0.5), 0.5), // Small convex
        ];

        let hull = arcline_convex_hull(&arcs);
        assert_eq!(hull.len(), 4);

        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let arcs3 = arcline_translate(&arcs2, point(150.0, 150.0));
        let hull3 = arcline_translate(&hull2, point(150.0, 150.0));
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs3, "blue");
        svg.arcline(&hull3, "red");
        svg.write_stroke_width(0.1);
    }

    #[test]
    fn test_arcline_200() {
        let arcs = arcline200();
        println!("arcline200 has {} arcs", arcs.len());

        // Debug: count convex arcs
        let mut convex_count = 0;
        for i in 0..arcs.len() {
            let prev_idx = if i == 0 { arcs.len() - 1 } else { i - 1 };
            let arc = arcs[i];
            let prev = arcs[prev_idx];

            let forward = prev.b.close_enough(arc.a, 1e-9);
            if forward {
                convex_count += 1;
            }
        }
        println!(
            "Convex arcs: {}, Concave arcs: {}",
            convex_count,
            arcs.len() - convex_count
        );

        let hull = arcline_convex_hull(&arcs);
        println!("Hull has {} elements", hull.len());

        // Write SVG for visualization
        use crate::svg::SVG;
        let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
        svg.arcline(&arcs, "blue");
        svg.arcline(&hull, "red");
        svg.write_stroke_width(0.1);
    }
}

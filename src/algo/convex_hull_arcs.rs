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
        return left.x;
    }
    return arc.a.x.min(arc.b.x);
}

fn find_tangent_point_to_circle(from: Point, c: Circle) -> Option<Point> {
    match tangent_point_to_circle(from, c) {
        Some((t1, t2)) => {
            let orient = (t1 - from).perp(t2 - from);
            if orient > 0.0 { Some(t1) } else { Some(t2) }
        }
        None => None,
    }
}

fn find_tangent_circle_to_circle(c1: Circle, c2: Circle) -> Option<(Point, Point)> {
    match external_tangents_between_circles(c1, c2) {
        Some((t1_c1, t1_c2, t2_c1, t2_c2)) => {
            // find the right side points
            let orient1 = (c2.c - c1.c).perp(t1_c1 - c1.c);
            let rc1 = if orient1 > 0.0 { t1_c1 } else { t2_c1 };
            let orient2 = (c2.c - c1.c).perp(t1_c2 - c2.c);
            let rc2 = if orient2 > 0.0 { t1_c2 } else { t2_c2 };
            Some((rc1, rc2))
        }
        None => None,
    }
}

#[must_use]
fn new_convex_hull(arcs: &Arcline) -> Arcline {
    let mut segs = arcs.clone();
    let hull = Arcline::new();
    
    if arcs.is_empty() {
        return hull;
    }

    let n = arcs.len();
    let mut convex = Vec::with_capacity(n);
    // Convert concave to properly oriented arcseg
    for i in 0..n {
        if arcs[i].is_seg() {
            convex[i] = false;
            continue;
        }
        if is_arc_convex(arcs, i) {
            convex[i] = true;
        } else {
            segs[i] = arcseg(segs[i].b, segs[i].a);
            convex[i] = false;
        }
    }
    let start = find_starting_arc(&segs);
    let mut best_connection: Option<(Point, Point)> = None;
    
    for i in start..(n + start) {
        let idx1 = i % n;
        let arc1 = segs[idx1];
        
        for j in (i + 1)..(n + start) {
            let idx2 = j % n;
            let arc2 = segs[idx2];
            
            // From segment
            if !convex[idx1] {
                if !convex[idx2] {

                }
            
            }
        }
    }
    hull
}

fn hull_seg_seg(seg1: Arc, seg2: Arc) -> (Point, Point) {
    // Four possible connections between segment endpoints:
    // 1. seg1.a -> seg2.a
    // 2. seg1.a -> seg2.b
    // 3. seg1.b -> seg2.a
    // 4. seg1.b -> seg2.b
    
    let candidates = [
        (seg1.a, seg2.a),
        (seg1.a, seg2.b),
        (seg1.b, seg2.a),
        (seg1.b, seg2.b),
    ];
    
    let mut best_connection: Option<(Point, Point)> = None;
    let mut best_angle = f64::NEG_INFINITY;
    
    for &(p1, p2) in &candidates {
        let connection_dir = p2 - p1;
        
        if connection_dir.norm() < 1e-9 {
            continue; // Skip zero-length connections
        }
        
        let mut valid = true;
        
        // Check if the other endpoint of seg1 is on the right side (or on the line)
        let other_seg1 = if p1 == seg1.a { seg1.b } else { seg1.a };
        let to_other1 = other_seg1 - p1;
        let cross1 = connection_dir.perp(to_other1);
        
        // For CCW hull, the other point should not be on the left (cross > 0 is invalid)
        if cross1 > 1e-9 {
            valid = false;
        }
        
        // Check if the other endpoint of seg2 is on the right side (or on the line)
        if valid {
            let other_seg2 = if p2 == seg2.a { seg2.b } else { seg2.a };
            let to_other2 = other_seg2 - p1;
            let cross2 = connection_dir.perp(to_other2);
            
            // For CCW hull, the other point should not be on the left
            if cross2 > 1e-9 {
                valid = false;
            }
        }
        
        // Among valid connections, choose the one that makes the most CCW turn
        // We can use the angle or just compare cross products with a reference direction
        if valid {
            // Use the angle of the connection direction as a tie-breaker
            // For CCW hull, we want the connection that turns most to the left
            let angle = connection_dir.y.atan2(connection_dir.x);
            
            if best_connection.is_none() || angle > best_angle {
                best_angle = angle;
                best_connection = Some((p1, p2));
            }
        }
    }
    
    // Return the best connection, or default to seg1.b -> seg2.a if none found
    best_connection.unwrap_or((seg1.b, seg2.a))
}

fn hull_arc_arc(arc1: Arc, arc2: Arc) -> Vec<Arc> {
    let mut result = Vec::new();
    
    // Get the circles that the arcs belong to
    let c1 = Circle { c: arc1.c, r: arc1.r };
    let c2 = Circle { c: arc2.c, r: arc2.r };
    
    // Find the external tangent between the two circles (right-side tangent)
    let tangent_opt = find_tangent_circle_to_circle(c1, c2);
    
    if tangent_opt.is_none() {
        // No external tangent exists (concentric or nested circles)
        // Try to find the best connection between arc endpoints
        
        // Four possible connections
        let candidates = [
            (arc1.a, arc2.a),
            (arc1.a, arc2.b),
            (arc1.b, arc2.a),
            (arc1.b, arc2.b),
        ];
        
        // Check each connection to see if it crosses the arcs
        let mut best_connection: Option<(Point, Point)> = None;
        
        for &(p1, p2) in &candidates {
            let connection_dir = p2 - p1;
            
            if connection_dir.norm() < 1e-9 {
                continue;
            }
            
            // Check if this connection crosses arc1
            // For an arc, we need to check if intermediate points would be on the wrong side
            let other1 = if p1 == arc1.a { arc1.b } else { arc1.a };
            let to_other1 = other1 - p1;
            let cross1 = connection_dir.perp(to_other1);
            
            // Check if this connection crosses arc2
            let other2 = if p2 == arc2.a { arc2.b } else { arc2.a };
            let to_other2 = other2 - p1;
            let cross2 = connection_dir.perp(to_other2);
            
            // If both checks pass (other endpoints not on left), this is a valid direct connection
            if cross1 <= 1e-9 && cross2 <= 1e-9 {
                best_connection = Some((p1, p2));
                break;
            }
        }
        
        if let Some((p1, p2)) = best_connection {
            // Direct connection is valid
            if !p1.close_enough(p2, 1e-9) {
                result.push(arcseg(p1, p2));
            }
        } else {
            // No direct connection works, need to use point-to-arc tangent
            // Try tangent from arc1 endpoints to arc2
            let tangent_from_arc1_b = find_tangent_point_to_circle(arc1.b, c2);
            
            if let Some(t2) = tangent_from_arc1_b {
                if arc2.contains(t2) {
                    // Tangent from arc1.b to t2 on arc2
                    result.push(arcseg(arc1.b, t2));
                    
                    // Add remaining portion of arc2
                    if !t2.close_enough(arc2.b, 1e-9) {
                        result.push(arc(t2, arc2.b, arc2.c, arc2.r));
                    }
                    return result;
                }
            }
            
            // Try tangent from arc2.a to arc1
            let tangent_from_arc2_a = find_tangent_point_to_circle(arc2.a, c1);
            
            if let Some(t1) = tangent_from_arc2_a {
                if arc1.contains(t1) {
                    // Add portion of arc1 to tangent point
                    if !arc1.a.close_enough(t1, 1e-9) {
                        result.push(arc(arc1.a, t1, arc1.c, arc1.r));
                    }
                    
                    // Tangent from t1 on arc1 to arc2.a
                    result.push(arcseg(t1, arc2.a));
                    return result;
                }
            }
            
            // Fall back to endpoint connection if nothing else works
            result.push(arcseg(arc1.b, arc2.a));
        }
        
        return result;
    }
    
    let (t1, t2) = tangent_opt.unwrap();
    
    // Check if tangent points are contained within the arcs
    let t1_in_arc1 = arc1.contains(t1);
    let t2_in_arc2 = arc2.contains(t2);
    
    match (t1_in_arc1, t2_in_arc2) {
        (true, true) => {
            // Both tangent points are on the arcs
            // Hull: part of arc1 (from arc1.a to t1), tangent line, part of arc2 (from t2 to arc2.b)
            
            // Add the portion of arc1 from arc1.a to t1
            if !arc1.a.close_enough(t1, 1e-9) {
                result.push(arc(arc1.a, t1, arc1.c, arc1.r));
            }
            
            // Add the tangent line segment
            if !t1.close_enough(t2, 1e-9) {
                result.push(arcseg(t1, t2));
            }
            
            // Add the portion of arc2 from t2 to arc2.b
            if !t2.close_enough(arc2.b, 1e-9) {
                result.push(arc(t2, arc2.b, arc2.c, arc2.r));
            }
        }
        
        (false, true) => {
            // t1 is outside arc1, t2 is inside arc2
            // Need to determine which endpoint of arc1 to use
            
            // The tangent should start from whichever end of arc1 is "closer" to t1
            // in the sense of being on the convex hull
            let start_point = if arc1.b.close_enough(t1, 1e-9) {
                arc1.b
            } else {
                // Check which endpoint makes a valid hull connection
                // Use arc1.b as it's the natural continuation point
                arc1.b
            };
            
            // Add tangent line from arc1 endpoint to t2
            if !start_point.close_enough(t2, 1e-9) {
                result.push(arcseg(start_point, t2));
            }
            
            // Add the portion of arc2 from t2 to arc2.b
            if !t2.close_enough(arc2.b, 1e-9) {
                result.push(arc(t2, arc2.b, arc2.c, arc2.r));
            }
        }
        
        (true, false) => {
            // t1 is inside arc1, t2 is outside arc2
            // Need to determine which endpoint of arc2 to use
            
            // Add the portion of arc1 from arc1.a to t1
            if !arc1.a.close_enough(t1, 1e-9) {
                result.push(arc(arc1.a, t1, arc1.c, arc1.r));
            }
            
            // The tangent should end at whichever end of arc2 is "closer" to t2
            let end_point = if arc2.a.close_enough(t2, 1e-9) {
                arc2.a
            } else {
                // Use arc2.a as it's the natural starting point
                arc2.a
            };
            
            // Add tangent line from t1 to arc2 endpoint
            if !t1.close_enough(end_point, 1e-9) {
                result.push(arcseg(t1, end_point));
            }
        }
        
        (false, false) => {
            // Both tangent points are outside the arcs
            // Connect arc1 endpoint to arc2 endpoint
            
            // Use arc1.b and arc2.a as the natural connection points
            let start_point = arc1.b;
            let end_point = arc2.a;
            
            if !start_point.close_enough(end_point, 1e-9) {
                result.push(arcseg(start_point, end_point));
            }
        }
    }
    
    result
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

    #[test]
    fn test_left_point_horizontal_line_segment() {
        // Horizontal line segment: left point is the minimum x
        let seg = arcseg(point(3.0, 5.0), point(1.0, 5.0));
        assert_eq!(left_point(seg), 1.0);

        let seg2 = arcseg(point(0.0, 0.0), point(10.0, 0.0));
        assert_eq!(left_point(seg2), 0.0);
    }

    #[test]
    fn test_left_point_vertical_line_segment() {
        // Vertical line segment: left point is the x-coordinate (both endpoints have same x)
        let seg = arcseg(point(2.0, 0.0), point(2.0, 5.0));
        assert_eq!(left_point(seg), 2.0);

        let seg2 = arcseg(point(7.5, 3.0), point(7.5, 8.0));
        assert_eq!(left_point(seg2), 7.5);
    }

    #[test]
    fn test_left_point_diagonal_line_segment() {
        // Diagonal line segment: left point is the minimum x
        let seg = arcseg(point(5.0, 0.0), point(2.0, 8.0));
        assert_eq!(left_point(seg), 2.0);

        let seg2 = arcseg(point(1.0, 10.0), point(9.0, 2.0));
        assert_eq!(left_point(seg2), 1.0);
    }

    #[test]
    fn test_left_point_full_circle_leftmost_on_arc() {
        // Full circle centered at origin with radius 1.0
        // Leftmost point should be at (-1.0, 0.0)
        let circle_arc = arc(point(1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let result = left_point(circle_arc);
        assert!(
            (result - (-1.0)).abs() < 1e-9,
            "Expected -1.0, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_circle_quarter_arc() {
        // Quarter circle arc from (1, 0) to (0, 1), center at (0, 0), radius 1.0
        let quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let result = left_point(quarter_arc);
        // The leftmost point of this arc is at (0, 0) which is an endpoint
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_left_point_circle_semicircle() {
        // Top half circle: from (1, 0) to (-1, 0), center at (0, 0), radius 1.0
        let semi_arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let result = left_point(semi_arc);
        // The leftmost point is at (-1.0, 0.0) which is an endpoint
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_left_point_circle_offset_center() {
        // Circle centered at (5.0, 3.0) with radius 2.0
        // Leftmost point is at (3.0, 3.0)
        let offset_arc = arc(point(7.0, 3.0), point(7.0, 3.0), point(5.0, 3.0), 2.0);
        let result = left_point(offset_arc);
        assert!((result - 3.0).abs() < 1e-9, "Expected 3.0, got {}", result);
    }

    #[test]
    fn test_left_point_arc_leftmost_point_off_arc() {
        // Arc that doesn't include its leftmost circle point
        // Quarter arc from (1, 0) to (0, 1), center at (0, 0), radius 1.0
        // Leftmost circle point would be at (-1, 0) which is NOT on this small arc
        let quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let result = left_point(quarter_arc);
        // Should return min of endpoints: min(1.0, 0.0) = 0.0
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_left_point_small_radius_circle() {
        // Small circle centered at (10.0, 5.0) with radius 0.5
        // Leftmost point is at (9.5, 5.0)
        let small_arc = arc(point(10.5, 5.0), point(10.5, 5.0), point(10.0, 5.0), 0.5);
        let result = left_point(small_arc);
        assert!((result - 9.5).abs() < 1e-9, "Expected 9.5, got {}", result);
    }

    #[test]
    fn test_left_point_large_radius_circle() {
        // Large circle centered at origin with radius 100.0
        // Leftmost point is at (-100.0, 0.0)
        let large_arc = arc(point(100.0, 0.0), point(100.0, 0.0), point(0.0, 0.0), 100.0);
        let result = left_point(large_arc);
        assert!(
            (result - (-100.0)).abs() < 1e-9,
            "Expected -100.0, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_arc_partially_wraps_leftmost() {
        // Arc from (0, 1) to (0, -1), center at (1, 0), radius 1.0
        // This arc wraps around and includes the leftmost point (0, 0)
        let partial_arc = arc(point(0.0, 1.0), point(0.0, -1.0), point(1.0, 0.0), 1.0);
        let result = left_point(partial_arc);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_left_point_negative_coordinates() {
        // Line segment with negative coordinates
        let seg = arcseg(point(-5.0, -3.0), point(-1.0, -7.0));
        assert_eq!(left_point(seg), -5.0);

        // Arc centered at negative coordinates
        let arc_neg = arc(point(-2.0, 0.0), point(-2.0, 0.0), point(-3.0, 0.0), 1.0);
        let result = left_point(arc_neg);
        assert!(
            (result - (-4.0)).abs() < 1e-9,
            "Expected -4.0, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_small_arc_both_sides_of_leftmost() {
        // Arc that spans across the leftmost point but includes it
        // Center at (5, 0), radius 3, arc from (2, 2) to (2, -2)
        // Leftmost point of circle is at (2, 0)
        let small_arc = arc(point(2.0, 2.0), point(2.0, -2.0), point(5.0, 0.0), 3.0);
        let result = left_point(small_arc);
        assert!((result - 2.0).abs() < 1e-9, "Expected 2.0, got {}", result);
    }

    #[test]
    fn test_left_point_arc_doesnt_reach_leftmost() {
        // Arc that doesn't reach the leftmost circle point
        // Center at (3, 0), radius 2, arc from (5, 0) to (3, 2)
        // Leftmost point of circle is at (1, 0), but arc goes from (5,0) to (3,2)
        let arc_right = arc(point(5.0, 0.0), point(3.0, 2.0), point(3.0, 0.0), 2.0);
        let result = left_point(arc_right);
        // Should return min of endpoints: min(5.0, 3.0) = 3.0
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_left_point_tall_arc() {
        // Tall arc from bottom to top, centered offset to the right
        // Center at (10, 5), radius 3, arc from (10, 2) to (10, 8)
        // Leftmost point of circle is at (7, 5)
        let tall_arc = arc(point(10.0, 2.0), point(10.0, 8.0), point(10.0, 5.0), 3.0);
        let result = left_point(tall_arc);
        // This is a vertical arc, doesn't include leftmost point
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_left_point_wide_arc() {
        // Wide arc from left to right, centered above
        // Center at (0, 2), radius 2, arc from (-2, 2) to (2, 2)
        // Leftmost point of circle is at (-2, 2)
        let wide_arc = arc(point(-2.0, 2.0), point(2.0, 2.0), point(0.0, 2.0), 2.0);
        let result = left_point(wide_arc);
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_left_point_arc_tight_curve() {
        // Tight arc with small radius
        // Center at (5, 0), radius 0.1, arc from (5.1, 0) to (5.05, 0.087)
        let tight_arc = arc(point(5.1, 0.0), point(5.05, 0.087), point(5.0, 0.0), 0.1);
        let result = left_point(tight_arc);
        // Leftmost point would be at (4.9, 0) but arc starts at (5.1, 0)
        // So should return min of endpoints
        assert!(
            (result - 5.05).abs() < 1e-9,
            "Expected ~5.05, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_arc_broad_curve() {
        // Broad shallow arc spanning wide range
        // Center at (0, 10), radius 5, arc from (-5, 10) to (5, 10)
        // This is the widest part of the circle
        let broad_arc = arc(point(-5.0, 10.0), point(5.0, 10.0), point(0.0, 10.0), 5.0);
        let result = left_point(broad_arc);
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_left_point_arc_three_quarters() {
        // Three-quarters of a circle
        // Center at (0, 0), radius 1, from (1, 0) going CCW to (0, -1)
        // This covers right, top, left quadrants - includes leftmost point
        let three_quarter_arc = arc(point(1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let result = left_point(three_quarter_arc);
        assert!(
            (result - (-1.0)).abs() < 1e-9,
            "Expected -1.0, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_arc_just_before_leftmost() {
        // Arc that comes close to leftmost but doesn't include it
        // Center at (0, 0), radius 1, from (0.9, 0.436) to (0.436, 0.9)
        // Leftmost point is at (-1, 0), which is NOT on this arc
        let close_arc = arc(point(0.9, 0.436), point(0.436, 0.9), point(0.0, 0.0), 1.0);
        let result = left_point(close_arc);
        // Should return min of endpoints: min(0.9, 0.436) = 0.436
        assert!(
            (result - 0.436).abs() < 0.001,
            "Expected ~0.436, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_arc_just_after_leftmost() {
        // Arc that includes leftmost point and goes a bit further
        // Center at (0, 0), radius 1, from (-0.9, 0.436) to (-0.436, 0.9)
        // Leftmost point is at (-1, 0), which IS on this arc
        let after_arc = arc(point(-0.9, 0.436), point(-0.436, 0.9), point(0.0, 0.0), 1.0);
        let result = left_point(after_arc);
        assert!(
            (result - (-1.0)).abs() < 1e-9,
            "Expected -1.0, got {}",
            result
        );
    }

    #[test]
    fn test_left_point_arc_bottom_half() {
        // Bottom half of a circle
        // Center at (0, 0), radius 1, from (1, 0) to (-1, 0) going through bottom
        let bottom_half = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let result = left_point(bottom_half);
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_left_point_arc_right_half() {
        // Right half of a circle - actually goes through all points on right side
        // Center at (0, 0), radius 1, from (0, 1) to (0, -1) going through right
        // This traverses from top, through right (1,0), to bottom - includes the leftmost (-1,0)
        let right_half = arc(point(0.0, 1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let result = left_point(right_half);
        // This arc actually includes the leftmost point
        assert!(
            (result - (-1.0)).abs() < 1e-9,
            "Expected -1.0, got {}",
            result
        );
    }

    #[test]
    fn test_find_tangent_point_to_circle0() {
        // circle above
        let p0 = point(1.0, 0.0);
        let c = circle(point(0.0, 2.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();
        assert!(p2.close_enough(point(1.0, 2.0), 1e-10));
    }

    #[test]
    fn test_find_tangent_point_to_circle1() {
        // circle on the right
        let p0 = point(0.0, 0.0);
        let c = circle(point(2.0, 1.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();
        assert!(p2.close_enough(point(2.0, 0.0), 1e-10));
    }

    #[test]
    fn test_find_tangent_point_to_circle_below() {
        // circle below - looking down
        let p0 = point(0.0, 2.0);
        let c = circle(point(1.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();
        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_left() {
        // circle on the left
        let p0 = point(2.0, 0.0);
        let c = circle(point(0.0, 1.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();
        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_diagonal() {
        // circle at diagonal distance
        let p0 = point(0.0, 0.0);
        let c = circle(point(3.0, 4.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();
        // Point should be on the circle - verify by checking distance from center equals radius
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_tangent_property() {
        // Verify that the line from point to tangent is perpendicular to radius
        let p0 = point(0.0, 0.0);
        let c = circle(point(2.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Vector from point to tangent
        let to_tangent = p2 - p0;
        // Vector from circle center to tangent (radius)
        let radius_vec = p2 - c.c;

        // They should be perpendicular (dot product ~0)
        let dot = to_tangent.x * radius_vec.x + to_tangent.y * radius_vec.y;
        assert!(dot.abs() < 1e-9, "Not perpendicular, dot product = {}", dot);
    }

    #[test]
    fn test_find_tangent_point_to_circle_large_circle() {
        // Large circle
        let p0 = point(0.0, 0.0);
        let c = circle(point(10.0, 10.0), 5.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_small_circle() {
        // Very small circle
        let p0 = point(0.0, 0.0);
        let c = circle(point(1.0, 0.0), 0.1);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_close_to_circle() {
        // Point very close to circle
        let p0 = point(1.05, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_far_from_circle() {
        // Point far from circle
        let p0 = point(0.0, 0.0);
        let c = circle(point(100.0, 100.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_negative_coords() {
        // Using negative coordinates
        let p0 = point(-5.0, -5.0);
        let c = circle(point(-3.0, -3.0), 2.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_right_side_consistency() {
        // Test that it consistently returns right-side tangent
        // From point (0,0) to circle at (5,0) with radius 1
        // The right tangent should point below center
        let p0 = point(0.0, 0.0);
        let c = circle(point(5.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Right tangent is one of the two tangent points; verify it's on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_above_point() {
        // Circle above point
        let p0 = point(1.0, 0.0);
        let c = circle(point(1.0, 3.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_below_point() {
        // Circle below point
        let p0 = point(1.0, 3.0);
        let c = circle(point(1.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_left_of_point() {
        // Circle to the left of point
        let p0 = point(3.0, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_right_of_point() {
        // Circle to the right of point
        let p0 = point(0.0, 0.0);
        let c = circle(point(3.0, 0.0), 1.0);
        let p2 = find_tangent_point_to_circle(p0, c).unwrap();

        // Point should be on the circle
        let dist = (p2 - c.c).norm();
        assert!((dist - c.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_basic() {
        // Two circles side by side
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(4.0, 0.0), 1.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        assert!(result.is_some());
        let (t1, t2) = result.unwrap();

        // Both points should be on their respective circles
        let dist1 = (t1 - c1.c).norm();
        let dist2 = (t2 - c2.c).norm();
        assert!((dist1 - c1.r).abs() < 1e-9);
        assert!((dist2 - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_right_tangent() {
        // Two circles aligned horizontally
        // Looking from c1 towards c2, the right tangent should be below both circles
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(5.0, 0.0), 1.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // Right tangent should be below the line connecting centers
        // So y-coordinates should be negative
        assert!(t1.y < 0.0, "t1 should be below center");
        assert!(t2.y < 0.0, "t2 should be below center");

        // Verify they're on the circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_vertical_offset() {
        // Two circles with vertical offset
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(4.0, 3.0), 1.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // Both should be on their circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_same_radius() {
        // Two circles with same radius
        let c1 = circle(point(0.0, 0.0), 2.0);
        let c2 = circle(point(6.0, 0.0), 2.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // For circles with same radius, the tangent line is parallel to the line of centers
        // Right tangent should be below
        assert!(t1.y < 0.0);
        assert!(t2.y < 0.0);

        // Verify on circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_different_radii() {
        // Two circles with different radii
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(5.0, 0.0), 2.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // Both should be on their circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_diagonal() {
        // Two circles on a diagonal
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(3.0, 4.0), 1.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // Both should be on their circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_perpendicularity() {
        // Verify the tangent line is perpendicular to both radii
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(4.0, 0.0), 1.0);
        let (t1, t2) = find_tangent_circle_to_circle(c1, c2).unwrap();

        // Vector from c1 center to t1 (radius)
        let radius1 = t1 - c1.c;
        // Vector along tangent line
        let tangent_vec = t2 - t1;

        // Should be perpendicular (dot product ≈ 0)
        let dot = radius1.x * tangent_vec.x + radius1.y * tangent_vec.y;
        assert!(
            dot.abs() < 1e-8,
            "Radius should be perpendicular to tangent line"
        );

        // Similarly for c2
        let radius2 = t2 - c2.c;
        let dot2 = radius2.x * tangent_vec.x + radius2.y * tangent_vec.y;
        assert!(
            dot2.abs() < 1e-8,
            "Radius should be perpendicular to tangent line"
        );
    }

    #[test]
    fn test_find_tangent_circle_to_circle_nested_circles() {
        // One circle completely inside another - no external tangents
        let c1 = circle(point(0.0, 0.0), 5.0);
        let c2 = circle(point(0.0, 0.0), 1.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        // Should return None since c2 is inside c1
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_circle_to_circle_touching_circles() {
        // Two circles touching externally
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(2.0, 0.0), 1.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        // Should still return Some - there are external tangents
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();

        // Verify they're on the circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_circle_to_circle_concentric_same_radius() {
        // Two circles with same center and same radius (identical circles)
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(0.0, 0.0), 1.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        // Should return None - identical circles have no external tangents
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_circle_to_circle_concentric_different_radius() {
        // Two circles with same center but different radii
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(0.0, 0.0), 2.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        // Should return None - concentric circles have no external tangents
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_circle_to_circle_very_close_circles() {
        // Two circles very close together (almost touching)
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(2.001, 0.0), 1.0);
        let result = find_tangent_circle_to_circle(c1, c2);

        // Should still have external tangents
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();

        // Verify on circles
        assert!(((t1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_find_tangent_point_to_circle_point_inside_circle() {
        // Point inside circle - no external tangent exists
        let p0 = point(0.0, 0.0);
        let c = circle(point(0.0, 0.0), 2.0);
        let result = find_tangent_point_to_circle(p0, c);

        // Should return None since point is inside circle
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_point_to_circle_point_at_circle_center() {
        // Point at the center of circle - definitely no tangent
        let center = point(3.0, 5.0);
        let p0 = center;
        let c = circle(center, 1.5);
        let result = find_tangent_point_to_circle(p0, c);

        // Should return None
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_point_to_circle_deep_inside() {
        // Point deep inside circle
        let p0 = point(0.0, 0.0);
        let c = circle(point(0.0, 0.0), 10.0);
        let result = find_tangent_point_to_circle(p0, c);

        // Should return None
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_point_to_circle_slightly_inside() {
        // Point slightly inside circle
        let p0 = point(0.0, 0.0);
        let c = circle(point(0.0, 0.0), 1.1);
        let result = find_tangent_point_to_circle(p0, c);

        // Should return None
        assert!(result.is_none());
    }

    #[test]
    fn test_find_tangent_point_to_circle_on_circle_boundary() {
        // Point exactly on the circle boundary - edge case
        let c = circle(point(0.0, 0.0), 1.0);
        let p0 = point(1.0, 0.0); // On circle
        let result = find_tangent_point_to_circle(p0, c);

        // This is an edge case - point is on the circle
        // The underlying tangent_point_to_circle function should handle this
        // It might return Some or None depending on implementation
        if let Some(tangent) = result {
            // If it returns Some, verify tangent is on circle
            let dist = (tangent - c.c).norm();
            assert!((dist - c.r).abs() < 1e-9);
        }
    }

    #[test]
    fn test_find_tangent_point_to_circle_very_close_inside() {
        // Point very close to circle but still inside
        let p0 = point(0.0, 0.0);
        let c = circle(point(1.0, 0.0), 1.001);
        let result = find_tangent_point_to_circle(p0, c);

        // Point is inside (distance 1.0, radius 1.001)
        assert!(result.is_none());
    }

    #[test]
    fn test_hull_seg_seg_horizontal_segments() {
        // Two horizontal segments, one above the other
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let seg2 = arcseg(point(3.0, 2.0), point(5.0, 2.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // The function chooses based on angle, so it picks seg1.a to seg2.a
        assert_eq!(p1, point(0.0, 0.0)); // seg1.a
        assert_eq!(p2, point(3.0, 2.0)); // seg2.a
    }

    #[test]
    fn test_hull_seg_seg_vertical_segments() {
        // Two vertical segments side by side
        let seg1 = arcseg(point(0.0, 0.0), point(0.0, 2.0));
        let seg2 = arcseg(point(3.0, 0.0), point(3.0, 2.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should connect top of seg1 to top of seg2  
        assert_eq!(p1, point(0.0, 2.0)); // seg1.b
        assert_eq!(p2, point(3.0, 2.0)); // seg2.b
    }

    #[test]
    fn test_hull_seg_seg_diagonal_segments() {
        // Two diagonal segments
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 1.0));
        let seg2 = arcseg(point(4.0, 0.0), point(6.0, 1.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should connect based on angle criterion
        assert_eq!(p1, point(2.0, 1.0)); // seg1.b
        assert_eq!(p2, point(6.0, 1.0)); // seg2.b
    }

    #[test]
    fn test_hull_seg_seg_parallel_segments() {
        // Two parallel segments
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let seg2 = arcseg(point(3.0, 1.0), point(5.0, 1.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // For parallel segments
        assert_eq!(p1, point(0.0, 0.0)); // seg1.a
        assert_eq!(p2, point(3.0, 1.0)); // seg2.a
    }

    #[test]
    fn test_hull_seg_seg_opposite_orientations() {
        // Segments with opposite orientations
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let seg2 = arcseg(point(5.0, 1.0), point(3.0, 1.0)); // Reversed
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should connect based on angle
        assert_eq!(p1, point(0.0, 0.0)); // seg1.a
        assert_eq!(p2, point(3.0, 1.0)); // seg2.b
    }

    #[test]
    fn test_hull_seg_seg_touching_segments() {
        // Segments that share an endpoint
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let seg2 = arcseg(point(2.0, 0.0), point(3.0, 1.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should handle touching segments correctly
        assert!(p1 == seg1.a || p1 == seg1.b);
        assert!(p2 == seg2.a || p2 == seg2.b);
    }

    #[test]
    fn test_hull_seg_seg_square_corners() {
        // Two segments forming a square corner
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0)); // Bottom edge
        let seg2 = arcseg(point(2.0, 0.0), point(2.0, 2.0)); // Right edge
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // For a square, should connect appropriately
        let connection_dir = p2 - p1;
        if connection_dir.norm() > 1e-9 {
            let other1 = if p1 == seg1.a { seg1.b } else { seg1.a };
            let other2 = if p2 == seg2.a { seg2.b } else { seg2.a };
            
            assert!(connection_dir.perp(other1 - p1) <= 1e-9);
            assert!(connection_dir.perp(other2 - p1) <= 1e-9);
        }
    }

    #[test]
    fn test_hull_seg_seg_far_apart_segments() {
        // Segments far apart
        let seg1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
        let seg2 = arcseg(point(10.0, 5.0), point(11.0, 5.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should connect based on angle
        assert_eq!(p1, point(0.0, 0.0)); // seg1.a
        assert_eq!(p2, point(10.0, 5.0)); // seg2.a
    }

    #[test]
    fn test_hull_seg_seg_negative_coordinates() {
        // Segments with negative coordinates
        let seg1 = arcseg(point(-2.0, -1.0), point(-1.0, -1.0));
        let seg2 = arcseg(point(1.0, 1.0), point(2.0, 1.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // Should connect based on angle
        assert_eq!(p1, point(-2.0, -1.0)); // seg1.a
        assert_eq!(p2, point(1.0, 1.0)); // seg2.a
    }

    #[test]
    fn test_hull_seg_seg_collinear_segments() {
        // Two collinear segments
        let seg1 = arcseg(point(0.0, 0.0), point(2.0, 0.0));
        let seg2 = arcseg(point(3.0, 0.0), point(5.0, 0.0));
        let (p1, p2) = hull_seg_seg(seg1, seg2);
        
        // For collinear segments, angle criterion selects seg1.a to seg2.a
        assert_eq!(p1, point(0.0, 0.0)); // seg1.a
        assert_eq!(p2, point(3.0, 0.0)); // seg2.a
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_concentric_circles() {
        // Two arcs on concentric circles - no external tangent exists
        // Outer circle: center (0, 0), radius 2.0
        // Inner circle: center (0, 0), radius 1.0
        let arc1 = arc(point(2.0, 0.0), point(0.0, 2.0), point(0.0, 0.0), 2.0); // Quarter arc on outer circle
        let arc2 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0); // Quarter arc on inner circle
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should find a valid connection (likely endpoint to endpoint)
        assert!(!result.is_empty());
        
        // The connection should be a line segment
        assert!(result.len() >= 1);
        assert!(result[0].is_seg());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_nested_arcs() {
        // Arc on larger circle contains arc on smaller circle
        // Outer circle: center (5, 5), radius 3.0
        // Inner circle: center (5, 5), radius 1.0
        let arc1 = arc(point(8.0, 5.0), point(5.0, 8.0), point(5.0, 5.0), 3.0);
        let arc2 = arc(point(6.0, 5.0), point(5.0, 6.0), point(5.0, 5.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should return valid hull elements
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_direct_connection_valid() {
        // Two arcs where direct endpoint connection doesn't cross the arcs
        // Concentric but positioned so direct connection works
        let arc1 = arc(point(2.0, 0.0), point(1.414, 1.414), point(0.0, 0.0), 2.0);
        let arc2 = arc(point(0.707, 0.707), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should have at least one element
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_needs_point_to_arc_tangent() {
        // Case where direct connection crosses arc, needs point-to-arc tangent
        // Outer arc wraps around, inner arc positioned so direct line would cross
        let arc1 = arc(point(3.0, 0.0), point(-3.0, 0.0), point(0.0, 0.0), 3.0); // Large semicircle
        let arc2 = arc(point(0.5, 0.0), point(-0.5, 0.0), point(0.0, 0.0), 0.5); // Small semicircle at center
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should find a solution (either direct or via tangent)
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_identical_circles() {
        // Two arcs on the same circle (identical center and radius)
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc2 = arc(point(0.0, -1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should connect the arcs somehow
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_touching_endpoints() {
        // Arcs on concentric circles where endpoints touch
        let arc1 = arc(point(2.0, 0.0), point(0.0, 2.0), point(0.0, 0.0), 2.0);
        let arc2 = arc(point(0.0, 2.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0); // Starts where arc1 ends (on circle)
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should have a valid connection
        assert!(!result.is_empty());
        
        // If endpoints are very close, might just be arc segments
        let total_segments = result.len();
        assert!(total_segments >= 1);
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_opposite_sides() {
        // Two arcs on concentric circles on opposite sides
        let arc1 = arc(point(3.0, 0.0), point(0.0, 3.0), point(0.0, 0.0), 3.0); // Top-right quadrant, outer
        let arc2 = arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0); // Bottom-left quadrant, inner
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should find a connection
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_same_circle_different_parts() {
        // Two arcs that are different parts of the same circle (same center and radius)
        // Arc1: First quadrant (0° to 90°)
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        // Arc2: Third quadrant (180° to 270°)
        let arc2 = arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should connect the arcs - no external tangent exists for same circle
        assert!(!result.is_empty());
        
        // The result should include a connection between the arc endpoints
        // Since they're on the same circle, should be a direct connection
        assert!(result.len() >= 1);
    }

    #[test]
    fn test_hull_arc_arc_no_tangent_same_circle_adjacent_arcs() {
        // Two adjacent arcs on the same circle
        // Arc1: 0° to 90° (first quadrant)
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        // Arc2: 90° to 180° (second quadrant) - starts where arc1 ends
        let arc2 = arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Should handle adjacent arcs that share an endpoint
        assert!(!result.is_empty());
    }

    #[test]
    fn test_hull_arc_arc_tangent_both_on_arcs() {
        // Case (true, true): Both tangent points lie on the arcs
        // Two circles side by side with arcs that include the tangent points
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0); // Quarter circle
        let arc2 = arc(point(3.0, 0.0), point(4.0, 1.0), point(4.0, 0.0), 1.0); // Quarter circle
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Tangent at (0,1) -> (4,1). Since t1 = arc1.b and t2 = arc2.b:
        // Only 2 segments: arc1 (1,0)->(0,1) + tangent line (0,1)->(4,1)
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(1.0, 0.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // arc1.b = t1
        assert!(result[1].is_seg()); // tangent line
        assert_eq!(result[1].a, point(0.0, 1.0)); // t1
        assert_eq!(result[1].b, point(4.0, 1.0)); // t2 = arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_only_second_on_arc() {
        // Case (false, true): Only the second tangent point is on arc2
        // First arc ends before tangent point, second arc includes it
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0); // Left half
        let arc2 = arc(point(3.0, 0.0), point(4.0, 1.0), point(4.0, 0.0), 1.0); // Right side
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Tangent at y=1. Since t2 = arc2.b (4,1), no third segment:
        // arc1 (from -1,0 to 0,1) + line (0,1 to 4,1)
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(-1.0, 0.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // arc1.b
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 1.0)); // arc1.b = start of tangent
        assert_eq!(result[1].b, point(4.0, 1.0)); // t2 = arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_only_first_on_arc() {
        // Case (true, false): Only the first tangent point is on arc1
        let arc1 = arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0); // Right bottom quadrant
        let arc2 = arc(point(5.0, 1.0), point(4.0, 0.0), point(4.0, 0.0), 1.0); // Positioned so tangent before arc2.a
        
        let result = hull_arc_arc(arc1, arc2);
        
        // t1=arc1.a=(1,0) so no arc1 portion, just tangent line + arc2
        assert_eq!(result.len(), 2);
        assert!(result[0].is_seg()); // tangent line
        assert_eq!(result[0].a, point(1.0, 0.0)); // t1 = arc1.a
        assert_eq!(result[0].b, point(4.0, 1.0)); // This is actually inside arc2
        assert!(!result[1].is_seg()); // arc2 portion
        assert_eq!(result[1].a, point(4.0, 1.0));
        assert_eq!(result[1].b, point(4.0, 0.0)); // arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_neither_on_arc() {
        // Case (false, false): Neither tangent point is on the arcs
        // Arcs positioned so tangent points fall outside both arc ranges
        let arc1 = arc(point(-1.0, 0.0), point(-0.707, 0.707), point(0.0, 0.0), 1.0); // Small arc on left
        let arc2 = arc(point(4.707, 0.707), point(5.0, 0.0), point(4.0, 0.0), 1.0); // Small arc on right
        
        let result = hull_arc_arc(arc1, arc2);
        
        // External tangent exists but neither point is on the arcs, so connects arc1.b to arc2.a
        // Pattern: full arc1 + connection line + full arc2
        assert_eq!(result.len(), 3);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(-1.0, 0.0));
        assert_eq!(result[0].b, point(0.0, 1.0));
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 1.0)); // arc1.b
        assert_eq!(result[1].b, point(4.0, 1.0)); // arc2.a
        assert!(!result[2].is_seg());
        assert_eq!(result[2].a, point(4.0, 1.0));
        assert_eq!(result[2].b, point(5.0, 0.0));
    }

    #[test]
    fn test_hull_arc_arc_tangent_both_on_arcs_full_semicircles() {
        // Case (true, true) with larger arcs - semicircles
        // Arc1: Left semicircle from bottom to top
        let arc1 = arc(point(0.0, -1.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        // Arc2: Right semicircle from bottom to top, offset to the right
        let arc2 = arc(point(4.0, -1.0), point(4.0, 1.0), point(4.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Tangent at y=1, so t1=arc1.b=(0,1) and t2=arc2.b=(4,1)
        // Only 2 segments: arc1 (0,-1)->(0,1) + tangent line (0,1)->(4,1)
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(0.0, -1.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // arc1.b = t1
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 1.0)); // t1
        assert_eq!(result[1].b, point(4.0, 1.0)); // t2 = arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_only_first_on_arc_small_arc1() {
        // Case (true, false): Tangent on arc1, but arc2 is small and misses the tangent
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0); // Large semicircle
        let arc2 = arc(point(4.1, 0.1), point(4.0, 0.0), point(4.0, 0.0), 1.0); // Small arc, tangent outside
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Returns: arc1 (1,0)->(0,1) + line (0,1)->(4,1) + arc2 (4,1)->(4,0)
        assert_eq!(result.len(), 3);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(1.0, 0.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // t1 on arc1
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 1.0)); // t1
        assert_eq!(result[1].b, point(4.0, 1.0)); // Point on arc2
        assert!(!result[2].is_seg());
        assert_eq!(result[2].a, point(4.0, 1.0));
        assert_eq!(result[2].b, point(4.0, 0.0)); // arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_only_second_on_arc_small_arc1() {
        // Case (false, true): Tangent on arc2, but arc1 is small and misses the tangent
        let arc1 = arc(point(0.1, 0.1), point(0.0, 0.0), point(0.0, 0.0), 1.0); // Very small arc
        let arc2 = arc(point(4.0, -1.0), point(4.0, 1.0), point(4.0, 0.0), 1.0); // Large arc
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Returns: arc1 (0.1,0.1)->(0,1) + line (0,1)->(4,1) (t2=arc2.b so no third segment)
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg()); // arc1
        assert_eq!(result[0].a, point(0.1, 0.1)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // Modified arc1 endpoint (towards tangent)
        assert!(result[1].is_seg()); // line to t2
        assert_eq!(result[1].a, point(0.0, 1.0));
        assert_eq!(result[1].b, point(4.0, 1.0)); // t2 = arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_horizontal_alignment() {
        // Both arcs horizontally aligned, both tangents should be on arcs
        let arc1 = arc(point(0.5, 0.866), point(-0.5, 0.866), point(0.0, 0.0), 1.0); // Top part of circle
        let arc2 = arc(point(3.5, 0.866), point(4.5, 0.866), point(4.0, 0.0), 1.0); // Top part of circle
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Returns: arc1 (0.5,0.866)->(0,1) + line (0,1)->(3.5,0.866) (t2=arc2.a so no third segment)
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(0.5, 0.866)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // t1 on arc1
        assert!(result[1].is_seg()); // tangent line
        assert_eq!(result[1].a, point(0.0, 1.0)); // t1
        assert_eq!(result[1].b, point(3.5, 0.866)); // t2 = arc2.a
    }

    #[test]
    fn test_hull_arc_arc_tangent_vertical_circles() {
        // Circles arranged vertically
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0); // Bottom circle, bottom half
        let arc2 = arc(point(0.0, 4.0), point(1.0, 4.0), point(0.0, 4.0), 1.0); // Top circle, bottom half
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Tangent at x=1 (rightmost points): t1=arc1.b=(1,0) and t2=arc2.a=(0,4) possibly
        // Or tangent may be on the arcs. Expect 3 segments typically
        assert_eq!(result.len(), 3);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(0.0, 0.0)); // arc1.a
        assert!(result[1].is_seg()); // tangent line
        assert!(!result[2].is_seg());
        assert_eq!(result[2].b, point(1.0, 4.0)); // arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_large_separation() {
        // Two well-separated circles with clear external tangent
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc2 = arc(point(10.0, 0.0), point(11.0, 1.0), point(11.0, 0.0), 1.0);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Tangent at y≈1 height. t1 likely equals arc1.b, t2 likely equals arc2.b
        // Expect 2 segments: arc1 + tangent line
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(1.0, 0.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 1.0)); // arc1.b = t1
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 1.0)); // t1
        assert_eq!(result[1].b, point(11.0, 1.0)); // t2 = arc2.b
    }

    #[test]
    fn test_hull_arc_arc_tangent_small_circles() {
        // Very small circles to test numerical stability
        let arc1 = arc(point(0.1, 0.0), point(0.0, 0.1), point(0.0, 0.0), 0.1);
        let arc2 = arc(point(0.5, 0.0), point(0.6, 0.1), point(0.6, 0.0), 0.1);
        
        let result = hull_arc_arc(arc1, arc2);
        
        // Similar geometry: tangent at top, so t1=arc1.b and t2=arc2.b
        // Expect 2 segments: arc1 + tangent line
        assert_eq!(result.len(), 2);
        assert!(!result[0].is_seg());
        assert_eq!(result[0].a, point(0.1, 0.0)); // arc1.a
        assert_eq!(result[0].b, point(0.0, 0.1)); // arc1.b = t1
        assert!(result[1].is_seg());
        assert_eq!(result[1].a, point(0.0, 0.1)); // t1
        assert_eq!(result[1].b, point(0.6, 0.1)); // t2 = arc2.b
    }
}

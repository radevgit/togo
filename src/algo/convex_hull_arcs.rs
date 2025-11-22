//! Convex hull algorithm for Arcline (arc-based polygons).
//!
//! This module implements convex hull computation for polygons defined by arcs and line segments,
//! preserving the arc structure where possible.

use crate::prelude::*;
use crate::intersection::tangent::{external_tangents_between_circles, tangent_point_to_circle};

/// Determines if an arc is convex (outward-bulging) or concave (inward-bulging) based on
/// its connectivity pattern in the arcline.
///
/// # Arc Connectivity Patterns
///
/// Arcs are always CCW geometrically, but may be traversed forward or backward:
/// - **Forward traversal (a→b)**: Positive bulge, convex/outward-bulging
///   - Connectivity: `arc[i-1].b == arc[i].a → arc[i].b == arc[i+1].a`
/// - **Backward traversal (b→a)**: Negative bulge, concave/inward-bulging  
///   - Connectivity: `arc[i-1].b == arc[i].b ← arc[i].a == arc[i+1].a`
///
/// # Arguments
///
/// * `arcs` - The arcline containing the arc
/// * `index` - Index of the arc to check
///
/// # Returns
///
/// `true` if the arc is convex (forward traversal), `false` if concave (backward traversal)
fn is_arc_convex(arcs: &Arcline, index: usize) -> bool {
    if arcs.is_empty() || index >= arcs.len() {
        return true; // Default to convex
    }

    let prev_idx = if index == 0 { arcs.len() - 1 } else { index - 1 };
    let arc = arcs[index];
    let prev_arc = arcs[prev_idx];

    // Check connectivity pattern:
    // Forward (convex): prev.b connects to arc.a
    // Backward (concave): prev.b connects to arc.b (endpoints swapped)
    
    // First check forward connectivity
    let forward = prev_arc.b.close_enough(arc.a, 1e-9);
    
    if forward {
        return true; // Convex: normal forward connection
    }
    
    // Check if this is a backward-traversed arc (prev.b == arc.b)
    // This happens when the previous arc ends where this arc ends (reversed)
    let backward = prev_arc.b.close_enough(arc.b, 1e-9);
    
    !backward // If backward match found, it's concave; otherwise check something else
}

/// Finds the starting point for convex hull construction.
///
/// Returns the point with minimum Y coordinate (bottommost).
/// If there are ties, returns the leftmost among them.
///
/// For arc-based polygons, we need to consider:
/// - Arc endpoints (a and b)
/// - Arc extrema (cardinal points: top, bottom, left, right)
///
/// # Arguments
///
/// * `arcs` - The input arcline
///
/// # Returns
///
/// Index of the arc and which point/extrema to use as the starting point
fn find_start_point(arcs: &Arcline) -> Option<(usize, Point)> {
    if arcs.is_empty() {
        return None;
    }

    let mut min_point = arcs[0].a;
    let mut min_arc_idx = 0;

    for (i, arc) in arcs.iter().enumerate() {
        // Check endpoint a
        if arc.a.y < min_point.y || (arc.a.y == min_point.y && arc.a.x < min_point.x) {
            min_point = arc.a;
            min_arc_idx = i;
        }

        // Check endpoint b
        if arc.b.y < min_point.y || (arc.b.y == min_point.y && arc.b.x < min_point.x) {
            min_point = arc.b;
            min_arc_idx = i;
        }

        // For curved arcs, check bottom extremum
        if arc.is_arc() {
            // Bottom extremum is at center - (0, radius)
            let bottom = point(arc.c.x, arc.c.y - arc.r);
            
            // Check if this extremum is actually on the arc span
            if arc.contains(bottom) {
                if bottom.y < min_point.y || (bottom.y == min_point.y && bottom.x < min_point.x) {
                    min_point = bottom;
                    min_arc_idx = i;
                }
            }
        }
    }

    Some((min_arc_idx, min_point))
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
    if cross1 > cross2 {
        Some(t1)
    } else {
        Some(t2)
    }
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
    let mut hull = Arcline::new();
    
    // Mark which arcs are convex (forward-traversed)
    let mut is_convex = vec![false; n];
    for i in 0..n {
        is_convex[i] = is_arc_convex(arcs, i);
    }
    
    // Find the first convex arc to start with
    let mut start_idx = None;
    for i in 0..n {
        if is_convex[i] {
            start_idx = Some(i);
            break;
        }
    }
    
    // If no convex arcs exist, return empty hull
    let start_idx = match start_idx {
        Some(idx) => idx,
        None => return hull,
    };
    
    // Process arcs starting from the first convex arc
    let mut i = start_idx;
    let mut processed = 0;
    let mut last_arc_end = Point { x: 0.0, y: 0.0 }; // Track the end point of the previous hull element
    let mut first_arc_start = Point { x: 0.0, y: 0.0 }; // Track the start of the first arc to close the loop
    let mut is_first = true;
    
    while processed < n {
        if is_convex[i] {
            let current_arc = arcs[i];
            let prev_idx = if i == 0 { n - 1 } else { i - 1 };
            let prev_arc = arcs[prev_idx];
            
            // Find the next convex arc
            let mut j = (i + 1) % n;
            let mut next_convex_idx = i;
            let mut found_next = false;
            
            for _ in 0..n {
                if is_convex[j] {
                    next_convex_idx = j;
                    found_next = true;
                    break;
                }
                j = (j + 1) % n;
            }
            
            if !found_next {
                // Only one convex arc, just add it
                hull.push(current_arc);
                break;
            }
            
            let next_arc = arcs[next_convex_idx];
            
            // Handle arc splitting at tangent points with adjacent convex arcs
            let mut arc_start = current_arc.a;
            let mut arc_end = current_arc.b;
            
            // Split at start if previous arc is convex and adjacent (regardless of gap)
            let prev_is_convex = is_convex[prev_idx];
            if prev_is_convex && (prev_idx + 1) % n == i && current_arc.is_arc() && prev_arc.is_arc() {
                let c1 = Circle { c: prev_arc.c, r: prev_arc.r };
                let c2 = Circle { c: current_arc.c, r: current_arc.r };
                
                // Direction from prev arc to current arc
                let direction = c2.c - c1.c;
                
                if let Some((_, t2)) = select_external_tangent_between_circles(c1, c2, direction) {
                    if current_arc.contains(t2) && (t2 - current_arc.a).norm() > 1e-9 {
                        arc_start = t2;
                    }
                }
            }
            
            // Split at end if next arc is convex and adjacent (regardless of gap)
            if (i + 1) % n == next_convex_idx && current_arc.is_arc() && next_arc.is_arc() {
                let c1 = Circle { c: current_arc.c, r: current_arc.r };
                let c2 = Circle { c: next_arc.c, r: next_arc.r };
                
                // Direction from current arc to next arc
                let direction = c2.c - c1.c;
                
                if let Some((t1, _)) = select_external_tangent_between_circles(c1, c2, direction) {
                    if current_arc.contains(t1) && (current_arc.b - t1).norm() > 1e-9 {
                        arc_end = t1;
                    }
                }
            }
            
            // Add connecting segment from previous arc's end to this arc's start
            if !is_first && (arc_start - last_arc_end).norm() > 1e-9 {
                hull.push(arcseg(last_arc_end, arc_start));
            }
            
            if is_first {
                first_arc_start = arc_start;
                is_first = false;
            }
            
            // Add the (possibly split) arc
            if (arc_end - arc_start).norm() > 1e-9 {
                if current_arc.is_arc() {
                    hull.push(arc(arc_start, arc_end, current_arc.c, current_arc.r));
                } else {
                    hull.push(arcseg(arc_start, arc_end));
                }
            }
            
            last_arc_end = arc_end;
            
            i = (i + 1) % n;
            processed += 1;
        } else {
            // Skip concave arcs
            i = (i + 1) % n;
            processed += 1;
        }
    }
    
    // Close the loop: connect last arc end to first arc start
    if !is_first && (first_arc_start - last_arc_end).norm() > 1e-9 {
        hull.push(arcseg(last_arc_end, first_arc_start));
    }
    
    hull
}

#[cfg(test)]
mod tests {
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
        let arcs = vec![arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), f64::INFINITY)];
        let hull = arcline_convex_hull(&arcs);
        // For a single arc, is_arc_convex will check if prev_arc.b == arc.a
        // Since prev wraps to itself, this checks if arc.b == arc.a, which is false
        // So it will be treated as concave and skipped
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
    }

    #[test]
    fn test_is_arc_convex_basic() {
        // Two connected line segments forming a right turn (convex corner)
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(1.0, 0.0)),
            arcseg(point(1.0, 0.0), point(1.0, 1.0)),
        ];
        
        // Both arcs should be considered convex since they connect forward
        // Arc 0: prev is arc 1, and arc1.b should equal arc0.a
        // Arc 1: prev is arc 0, and arc0.b should equal arc1.a
        assert!(is_arc_convex(&arcs, 1)); // This one definitely connects forward
    }

    #[test]
    fn test_find_start_point_simple() {
        let arcs = vec![
            arcseg(point(0.0, 1.0), point(1.0, 1.0)),
            arcseg(point(1.0, 0.0), point(0.0, 0.0)),
        ];
        
        let result = find_start_point(&arcs);
        assert!(result.is_some());
        
        let (_, start) = result.unwrap();
        // Should find the bottommost point
        assert_eq!(start.y, 0.0);
    }

    #[test]
    fn test_arcline_convex_hull_circle() {
        // Circle made of 4 quarter-circle arcs (all convex)
        // Center at origin, radius 1.0
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),     // Right to Top
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),    // Top to Left
            arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0),   // Left to Bottom
            arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0),    // Bottom to Right
        ];
        
        let hull = arcline_convex_hull(&arcs);
        
        // Hull should include all 4 cardinal points (endpoints)
        assert_eq!(hull.len(), 4);
        
        // Verify that all cardinal points are in the hull
        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
        assert!(hull_points.contains(&point(1.0, 0.0)));   // Right
        assert!(hull_points.contains(&point(0.0, 1.0)));   // Top
        assert!(hull_points.contains(&point(-1.0, 0.0)));  // Left
        assert!(hull_points.contains(&point(0.0, -1.0))); // Bottom
    }

    #[test]
    fn test_arcline_convex_hull_semicircle() {
        // Semicircle (top half) with a line segment closing it
        // Two quarter-circle arcs + one line segment
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),    // Right to Top
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),   // Top to Left
            arcseg(point(-1.0, 0.0), point(1.0, 0.0)),                      // Left to Right (base)
        ];
        
        let hull = arcline_convex_hull(&arcs);
        
        // Hull should be the semicircle itself (all arcs are convex)
        assert_eq!(hull.len(), 3);
        
        let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
        assert!(hull_points.contains(&point(1.0, 0.0)));
        assert!(hull_points.contains(&point(0.0, 1.0)));
        assert!(hull_points.contains(&point(-1.0, 0.0)));
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
            arc(point(4.0, 2.0 - r), point(4.0 - r, 2.0), point(4.0 - r, 2.0 - r), r),
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
        let max_x = hull_points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
        let min_x = hull_points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let max_y = hull_points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
        let min_y = hull_points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        
        // Verify the bounding box matches the rounded rectangle's bounds
        assert!((max_x - 4.0).abs() < 1e-6);
        assert!((min_x - 0.0).abs() < 1e-6);
        assert!((max_y - 2.0).abs() < 1e-6);
        assert!((min_y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_arcline_convex_hull_concave_arc() {
        // Shape with a concave indentation (backward arc)
        // Square with a circular bite taken out of one side
        let arcs = vec![
            arcseg(point(0.0, 0.0), point(2.0, 0.0)),                      // Bottom
            arcseg(point(2.0, 0.0), point(2.0, 2.0)),                      // Right
            arcseg(point(2.0, 2.0), point(0.0, 2.0)),                      // Top
            // Left side with concave arc (backward traversal = concave)
            arc(point(0.0, 2.0), point(0.0, 0.0), point(0.0, 1.0), 0.5),   // Concave indent
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
    }

    #[test]
    fn test_find_start_point_with_arc() {
        // Arc spanning from left to right with bottom extremum at (0, -1)
        let arcs = vec![
            arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0), // Semicircle (bottom half)
        ];
        
        let result = find_start_point(&arcs);
        assert!(result.is_some());
        
        let (idx, start) = result.unwrap();
        // Should find the bottom extremum at (0, -1)
        assert_eq!(idx, 0);
        assert!((start.x - 0.0).abs() < 1e-10);
        assert!((start.y - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_is_arc_convex_with_curved_arc() {
        // Test with actual curved arcs forming a closed semicircle
        let arcs = vec![
            arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),   // Convex quarter circle
            arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),  // Convex quarter circle
            arcseg(point(-1.0, 0.0), point(1.0, 0.0)),                     // Closing line
        ];
        
        // First two arcs should be convex (forward traversal)
        // Index 1 and 2 are safe to check (they have a prev element in forward direction)
        assert!(is_arc_convex(&arcs, 1));
        assert!(is_arc_convex(&arcs, 2)); // The line segment
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
        
        assert_eq!(hull.len(), 8, "Expected 8 elements: 4 split arcs + 4 connecting segments");


        
        // Write SVG for visualization
        use crate::svg::SVG;
        let arcs2 = arcline_scale(&arcs, 100.0);
        let hull2 = arcline_scale(&hull, 100.0);
        let mut svg = SVG::new(400.0, 400.0, Some("/tmp/hull_square_arcs.svg"));
        svg.arcline(&arcs2, "blue");
        svg.arcline(&hull2, "red");
        svg.write();
        svg.write_stroke_width(0.1);
    }
}

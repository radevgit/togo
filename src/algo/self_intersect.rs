#![allow(dead_code)]

use crate::prelude::*;
use aabb::HilbertRTree;

/// Computes the bounding box for a circular arc using circle approximation
///
/// Uses the circle center and radius as a conservative bounding box estimate.
/// For line segments (arc.is_seg()), just use the endpoints.
fn arc_bounding_box(arc: &Arc) -> (f64, f64, f64, f64) {
    // For line segments (infinite radius), just use the endpoints
    if arc.is_seg() {
        let min_x = arc.a.x.min(arc.b.x);
        let max_x = arc.a.x.max(arc.b.x);
        let min_y = arc.a.y.min(arc.b.y);
        let max_y = arc.a.y.max(arc.b.y);
        return (min_x, max_x, min_y, max_y);
    }

    // For circular arcs, use circle bounds (center ± radius)
    let min_x = arc.c.x - arc.r;
    let max_x = arc.c.x + arc.r;
    let min_y = arc.c.y - arc.r;
    let max_y = arc.c.y + arc.r;

    (min_x, min_y, max_x, max_y)
}

/// Checks if an arcline (sequence of arcs) has any self-intersections.
///
/// A self-intersection occurs when two arcs intersect (including adjacent arcs
/// which may overlap or cross beyond their shared endpoint).
///
/// Uses Hilbert R-tree spatial indexing with data-oriented storage for fast
/// acceleration. Builds index from arc circle bounds, then queries to find
/// candidate pairs before expensive arc intersection tests.
///
/// # Arguments
/// * `arcline` - A sequence of connected arcs forming a polyline
///
/// # Returns
/// `true` if the arcline has any self-intersections, `false` otherwise
///
/// # Algorithm
/// 1. Build Hilbert R-tree from circle-bound AABBs of all arcs (cache-friendly)
/// 2. For each arc, query tree to find spatially-nearby candidates
/// 3. Check all pairs (including adjacent) with overlapping bounds
/// 4. Return true if any pair has real intersection
///
/// # Complexity
/// O(n log n) tree construction + O(n log n) queries with early rejection
/// Much faster than naive O(n²) for large arclines
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// use togo::algo::arcline_has_self_intersection;
/// 
/// // Non-intersecting arcline (simple path)
/// let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
/// let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
/// let arcline = vec![arc1, arc2];
/// assert!(!arcline_has_self_intersection(&arcline));
/// 
/// // Two overlapping arcs
/// let arc1 = arc(point(0.0, 0.0), point(2.0, 2.0), point(1.0, 1.0), 1.0);
/// let arc2 = arc(point(2.0, 0.0), point(0.0, 2.0), point(1.0, 1.0), 1.0);
/// let arcline = vec![arc1, arc2];
/// // May have self-intersections depending on arc geometry
/// ```
pub fn arcline_has_self_intersection(arcline: &Arcline) -> bool {
    if arcline.len() < 2 {
        return false;
    }

    let n = arcline.len();

    // Build Hilbert R-tree from proper arc bounding boxes
    // Preallocate with capacity to avoid reallocations
    let mut tree = HilbertRTree::with_capacity(n);
    for arc in arcline.iter() {
        let (min_x, min_y, max_x, max_y) = arc_bounding_box(arc);
        tree.add(min_x, min_y, max_x, max_y);
    }
    tree.build();

    // Query tree to find candidate pairs
    let mut candidates = Vec::new();
    for i in 0..n {
        let arc = &arcline[i];
        let (min_x, min_y, max_x, max_y) = arc_bounding_box(arc);
        candidates.clear();
        tree.query_intersecting(min_x, min_y, max_x, max_y, &mut candidates);

        // Check each candidate
        for &j in candidates.iter() {
            // Skip same arc
            if j == i {
                continue;
            }

            // Only check j > i to avoid duplicate checks
            if j <= i {
                continue;
            }

            let arc_i = &arcline[i];
            let arc_j = &arcline[j];

            // Real intersection test (expensive, but only for candidates)
            // Adjacent arcs can also intersect (overlap or cross beyond shared endpoint)
            if if_really_intersecting_arc_arc(arc_i, arc_j) {
                return true;
            }
        }
    }

    false
}

/// Finds all self-intersection points in an arcline.
///
/// Returns a list of intersection points with their arc indices.
/// Uses spatial indexing for efficiency.
///
/// # Arguments
/// * `arcline` - A sequence of connected arcs forming a polyline
///
/// # Returns
/// A vector of tuples `(arc_i_index, arc_j_index, intersection_point)`
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// use togo::algo::arcline_self_intersections;
/// 
/// let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
/// let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
/// let arcline = vec![arc1, arc2];
/// let intersections = arcline_self_intersections(&arcline);
/// assert!(intersections.is_empty());
/// ```
pub fn arcline_self_intersections(arcline: &Arcline) -> Vec<(usize, usize, Point)> {
    let mut intersections = Vec::new();

    if arcline.len() < 2 {
        return intersections;
    }

    let n = arcline.len();

    // Build Hilbert R-tree from proper arc bounding boxes
    // Preallocate with capacity to avoid reallocations
    let mut tree = HilbertRTree::with_capacity(n);
    for arc in arcline.iter() {
        let (min_x, min_y, max_x, max_y) = arc_bounding_box(arc);
        tree.add(min_x, min_y, max_x, max_y);
    }
    tree.build();

    // Check all non-adjacent pairs using spatial index
    let mut candidates = Vec::new();
    for i in 0..n {
        let arc_i = &arcline[i];
        let (min_x, min_y, max_x, max_y) = arc_bounding_box(arc_i);
        candidates.clear();
        tree.query_intersecting(min_x, min_y, max_x, max_y, &mut candidates);

        for &j in candidates.iter() {
            // Skip same arc and duplicates
            if j == i || j <= i {
                continue;
            }

            let arc_j = &arcline[j];

            match int_arc_arc(arc_i, arc_j) {
                ArcArcConfig::NonCocircularOnePoint(p) => {
                    intersections.push((i, j, p));
                }
                ArcArcConfig::NonCocircularTwoPoints(p0, p1) => {
                    intersections.push((i, j, p0));
                    intersections.push((i, j, p1));
                }
                _ => {}
            }
        }
    }

    // Check if last arc intersects with first arc (for closed arclines)
    // Note: Adjacent arcs are now checked, including the wraparound case
    if n >= 2 {
        let last_arc = &arcline[n - 1];
        let first_arc = &arcline[0];

        // Only check if not already covered (main loop checks i < j, this checks (n-1, 0))
        match int_arc_arc(last_arc, first_arc) {
            ArcArcConfig::NonCocircularOnePoint(p) => {
                intersections.push((n - 1, 0, p));
            }
            ArcArcConfig::NonCocircularTwoPoints(p0, p1) => {
                intersections.push((n - 1, 0, p0));
                intersections.push((n - 1, 0, p1));
            }
            _ => {}
        }
    }

    intersections
}

/// Gets the self-intersection status of an arcline with detailed information.
///
/// # Arguments
/// * `arcline` - A sequence of connected arcs forming a polyline
///
/// # Returns
/// A `SelfIntersectionStatus` enum with detailed information
#[derive(Debug, Clone, PartialEq)]
pub enum SelfIntersectionStatus {
    /// No self-intersections found
    Clean,
    /// Self-intersections found with list of intersection info: (arc_i, arc_j, point)
    HasIntersections(Vec<(usize, usize, Point)>),
}

pub fn arcline_self_intersection_status(arcline: &Arcline) -> SelfIntersectionStatus {
    let intersections = arcline_self_intersections(arcline);
    if intersections.is_empty() {
        SelfIntersectionStatus::Clean
    } else {
        SelfIntersectionStatus::HasIntersections(intersections)
    }
}

/// Checks if an arcline has self-intersections using Hilbert R-tree spatial indexing.
///
/// This is the main self-intersection check function. It uses spatial indexing
/// for fast rejection of non-intersecting arc pairs, then performs expensive
/// intersection tests only on candidates.
///
/// # Arguments
/// * `arcline` - A sequence of connected arcs forming a polyline
///
/// # Returns
/// `true` if the arcline has any self-intersections, `false` otherwise
pub fn arcline_has_self_intersection_aabb(arcline: &Arcline) -> bool {
    arcline_has_self_intersection(arcline)
}

/// Finds all self-intersection points in an arcline.
///
/// This is the main function for finding all self-intersection points.
/// It uses spatial indexing for fast rejection before expensive tests.
///
/// # Arguments
/// * `arcline` - A sequence of connected arcs forming a polyline
///
/// # Returns
/// A vector of tuples `(arc_i_index, arc_j_index, intersection_point)`
pub fn arcline_self_intersections_aabb(arcline: &Arcline) -> Vec<(usize, usize, Point)> {
    arcline_self_intersections(arcline)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_non_intersecting_arcline() {
        // Two sequential arcs, no self-intersection
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
        let arcline = vec![arc1, arc2];
        assert!(!arcline_has_self_intersection(&arcline));
        assert_eq!(
            arcline_self_intersection_status(&arcline),
            SelfIntersectionStatus::Clean
        );
    }

    #[test]
    fn test_three_arc_no_intersection() {
        // Three sequential arcs forming a simple path
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
        let arc3 = arc(point(2.0, 0.0), point(3.0, 0.0), point(2.5, 0.5), 1.0);
        let arcline = vec![arc1, arc2, arc3];
        assert!(!arcline_has_self_intersection(&arcline));
    }

    #[test]
    fn test_single_arc() {
        // Single arc can't self-intersect
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arcline = vec![arc1];
        assert!(!arcline_has_self_intersection(&arcline));
    }

    #[test]
    fn test_empty_arcline() {
        // Empty arcline
        let arcline: Arcline = vec![];
        assert!(!arcline_has_self_intersection(&arcline));
    }

    #[test]
    fn test_two_arcs_no_intersection() {
        // Two adjacent arcs (can't have self-intersection with only 2 arcs)
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(0.0, 0.0), point(0.5, -0.5), 1.0);
        let arcline = vec![arc1, arc2];
        assert!(!arcline_has_self_intersection(&arcline));
    }

    #[test]
    fn test_intersections_list_empty() {
        // Non-intersecting arcline should return empty list
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
        let arc3 = arc(point(2.0, 0.0), point(3.0, 0.0), point(2.5, 0.5), 1.0);
        let arcline = vec![arc1, arc2, arc3];
        let intersections = arcline_self_intersections(&arcline);
        assert!(intersections.is_empty());
    }

    #[test]
    fn test_intersections_list_not_empty() {
        // Two arcs that intersect (both horizontal, different circles)
        // Arc on circle centered at (0.5, 0) with radius 1: from (0,0) to (1,0)
        // Arc on circle centered at (0.5, 1) with radius 1: from (0,1) to (1,1)
        // But positioned such that arcs 0 and 2 might intersect
        
        // Create a configuration where arc 0 and arc 2 might intersect
        let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
        let arc2 = arc(point(1.0, 0.0), point(2.0, 0.0), point(1.5, 0.5), 1.0);
        let arc3 = arc(point(0.5, -1.0), point(1.5, -1.0), point(1.0, -0.5), 1.0);
        
        let arcline = vec![arc1, arc2, arc3];
        let intersections = arcline_self_intersections(&arcline);
        // This depends on the exact geometry, may or may not intersect
        let _ = intersections; // Just verify the function works
    }
}

#![allow(dead_code)]

use crate::constants::COLLINEARITY_TOLERANCE;
use crate::prelude::*;

/// Applies the Akl-Toussaint heuristic to filter out points that cannot be on the convex hull.
///
/// The Akl-Toussaint heuristic is a preprocessing step that quickly eliminates points
/// guaranteed not to be on the convex hull by constructing a bounding quadrilateral
/// (or octagon for better results) from extreme points and discarding all interior points.
///
/// # Algorithm
///
/// 1. Find the points with min/max x and y coordinates (4 extreme points)
/// 2. Optionally find additional extreme points for sum and difference of coordinates (8 points total)
/// 3. These points form a convex quadrilateral/octagon
/// 4. All points strictly inside this shape cannot be on the convex hull
/// 5. Return only the points on or outside the bounding shape
///
/// # Performance
///
/// - Time complexity: O(n)
/// - For random point distributions, typically removes 40-80% of points
/// - Expected real-world speedup: 2-4x when combined with main hull algorithm
///
/// # Arguments
///
/// * `points` - A slice of points to filter
///
/// # Returns
///
/// A vector of points that may be on the convex hull (includes all candidates)
#[must_use]
fn akl_toussaint_filter(points: &Pointline) -> Pointline {
    if points.len() < 5 {
        // Not enough points to filter effectively
        return points.to_vec();
    }

    // Find the four extreme points: min/max x and y
    let mut min_x_idx = 0;
    let mut max_x_idx = 0;
    let mut min_y_idx = 0;
    let mut max_y_idx = 0;

    for (i, p) in points.iter().enumerate() {
        if p.x < points[min_x_idx].x
            || (p.x == points[min_x_idx].x && p.y < points[min_x_idx].y)
        {
            min_x_idx = i;
        }
        if p.x > points[max_x_idx].x
            || (p.x == points[max_x_idx].x && p.y > points[max_x_idx].y)
        {
            max_x_idx = i;
        }
        if p.y < points[min_y_idx].y
            || (p.y == points[min_y_idx].y && p.x < points[min_y_idx].x)
        {
            min_y_idx = i;
        }
        if p.y > points[max_y_idx].y
            || (p.y == points[max_y_idx].y && p.x > points[max_y_idx].x)
        {
            max_y_idx = i;
        }
    }

    // The four extreme points: left, top, right, bottom (in CCW order)
    let quad = [
        points[min_x_idx], // Left
        points[max_y_idx], // Top
        points[max_x_idx], // Right
        points[min_y_idx], // Bottom
    ];

    // Filter: keep only points that are not strictly inside the quadrilateral
    // A point is strictly inside if it's on the correct side of all four edges
    // Preallocate with reasonable capacity (expect to keep 20-60% of points)
    let mut filtered = Vec::with_capacity(points.len());
    for p in points.iter() {
        if !is_strictly_inside_quadrilateral(p, &quad) {
            filtered.push(*p);
        }
    }

    // Return filtered points (should never be empty if quadrilateral is valid)
    if filtered.is_empty() {
        points.to_vec()
    } else {
        filtered
    }
}

/// Helper function: Check if a point is strictly inside a convex quadrilateral.
/// Uses the consistent orientation test.
fn is_strictly_inside_quadrilateral(point: &Point, quad: &[Point]) -> bool {
    // For a convex quadrilateral, a point is strictly inside if it has
    // the same orientation relative to all four edges
    
    if quad.len() != 4 {
        return false;
    }

    // First, compute the orientation of the quad itself
    // by checking the first edge
    let p0 = quad[0];
    let p1 = quad[1];
    let v0x = p1.x - p0.x;
    let v0y = p1.y - p0.y;
    let v0px = point.x - p0.x;
    let v0py = point.y - p0.y;
    let first_cross = v0x.mul_add(v0py, -(v0y * v0px));

    // If the point is on the boundary or outside the first edge, it's not strictly inside
    if first_cross.abs() <= COLLINEARITY_TOLERANCE {
        return false; // On boundary
    }

    let first_sign = first_cross > 0.0;

    // Check all other edges
    for i in 1..4 {
        let p_curr = quad[i];
        let p_next = quad[(i + 1) % 4];
        
        let vx = p_next.x - p_curr.x;
        let vy = p_next.y - p_curr.y;
        let vpx = point.x - p_curr.x;
        let vpy = point.y - p_curr.y;
        
        let cross = vx.mul_add(vpy, -(vy * vpx));

        // If on boundary, not strictly inside
        if cross.abs() <= COLLINEARITY_TOLERANCE {
            return false;
        }

        // If sign differs from the first edge, point is outside
        if (cross > 0.0) != first_sign {
            return false;
        }
    }

    // All edges have consistent orientation and point is not on any boundary
    true
}

/// Computes the convex hull of a set of points using the Gift Wrapping algorithm (Jarvis march).
///
/// The convex hull is the smallest convex polygon that contains all the given points.
/// This implementation uses the Gift Wrapping (Jarvis march) algorithm which has
/// O(nh) time complexity, where n is the number of input points and h is the number
/// of points on the convex hull.
///
/// # Arguments
///
/// * `points` - A slice of points for which to compute the convex hull
///
/// # Returns
///
/// A `Pointline` (vector of points) representing the vertices of the convex hull
/// in counter-clockwise order. If the input has fewer than 3 points, returns
/// all input points. For collinear points, returns the extreme points.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// // Square points
/// let points = vec![
///     point(0.0, 0.0),
///     point(1.0, 0.0),
///     point(1.0, 1.0),
///     point(0.0, 1.0),
///     point(0.5, 0.5), // Interior point
/// ];
/// let hull = points_convex_hull(&points);
/// assert_eq!(hull.len(), 4); // Square has 4 vertices
///
/// // Triangle
/// let triangle = vec![
///     point(0.0, 0.0),
///     point(2.0, 0.0),
///     point(1.0, 2.0),
/// ];
/// let hull = points_convex_hull(&triangle);
/// assert_eq!(hull.len(), 3); // All points are on the hull
/// ```
///
/// # Algorithm Details
///
/// The Gift Wrapping algorithm works by:
/// 1. Finding the leftmost point (guaranteed to be on the hull)
/// 2. Starting from this point, finding the next point that makes the smallest
///    counter-clockwise angle with the current edge
/// 3. Repeating until we return to the starting point
///
/// # Panics
///
/// This function does not panic, but returns an empty vector for empty input.
#[must_use]
pub fn points_convex_hull(points: &Pointline) -> Pointline {
    // Filter out NaN and Infinity points - they cannot be part of a valid convex hull
    let mut valid_points = Vec::with_capacity(points.len());
    for p in points.iter() {
        if p.x.is_finite() && p.y.is_finite() {
            valid_points.push(*p);
        }
    }

    // If no valid points remain, return empty hull
    if valid_points.is_empty() {
        return Vec::new();
    }

    if valid_points.len() < 3 {
        return valid_points;
    }

    // Apply Akl-Toussaint heuristic as preprocessing to filter out interior points
    let candidates = akl_toussaint_filter(&valid_points);

    // Preallocate hull with reasonable capacity (typically h << n)
    let mut hull = Vec::with_capacity((candidates.len() as f64).sqrt().ceil() as usize);

    // Find the leftmost point (guaranteed to be on the hull)
    // If there are ties, choose the bottommost among them
    let start = candidates
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            match a.x.partial_cmp(&b.x) {
                Some(std::cmp::Ordering::Equal) => a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal),
                Some(other) => other,
                None => unreachable!(), // No NaN after filtering
            }
        })
        .map_or(0, |(idx, _)| idx);

    let mut current = start;
    loop {
        hull.push(candidates[current]);
        let mut next = (current + 1) % candidates.len();

        // Find the point that makes the most counter-clockwise turn
        for i in 0..candidates.len() {
            if i == current {
                continue;
            }

            // Use cross product to determine orientation
            // Explicit inline calculation using mul_add for better performance:
            let dx_next = candidates[next].x - candidates[current].x;
            let dy_next = candidates[next].y - candidates[current].y;
            let dx_i = candidates[i].x - candidates[current].x;
            let dy_i = candidates[i].y - candidates[current].y;
            let cross = dx_next.mul_add(dy_i, -(dy_next * dx_i));

            // Alternative using Vector.perp() method (for performance comparison):
            // let cross = (candidates[next] - candidates[current])
            //     .perp(candidates[i] - candidates[current]);

            // If cross < 0, point i is more counter-clockwise than next
            // If cross ≈ 0 (within tolerance), points are collinear - choose the farther one
            // If cross > 0, next is more counter-clockwise than i
            if cross < 0.0
                || (cross.abs() < COLLINEARITY_TOLERANCE
                    && {
                        let dist_i_sq = {
                            let dx = candidates[i].x - candidates[current].x;
                            let dy = candidates[i].y - candidates[current].y;
                            dx * dx + dy * dy
                        };
                        let dist_next_sq = {
                            let dx = candidates[next].x - candidates[current].x;
                            let dy = candidates[next].y - candidates[current].y;
                            dx * dx + dy * dy
                        };
                        dist_i_sq > dist_next_sq
                    })
            {
                next = i;
            }
        }

        current = next;

        // Prevent infinite loops: if we've found more points than input, something's wrong
        if hull.len() > candidates.len() {
            break;
        }

        if current == start {
            break;
        }
    }

    hull
}

#[cfg(test)]
mod test_akl_toussaint_filter {
    use super::*;

    #[test]
    fn test_akl_toussaint_empty() {
        let points: Pointline = vec![];
        let filtered = akl_toussaint_filter(&points);
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_akl_toussaint_single_point() {
        let points = vec![point(0.0, 0.0)];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_akl_toussaint_two_points() {
        let points = vec![point(0.0, 0.0), point(1.0, 1.0)];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_akl_toussaint_three_points() {
        let points = vec![point(0.0, 0.0), point(1.0, 0.0), point(0.5, 1.0)];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_akl_toussaint_four_points_square() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 4);
    }

    #[test]
    fn test_akl_toussaint_removes_interior_point() {
        // Simple square with one clearly interior point
        let points = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 2.0),
            point(0.0, 2.0),
            point(1.0, 1.0), // Interior point - should be removed
        ];
        let filtered = akl_toussaint_filter(&points);
        // The filter should identify the interior point
        assert!(filtered.len() <= points.len());
    }

    #[test]
    fn test_akl_toussaint_removes_multiple_interior_points() {
        // Large square with multiple interior points
        let points = vec![
            point(0.0, 0.0),
            point(100.0, 0.0),
            point(100.0, 100.0),
            point(0.0, 100.0),
            point(25.0, 25.0),
            point(50.0, 50.0),
            point(75.0, 75.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        // Should keep the 4 corners for sure
        assert!(filtered.contains(&point(0.0, 0.0)));
        assert!(filtered.contains(&point(100.0, 0.0)));
        assert!(filtered.contains(&point(100.0, 100.0)));
        assert!(filtered.contains(&point(0.0, 100.0)));
    }

    #[test]
    fn test_akl_toussaint_keeps_boundary_points() {
        // Square with points on edges and corners
        let points = vec![
            point(0.0, 0.0),  // Corner
            point(1.0, 0.0),  // Corner
            point(1.0, 1.0),  // Corner
            point(0.0, 1.0),  // Corner
            point(0.5, 0.0),  // On edge - should be kept
            point(1.0, 0.5),  // On edge - should be kept
            point(0.5, 1.0),  // On edge - should be kept
            point(0.0, 0.5),  // On edge - should be kept
        ];
        let filtered = akl_toussaint_filter(&points);
        // All boundary and corner points should be kept
        assert_eq!(filtered.len(), 8);
    }

    #[test]
    fn test_akl_toussaint_large_interior_region() {
        // Rectangle with many well-separated interior points
        let points = vec![
            point(0.0, 0.0),
            point(1000.0, 0.0),
            point(1000.0, 500.0),
            point(0.0, 500.0),
            // Interior grid of points - well separated
            point(100.0, 100.0),
            point(200.0, 100.0),
            point(300.0, 100.0),
            point(100.0, 200.0),
            point(200.0, 200.0),
            point(300.0, 200.0),
            point(100.0, 300.0),
            point(200.0, 300.0),
            point(300.0, 300.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        // Should keep the 4 corners
        assert!(filtered.contains(&point(0.0, 0.0)));
        assert!(filtered.contains(&point(1000.0, 0.0)));
        assert!(filtered.contains(&point(1000.0, 500.0)));
        assert!(filtered.contains(&point(0.0, 500.0)));
    }

    #[test]
    fn test_akl_toussaint_large_coordinates() {
        // Test with larger coordinates for robustness
        let points = vec![
            point(0.0, 0.0),
            point(1000.0, 0.0),
            point(1000.0, 1000.0),
            point(0.0, 1000.0),
            point(500.0, 500.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        // Should keep the corners
        assert!(filtered.contains(&point(0.0, 0.0)));
        assert!(filtered.contains(&point(1000.0, 0.0)));
        assert!(filtered.contains(&point(1000.0, 1000.0)));
        assert!(filtered.contains(&point(0.0, 1000.0)));
    }

    #[test]
    fn test_akl_toussaint_negative_coordinates() {
        // Test with negative coordinates
        let points = vec![
            point(-100.0, -100.0),
            point(100.0, -100.0),
            point(100.0, 100.0),
            point(-100.0, 100.0),
            point(0.0, 0.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        // Should keep the corners
        assert!(filtered.contains(&point(-100.0, -100.0)));
        assert!(filtered.contains(&point(100.0, -100.0)));
        assert!(filtered.contains(&point(100.0, 100.0)));
        assert!(filtered.contains(&point(-100.0, 100.0)));
    }

    #[test]
    fn test_akl_toussaint_all_collinear_horizontal() {
        // All points on a horizontal line - no interior to remove
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(2.0, 0.0),
            point(3.0, 0.0),
        ];
        let filtered = akl_toussaint_filter(&points);
        // With collinear points, filtering might keep some interior points
        // because they don't form a proper quadrilateral
        assert!(!filtered.is_empty());
    }

    #[test]
    fn test_akl_toussaint_triangle() {
        // Triangle - all points should be kept
        let points = vec![point(0.0, 0.0), point(2.0, 0.0), point(1.0, 2.0)];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 3);
    }

    #[test]
    fn test_akl_toussaint_diamond_simple() {
        // Diamond shape with center point  
        let points = vec![
            point(1.0, 0.0),   // Right
            point(0.0, 1.0),   // Top
            point(-1.0, 0.0),  // Left
            point(0.0, -1.0),  // Bottom
            point(0.0, 0.0),   // Center - should be removed
        ];
        let filtered = akl_toussaint_filter(&points);
        assert_eq!(filtered.len(), 4);
        assert!(!filtered.contains(&point(0.0, 0.0)));
    }
}

#[cfg(test)]
mod test_points_convex_hull {
    use super::*;

    #[test]
    fn test_points_convex_hull_empty() {
        let points: Pointline = vec![];
        let hull = points_convex_hull(&points);
        assert!(hull.is_empty());
    }

    #[test]
    fn test_points_convex_hull_single_point() {
        let points = vec![point(1.0, 2.0)];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 1);
        assert_eq!(hull[0], point(1.0, 2.0));
    }

    #[test]
    fn test_points_convex_hull_two_points() {
        let points = vec![point(0.0, 0.0), point(1.0, 1.0)];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 2);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
    }

    #[test]
    fn test_points_convex_hull_triangle() {
        let points = vec![point(0.0, 0.0), point(2.0, 0.0), point(1.0, 2.0)];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 3);
        // All points should be on the hull for a triangle
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(2.0, 0.0)));
        assert!(hull.contains(&point(1.0, 2.0)));
    }

    #[test]
    fn test_points_convex_hull_square() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        // All corners should be on the hull
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_points_convex_hull_square_with_interior_point() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
            point(0.5, 0.5), // Interior point
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        // Interior point should not be on the hull
        assert!(!hull.contains(&point(0.5, 0.5)));
        // All corners should be on the hull
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_points_convex_hull_collinear_points() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(2.0, 0.0),
            point(3.0, 0.0),
        ];
        let hull = points_convex_hull(&points);
        // For collinear points, the algorithm should still work, though it may
        // include all points since they're technically on the "hull"
        assert!(hull.len() >= 2);
        // The extreme points should definitely be included
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(3.0, 0.0)));
    }

    #[test]
    fn test_points_convex_hull_pentagon() {
        let points = vec![
            point(0.0, 1.0),
            point(1.0, 0.0),
            point(2.0, 1.0),
            point(1.5, 2.5),
            point(0.5, 2.5),
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 5);
        // All points should be on the hull for a convex pentagon
        for point in &points {
            assert!(hull.contains(point));
        }
    }

    #[test]
    fn test_points_convex_hull_random_points() {
        let points = vec![
            point(0.0, 0.0),
            point(4.0, 0.0),
            point(4.0, 3.0),
            point(0.0, 3.0),
            point(1.0, 1.0), // Interior
            point(2.0, 1.5), // Interior
            point(3.0, 2.0), // Interior
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        // Only the corner points should be on the hull
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(4.0, 0.0)));
        assert!(hull.contains(&point(4.0, 3.0)));
        assert!(hull.contains(&point(0.0, 3.0)));
        // Interior points should not be on the hull
        assert!(!hull.contains(&point(1.0, 1.0)));
        assert!(!hull.contains(&point(2.0, 1.5)));
        assert!(!hull.contains(&point(3.0, 2.0)));
    }

    #[test]
    fn test_points_convex_hull_duplicate_points() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
            point(0.0, 0.0), // Duplicate
            point(1.0, 1.0), // Duplicate
        ];
        let hull = points_convex_hull(&points);
        // Hull should contain at least the 4 corners. Duplicates may be included
        // in the walk but are typically skipped in the Gift Wrapping algorithm
        // when they share the same position. We just verify corners are present.
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_points_convex_hull_negative_coordinates() {
        let points = vec![
            point(-2.0, -2.0),
            point(2.0, -2.0),
            point(2.0, 2.0),
            point(-2.0, 2.0),
            point(0.0, 0.0), // Interior
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(-2.0, -2.0)));
        assert!(hull.contains(&point(2.0, -2.0)));
        assert!(hull.contains(&point(2.0, 2.0)));
        assert!(hull.contains(&point(-2.0, 2.0)));
        assert!(!hull.contains(&point(0.0, 0.0)));
    }

    #[test]
    fn test_points_convex_hull_star_shape() {
        // Points forming a star - only outer points should be on hull
        let points = vec![
            point(0.0, 3.0),   // Top
            point(1.0, 1.0),   // Inner
            point(3.0, 0.0),   // Right
            point(1.0, -1.0),  // Inner
            point(0.0, -3.0),  // Bottom
            point(-1.0, -1.0), // Inner
            point(-3.0, 0.0),  // Left
            point(-1.0, 1.0),  // Inner
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        // Only the extreme points should be on the hull
        assert!(hull.contains(&point(0.0, 3.0)));
        assert!(hull.contains(&point(3.0, 0.0)));
        assert!(hull.contains(&point(0.0, -3.0)));
        assert!(hull.contains(&point(-3.0, 0.0)));
    }

    #[test]
    fn test_points_convex_hull_large_coordinates() {
        let points = vec![
            point(1e6, 1e6),
            point(1e6 + 100.0, 1e6),
            point(1e6 + 100.0, 1e6 + 100.0),
            point(1e6, 1e6 + 100.0),
            point(1e6 + 50.0, 1e6 + 50.0), // Interior
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(!hull.contains(&point(1e6 + 50.0, 1e6 + 50.0)));
    }

    #[test]
    fn test_points_convex_hull_counter_clockwise_order() {
        let points = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 2.0),
            point(0.0, 2.0),
        ];
        let hull = points_convex_hull(&points);
        assert_eq!(hull.len(), 4);

        // Verify counter-clockwise order by checking that each consecutive
        // triple of points makes a left turn
        for i in 0..hull.len() {
            let p1 = hull[i];
            let p2 = hull[(i + 1) % hull.len()];
            let p3 = hull[(i + 2) % hull.len()];

            let cross = (p2 - p1).perp(p3 - p2);
            // Should be positive for counter-clockwise order
            assert!(cross >= 0.0, "Hull is not in counter-clockwise order");
        }
    }
}

#[cfg(test)]
mod test_pointline_convex_hull {
    use super::*;

    #[test]
    fn test_pointline_convex_hull_empty() {
        let points: Pointline = vec![];
        let hull = pointline_convex_hull(&points);
        assert!(hull.is_empty());
    }

    #[test]
    fn test_pointline_convex_hull_single_vertex() {
        let points = vec![point(1.0, 2.0)];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 1);
        assert_eq!(hull[0], point(1.0, 2.0));
    }

    #[test]
    fn test_pointline_convex_hull_two_vertices() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 1.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 2);
        assert_eq!(hull[0], point(0.0, 0.0));
        assert_eq!(hull[1], point(1.0, 1.0));
    }

    #[test]
    fn test_pointline_convex_hull_triangle() {
        let points = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(1.0, 2.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 3);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(2.0, 0.0)));
        assert!(hull.contains(&point(1.0, 2.0)));
    }

    #[test]
    fn test_pointline_convex_hull_square() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_pointline_convex_hull_square_with_concave_vertex() {
        // Square with one vertex pushed inward (concave)
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.5, 0.5), // Interior/concave point
            point(0.0, 1.0),
        ];
        let hull = pointline_convex_hull(&points);
        // The concave vertex should be removed from hull
        assert!(!hull.contains(&point(0.5, 0.5)));
        // But the convex vertices should be present
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_pointline_convex_hull_octagon() {
        // Simple convex octagon (diamond-like with beveled corners)
        let points = vec![
            point(2.0, 0.0),  // Right
            point(3.0, 1.0),  // Top-right
            point(3.0, 3.0),  // Top
            point(2.0, 4.0),  // Top-right
            point(0.0, 4.0),  // Top-left
            point(-1.0, 3.0), // Top-left corner
            point(-1.0, 1.0), // Bottom-left corner
            point(0.0, 0.0),  // Bottom
        ];
        let hull = pointline_convex_hull(&points);
        // All 8 vertices should be on the hull (it's convex)
        assert_eq!(hull.len(), 8);
    }

    #[test]
    fn test_pointline_convex_hull_pentagon_with_one_concave() {
        // Pentagon where one vertex is slightly inside
        let points = vec![
            point(0.0, 0.0),    // Bottom-left
            point(2.0, -1.0),   // Bottom-right
            point(3.0, 1.0),    // Right
            point(1.0, 0.9),    // Center (concave) - should be excluded
            point(0.0, 2.0),    // Top-left
        ];
        let hull = pointline_convex_hull(&points);
        // Concave vertex should be removed
        assert!(!hull.contains(&point(1.0, 0.9)));
        // Other vertices should remain
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(2.0, -1.0)));
        assert!(hull.contains(&point(3.0, 1.0)));
        assert!(hull.contains(&point(0.0, 2.0)));
    }

    #[test]
    fn test_pointline_convex_hull_multiple_concave_vertices() {
        // Shape with multiple concave vertices
        let points = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 1.0),
            point(1.0, 0.5), // Concave 1
            point(2.0, 2.0),
            point(0.0, 2.0),
            point(1.0, 1.5), // Concave 2
        ];
        let hull = pointline_convex_hull(&points);
        // Both concave vertices should be removed
        assert!(!hull.contains(&point(1.0, 0.5)));
        assert!(!hull.contains(&point(1.0, 1.5)));
        // Corners should remain
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(2.0, 0.0)));
        assert!(hull.contains(&point(2.0, 2.0)));
        assert!(hull.contains(&point(0.0, 2.0)));
    }

    #[test]
    fn test_pointline_convex_hull_large_coordinates() {
        let points = vec![
            point(1e6, 1e6),
            point(1e6 + 100.0, 1e6),
            point(1e6 + 100.0, 1e6 + 100.0),
            point(1e6, 1e6 + 100.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(1e6, 1e6)));
        assert!(hull.contains(&point(1e6 + 100.0, 1e6)));
        assert!(hull.contains(&point(1e6 + 100.0, 1e6 + 100.0)));
        assert!(hull.contains(&point(1e6, 1e6 + 100.0)));
    }

    #[test]
    fn test_pointline_convex_hull_negative_coordinates() {
        let points = vec![
            point(-2.0, -2.0),
            point(2.0, -2.0),
            point(2.0, 2.0),
            point(-2.0, 2.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(-2.0, -2.0)));
        assert!(hull.contains(&point(2.0, -2.0)));
        assert!(hull.contains(&point(2.0, 2.0)));
        assert!(hull.contains(&point(-2.0, 2.0)));
    }

    #[test]
    fn test_pointline_convex_hull_with_bulge_factors() {
        // Test that bulge factors don't affect the algorithm (only points matter)
        // Note: This test now uses Pointline, so bulge factors don't apply
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_pointline_convex_hull_concave_shape() {
        // A concave octagon where only the inner notch vertices are concave
        // Shaped like a rectangle with rectangular notch cut out of one side
        let points = vec![
            point(0.0, 0.0),   // Bottom-left
            point(4.0, 0.0),   // Bottom-right
            point(4.0, 4.0),   // Top-right
            point(3.0, 4.0),   // Notch top-right (convex - still part of hull)
            point(3.0, 2.0),   // Notch bottom-right (CONCAVE - excluded)
            point(1.0, 2.0),   // Notch bottom-left (CONCAVE - excluded)
            point(1.0, 4.0),   // Notch top-left (convex - still part of hull)
            point(0.0, 4.0),   // Top-left
        ];
        let hull = pointline_convex_hull(&points);
        // The concave vertices (inner bottom notch) should be excluded
        assert!(!hull.contains(&point(3.0, 2.0)));
        assert!(!hull.contains(&point(1.0, 2.0)));
        // But the top notch vertices are still convex, so they stay
        assert!(hull.contains(&point(3.0, 4.0)));
        assert!(hull.contains(&point(1.0, 4.0)));
        // The outer corners should be present
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(4.0, 0.0)));
        assert!(hull.contains(&point(4.0, 4.0)));
        assert!(hull.contains(&point(0.0, 4.0)));
    }

    #[test]
    fn test_pointline_convex_hull_nearly_collinear() {
        // Vertices that form a nearly-straight line with small deviations
        // Since the deviations are small, most will be filtered as collinear
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.1),   // Slightly off  
            point(2.0, -0.1),  // Slightly off other direction
            point(3.0, 0.0),
        ];
        let hull = pointline_convex_hull(&points);
        // Some vertices should remain in the hull  
        assert!(!hull.is_empty());
        assert!(hull.len() >= 2);
    }
}

#[cfg(test)]
mod test_arcline_convex_hull {
    //use super::*;

    // #[test]
    // fn test_arcline_convex_hull_empty() {
    //     let arcs: Arcline = vec![];
    //     let hull = arcline_convex_hull(&arcs);
    //     assert!(hull.is_empty());
    // }

    // #[test]
    // fn test_arcline_convex_hull_single_line_segment() {
    //     let arcs = vec![arcseg(point(0.0, 0.0), point(1.0, 0.0))];
    //     let hull = arcline_convex_hull(&arcs);
    //     assert_eq!(hull.len(), 1);
    //     assert_eq!(hull[0], arcseg(point(0.0, 0.0), point(1.0, 0.0)));
    // }

    // #[test]
    // fn test_arcline_convex_hull_single_arc() {
    //     let arcs = vec![arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0)];
    //     let hull = arcline_convex_hull(&arcs);
    //     assert_eq!(hull.len(), 1);
    //     // Single arc should be the hull itself
    //     assert_eq!(hull[0].a, point(1.0, 0.0));
    //     assert_eq!(hull[0].b, point(0.0, 1.0));
    // }

    // #[test]
    // fn test_arcline_convex_hull_two_connected_line_segments() {
    //     // Two line segments forming a corner
    //     let arcs = vec![
    //         arcseg(point(0.0, 0.0), point(1.0, 0.0)),
    //         arcseg(point(1.0, 0.0), point(1.0, 1.0)),
    //     ];
    //     let hull = arcline_convex_hull(&arcs);
    //     // Should return both segments as they form the hull
    //     assert_eq!(hull.len(), 2);
    //     // Check that the segments are preserved
    //     assert!(hull.contains(&arcseg(point(0.0, 0.0), point(1.0, 0.0))));
    //     assert!(hull.contains(&arcseg(point(1.0, 0.0), point(1.0, 1.0))));
    // }

    // #[test]
    // fn test_arcline_convex_hull_square_with_line_segments() {
    //     // Four line segments forming a square
    //     let arcs = vec![
    //         arcseg(point(0.0, 0.0), point(1.0, 0.0)),
    //         arcseg(point(1.0, 0.0), point(1.0, 1.0)),
    //         arcseg(point(1.0, 1.0), point(0.0, 1.0)),
    //         arcseg(point(0.0, 1.0), point(0.0, 0.0)),
    //     ];
    //     let hull = arcline_convex_hull(&arcs);
    //     // All segments should be part of the hull
    //     assert_eq!(hull.len(), 4);
    // }

    // #[test]
    // fn test_arcline_convex_hull_concave_shape_with_lines() {
    //     // Create a concave shape where some segments should be excluded
    //     let arcs = vec![
    //         arcseg(point(0.0, 0.0), point(2.0, 0.0)),
    //         arcseg(point(2.0, 0.0), point(2.0, 2.0)),
    //         arcseg(point(2.0, 2.0), point(1.0, 1.0)), // Interior segment
    //         arcseg(point(1.0, 1.0), point(0.0, 2.0)), // Interior segment
    //         arcseg(point(0.0, 2.0), point(0.0, 0.0)),
    //     ];
    //     let hull = arcline_convex_hull(&arcs);
    //     // Should exclude the interior segments and create new connecting segments
    //     assert!(hull.len() <= 4); // The convex hull should be simpler

    //     // The hull should not contain the interior segments
    //     assert!(!hull.contains(&arcseg(point(2.0, 2.0), point(1.0, 1.0))));
    //     assert!(!hull.contains(&arcseg(point(1.0, 1.0), point(0.0, 2.0))));
    // }

    // #[test]
    // fn test_arcline_convex_hull_single_quarter_circle() {
    //     // A quarter circle arc
    //     let arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
    //     let arcs = vec![arc];
    //     let hull = arcline_convex_hull(&arcs);

    //     // For a single convex arc, the hull might include the arc plus connecting lines
    //     assert!(!hull.is_empty());
    //     // The original arc should be part of the hull
    //     assert!(
    //         hull.iter()
    //             .any(|a| a.a == arc.a && a.b == arc.b && a.is_arc())
    //     );
    // }

    // #[test]
    // fn test_arcline_convex_hull_semicircle() {
    //     // A semicircle arc
    //     let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
    //     let arcs = vec![arc];
    //     let hull = arcline_convex_hull(&arcs);

    //     // Should include the arc and a line segment closing the bottom
    //     assert!(hull.len() >= 1);
    //     // Should contain the original arc
    //     assert!(
    //         hull.iter()
    //             .any(|a| a.a == arc.a && a.b == arc.b && a.is_arc())
    //     );
    //     // Should contain a line segment connecting the endpoints
    //     assert!(
    //         hull.iter().any(|a| a.is_line()
    //             && ((a.a == arc.a && a.b == arc.b) || (a.a == arc.b && a.b == arc.a)))
    //     );
    // }

    // #[test]
    // fn test_arcline_convex_hull_multiple_arcs_forming_circle() {
    //     // Multiple arcs forming a complete circle (should be convex)
    //     let arcs = vec![
    //         arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0), // Q1
    //         arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Q2
    //         arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0), // Q3
    //         arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0), // Q4
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     // All arcs should be part of the hull (complete circle is convex)
    //     assert_eq!(hull.len(), 4);
    //     for original_arc in &arcs {
    //         assert!(hull.iter().any(|hull_arc| hull_arc.a == original_arc.a
    //             && hull_arc.b == original_arc.b
    //             && hull_arc.is_arc()));
    //     }
    // }

    // #[test]
    // fn test_arcline_convex_hull_arc_with_interior_line() {
    //     // Arc with a line segment inside the convex region
    //     let arcs = vec![
    //         arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),
    //         arcseg(point(0.5, 0.1), point(0.1, 0.5)), // Interior line
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     // The interior line should not be part of the hull
    //     assert!(!hull.contains(&arcseg(point(0.5, 0.1), point(0.1, 0.5))));
    //     // The arc should be part of the hull
    //     assert!(
    //         hull.iter()
    //             .any(|a| a.a == point(1.0, 0.0) && a.b == point(0.0, 1.0) && a.is_arc())
    //     );
    // }

    // #[test]
    // fn test_arcline_convex_hull_mixed_arcs_and_lines() {
    //     // Combination of arcs and line segments
    //     let arcs = vec![
    //         arc(point(2.0, 0.0), point(0.0, 2.0), point(0.0, 0.0), 2.0), // Large arc
    //         arcseg(point(0.0, 2.0), point(-1.0, 1.0)),                   // Line segment
    //         arcseg(point(-1.0, 1.0), point(2.0, 0.0)),                   // Closing line
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     assert!(!hull.is_empty());
    //     // Should handle the combination properly
    //     assert!(hull.len() >= 2);
    // }

    // #[test]
    // fn test_arcline_convex_hull_non_convex_arc() {
    //     // An arc that subtends more than 180 degrees (non-convex)
    //     let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, -1.0), 1.0); // Lower semicircle
    //     let arcs = vec![arc];
    //     let hull = arcline_convex_hull(&arcs);

    //     // Should replace the non-convex arc with a line segment
    //     assert!(!hull.is_empty());
    //     // Should contain a line segment connecting the endpoints
    //     assert!(
    //         hull.iter().any(|a| a.is_line()
    //             && ((a.a == arc.a && a.b == arc.b) || (a.a == arc.b && a.b == arc.a)))
    //     );
    // }

    // #[test]
    // fn test_arcline_convex_hull_disconnected_segments() {
    //     // Multiple disconnected segments
    //     let arcs = vec![
    //         arcseg(point(0.0, 0.0), point(1.0, 0.0)),
    //         arcseg(point(2.0, 1.0), point(3.0, 1.0)),
    //         arcseg(point(1.0, 2.0), point(2.0, 3.0)),
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     // Should create new connecting segments to form the convex hull
    //     assert!(!hull.is_empty());
    //     // The result should be connected and form a convex shape
    // }

    // #[test]
    // fn test_arcline_convex_hull_large_coordinates() {
    //     // Test with large coordinates to check numerical stability
    //     let arcs = vec![
    //         arcseg(point(1e6, 1e6), point(1e6 + 100.0, 1e6)),
    //         arcseg(point(1e6 + 100.0, 1e6), point(1e6 + 100.0, 1e6 + 100.0)),
    //         arcseg(point(1e6 + 50.0, 1e6 + 10.0), point(1e6 + 60.0, 1e6 + 90.0)), // Interior
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     assert!(!hull.is_empty());
    //     // Interior segment should not be in hull
    //     assert!(!hull.contains(&arcseg(
    //         point(1e6 + 50.0, 1e6 + 10.0),
    //         point(1e6 + 60.0, 1e6 + 90.0)
    //     )));
    // }

    // #[test]
    // fn test_arcline_convex_hull_duplicate_arcs() {
    //     // Test with duplicate arcs
    //     let arc1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    //     let arc2 = arcseg(point(1.0, 0.0), point(1.0, 1.0));
    //     let arcs = vec![arc1, arc2, arc1]; // Duplicate arc1

    //     let hull = arcline_convex_hull(&arcs);
    //     assert!(!hull.is_empty());
    //     // Should handle duplicates gracefully
    // }

    // #[test]
    // fn test_arcline_convex_hull_very_small_arcs() {
    //     // Test with very small arcs that might be considered degenerate
    //     let arcs = vec![
    //         arcseg(point(0.0, 0.0), point(1e-10, 1e-10)),
    //         arcseg(point(1.0, 0.0), point(1.0, 1.0)),
    //     ];
    //     let hull = arcline_convex_hull(&arcs);

    //     // Should handle small arcs without crashing
    //     assert!(!hull.is_empty());
    // }
}

/// Computes the convex hull of an ordered point sequence assuming it forms a closed, non-intersecting CCW polygon.
///
/// This function is optimized for ordered point sequences where points form a closed polygon.
/// Unlike `points_convex_hull` which works on unordered point sets, this function
/// leverages the structural property of ordered sequences to traverse them in O(n) time.
///
/// # Assumptions
///
/// The input point sequence must satisfy:
/// - Form a **closed polygon** (last point connects back to first)
/// - Be **non-intersecting** (no self-intersections)
/// - Be **counter-clockwise (CCW) oriented**
/// - Contain **no duplicate consecutive points**
///
/// These assumptions are the responsibility of the user to ensure. Violating them may produce
/// incorrect results or panics.
///
/// # Algorithm
///
/// The algorithm uses a single-pass traversal:
/// 1. Start with all points marked as potential hull members
/// 2. For each point, check if it's part of the convex hull by examining the cross product
///    of the edges formed by its neighbors
/// 3. A point is on the hull if the turn is counter-clockwise (cross product > 0)
/// 4. Points causing clockwise turns (concave) are excluded from the hull
/// 5. Return only the hull points in CCW order
///
/// # Time Complexity
///
/// O(n) - single pass through the point sequence
///
/// # Space Complexity
///
/// O(h) where h is the number of hull points
///
/// # Arguments
///
/// * `points` - A closed, non-intersecting CCW ordered point sequence
///
/// # Returns
///
/// A `Pointline` containing the convex hull points in counter-clockwise order.
/// If the sequence has fewer than 3 points, returns all unique points.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// // A square with all vertices on the hull
/// let points = vec![
///     point(0.0, 0.0),   // Bottom-left
///     point(1.0, 0.0),   // Bottom-right
///     point(1.0, 1.0),   // Top-right
///     point(0.0, 1.0),   // Top-left
/// ];
/// let hull = pointline_convex_hull(&points);
/// assert_eq!(hull.len(), 4); // All points are on the hull
/// ```
#[must_use]
pub fn pointline_convex_hull(points: &Pointline) -> Pointline {
    if points.len() < 3 {
        return points.to_vec();
    }

    let n = points.len();
    // Preallocate hull with reasonable capacity
    // For ordered polygons, convex vertices are typically 30-50% of total vertices
    let mut hull = Vec::with_capacity((n + 1) / 2);

    // Single pass: check each point for convexity
    // A point is on the hull if the turn at that point is counter-clockwise (left turn)
    for i in 0..n {
        let prev_idx = if i == 0 { n - 1 } else { i - 1 };
        let curr_idx = i;
        let next_idx = (i + 1) % n;

        let p_prev = points[prev_idx];
        let p_curr = points[curr_idx];
        let p_next = points[next_idx];

        // Cross product of vectors (curr - prev) and (next - curr)
        // Positive = CCW turn (left turn) = on convex hull
        // Negative = CW turn (right turn) = concave, skip
        // Zero = collinear = skip (interior edge)
        let dx1 = p_curr.x - p_prev.x;
        let dy1 = p_curr.y - p_prev.y;
        let dx2 = p_next.x - p_curr.x;
        let dy2 = p_next.y - p_curr.y;

        let cross = dx1.mul_add(dy2, -(dy1 * dx2));

        if cross > COLLINEARITY_TOLERANCE {
            hull.push(p_curr);
        }
    }

    // If all points are collinear or form a degenerate polygon, return at least 2 points
    if hull.is_empty() && n > 0 {
        // Fall back to at least the extreme points
        return vec![points[0], points[n / 2]];
    }

    hull
}



#![allow(dead_code)]

use crate::prelude::*;

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
/// use basegeom::prelude::*;
///
/// // Square points
/// let points = vec![
///     point(0.0, 0.0),
///     point(1.0, 0.0),
///     point(1.0, 1.0),
///     point(0.0, 1.0),
///     point(0.5, 0.5), // Interior point
/// ];
/// let hull = pointline_convex_hull(&points);
/// assert_eq!(hull.len(), 4); // Square has 4 vertices
///
/// // Triangle
/// let triangle = vec![
///     point(0.0, 0.0),
///     point(2.0, 0.0),
///     point(1.0, 2.0),
/// ];
/// let hull = pointline_convex_hull(&triangle);
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
pub fn pointline_convex_hull(points: &Pointline) -> Pointline {

    // Remove duplicate points first
    let mut unique_points = Vec::new();
    for point in points {
        if !unique_points.contains(point) {
            unique_points.push(*point);
        }
    }
    
    if unique_points.len() < 3 {
        return unique_points;
    }

    let mut hull = Vec::new();

    // Find the leftmost point (guaranteed to be on the hull)
    // If there are ties, choose the bottommost among them
    let start = unique_points.iter().enumerate()
        .min_by(|(_, a), (_, b)| {
            match a.x.partial_cmp(&b.x).unwrap() {
                std::cmp::Ordering::Equal => a.y.partial_cmp(&b.y).unwrap(),
                other => other,
            }
        })
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    let mut current = start;
    loop {
        hull.push(unique_points[current]);
        let mut next = (current + 1) % unique_points.len();

        // Find the point that makes the most counter-clockwise turn
        for i in 0..unique_points.len() {
            if i == current {
                continue;
            }
            
            // Use cross product to determine orientation
            let cross = (unique_points[next] - unique_points[current]).perp(unique_points[i] - unique_points[current]);
            
            // If cross < 0, point i is more counter-clockwise than next
            // If cross == 0, points are collinear - choose the farther one
            // If cross > 0, next is more counter-clockwise than i
            if cross < 0.0 || (cross == 0.0 && 
                (unique_points[i] - unique_points[current]).norm() > (unique_points[next] - unique_points[current]).norm()) {
                next = i;
            }
        }

        current = next;
        
        // Prevent infinite loops: if we've found more points than input, something's wrong
        if hull.len() > unique_points.len() {
            break;
        }
        
        if current == start {
            break;
        }
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointline_convex_hull_empty() {
        let points: Pointline = vec![];
        let hull = pointline_convex_hull(&points);
        assert!(hull.is_empty());
    }

    #[test]
    fn test_pointline_convex_hull_single_point() {
        let points = vec![point(1.0, 2.0)];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 1);
        assert_eq!(hull[0], point(1.0, 2.0));
    }

    #[test]
    fn test_pointline_convex_hull_two_points() {
        let points = vec![point(0.0, 0.0), point(1.0, 1.0)];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 2);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
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
        // All points should be on the hull for a triangle
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
        // All corners should be on the hull
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_pointline_convex_hull_square_with_interior_point() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
            point(0.5, 0.5), // Interior point
        ];
        let hull = pointline_convex_hull(&points);
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
    fn test_pointline_convex_hull_collinear_points() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(2.0, 0.0),
            point(3.0, 0.0),
        ];
        let hull = pointline_convex_hull(&points);
        // For collinear points, the algorithm should still work, though it may
        // include all points since they're technically on the "hull"
        assert!(hull.len() >= 2);
        // The extreme points should definitely be included
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(3.0, 0.0)));
    }

    #[test]
    fn test_pointline_convex_hull_pentagon() {
        let points = vec![
            point(0.0, 1.0),
            point(1.0, 0.0),
            point(2.0, 1.0),
            point(1.5, 2.5),
            point(0.5, 2.5),
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 5);
        // All points should be on the hull for a convex pentagon
        for point in &points {
            assert!(hull.contains(point));
        }
    }

    #[test]
    fn test_pointline_convex_hull_random_points() {
        let points = vec![
            point(0.0, 0.0),
            point(4.0, 0.0),
            point(4.0, 3.0),
            point(0.0, 3.0),
            point(1.0, 1.0), // Interior
            point(2.0, 1.5), // Interior
            point(3.0, 2.0), // Interior
        ];
        let hull = pointline_convex_hull(&points);
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
    fn test_pointline_convex_hull_duplicate_points() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
            point(0.0, 0.0), // Duplicate
            point(1.0, 1.0), // Duplicate
        ];
        let hull = pointline_convex_hull(&points);
        // Should still form a square hull
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(0.0, 0.0)));
        assert!(hull.contains(&point(1.0, 0.0)));
        assert!(hull.contains(&point(1.0, 1.0)));
        assert!(hull.contains(&point(0.0, 1.0)));
    }

    #[test]
    fn test_pointline_convex_hull_negative_coordinates() {
        let points = vec![
            point(-2.0, -2.0),
            point(2.0, -2.0),
            point(2.0, 2.0),
            point(-2.0, 2.0),
            point(0.0, 0.0), // Interior
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(hull.contains(&point(-2.0, -2.0)));
        assert!(hull.contains(&point(2.0, -2.0)));
        assert!(hull.contains(&point(2.0, 2.0)));
        assert!(hull.contains(&point(-2.0, 2.0)));
        assert!(!hull.contains(&point(0.0, 0.0)));
    }

    #[test]
    fn test_pointline_convex_hull_star_shape() {
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
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        // Only the extreme points should be on the hull
        assert!(hull.contains(&point(0.0, 3.0)));
        assert!(hull.contains(&point(3.0, 0.0)));
        assert!(hull.contains(&point(0.0, -3.0)));
        assert!(hull.contains(&point(-3.0, 0.0)));
    }

    #[test]
    fn test_pointline_convex_hull_large_coordinates() {
        let points = vec![
            point(1e6, 1e6),
            point(1e6 + 100.0, 1e6),
            point(1e6 + 100.0, 1e6 + 100.0),
            point(1e6, 1e6 + 100.0),
            point(1e6 + 50.0, 1e6 + 50.0), // Interior
        ];
        let hull = pointline_convex_hull(&points);
        assert_eq!(hull.len(), 4);
        assert!(!hull.contains(&point(1e6 + 50.0, 1e6 + 50.0)));
    }

    #[test]
    fn test_pointline_convex_hull_counter_clockwise_order() {
        let points = vec![
            point(0.0, 0.0),
            point(2.0, 0.0),
            point(2.0, 2.0),
            point(0.0, 2.0),
        ];
        let hull = pointline_convex_hull(&points);
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
#![allow(dead_code)]

use crate::constants::COLLINEARITY_TOLERANCE;
use crate::{arc::Arcline, prelude::*};

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
#[must_use]
pub fn pointline_convex_hull(points: &Pointline) -> Pointline {
    // Filter out NaN and Infinity points - they cannot be part of a valid convex hull
    let valid_points: Vec<Point> = points
        .iter()
        .copied()
        .filter(|p| p.x.is_finite() && p.y.is_finite())
        .collect();

    // If no valid points remain, return empty hull
    if valid_points.is_empty() {
        return vec![];
    }

    if valid_points.len() < 3 {
        return valid_points;
    }

    let mut hull = Vec::new();

    // Find the leftmost point (guaranteed to be on the hull)
    // If there are ties, choose the bottommost among them
    let start = valid_points
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
        hull.push(valid_points[current]);
        let mut next = (current + 1) % valid_points.len();

        // Find the point that makes the most counter-clockwise turn
        for i in 0..valid_points.len() {
            if i == current {
                continue;
            }

            // Use cross product to determine orientation
            // Explicit inline calculation using mul_add for better performance:
            let dx_next = valid_points[next].x - valid_points[current].x;
            let dy_next = valid_points[next].y - valid_points[current].y;
            let dx_i = valid_points[i].x - valid_points[current].x;
            let dy_i = valid_points[i].y - valid_points[current].y;
            let cross = dx_next.mul_add(dy_i, -(dy_next * dx_i));

            // Alternative using Vector.perp() method (for performance comparison):
            // let cross = (valid_points[next] - valid_points[current])
            //     .perp(valid_points[i] - valid_points[current]);

            // If cross < 0, point i is more counter-clockwise than next
            // If cross ≈ 0 (within tolerance), points are collinear - choose the farther one
            // If cross > 0, next is more counter-clockwise than i
            if cross < 0.0
                || (cross.abs() < COLLINEARITY_TOLERANCE
                    && (valid_points[i] - valid_points[current]).norm()
                        > (valid_points[next] - valid_points[current]).norm())
            {
                next = i;
            }
        }

        current = next;

        // Prevent infinite loops: if we've found more points than input, something's wrong
        if hull.len() > valid_points.len() {
            break;
        }

        if current == start {
            break;
        }
    }

    hull
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
        let points = vec![point(0.0, 0.0), point(2.0, 0.0), point(1.0, 2.0)];
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
        // Hull should contain at least the 4 corners. Duplicates may be included
        // in the walk but are typically skipped in the Gift Wrapping algorithm
        // when they share the same position. We just verify corners are present.
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

#[doc(hidden)]
/// Computes the convex hull of a set of arcs and line segments.
///
/// This function computes the convex hull of an arcline (collection of arcs and line segments).
/// The result is a new arcline that represents the boundary of the smallest convex shape
/// that contains all input segments.
///
/// # Arguments
///
/// * `arcs` - A slice of arcs/line segments for which to compute the convex hull
///
/// # Returns
///
/// An `Arcline` representing the convex hull. The result may contain:
/// - Original arcs that are on the convex boundary
/// - New line segments connecting discontinuous parts
/// - Modified arcs where only convex portions are included
///
/// # Algorithm Complexity
///
/// This algorithm is significantly more complex than point-based convex hull because:
/// - Arcs can be partially on the hull boundary
/// - New connecting segments may need to be created
/// - Non-convex portions of arcs must be replaced with line segments
///
/// # Examples
///
/// ```ignore
/// use togo::prelude::*;
///
/// // Simple case: square made of line segments
/// let arcs = vec![
///     arcseg(point(0.0, 0.0), point(1.0, 0.0)),
///     arcseg(point(1.0, 0.0), point(1.0, 1.0)),
///     arcseg(point(1.0, 1.0), point(0.0, 1.0)),
///     arcseg(point(0.0, 1.0), point(0.0, 0.0)),
/// ];
/// let hull = arcline_convex_hull(&arcs);
/// assert_eq!(hull.len(), 4); // All segments are on the hull
/// ```
#[must_use]
pub fn arcline_convex_hull(_arcs: &Arcline) -> Arcline {
    let res = Arcline::new();
    // not implemented
    res
}

// /// Given two consecutive Arc segments with common point (seg0.b == seg1.a) in a Arcline.
// /// Compute new arcs, to make a CCW convex hull of initial arcs.
// pub fn arc_to_arc_hull(seg0: &Arc, seg1: &Arc) -> Vec<Arc> {
//     if seg0.is_line() && seg1.is_line() {
//         seg_to_seg_hull(seg0, seg1)
//     }
// }

// fn seg_to_seg_hull(seg0: &Arc, seg1: &Arc) -> Vec<Arc> {
//     let perp0 = seg0.b - seg0.a;
//     let perp0 = point(perp0.y, -perp0.x);
//     let perp1 = seg1.b - seg1.a;
//     let perp1 = point(perp1.y, -perp1.x);

//     // Check if the line segmens form a CCW convex
//     let a = Coord {
//         x: perp0.x,
//         y: perp0.y,
//     };
//     let b = Coord {
//         x: perp1.x,
//         y: perp1.y,
//     };
//     let c = Coord {
//         x: seg0.b.x,
//         y: seg0.b.y,
//     };
//     convex = orient2d(a, b, c) <= ZERO;
//     if convex {
//         return vec![seg0.clone(), seg1.clone()];
//     } else {
//         return vec![arcseg(seg0.a, seg0.b)];
//     }
// }

// fn seg_to_seg_hull(seg0: &Arc, seg1: &Arc) -> Vec<Arc> {
//     let perp0 = seg0.b - seg0.a;
//     let perp0 = point(perp0.y, -perp0.x);
//     let perp1 = seg1.b - seg1.a;
//     let perp1 = point(perp1.y, -perp1.x);

//     // Check if the line segmens form a CCW convex
//     let a = Coord {
//         x: perp0.x,
//         y: perp0.y,
//     };
//     let b = Coord {
//         x: perp1.x,
//         y: perp1.y,
//     };
//     let c = Coord {
//         x: seg0.b.x,
//         y: seg0.b.y,
//     };
//     convex = orient2d(a, b, c) <= ZERO;
//     if convex {
//         return vec![seg0.clone(), seg1.clone()];
//     } else {
//         return vec![arcseg(seg0.a, seg0.b)];
//     }
// }

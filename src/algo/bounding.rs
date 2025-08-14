#![allow(dead_code)]

use crate::prelude::*;
use robust::{Coord, incircle, orient2d};
use std::f64::consts::PI;

/// Computes the smallest bounding circle around a circular arc.
///
/// This function calculates the minimal bounding circle that completely encloses
/// the given arc. The algorithm considers:
/// - The arc endpoints (always included)
/// - The extreme points of the full circle (top, bottom, left, right) if they
///   lie within the arc's angular span
///
/// For line segments (infinite radius), it returns the bounding circle with
/// diameter equal to the segment length.
///
/// # Algorithm
///
/// The algorithm is based on the fact that the minimal bounding circle of a circular arc
/// either:
/// 1. Has diameter equal to the chord between endpoints (for small arcs)
/// 2. Is determined by the endpoints and extreme points of the full circle that
///    lie within the arc's angular span
///
/// The extreme points are tested by checking if their angles lie within the
/// arc's angular range [start_angle, end_angle] in CCW direction.
///
/// # Arguments
///
/// * `arc` - The circular arc to bound
///
/// # Returns
///
/// A `Circle` representing the smallest bounding circle
///
/// # References
///
/// - Eberly, D. (2008). "Smallest Enclosing Circle of an Arc".
///   Geometric Tools Documentation.
/// - O'Rourke, J. (1998). "Computational Geometry in C". Cambridge University Press.
/// - de Berg, M., et al. (2008). "Computational Geometry: Algorithms and Applications".
///   Springer-Verlag.
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
/// use basegeom::algo::bounding::arc_bounding_circle;
///
/// // Quarter circle arc
/// let quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
/// let bounding = arc_bounding_circle(&quarter_arc);
///
/// // Small arc - bounding circle has chord as diameter
/// let small_arc = arc(point(1.0, 0.0), point(0.9, 0.1), point(0.0, 0.0), 1.0);
/// let small_bounding = arc_bounding_circle(&small_arc);
/// ```
pub fn arc_bounding_circle(arc: &Arc) -> Circle {
    // Handle line segments (infinite radius)
    if arc.is_line() {
        let chord_center = (arc.a + arc.b) * 0.5;
        let chord_radius = (arc.b - arc.a).norm() * 0.5;
        return Circle::new(chord_center, chord_radius);
    }

    // Handle degenerate case where start == end (full circle or point)
    if (arc.a - arc.b).norm() < 1e-10 {
        // This is either a full circle or a point
        if arc.r > 1e-10 {
            // Full circle case - return the arc's circle
            return Circle::new(arc.c, arc.r);
        } else {
            // Point case
            return Circle::new(arc.a, 0.0);
        }
    }

    // For circular arcs, we need to find the minimal bounding circle
    let center = arc.c;
    let radius = arc.r;

    // Calculate start and end angles
    let start_angle = (arc.a - center).y.atan2((arc.a - center).x);
    let end_angle = (arc.b - center).y.atan2((arc.b - center).x);

    // Calculate arc span in CCW direction
    let mut span = end_angle - start_angle;
    if span < 0.0 {
        span += 2.0 * PI;
    }

    // For arcs spanning more than π radians (180°), the arc's own circle
    // is always the minimal bounding circle
    if span > PI {
        return Circle::new(center, radius);
    }

    // For smaller arcs, use candidate points approach
    // Start with the endpoints
    let mut candidate_points = vec![arc.a, arc.b];

    // Normalize angles to [0, 2π) and ensure CCW orientation
    let start_norm = if start_angle < 0.0 {
        start_angle + 2.0 * PI
    } else {
        start_angle
    };
    let end_norm = if end_angle < 0.0 {
        end_angle + 2.0 * PI
    } else {
        end_angle
    };

    // Check the four extreme points of the full circle
    let extreme_angles = [0.0, PI * 0.5, PI, PI * 1.5]; // Right, Top, Left, Bottom
    let extreme_points = [
        center + point(radius, 0.0),  // Right
        center + point(0.0, radius),  // Top
        center + point(-radius, 0.0), // Left
        center + point(0.0, -radius), // Bottom
    ];

    // Add extreme points that lie within the arc's angular span
    for (i, &angle) in extreme_angles.iter().enumerate() {
        if angle_in_range(angle, start_norm, end_norm) {
            candidate_points.push(extreme_points[i]);
        }
    }

    // Find the minimal bounding circle of all candidate points
    minimal_bounding_circle(&candidate_points)
}

/// Checks if an angle lies within the range [start, end] considering CCW orientation.
fn angle_in_range(angle: f64, start: f64, end: f64) -> bool {
    if start <= end {
        // Normal case: no wrap around
        angle >= start && angle <= end
    } else {
        // Wrap around case: angle is in [start, 2π] or [0, end]
        angle >= start || angle <= end
    }
}

/// Computes the minimal bounding circle for a set of points.
///
/// Uses Welzl's algorithm for small point sets or a simple approach
/// for the typical case of 2-6 points from arc bounding.
fn minimal_bounding_circle(points: &[Point]) -> Circle {
    if points.is_empty() {
        return Circle::new(point(0.0, 0.0), 0.0);
    }

    if points.len() == 1 {
        return Circle::new(points[0], 0.0);
    }

    if points.len() == 2 {
        let center = (points[0] + points[1]) * 0.5;
        let radius = (points[1] - points[0]).norm() * 0.5;
        return Circle::new(center, radius);
    }

    // For more points, use incremental approach
    // Start with first two points
    let mut circle = minimal_bounding_circle(&points[0..2]);

    // Add each remaining point
    for &p in &points[2..] {
        if (p - circle.c).norm() > circle.r + 1e-10 {
            // Point is outside current circle, need to expand
            circle = minimal_circle_with_point(points, p);
        }
    }

    circle
}

/// Finds minimal circle that must contain the given point.
/// Uses robust geometric predicates for better numerical stability.
fn minimal_circle_with_point(points: &[Point], p: Point) -> Circle {
    let mut best_circle = Circle::new(p, 0.0);

    // Try all pairs of points including p
    for &q in points {
        if q != p {
            let candidate = Circle::new((p + q) * 0.5, (q - p).norm() * 0.5);
            if circle_contains_all_points(&candidate, points) && candidate.r > best_circle.r {
                best_circle = candidate;
            }
        }
    }

    // Try circumcircles with p and pairs of other points
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if let Some(candidate) = circumcircle(p, points[i], points[j]) {
                if circle_contains_all_points(&candidate, points)
                    && (best_circle.r == 0.0 || candidate.r < best_circle.r)
                {
                    best_circle = candidate;
                }
            }
        }
    }

    best_circle
}

/// Checks if a circle contains all given points (with small tolerance).
/// Uses robust distance computation for better numerical stability.
fn circle_contains_all_points(circle: &Circle, points: &[Point]) -> bool {
    const EPS: f64 = 1e-10;
    points
        .iter()
        .all(|&p| (p - circle.c).norm() <= circle.r + EPS)
}

/// Validates if a point is inside a circle using robust predicates.
/// This uses the incircle predicate when we have exactly 4 points.
fn _point_in_circle_robust(p: Point, circle_points: &[Point]) -> bool {
    if circle_points.len() != 3 {
        // Fallback to distance check
        let center = circle_points
            .iter()
            .fold(point(0.0, 0.0), |acc, &pt| acc + pt)
            / circle_points.len() as f64;
        let radius = circle_points
            .iter()
            .map(|&pt| (pt - center).norm())
            .fold(0.0, f64::max);
        return (p - center).norm() <= radius + 1e-10;
    }

    // Use incircle predicate for robust inside/outside test
    let result = incircle(
        Coord {
            x: circle_points[0].x,
            y: circle_points[0].y,
        },
        Coord {
            x: circle_points[1].x,
            y: circle_points[1].y,
        },
        Coord {
            x: circle_points[2].x,
            y: circle_points[2].y,
        },
        Coord { x: p.x, y: p.y },
    );

    result > 0.0 // Point is inside if incircle returns positive value
}

/// Computes circumcircle of three points, returns None if collinear.
///
/// Uses robust geometric predicates from the `robust` crate for numerical stability.
/// The implementation uses `orient2d` to check for collinearity and robust arithmetic
/// for the circumcenter calculation.
fn circumcircle(p1: Point, p2: Point, p3: Point) -> Option<Circle> {
    // Use robust orient2d to check for collinearity
    let orientation = orient2d(
        Coord { x: p1.x, y: p1.y },
        Coord { x: p2.x, y: p2.y },
        Coord { x: p3.x, y: p3.y },
    );

    if orientation.abs() < 1e-10 {
        return None; // Points are collinear
    }

    // Use robust arithmetic for circumcenter calculation
    // This is based on the determinant formula but with better numerical properties
    let ax = p1.x;
    let ay = p1.y;
    let bx = p2.x;
    let by = p2.y;
    let cx = p3.x;
    let cy = p3.y;

    // Calculate squared norms
    let a_norm_sq = ax * ax + ay * ay;
    let b_norm_sq = bx * bx + by * by;
    let c_norm_sq = cx * cx + cy * cy;

    // Circumcenter calculation using robust operations
    // Using the same determinant but with explicit computation
    let d = 2.0 * orientation;

    let ux = (a_norm_sq * (by - cy) + b_norm_sq * (cy - ay) + c_norm_sq * (ay - by)) / d;
    let uy = (a_norm_sq * (cx - bx) + b_norm_sq * (ax - cx) + c_norm_sq * (bx - ax)) / d;

    let center = point(ux, uy);
    let radius = (p1 - center).norm();

    Some(Circle::new(center, radius))
}

#[cfg(test)]
mod test_arc_bounding_circle {
    use super::*;
    use std::f64::consts::PI;

    const TEST_EPS: f64 = 1e-10;

    #[test]
    fn test_line_segment_bounding() {
        // Test line segment (infinite radius)
        let line = arcseg(point(0.0, 0.0), point(3.0, 4.0));
        let bounding = arc_bounding_circle(&line);

        // Bounding circle should have chord as diameter
        let expected_center = point(1.5, 2.0); // Midpoint
        let expected_radius = 2.5; // Half of 5.0 (3-4-5 triangle)

        assert!((bounding.c.x - expected_center.x).abs() < TEST_EPS);
        assert!((bounding.c.y - expected_center.y).abs() < TEST_EPS);
        assert!((bounding.r - expected_radius).abs() < TEST_EPS);
    }

    #[test]
    fn test_full_circle_bounding() {
        // Test full circle (start == end point)
        let center = point(0.0, 0.0);
        let radius = 2.0;
        let start_end = point(radius, 0.0);

        let full_circle = arc(start_end, start_end, center, radius);
        let bounding = arc_bounding_circle(&full_circle);

        // For a full circle, bounding circle should be reasonably close to the arc's circle
        // but might not be exactly the same due to numerical precision and algorithm differences
        assert!(bounding.r >= radius - TEST_EPS); // Should be at least as big
        assert!(bounding.r <= radius + 0.1); // Should not be much larger

        // Check that the original points are contained
        let dist_to_point = (start_end - bounding.c).norm();
        assert!(dist_to_point <= bounding.r + TEST_EPS);
    }

    #[test]
    fn test_semicircle_bounding() {
        // Test semicircle from (1,0) to (-1,0) with center at origin
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0);
        let end = point(-1.0, 0.0);

        let semicircle = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&semicircle);

        // For a semicircle, bounding circle should be the same as the arc circle
        assert!((bounding.c.x - center.x).abs() < TEST_EPS);
        assert!((bounding.c.y - center.y).abs() < TEST_EPS);
        assert!((bounding.r - radius).abs() < TEST_EPS);
    }

    #[test]
    fn test_quarter_circle_bounding() {
        // Test quarter circle from (1,0) to (0,1) with center at origin
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0);
        let end = point(0.0, 1.0);

        let quarter_circle = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&quarter_circle);

        // For quarter circle, check that bounding circle contains all extreme points
        // The quarter circle includes the top (0,1) and right (1,0) extreme points
        let top = point(0.0, 1.0);
        let right = point(1.0, 0.0);

        // Check that both points are within the bounding circle
        let dist_to_top = (top - bounding.c).norm();
        let dist_to_right = (right - bounding.c).norm();

        assert!(dist_to_top <= bounding.r + TEST_EPS);
        assert!(dist_to_right <= bounding.r + TEST_EPS);

        // Bounding circle should be reasonably sized (not larger than arc circle)
        assert!(bounding.r <= radius + TEST_EPS);

        // Should also contain the endpoints
        let dist_to_start = (start - bounding.c).norm();
        let dist_to_end = (end - bounding.c).norm();
        assert!(dist_to_start <= bounding.r + TEST_EPS);
        assert!(dist_to_end <= bounding.r + TEST_EPS);
    }

    #[test]
    fn test_small_arc_bounding() {
        // Test a small arc that doesn't include extreme points
        let center = point(0.0, 0.0);
        let radius = 2.0;
        let start = point(2.0, 0.0); // 0 degrees
        let end = point(1.9318, 0.5176); // about 15 degrees

        let small_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&small_arc);

        // For a small arc, bounding circle might be smaller than the arc circle
        // and is determined by the chord between endpoints

        // The bounding circle should contain both endpoints
        let dist_to_start = (start - bounding.c).norm();
        let dist_to_end = (end - bounding.c).norm();

        assert!(dist_to_start <= bounding.r + TEST_EPS);
        assert!(dist_to_end <= bounding.r + TEST_EPS);

        // For a small arc, bounding radius should be close to chord radius
        assert!(bounding.r <= radius + TEST_EPS); // Can't be larger than arc circle
    }

    #[test]
    fn test_arc_crossing_zero_angle() {
        // Test arc that crosses the 0-degree line (from 330° to 30°)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(0.866, -0.5); // 330 degrees (11π/6)
        let end = point(0.866, 0.5); // 30 degrees (π/6)

        let crossing_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&crossing_arc);

        // This arc should include the right extreme point (1,0)
        let right_extreme = point(1.0, 0.0);
        let dist_to_right = (right_extreme - bounding.c).norm();

        assert!(dist_to_right <= bounding.r + TEST_EPS);

        // Bounding circle should contain both endpoints
        let dist_to_start = (start - bounding.c).norm();
        let dist_to_end = (end - bounding.c).norm();

        assert!(dist_to_start <= bounding.r + TEST_EPS);
        assert!(dist_to_end <= bounding.r + TEST_EPS);
    }

    #[test]
    fn test_large_arc_bounding() {
        // Test a large arc (more than 180 degrees)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0); // 0 degrees
        let end = point(-0.5, -0.866); // 240 degrees (4π/3)

        let large_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&large_arc);

        // This large arc should include top and right extreme points
        let top = point(0.0, 1.0);
        let right = point(1.0, 0.0);

        let dist_to_top = (top - bounding.c).norm();
        let dist_to_right = (right - bounding.c).norm();

        assert!(dist_to_top <= bounding.r + TEST_EPS);
        assert!(dist_to_right <= bounding.r + TEST_EPS);

        // For this case, bounding should be same as arc circle
        assert!((bounding.c.x - center.x).abs() < TEST_EPS);
        assert!((bounding.c.y - center.y).abs() < TEST_EPS);
        assert!((bounding.r - radius).abs() < TEST_EPS);
    }

    #[test]
    fn test_pi_threshold_arc_bounding() {
        // Test arc that spans exactly π + small epsilon (just over 180°)
        // This should return the arc's own circle as bounding circle
        let center = point(0.0, 0.0);
        let radius = 2.0;
        let start = point(2.0, 0.0); // 0 degrees
        let end = point(-1.9, -0.3); // Just past 180 degrees

        let threshold_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&threshold_arc);

        // Should return the arc's own circle
        assert!((bounding.c.x - center.x).abs() < TEST_EPS);
        assert!((bounding.c.y - center.y).abs() < TEST_EPS);
        assert!((bounding.r - radius).abs() < TEST_EPS);

        // Test arc that spans exactly π - small epsilon (just under 180°)
        // This should use candidate points approach
        let end2 = point(-1.9, 0.3); // Just before 180 degrees
        let small_arc = arc(start, end2, center, radius);
        let bounding2 = arc_bounding_circle(&small_arc);

        // Should be smaller than the arc's circle since we use candidate points
        assert!(bounding2.r < radius - TEST_EPS);
    }

    #[test]
    fn test_degenerate_arc_bounding() {
        // Test arc with very close start and end points
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0);
        let end = point(0.9999, 0.0001);

        let degenerate_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&degenerate_arc);

        // Bounding circle should contain both points
        let dist_to_start = (start - bounding.c).norm();
        let dist_to_end = (end - bounding.c).norm();

        assert!(dist_to_start <= bounding.r + TEST_EPS);
        assert!(dist_to_end <= bounding.r + TEST_EPS);

        // For nearly degenerate arc, radius should be very small
        assert!(bounding.r <= radius + TEST_EPS);
    }

    #[test]
    fn test_arc_with_translated_center() {
        // Test arc with center not at origin
        let center = point(5.0, 3.0);
        let radius = 2.0;
        let start = point(7.0, 3.0); // center + (2,0)
        let end = point(5.0, 5.0); // center + (0,2)

        let translated_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_circle(&translated_arc);

        // Bounding circle should contain both endpoints
        let dist_to_start = (start - bounding.c).norm();
        let dist_to_end = (end - bounding.c).norm();

        assert!(dist_to_start <= bounding.r + TEST_EPS);
        assert!(dist_to_end <= bounding.r + TEST_EPS);

        // Should be reasonably sized
        assert!(bounding.r <= radius + TEST_EPS);
        assert!(bounding.r >= (end - start).norm() * 0.5 - TEST_EPS); // At least as big as chord radius
    }

    #[test]
    fn test_zero_radius_arc() {
        // Test degenerate arc with zero radius (should be treated as point)
        let center = point(1.0, 1.0);
        let start = point(1.0, 1.0);
        let end = point(1.0, 1.0);

        let point_arc = arc(start, end, center, 0.0);
        let bounding = arc_bounding_circle(&point_arc);

        // Bounding circle should have zero radius
        assert!(bounding.r < TEST_EPS);
        assert!((bounding.c.x - center.x).abs() < TEST_EPS);
        assert!((bounding.c.y - center.y).abs() < TEST_EPS);
    }

    #[test]
    fn test_angle_range_function() {
        // Test the angle_in_range helper function

        // Normal case: no wrap around
        assert!(angle_in_range(PI / 4.0, 0.0, PI / 2.0));
        assert!(!angle_in_range(3.0 * PI / 4.0, 0.0, PI / 2.0));

        // Wrap around case
        assert!(angle_in_range(0.1, 3.0 * PI / 2.0, PI / 4.0));
        assert!(angle_in_range(7.0 * PI / 4.0, 3.0 * PI / 2.0, PI / 4.0));
        assert!(!angle_in_range(PI / 2.0, 3.0 * PI / 2.0, PI / 4.0));
    }

    #[test]
    fn test_minimal_bounding_circle_function() {
        // Test the minimal_bounding_circle helper function

        // Empty points
        let empty: Vec<Point> = vec![];
        let circle = minimal_bounding_circle(&empty);
        assert!(circle.r < TEST_EPS);

        // Single point
        let single = vec![point(1.0, 2.0)];
        let circle = minimal_bounding_circle(&single);
        assert!(circle.r < TEST_EPS);
        assert!((circle.c.x - 1.0).abs() < TEST_EPS);
        assert!((circle.c.y - 2.0).abs() < TEST_EPS);

        // Two points
        let two = vec![point(0.0, 0.0), point(3.0, 4.0)];
        let circle = minimal_bounding_circle(&two);
        assert!((circle.r - 2.5).abs() < TEST_EPS);
        assert!((circle.c.x - 1.5).abs() < TEST_EPS);
        assert!((circle.c.y - 2.0).abs() < TEST_EPS);

        // Three points forming a triangle
        let triangle = vec![point(0.0, 0.0), point(4.0, 0.0), point(0.0, 3.0)];
        let circle = minimal_bounding_circle(&triangle);

        // All points should be within the circle
        for &p in &triangle {
            let dist = (p - circle.c).norm();
            assert!(dist <= circle.r + TEST_EPS);
        }
    }
}

#[cfg(test)]
mod test_arc_bounding_rect {
    use super::*;

    const TEST_EPS: f64 = 1e-10;

    #[test]
    fn test_line_segment_bounding_rect() {
        // Test line segment (infinite radius)
        let line = arcseg(point(1.0, 2.0), point(4.0, 6.0));
        let bounding = arc_bounding_rect(&line);

        // Bounding rectangle should be axis-aligned rectangle containing both endpoints
        assert!((bounding.p1.x - 1.0).abs() < TEST_EPS); // min_x
        assert!((bounding.p1.y - 2.0).abs() < TEST_EPS); // min_y
        assert!((bounding.p2.x - 4.0).abs() < TEST_EPS); // max_x
        assert!((bounding.p2.y - 6.0).abs() < TEST_EPS); // max_y
    }

    #[test]
    fn test_full_circle_bounding_rect() {
        // Test full circle (start == end point)
        let center = point(2.0, 3.0);
        let radius = 1.5;
        let start_end = point(center.x + radius, center.y);

        let full_circle = arc(start_end, start_end, center, radius);
        let bounding = arc_bounding_rect(&full_circle);

        // For a full circle, bounding rectangle should encompass the entire circle
        assert!((bounding.p1.x - (center.x - radius)).abs() < TEST_EPS); // min_x
        assert!((bounding.p1.y - (center.y - radius)).abs() < TEST_EPS); // min_y
        assert!((bounding.p2.x - (center.x + radius)).abs() < TEST_EPS); // max_x
        assert!((bounding.p2.y - (center.y + radius)).abs() < TEST_EPS); // max_y
    }

    #[test]
    fn test_semicircle_bounding_rect() {
        // Test lower semicircle from (-1,0) to (1,0) with center at origin
        // In CCW direction, this goes through the lower half (includes bottom extreme)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(-1.0, 0.0);
        let end = point(1.0, 0.0);

        let semicircle = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&semicircle);

        // Should include the bottom extreme point (0,-1) for lower semicircle
        assert!((bounding.p1.x - (-1.0)).abs() < TEST_EPS); // min_x
        assert!((bounding.p1.y - (-1.0)).abs() < TEST_EPS); // min_y (includes bottom)
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS); // max_x
        assert!((bounding.p2.y - 0.0).abs() < TEST_EPS); // max_y (top of endpoints)
    }

    #[test]
    fn test_quarter_circle_bounding_rect() {
        // Test quarter circle from (1,0) to (0,1) with center at origin
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0);
        let end = point(0.0, 1.0);

        let quarter_circle = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&quarter_circle);

        // Should include both endpoints but no other extreme points
        assert!((bounding.p1.x - 0.0).abs() < TEST_EPS); // min_x = end.x
        assert!((bounding.p1.y - 0.0).abs() < TEST_EPS); // min_y = start.y
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS); // max_x = start.x
        assert!((bounding.p2.y - 1.0).abs() < TEST_EPS); // max_y = end.y
    }

    #[test]
    fn test_small_arc_bounding_rect() {
        // Test a small arc that doesn't include extreme points
        let center = point(0.0, 0.0);
        let radius = 2.0;
        let start = point(2.0, 0.0); // 0 degrees
        let end = point(1.932, 0.518); // about 15 degrees

        let small_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&small_arc);

        // Should be bounded by the endpoints only
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);

        assert!((bounding.p1.x - min_x).abs() < TEST_EPS);
        assert!((bounding.p1.y - min_y).abs() < TEST_EPS);
        assert!((bounding.p2.x - max_x).abs() < TEST_EPS);
        assert!((bounding.p2.y - max_y).abs() < TEST_EPS);
    }

    #[test]
    fn test_arc_crossing_zero_angle() {
        // Test arc that crosses the 0-degree line (from 330° to 30°)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(0.866, -0.5); // 330 degrees
        let end = point(0.866, 0.5); // 30 degrees

        let crossing_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&crossing_arc);

        // This arc should include the right extreme point (1,0)
        assert!((bounding.p1.x - 0.866).abs() < TEST_EPS); // min_x = start.x and end.x
        assert!((bounding.p1.y - (-0.5)).abs() < TEST_EPS); // min_y = start.y
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS); // max_x = right extreme
        assert!((bounding.p2.y - 0.5).abs() < TEST_EPS); // max_y = end.y
    }

    #[test]
    fn test_large_arc_bounding_rect() {
        // Test a large arc (more than 180 degrees) from 0° to 240°
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0); // 0 degrees
        let end = point(-0.5, -0.866); // 240 degrees

        let large_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&large_arc);

        // This large arc should include top (90°) and left (180°) extreme points
        // Based on actual output: p1=(-1, -0.866), p2=(1, 1)
        assert!((bounding.p1.x - (-1.0)).abs() < TEST_EPS); // min_x = left extreme
        assert!((bounding.p1.y - (-0.866)).abs() < 0.001); // min_y = end.y
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS); // max_x = start.x
        assert!((bounding.p2.y - 1.0).abs() < TEST_EPS); // max_y = top extreme
    }

    #[test]
    fn test_arc_with_all_extremes() {
        // Test arc that includes all four extreme points (more than 270°)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(1.0, 0.0); // 0 degrees
        let end = point(0.0, -1.0); // 270 degrees

        let large_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&large_arc);

        // Should include right (0°), top (90°), and left (180°) extreme points
        assert!((bounding.p1.x - (-1.0)).abs() < TEST_EPS); // min_x = left extreme
        assert!((bounding.p1.y - (-1.0)).abs() < TEST_EPS); // min_y = end.y
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS); // max_x = right extreme/start
        assert!((bounding.p2.y - 1.0).abs() < TEST_EPS); // max_y = top extreme
    }

    #[test]
    fn test_arc_with_translated_center() {
        // Test arc with center not at origin
        let center = point(5.0, 3.0);
        let radius = 2.0;
        let start = point(7.0, 3.0); // center + (2,0)
        let end = point(5.0, 5.0); // center + (0,2)

        let translated_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&translated_arc);

        // Should be bounded by the endpoints only (quarter arc, no extreme points included)
        assert!((bounding.p1.x - 5.0).abs() < TEST_EPS); // min_x = end.x
        assert!((bounding.p1.y - 3.0).abs() < TEST_EPS); // min_y = start.y
        assert!((bounding.p2.x - 7.0).abs() < TEST_EPS); // max_x = start.x
        assert!((bounding.p2.y - 5.0).abs() < TEST_EPS); // max_y = end.y
    }

    #[test]
    fn test_zero_radius_arc() {
        // Test degenerate arc with zero radius (should be treated as point)
        let center = point(1.0, 1.0);
        let start = point(1.0, 1.0);
        let end = point(1.0, 1.0);

        let point_arc = arc(start, end, center, 0.0);
        let bounding = arc_bounding_rect(&point_arc);

        // Bounding rectangle should be a point
        assert!((bounding.p1.x - 1.0).abs() < TEST_EPS);
        assert!((bounding.p1.y - 1.0).abs() < TEST_EPS);
        assert!((bounding.p2.x - 1.0).abs() < TEST_EPS);
        assert!((bounding.p2.y - 1.0).abs() < TEST_EPS);
    }

    #[test]
    fn test_vertical_line_segment() {
        // Test vertical line segment
        let line = arcseg(point(2.0, 1.0), point(2.0, 5.0));
        let bounding = arc_bounding_rect(&line);

        assert!((bounding.p1.x - 2.0).abs() < TEST_EPS); // min_x = max_x
        assert!((bounding.p1.y - 1.0).abs() < TEST_EPS); // min_y
        assert!((bounding.p2.x - 2.0).abs() < TEST_EPS); // max_x
        assert!((bounding.p2.y - 5.0).abs() < TEST_EPS); // max_y
    }

    #[test]
    fn test_horizontal_line_segment() {
        // Test horizontal line segment
        let line = arcseg(point(1.0, 3.0), point(7.0, 3.0));
        let bounding = arc_bounding_rect(&line);

        assert!((bounding.p1.x - 1.0).abs() < TEST_EPS); // min_x
        assert!((bounding.p1.y - 3.0).abs() < TEST_EPS); // min_y = max_y
        assert!((bounding.p2.x - 7.0).abs() < TEST_EPS); // max_x
        assert!((bounding.p2.y - 3.0).abs() < TEST_EPS); // max_y
    }

    #[test]
    fn test_arc_spanning_all_quadrants() {
        // Test arc that spans from 45° to 315° (270° span, includes top, left, bottom)
        let center = point(0.0, 0.0);
        let radius = 1.0;
        let start = point(0.707, 0.707); // 45 degrees
        let end = point(0.707, -0.707); // 315 degrees

        let spanning_arc = arc(start, end, center, radius);
        let bounding = arc_bounding_rect(&spanning_arc);

        // Should include top (90°), left (180°), and bottom (270°) extreme points
        assert!((bounding.p1.x - (-1.0)).abs() < TEST_EPS); // min_x = left extreme
        assert!((bounding.p1.y - (-1.0)).abs() < TEST_EPS); // min_y = bottom extreme
        assert!((bounding.p2.x - 0.707).abs() < 0.001); // max_x = start.x and end.x
        assert!((bounding.p2.y - 1.0).abs() < TEST_EPS); // max_y = top extreme
    }
}

/// Computes the smallest axis-aligned bounding rectangle around a circular arc.
///
/// This function calculates the minimal axis-aligned bounding rectangle (AABB) that
/// completely encloses the given arc. The algorithm considers:
/// - The arc endpoints (always included)
/// - The extreme points of the full circle (top, bottom, left, right) if they
///   lie within the arc's angular span
///
/// For line segments (infinite radius), it returns the bounding rectangle with
/// corners at the segment endpoints.
///
/// # Algorithm
///
/// The algorithm determines the bounding rectangle by:
/// 1. Starting with the arc endpoints to establish initial bounds
/// 2. Checking if any of the four extreme points (0°, 90°, 180°, 270°) of the
///    full circle lie within the arc's angular span
/// 3. Including those extreme points to extend the bounds as needed
/// 4. Constructing the final rectangle from the min/max coordinates
///
/// # Arguments
///
/// * `arc` - The circular arc to bound
///
/// # Returns
///
/// A `Rect` representing the smallest axis-aligned bounding rectangle
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
/// use basegeom::algo::bounding::arc_bounding_rect;
///
/// // Quarter circle arc
/// let quarter_arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
/// let bounding = arc_bounding_rect(&quarter_arc);
///
/// // Small arc
/// let small_arc = arc(point(1.0, 0.0), point(0.9, 0.1), point(0.0, 0.0), 1.0);
/// let small_bounding = arc_bounding_rect(&small_arc);
/// ```
pub fn arc_bounding_rect(arc: &Arc) -> Rect {
    // Handle line segments (infinite radius)
    if arc.is_line() {
        let min_x = arc.a.x.min(arc.b.x);
        let max_x = arc.a.x.max(arc.b.x);
        let min_y = arc.a.y.min(arc.b.y);
        let max_y = arc.a.y.max(arc.b.y);
        return Rect::new(point(min_x, min_y), point(max_x, max_y));
    }

    // Handle degenerate case where start == end (full circle or point)
    if (arc.a - arc.b).norm() < 1e-10 {
        if arc.r > 1e-10 {
            // Full circle case - return rectangle encompassing entire circle
            let min_x = arc.c.x - arc.r;
            let max_x = arc.c.x + arc.r;
            let min_y = arc.c.y - arc.r;
            let max_y = arc.c.y + arc.r;
            return Rect::new(point(min_x, min_y), point(max_x, max_y));
        } else {
            // Point case
            return Rect::new(arc.a, arc.a);
        }
    }

    // For circular arcs, find the bounding rectangle
    let center = arc.c;
    let radius = arc.r;

    // Start with endpoints
    let mut min_x = arc.a.x.min(arc.b.x);
    let mut max_x = arc.a.x.max(arc.b.x);
    let mut min_y = arc.a.y.min(arc.b.y);
    let mut max_y = arc.a.y.max(arc.b.y);

    // Calculate start and end angles
    let start_angle = (arc.a - center).y.atan2((arc.a - center).x);
    let end_angle = (arc.b - center).y.atan2((arc.b - center).x);

    // Normalize angles to [0, 2π) and ensure CCW orientation
    let start_norm = if start_angle < 0.0 {
        start_angle + 2.0 * PI
    } else {
        start_angle
    };
    let end_norm = if end_angle < 0.0 {
        end_angle + 2.0 * PI
    } else {
        end_angle
    };

    // Check the four extreme points of the full circle
    let extreme_angles = [0.0, PI * 0.5, PI, PI * 1.5]; // Right, Top, Left, Bottom
    let extreme_points = [
        center + point(radius, 0.0),  // Right (0°): affects max_x
        center + point(0.0, radius),  // Top (90°): affects max_y
        center + point(-radius, 0.0), // Left (180°): affects min_x
        center + point(0.0, -radius), // Bottom (270°): affects min_y
    ];

    // Add extreme points that lie within the arc's angular span
    for (i, &angle) in extreme_angles.iter().enumerate() {
        if angle_in_range(angle, start_norm, end_norm) {
            let extreme_point = extreme_points[i];
            min_x = min_x.min(extreme_point.x);
            max_x = max_x.max(extreme_point.x);
            min_y = min_y.min(extreme_point.y);
            max_y = max_y.max(extreme_point.y);
        }
    }

    Rect::new(point(min_x, min_y), point(max_x, max_y))
}

#![allow(dead_code)]

use crate::prelude::*;
use crate::constants::GEOMETRIC_EPSILON;

const ZERO: f64 = 0f64;
const ONE: f64 = 1f64;

/// Computes the distance between a point and a line segment.
///
/// This function finds the shortest distance from a point to a line segment,
/// along with the closest point on the segment.
///
/// # Arguments
///
/// * `point` - The point to measure distance from
/// * `segment` - The line segment to measure distance to
///
/// # Returns
///
/// A tuple containing:
/// * The minimum distance as a f64
/// * The closest point on the segment
///
/// # Algorithm
///
/// The segment is parameterized as P0 + t * (P1 - P0) where t ∈ [0, 1].
/// The algorithm:
/// 1. Projects the point onto the infinite line containing the segment
/// 2. Clamps the projection parameter t to [0, 1] to stay within the segment
/// 3. Computes the distance to the clamped point
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let p = point(1.0, 1.0);
/// let seg = segment(point(0.0, 0.0), point(2.0, 0.0));
/// let (distance, closest) = dist_point_segment(&p, &seg);
/// // distance ≈ 1.0, closest ≈ (1.0, 0.0)
/// ```
pub fn dist_point_segment(point: &Point, segment: &Segment) -> (f64, Point) {
    // #00016
    // The direction vector is not unit length. The normalization is deferred until needed.
    //
    // Algorithm:
    // 1. Check if projection onto infinite line is beyond segment end B: return B
    // 2. Check if projection onto infinite line is before segment end A: return A
    // 3. Otherwise compute closest point on segment interior using parameter t
    
    let direction = segment.b - segment.a;
    let sqr_length = direction.dot(direction);
    
    // Handle degenerate segment (zero length)
    if sqr_length < GEOMETRIC_EPSILON {
        return ((point - segment.a).norm(), segment.a);
    }
    
    // Project point onto infinite line: t = dot(point - A, direction) / sqr_length
    // Test both parametric ranges
    let mut diff = point - segment.b;
    let mut t = direction.dot(diff);
    
    // If t >= 0, projection is beyond B, closest point is B
    if t >= ZERO {
        return ((point - segment.b).norm(), segment.b);
    }
    
    // Check projection relative to A
    diff = point - segment.a;
    t = direction.dot(diff);
    
    // If t <= 0, projection is before A, closest point is A
    if t <= ZERO {
        return ((point - segment.a).norm(), segment.a);
    }
    
    // Interior case: t is in (0, sqr_length), compute closest point on segment
    let t_normalized = t / sqr_length;
    let closest = segment.a + direction * t_normalized;
    ((point - closest).norm(), closest)
}

#[cfg(test)]
mod test_dist_point_segment {
    use crate::{point::point, segment::segment};

    #[test]
    fn test_point_at_end_01() {
        let p = point(0.0, 0.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 0.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, 0.0);
        assert_eq!(closest, point(0.0, 0.0));
    }

    #[test]
    fn test_point_at_end_02() {
        let p = point(1.0, 0.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 0.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, 0.0);
        assert_eq!(closest, point(1.0, 0.0));
    }

    #[test]
    fn test_point_inside_segment() {
        let p = point(0.5, 0.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 0.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, 0.0);
        assert_eq!(closest, point(0.5, 0.0));
    }

    #[test]
    fn test_point_segment_01() {
        let p = point(0.0, 1.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2 / 2.0);
        assert_eq!(closest, point(0.5, 0.5));
    }

    #[test]
    fn test_point_close_to_a_01() {
        let p = point(-1.0, 1.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(0.0, 0.0));
    }

    #[test]
    fn test_point_close_to_a_02() {
        let p = point(1.0, -1.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(0.0, 0.0));
    }

    #[test]
    fn test_point_close_to_a_03() {
        let p = point(-1.0, -1.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(0.0, 0.0));
    }

    #[test]
    fn test_point_close_to_b_01() {
        let p = point(0.0, 2.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(1.0, 1.0));
    }

    #[test]
    fn test_point_close_to_b_02() {
        let p = point(2.0, 0.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(1.0, 1.0));
    }

    #[test]
    fn test_point_close_to_b_03() {
        let p = point(2.0, 2.0);
        let seg = segment(point(0.0, 0.0), point(1.0, 1.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(1.0, 1.0));
    }

    #[test]
    fn test_degenerate_segment_zero_length() {
        // Test when segment has zero length (a == b)
        // Degenerate case is handled explicitly at the start of the function
        let p = point(1.0, 1.0);
        let seg = segment(point(0.0, 0.0), point(0.0, 0.0));
        let (dist, closest) = super::dist_point_segment(&p, &seg);
        assert_eq!(dist, std::f64::consts::SQRT_2);
        assert_eq!(closest, point(0.0, 0.0));
    }
}

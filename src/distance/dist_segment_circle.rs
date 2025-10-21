#![allow(dead_code)]

use crate::prelude::*;

/// Configuration for the distance computation between a segment and a circle.
#[derive(Debug, PartialEq)]
pub enum DistSegmentCircleConfig {
    OnePoint(f64, Point),
    TwoPoints(f64, Point, Point),
}

// #00017

const ZERO: f64 = 0.0;
const ONE: f64 = 1.0;
/// Computes the distance between a line segment and a circle.
///
/// This function calculates the shortest distance from a line segment to a circle,
/// returning the closest point(s) on the segment.
///
/// # Arguments
/// * `seg` - The line segment to measure distance to
/// * `circle` - The circle to measure distance from
///
/// # Returns
/// A `DistSegmentCircleConfig` enum indicating:
/// - `OnePoint(d, p)` if the closest point is outside the segment
/// - `TwoPoints(d, p0, p1)` if the segment intersects the circle
///
/// # Algorithm
/// 1. Projects the segment onto the circle using `dist_line_circle`.
/// 2. Analyzes the parameters of intersection to determine the closest points.
/// 3. Handles cases where the segment is entirely outside, crosses, or is inside the circle.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// let seg = segment(point(0.0, 0.0), point(3.0, 4.0));
/// let c = circle(point(1.0, 1.0), 2.0);
/// let dist = dist_segment_circle(&seg, &c);
/// // dist will be DistSegmentCircleConfig::OnePoint(1.0, point(1.0, 1.0));
/// ```
pub fn dist_segment_circle(seg: &Segment, circle: &Circle) -> DistSegmentCircleConfig {
    //let (dir, _) = (seg.b - seg.a).normalize();
    let line = line(seg.a, seg.b - seg.a);
    let dlc = dist_line_circle(&line, circle);
    match dlc {
        DistLineCircleConfig::TwoPairs(
            _,
            param0,
            param1,
            _closest00,
            closest01,
            _closest10,
            closest11,
        ) => {
            // p0, p1 outside the circle. Segment outside the circle.
            if param0 > ONE && param1 > ONE {
                let (dist, p0, _) = dist_to_circle(seg, circle);
                DistSegmentCircleConfig::OnePoint(dist, p0)
            } else
            // p0 outside p1 inside the circle.
            if param0 >= ZERO && param0 <= ONE && param1 > ONE {
                DistSegmentCircleConfig::OnePoint(ZERO, closest01)
            } else
            // p0, p1 outside the circle. Segment cross the circle
            if param0 >= ZERO && param0 <= ONE && param1 >= ZERO && param1 <= ONE {
                DistSegmentCircleConfig::TwoPoints(ZERO, closest01, closest11)
            } else
            // p0, p1 inside the circle.
            if param0 < ZERO && param1 > ONE {
                let (dist, p0, _) = dist_to_circle(seg, circle);
                DistSegmentCircleConfig::OnePoint(dist, p0)
            } else
            // p0 inside, p1 outside
            if param0 < ZERO && param1 >= ZERO && param1 <= ONE {
                DistSegmentCircleConfig::OnePoint(ZERO, closest11)
            } else {
                // p0, p1 outise the circle. Segment outside the circle.
                // if param0 < ZERO && param1 < ZERO {
                let (dist, p0, _) = dist_to_circle(seg, circle);
                DistSegmentCircleConfig::OnePoint(dist, p0)
            }
        }
        DistLineCircleConfig::OnePair(d, param, _, closest1) => {
            // The line does not intersect the circle.
            if param < ZERO {
                // Closest point is outside the segment.
                let (dist, closest, _) = dist_point_circle(&seg.a, circle);
                DistSegmentCircleConfig::OnePoint(dist, closest)
            } else if param > ONE {
                // Closest point is outside the segment.
                let (dist0, closest0, _) = dist_point_circle(&seg.a, circle);
                let (dist1, closest1, _) = dist_point_circle(&seg.b, circle);
                if dist0 <= dist1 {
                    DistSegmentCircleConfig::OnePoint(dist0, closest0)
                } else {
                    DistSegmentCircleConfig::OnePoint(dist1, closest1)
                }
            } else {
                // Closest point is inside the segment.
                DistSegmentCircleConfig::OnePoint(d, closest1)
            }
        }
    }
}

fn dist_to_circle(seg: &Segment, circle: &Circle) -> (f64, Point, Point) {
    let (dist0, p0, _) = dist_point_circle(&seg.a, circle);
    let (dist1, p1, _) = dist_point_circle(&seg.b, circle);
    if dist0 <= dist1 {
        (dist0, p0, p1)
    } else {
        (dist1, p1, p0)
    }
}

#[cfg(test)]
mod test_dist_segment_circle {

    use crate::{
        circle::circle,
        distance::dist_segment_circle::DistSegmentCircleConfig,
        point::point,
        segment::{Segment, segment},
    };

    // Revert segment
    fn rev(seg: Segment) -> Segment {
        segment(seg.b, seg.a)
    }

    #[test]
    fn test_p0_p1_outside_segment_outside_circle_01() {
        let c = circle(point(0.0, 0.0), 2.0);
        let seg = segment(point(-5.0, 1.0), point(-4.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(
                2.1231056256176606,
                point(-1.9402850002906638, 0.48507125007266594)
            )
        );
        let seg = rev(seg);
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(
                2.1231056256176606,
                point(-1.9402850002906638, 0.48507125007266594)
            )
        );
    }

    #[test]
    fn test_p0_outside_p1_inside_circle() {
        let c = circle(point(0.0, 0.0), 2.0);
        let seg = segment(point(-3.0, 1.0), point(1.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(0.0, point(-1.7320508075688772, 1.0))
        );
        let seg = rev(seg);
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(0.0, point(-1.7320508075688772, 1.0))
        );
    }

    #[test]
    fn test_p0_outside_p1_outside_segment_inside_circle() {
        let c = circle(point(0.0, 0.0), 2.0);
        let seg = segment(point(-3.0, 1.0), point(3.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::TwoPoints(
                0.0,
                point(-1.7320508075688774, 1.0),
                point(1.7320508075688767, 1.0)
            )
        );
        let seg = rev(seg);
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::TwoPoints(
                0.0,
                point(1.7320508075688774, 1.0),
                point(-1.7320508075688767, 1.0)
            )
        );
    }

    #[test]
    fn test_p0_inside_p1_inside_circle() {
        let c = circle(point(0.0, 0.0), 2.0);
        let seg = segment(point(-1.0, 1.0), point(1.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(
                0.5857864376269049,
                point(-1.414213562373095, 1.414213562373095)
            )
        );
        let seg = rev(seg);
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(
                0.5857864376269049,
                point(1.414213562373095, 1.414213562373095)
            )
        );
    }

    #[test]
    fn test_segment_tangent_to_circle() {
        // Segment is tangent to circle - closest point at param in [0, 1]
        let c = circle(point(0.0, 0.0), 1.0);
        let seg = segment(point(-2.0, 1.0), point(2.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(0.0, point(0.0, 1.0))
        );
    }

    #[test]
    fn test_closest_point_outside_segment_left() {
        // Line intersects circle but closest point is before segment start (param < 0)
        // The segment doesn't reach the circle, so closest is one of the endpoints
        let c = circle(point(0.0, 0.0), 1.0);
        let seg = segment(point(2.0, 1.0), point(4.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        // The closest point on the circle from segment start point (2.0, 1.0)
        let (expected_dist, expected_closest, _) = super::super::dist_point_circle::dist_point_circle(&point(2.0, 1.0), &c);
        assert_eq!(dist, DistSegmentCircleConfig::OnePoint(expected_dist, expected_closest));
    }

    #[test]
    fn test_closest_point_outside_segment_right() {
        // Line is tangent to circle, but the tangent point is past segment end (param > 1)
        let c = circle(point(0.0, 0.0), 1.0);
        // Create a horizontal line y=1 (tangent to circle at (0, 1))
        // Segment from (-1, 1) to (0, 1). Tangent point (0, 1) is RIGHT of segment end at (0, 1)
        // Actually, (0, 1) IS the segment end, so let me use (-2, 1) to (-1, 1)
        // Tangent point is still (0, 1), which is RIGHT of (-1, 1)
        let seg = segment(point(-2.0, 1.0), point(-1.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        // Distances from endpoints to circle
        let (_, _, _) = super::super::dist_point_circle::dist_point_circle(&seg.a, &c);
        let (dist_b, closest_b, _) = super::super::dist_point_circle::dist_point_circle(&seg.b, &c);
        // dist_a > dist_b since (-2, 1) is farther than (-1, 1)
        // So the result should use dist_b and closest_b (line 99)
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(dist_b, closest_b)
        );
    }

    #[test]
    fn test_tangent_line_param_greater_than_one_dist0_less_equal() {
        // Test line 97: OnePair case where param > ONE and dist0 <= dist1
        // We need: tangent line, tangent point beyond segment end, seg.a closer to circle
        let c = circle(point(0.0, 0.0), 1.0);
        // Vertical line x=1 is tangent to unit circle at (1, 0)
        // Use segment from (1, -1) to (1, 0): tangent point is (1, 0) which is at segment end
        // We want tangent point BEYOND segment end (param > 1)
        // Use segment from (1, -2) to (1, -1): tangent point (1, 0) is beyond segment end
        // seg.a at (1, -2) is dist=sqrt(1 + 4) = sqrt(5) from origin
        // seg.b at (1, -1) is dist=sqrt(1 + 1) = sqrt(2) from origin
        // So dist_a > dist_b, which triggers line 99
        
        // Let me try: segment from (1, 0) to (1, 1)
        // seg.a at (1, 0) is dist=1 from origin (ON circle!)
        // This won't work.
        
        // Try: segment from (1, -0.5) to (1, 0.5)
        // seg.a at (1, -0.5) is dist=sqrt(1.25) ≈ 1.118
        // seg.b at (1, 0.5) is dist=sqrt(1.25) ≈ 1.118
        // They're equidistant! But line may intersect, not be tangent
        
        // For a tangent line x=1, with segment endpoints at different distances,
        // where seg.a is closer:
        // seg.a = (1, -1) at dist sqrt(2) ≈ 1.414
        // seg.b = (1, 0) at dist 1 (ON circle)
        // But this puts seg.b on the circle...
        
        // Actually, let me use a negative tangent line: x = -1
        // Tangent point is (-1, 0)
        // Segment from (-1, -1) to (-1, 0): seg.b is ON circle
        // Segment from (-1, -1) to (-1, -0.5):
        // seg.a at (-1, -1) is dist sqrt(2)
        // seg.b at (-1, -0.5) is dist sqrt(1.25)
        // dist_a > dist_b, still line 99
        
        // Try reversed: segment from (-1, 0.5) to (-1, 1)
        // seg.a at (-1, 0.5) is dist sqrt(1.25) ≈ 1.118
        // seg.b at (-1, 1) is dist sqrt(2) ≈ 1.414
        // dist_a < dist_b! This should trigger line 97
        let seg = segment(point(-1.0, 0.5), point(-1.0, 1.0));
        let dist = super::dist_segment_circle(&seg, &c);
        
        // Compute distances from endpoints
        let (dist_a, closest_a, _) = super::super::dist_point_circle::dist_point_circle(&seg.a, &c);
        let (dist_b, _, _) = super::super::dist_point_circle::dist_point_circle(&seg.b, &c);
        
        // With dist_a < dist_b, we should get line 97
        assert!(dist_a < dist_b);
        assert_eq!(dist, DistSegmentCircleConfig::OnePoint(dist_a, closest_a));
    }
}

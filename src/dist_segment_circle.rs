#![allow(dead_code)]

use base_geom::prelude::*;

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
/// use base_geom::prelude::*;
/// let seg = segment(point(0.0, 0.0), point(3.0, 4.0));
/// let c = circle(point(1.0, 1.0), 2.0);
/// let dist = dist_segment_circle(&seg, &c);
/// // dist will be DistSegmentCircleConfig::OnePoint(1.0, point(1.0, 1.0));
/// ```
pub fn dist_segment_circle(seg: &Segment, circle: &Circle) -> DistSegmentCircleConfig {
    //let (dir, _) = (seg.b - seg.a).normalize();
    let line = line(seg.a, seg.b - seg.a);
    let dlc = dist_line_circle(&line, &circle);
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
                return DistSegmentCircleConfig::OnePoint(ZERO, closest01);
            } else
            // p0, p1 outside the circle. Segment cross the circle
            if param0 >= ZERO && param0 <= ONE && param1 >= ZERO && param1 <= ONE {
                return DistSegmentCircleConfig::TwoPoints(ZERO, closest01, closest11);
            } else
            // p0, p1 inside the circle.
            if param0 < ZERO && param1 > ONE {
                let (dist, p0, _) = dist_to_circle(seg, circle);
                return DistSegmentCircleConfig::OnePoint(dist, p0);
            } else
            // p0 inside, p1 outside
            if param0 < ZERO && param1 >= ZERO && param1 <= ONE {
                return DistSegmentCircleConfig::OnePoint(ZERO, closest11);
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
                let (dist, closest, _) = dist_point_circle(&seg.a, &circle);
                DistSegmentCircleConfig::OnePoint(dist, closest)
            } else if param > ONE {
                // Closest point is outside the segment.
                let (dist0, closest0, _) = dist_point_circle(&seg.a, &circle);
                let (dist1, closest1, _) = dist_point_circle(&seg.b, &circle);
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
    let (dist0, p0, _) = dist_point_circle(&seg.a, &circle);
    let (dist1, p1, _) = dist_point_circle(&seg.b, &circle);
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
        dist_segment_circle::DistSegmentCircleConfig,
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
            DistSegmentCircleConfig::OnePoint(
                0.0,
                point(-1.7320508075688772, 1.0)
            )
        );
        let seg = rev(seg);
        let dist = super::dist_segment_circle(&seg, &c);
        assert_eq!(
            dist,
            DistSegmentCircleConfig::OnePoint(
                0.0,
                point(-1.7320508075688772, 1.0)
            )
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
}

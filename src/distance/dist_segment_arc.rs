#![allow(dead_code)]

use crate::constants::GEOMETRIC_EPSILON;
use crate::prelude::*;

const ZERO: f64 = 0f64;
const ONE: f64 = 1f64;

/// Computes the distance between a segment and an arc.
///
/// This function checks if the segment and arc intersect. If they do, the distance is zero.
/// If they do not intersect, it computes the minimum distance from the segment endpoints to the arc
/// and from the segment to the arc endpoints.
///
/// # Arguments
/// * `seg` - The segment to measure distance from
/// * `arc` - The arc to measure distance to
///
/// # Returns
/// The minimum distance between the segment and the arc.
///
/// # Algorithm
/// 1. Check if the segment and arc intersect using `int_segment_arc`.
/// 2. If they intersect, return zero.
/// 3. If they do not intersect, compute:
///    - The distance from the segment endpoints to the arc using `dist_point_arc_dist`.
///    - The distance from the arc endpoints to the segment using `dist_point_segment`.
/// 4. Return the minimum of these distances.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// let seg = segment(point(0.0, 0.0), point(3.0, 4.0));
/// let arc = arc(point(1.0, 1.0), point(2.0, 2.0), point(1.5, 1.5), 1.0);
/// let distance = dist_segment_arc(&seg, &arc);
/// // distance will be the minimum distance between the segment and the arc
/// ```
pub fn dist_segment_arc(seg: &Segment, arc: &Arc) -> f64 {
    let res = int_segment_arc(seg, arc);
    match res {
        SegmentArcConfig::NoIntersection() => {
            // Compute distances from segment endpoints to arc
            let dist0 = dist_point_arc_dist(&seg.a, arc);
            let dist1 = dist_point_arc_dist(&seg.b, arc);
            let mut min_dist = dist0.min(dist1);
            
            // Early exit if we found a very close point
            if min_dist < GEOMETRIC_EPSILON {
                return min_dist;
            }

            // Compute distances from arc endpoints to segment
            let (dist2, _) = dist_point_segment(&arc.a, seg);
            let (dist3, _) = dist_point_segment(&arc.b, seg);
            min_dist = min_dist.min(dist2).min(dist3);
            
            // Early exit if we found a very close point
            if min_dist < GEOMETRIC_EPSILON {
                return min_dist;
            }

            // Only compute expensive line-circle distance if other distances aren't small
            let line = line(seg.a, seg.b - seg.a);
            let circle = circle(arc.c, arc.r);
            let res2 = dist_line_circle(&line, &circle);
            let dist4 = match res2 {
                DistLineCircleConfig::OnePair(_, param, closest0, closest1) => {
                    if param >= ZERO && param <= ONE && arc.contains(closest1) {
                        (closest0 - closest1).norm()
                    } else {
                        f64::MAX
                    }
                }
                DistLineCircleConfig::TwoPairs(..) => f64::MAX,
            };
            
            min_dist.min(dist4)
        }
        _ => {
            // The segment and arc intersect.
            0.0
        }
    }
}

#[cfg(test)]
mod tests_distance_segment_arc {
    use crate::{arc::arc, point::point, segment::segment};

    #[test]
    fn test_segment_outside_circle_01() {
        // closest point in arc
        let seg = segment(point(-1.0, 2.0), point(1.0, 2.0));
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_segment_outside_circle_02() {
        // closest point outside arc
        let seg = segment(point(-1.0, 2.0), point(1.0, 2.0));
        let arc = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 2.0);
    }

    #[test]
    fn test_segment_inside_circle_01() {
        // closest point inside arc
        let seg = segment(point(-2.0, 0.0), point(0.5, 0.0));
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_segment_inside_circle_02() {
        // closest point inside arc
        let seg = segment(point(-0.5, 0.0), point(2.0, 0.0));
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_segment_inside_circle_03() {
        // closest points inside arc
        let seg = segment(point(-2.0, 0.0), point(2.0, 0.0));
        let arc = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 0.0);
    }

    #[test]
    fn test_segment_inside_circle_04() {
        // closest point inside arc
        let seg = segment(point(-0.5, 0.0), point(2.0, 0.0));
        let arc = arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 0.5);
    }

    #[test]
    fn test_segment_inside_circle_05() {
        // closest point inside arc
        let seg = segment(point(-2.0, 0.0), point(0.5, 0.0));
        let arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 0.5);
    }

    #[test]
    fn test_segment_inside_circle_06() {
        // closest point outside arcs
        let seg = segment(point(-2.0, 0.0), point(2.0, 0.0));
        let arc = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 0.0), 2.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_segment_inside_circle_07() {
        // closest point outside arcs
        // segment start in arc center
        let seg = segment(point(0.0, 0.0), point(2.0, 0.0));
        let arc = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 0.0), 2.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 1.0);
    }

    #[test]
    fn test_segment_inside_circle_08() {
        // closest point outside arcs
        // segment start in arc center
        let seg = segment(point(-2.0, 0.0), point(0.0, 0.0));
        let arc = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 0.0), 2.0);
        let res = super::dist_segment_arc(&seg, &arc);
        assert_eq!(res, 1.0);
    }
}

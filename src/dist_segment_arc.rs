#![allow(dead_code)]

use crate::prelude::*;

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
/// use base_geom::prelude::*;
/// let seg = segment(point(0.0, 0.0), point(3.0, 4.0));
/// let arc = arc(point(1.0, 1.0), point(2.0, 2.0), point(1.5, 1.5), 1.0);
/// let distance = dist_segment_arc(&seg, &arc);
/// // distance will be the minimum distance between the segment and the arc
/// ```
pub fn dist_segment_arc(seg: &Segment, arc: &Arc) -> f64 {
    let res = int_segment_arc(seg, arc);
    match res {
        SegmentArcConfig::NoIntersection() => {
            let (dist0, _) = dist_point_segment(&arc.a, seg);
            let (dist1, _) = dist_point_segment(&arc.b, seg);
            let dist2 = dist_point_arc_dist(&seg.a, arc);
            let dist3 = dist_point_arc_dist(&seg.b, arc);
            let dist = min_4(dist0, dist1, dist2, dist3);
            let line = crate::line::line(seg.a, seg.b - seg.a);
            let circle = circle(arc.c, arc.r);
            let res2 = dist_line_circle(&line, &circle);
            let dist4 = match res2 {
                DistLineCircleConfig::OnePair(_, param, closest0, closest1) => {
                    if param >= 0.0 && param <= 1.0 && arc.contains(closest1) {
                        (closest0 - closest1).norm()
                    } else {
                        f64::MAX
                    }
                }
                _ => f64::MAX,
            };
            return f64::min(dist, dist4);
        }
        _ => {
            // The segment and arc intersect.
            return 0.0;
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

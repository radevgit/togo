#![allow(dead_code)]

use crate::{
    constants::GEOMETRIC_EPSILON,
    distance::dist_point_segment::dist_point_segment,
    intersection::int_segment_segment::{SegmentSegmentConfig, int_segment_segment},
    segment::Segment,
};

// https://stackoverflow.com/questions/2824478/shortest-distance-between-two-line-segments
const ZERO: f64 = 0.0;
/// Computes the distance between two segments.
///
/// This function calculates the shortest distance between two line segments.
/// If the segments intersect, the distance is zero.
/// If they do not intersect, it computes the minimum distance from the endpoints of one segment to
/// the other segment.
///
/// # Arguments
/// * `seg0` - The first segment
/// * `seg1` - The second segment
///
/// # Returns
/// The minimum distance as a f64
///
/// # Algorithm
/// 1. Checks if the segments intersect using `int_segment_segment`.
/// 2. If they intersect, returns zero.
/// 3. If they do not intersect, computes:
///    - The distance from the endpoints of `seg0` to `seg1` using `dist_point_segment`.
///    - The distance from the endpoints of `seg1` to `seg0` using `dist_point_segment`.
/// 4. Returns the minimum of these distances.
///
/// # Examples
/// ```
/// use togo::prelude::*;
/// let seg0 = segment(point(0.0, 0.0), point(1.0, 0.0));
/// let seg1 = segment(point(2.0, 0.0), point(3.0, 0.0));
/// let distance = dist_segment_segment(&seg0, &seg1);
/// // distance will be 1.0
/// ```
pub fn dist_segment_segment(seg0: &Segment, seg1: &Segment) -> f64 {
    // Execute the query for segment-segment. Test whether the segments
    // intersect. If they do, there is no need to test endpoints for
    // closeness.
    let inter = int_segment_segment(seg0, seg1);
    match inter {
        SegmentSegmentConfig::NoIntersection() => {
            // Compute distances from seg0 endpoints to seg1 sequentially with early exits
            let a = dist_point_segment(&seg0.a, seg1).0;
            if a < GEOMETRIC_EPSILON {
                return a;
            }
            
            let b = dist_point_segment(&seg0.b, seg1).0;
            let mut min_dist = a.min(b);
            if min_dist < GEOMETRIC_EPSILON {
                return min_dist;
            }
            
            // Compute distances from seg1 endpoints to seg0
            let c = dist_point_segment(&seg1.a, seg0).0;
            min_dist = min_dist.min(c);
            if min_dist < GEOMETRIC_EPSILON {
                return min_dist;
            }
            
            let d = dist_point_segment(&seg1.b, seg0).0;
            min_dist.min(d)
        }
        _ => ZERO,
    }
}

#[cfg(test)]
mod test_distance_segment_segment {
    use crate::distance::dist_segment_segment::dist_segment_segment;
    use crate::point::point;
    use crate::segment::segment;

    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;

    #[test]
    fn test_same_line_no_intersect() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(2.0, 0.0), point(3.0, 0.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ONE);
        assert_eq!(dist_segment_segment(&s1, &s0), ONE);
    }

    #[test]
    fn test_same_line_no_intersect_parallel() {
        let s0 = segment(point(0.0, 0.0), point(0.0, 2.0));
        let s1 = segment(point(1.0, 0.0), point(1.0, 2.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ONE);
        assert_eq!(dist_segment_segment(&s1, &s0), ONE);
    }

    #[test]
    fn test_same_line_touching() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(1.0, 0.0), point(2.0, 0.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ZERO);
        assert_eq!(dist_segment_segment(&s1, &s0), ZERO);
    }

    #[test]
    fn test_same_line_overlaping_01() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(1.0, 0.0), point(3.0, 0.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ZERO);
        assert_eq!(dist_segment_segment(&s1, &s0), ZERO);
    }

    #[test]
    fn test_same_line_overlaping_02() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(0.0, 0.0), point(2.0, 0.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ZERO);
        assert_eq!(dist_segment_segment(&s1, &s0), ZERO);
    }

    #[test]
    fn test_parallel_segments_01() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(2.0, 1.0), point(3.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), f64::sqrt(2.0));
        assert_eq!(dist_segment_segment(&s1, &s0), f64::sqrt(2.0));
    }

    #[test]
    fn test_parallel_segments_02() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(1.0, 1.0), point(2.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ONE);
        assert_eq!(dist_segment_segment(&s1, &s0), ONE);
    }

    #[test]
    fn test_parallel_segments_03() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let s1 = segment(point(0.0, 1.0), point(1.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ONE);
        assert_eq!(dist_segment_segment(&s1, &s0), ONE);
    }

    #[test]
    fn test_non_parallel_segments_04() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(1.0, 0.5), point(2.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), 0.5);
        assert_eq!(dist_segment_segment(&s1, &s0), 0.5);
    }

    #[test]
    fn test_non_parallel_segments_05() {
        let s0 = segment(point(0.0, 0.0), point(3.0, 0.0));
        let s1 = segment(point(1.0, 1.0), point(2.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), 1.0);
        assert_eq!(dist_segment_segment(&s1, &s0), 1.0);
    }

    #[test]
    fn test_intersecting_segments() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(0.0, -1.0), point(2.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ZERO);
        assert_eq!(dist_segment_segment(&s1, &s0), ZERO);
    }

    #[test]
    fn test_touching_segments() {
        let s0 = segment(point(0.0, 0.0), point(2.0, 0.0));
        let s1 = segment(point(1.0, 0.0), point(2.0, 1.0));
        assert_eq!(dist_segment_segment(&s0, &s1), ZERO);
        assert_eq!(dist_segment_segment(&s1, &s0), ZERO);
    }
}

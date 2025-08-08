#![allow(dead_code)]

use base_geom::prelude::*;

/// Represents the configuration of the intersection between a segment and a point.
#[derive(Debug, PartialEq)]
pub enum SegmentPointConfig {
    NoIntersection(),
    OnePoint(Point),
}

pub fn int_segment_point(_segment0: &Segment, _point1: &Point) -> SegmentPointConfig {
    SegmentPointConfig::NoIntersection()
}

#[cfg(test)]
mod test_int_segment_point {
    use base_geom::prelude::*;

    #[test]
    fn test_no_intersection() {
        let s0 = segment(point(0.0, 0.0), point(1.0, 0.0));
        let p1 = point(2.0, 0.0);
        assert_eq!(
            int_segment_point(&s0, &p1),
            SegmentPointConfig::NoIntersection()
        );
    }

    #[test]
    fn test_one_point_intersection() {
        let seg = segment(point(0.0, 0.0), point(5.0, 0.0));
        let p = point(2.5, 3.0);
        let (dist, _closest) = dist_point_segment(&p, &seg);
        assert_eq!(dist, 3.0);
    }
}

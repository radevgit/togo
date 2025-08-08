#![allow(dead_code)]

// https://stackoverflow.com/questions/18949449/calculate-the-minimum-distance-between-two-given-circular-arcs
// https://math.stackexchange.com/questions/95009/how-can-one-calculate-the-minimum-and-maximum-distance-between-two-given-circula

use crate::{
    Arc,
    dist_point_arc::{DistPointArcConfig, dist_point_arc},
    int_arc_arc::{ArcArcConfig, int_arc_arc},
    int_line_arc::int_line_arc,
    line::line,
    utils::min_4,
};

/// Computes the distance between two arcs.
///
/// This function calculates the minimum distance between two arcs, which can be:
/// 1. The endpoints of both arcs
/// 2. An endpoint of one arc and an interior point of the other arc
/// 3. Interior points of both arcs, which are on the line through the centers of the two arcs.
///
/// If the arcs intersect, the distance is zero.
/// If they are cocircular, the distance is the distance between their centers minus their radii.
///
/// # Arguments
/// /// * `arc0` - The first arc
/// * `arc1` - The second arc
///
/// # Returns
/// The minimum distance as a f64
///
/// # Examples
/// ```
/// use base_geom::{arc, dist_arc_arc, point};
/// let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
/// let arc1 = arc(point(2.0, 0.0), point(0.0, 0.0), point(1.0, 1.0), 1.0);
/// let dist = dist_arc_arc(&arc0, &arc1);
/// assert_eq!(dist, 0.0);
/// ```
pub fn dist_arc_arc(arc0: &Arc, arc1: &Arc) -> f64 {
    let res = int_arc_arc(arc0, arc1);
    match res {
        ArcArcConfig::NoIntersection() => {}
        _ => {
            return 0.0;
        }
    }

    // 1) Endpoints of both arcs
    // 2) An endpoint of one and an interior point of the other
    let dist0 = match dist_point_arc(&arc0.a, arc1) {
        DistPointArcConfig::OnePoint(dist, _) | DistPointArcConfig::Equidistant(dist, _) => dist,
    };
    let dist1 = match dist_point_arc(&arc0.b, arc1) {
        DistPointArcConfig::OnePoint(dist, _) | DistPointArcConfig::Equidistant(dist, _) => dist,
    };
    let dist2 = match dist_point_arc(&arc1.a, arc0) {
        DistPointArcConfig::OnePoint(dist, _) | DistPointArcConfig::Equidistant(dist, _) => dist,
    };
    let dist3 = match dist_point_arc(&arc1.b, arc0) {
        DistPointArcConfig::OnePoint(dist, _) | DistPointArcConfig::Equidistant(dist, _) => dist,
    };
    let mut min_dist = min_4(dist0, dist1, dist2, dist3);

    // The arcs are cocircular
    if arc0.c.close_enough(arc1.c, 10E-10) {
        // TODO: use a constant
        return min_dist;
    }

    // 3) Interior points of both arcs, which are on the line through the centres of the two arcs.
    // The line through the centres of the two arcs
    let line_aa = line(arc0.c, arc1.c - arc0.c);
    let res0 = int_line_arc(&line_aa, arc0);
    let res1 = int_line_arc(&line_aa, arc1);
    match (res0, res1) {
        (
            crate::int_line_arc::LineArcConfig::OnePoint(p0, _),
            crate::int_line_arc::LineArcConfig::OnePoint(p1, _),
        ) => {
            let dist = (p0 - p1).norm();
            if dist < min_dist {
                min_dist = dist;
            }
        }
        (
            crate::int_line_arc::LineArcConfig::TwoPoints(p0, p1, _, _),
            crate::int_line_arc::LineArcConfig::OnePoint(p2, _),
        ) => {
            let dists = [(p0 - p2).norm(), (p1 - p2).norm()];
            for &dist in &dists {
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        (
            crate::int_line_arc::LineArcConfig::OnePoint(p0, _),
            crate::int_line_arc::LineArcConfig::TwoPoints(p1, p2, _, _),
        ) => {
            let dists = [(p0 - p1).norm(), (p0 - p2).norm()];
            for &dist in &dists {
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        (
            crate::int_line_arc::LineArcConfig::TwoPoints(p0, p1, _, _),
            crate::int_line_arc::LineArcConfig::TwoPoints(p2, p3, _, _),
        ) => {
            let dists = [
                (p0 - p2).norm(),
                (p0 - p3).norm(),
                (p1 - p2).norm(),
                (p1 - p3).norm(),
            ];
            for &dist in &dists {
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        _ => {
            // No intersection or no points on the line
            // This case is already handled by the initial distance checks
        }
    }

    min_dist
}

#[cfg(test)]
mod test_dist_arc_arc {
    use core::f64;

    use crate::{arc::arc, dist_arc_arc::dist_arc_arc, point::point};

    #[test]
    fn test_intersected_arc_arc_0() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(2.0, 0.0), point(0.0, 0.0), point(1.0, 1.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_intersected_arc_arc_1() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(3.0, 0.0), point(-1.0, 0.0), point(1.0, 1.0), 2.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_two_equidistant_points_0() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_two_almost_equidistant_points_1() {
        let e = 1e-10;
        let arc0 = arc(
            point(1.0 + e, 0.0),
            point(-1.0 + e, 0.0),
            point(0.0 + e, 0.0),
            1.0,
        );
        let arc1 = arc(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0 - e);
    }

    #[test]
    fn test_two_equidistant_points_2() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-2.0, 0.0), point(0.0, 0.0), point(-1.0, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_arc_endpoints_0() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.1, 0.0), point(2.1, 0.0), point(1.0, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 0.9);
    }

    #[test]
    fn test_interior_points_no_intersection_0() {
        let arc0 = arc(point(1.0, -0.5), point(1.0, 0.5), point(0.0, 0.0), 1.5);
        let arc1 = arc(point(-1.0, 1.5), point(-1.0, 0.5), point(0.0, 1.0), 1.5);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 2.0);
    }

    #[test]
    fn test_interior_points_no_intersection_1() {
        let arc0 = arc(point(1.0, 0.5), point(1.0, -0.5), point(1.0, 0.0), 0.5);
        let arc1 = arc(point(-1.0, -0.5), point(-1.0, 0.5), point(-1.0, 0.0), 0.5);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_interior_points_one_intersection_0() {
        let arc0 = arc(point(1.5, 1.0), point(1.5, -1.0), point(1.5, 0.0), 1.0);
        let arc1 = arc(point(-1.5, -1.0), point(-1.5, 1.0), point(-1.5, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_interior_points_one_and_two_intersection_0() {
        let arc0 = arc(point(1.5, 1.0), point(1.5, -1.0), point(1.5, 0.0), 1.0);
        let arc1 = arc(point(-1.5, -1.0), point(-2.5, 0.0), point(-1.5, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_interior_points_two_and_one_intersection_0() {
        let arc0 = arc(point(2.5, 0.0), point(1.5, -1.0), point(1.5, 0.0), 1.0);
        let arc1 = arc(point(-1.5, -1.0), point(-1.5, 1.0), point(-1.5, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_interior_points_two_and_two_intersection_0() {
        let arc0 = arc(point(2.5, 0.0), point(1.5, -1.0), point(1.5, 0.0), 1.0);
        let arc1 = arc(point(-1.5, -1.0), point(-2.5, 0.0), point(-1.5, 0.0), 1.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_cocircular_arcs_01() {
        let arc0 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, -2.0), point(0.0, 2.0), point(0.0, 0.0), 2.0);
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 1.0);
    }

    #[test]
    fn test_cocircular_arcs_02() {
        let eps = f64::EPSILON;
        let arc0 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(
            point(0.0, -2.0),
            point(0.0, 2.0),
            point(0.0 - eps, 0.0 + eps),
            2.0,
        );
        let dist = dist_arc_arc(&arc0, &arc1);
        assert_eq!(dist, 0.9999999999999998);
    }
}

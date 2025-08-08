#![allow(dead_code)]

use crate::arc::arc;
use crate::int_circle_circle::CircleCircleConfig;
use crate::{arc::Arc, circle::circle, int_circle_circle::int_circle_circle, point::Point};

// #00018
/// Configuration for arc-arc intersection results.
/// This enum captures various intersection scenarios between two arcs.
#[derive(Debug, PartialEq)]
pub enum ArcArcConfig {
    NoIntersection(),
    NonCocircularOnePoint(Point),                 // point[0]
    NonCocircularOnePointTouching(Point),         // point[0]
    NonCocircularTwoPoints(Point, Point),         // point[0], point[1]
    NonCocircularTwoPointsTouching(Point, Point), // point[0], point[1]
    CocircularOnePoint0(Point),                   // point[0]
    CocircularOnePoint1(Point),
    CocircularTwoPoints(Point, Point),     // point[0], point[1]
    CocircularOnePointOneArc0(Point, Arc), // point[0], arc[0]
    CocircularOnePointOneArc1(Point, Arc), // point[0], arc[1]
    CocircularOneArc0(Arc),                // arc[0]
    CocircularOneArc1(Arc),
    CocircularOneArc2(Arc),
    CocircularOneArc3(Arc),
    CocircularOneArc4(Arc),
    CocircularTwoArcs(Arc, Arc), // arc[0], arc[1]
}

/// Computes the intersection of two arcs.
///
/// This function checks if two arcs intersect, are cocircular, or touch at their endpoints.
/// If they are cocircular, it determines whether they overlap or are the same arc.
///
/// # Arguments
/// * `arc0` - First arc to check
/// * `arc1` - Second arc to check
///
/// # Returns
/// The configuration of the intersection result.
///
/// # Algorithm
/// The arcs are represented as circles with a center and radius.
/// The algorithm:
/// 1. Computes the intersection of the circles representing the arcs
/// 2. Checks if the intersection points lie on both arcs
/// 3. Determines the type of intersection based on the points and arcs' properties
///
/// # Examples
/// ```
/// use base_geom::{arc, int_arc_arc, Arc, point, ArcArcConfig};
/// // Define two arcs
/// let arc0 = arc(point(0.0, 0.0), point(1.0, 1.0), point(1.0, 0.0), 1.0);
/// let arc1 = arc(point(0.0, 2.0), point(1.0, 1.0), point(1.0, 2.0), 1.0);
/// // Compute intersection
/// let result = int_arc_arc(&arc0, &arc1);
/// match result {
///      ArcArcConfig::NoIntersection() => println!("No intersection"),
///      ArcArcConfig::CocircularTwoArcs(arc_a, arc_b) => println!("Cocircular arcs: {:?}, {:?}", arc_a, arc_b),
///      _ => println!("Other intersection type"),
/// }
/// ```
pub fn int_arc_arc(arc0: &Arc, arc1: &Arc) -> ArcArcConfig {
    const EPS_CONTAINS: f64 = 1E-10;
    let circle0 = circle(arc0.c, arc0.r);
    let circle1 = circle(arc1.c, arc1.r);
    let cc_result = int_circle_circle(circle0, circle1);

    match cc_result {
        CircleCircleConfig::NoIntersection() => return ArcArcConfig::NoIntersection(),
        CircleCircleConfig::SameCircles() => {
            // The arcs are cocircular. Determine whether they overlap.
            // Let arc0 be <A0,A1> and arc1 be <B0,B1>. The points are
            // ordered counterclockwise around the circle of the arc.
            if arc1.contains(arc0.a) {
                if arc1.contains(arc0.b) {
                    if arc0.contains(arc1.a) && arc0.contains(arc1.b) {
                        if arc0.a == arc1.a && arc0.b == arc1.b {
                            // The arcs are the same.
                            return ArcArcConfig::CocircularOneArc0(arc0.clone());
                        } else {
                            // arc0 and arc1 overlap in two disjoint subsets.
                            if arc0.a != arc1.b {
                                if arc1.a != arc0.b {
                                    // The arcs overlap in two disjoint
                                    // subarcs, each of positive subtended
                                    // angle: <A0,B1>, <A1,B0>
                                    let res_arc0 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
                                    let res_arc1 = arc(arc1.a, arc0.b, arc0.c, arc0.r);
                                    return ArcArcConfig::CocircularTwoArcs(res_arc0, res_arc1);
                                } else {
                                    // B0 = A1
                                    // The intersection is a point {A1}
                                    // and an arc <A0,B1>.
                                    let res_point0 = arc0.b;
                                    let res_arc0 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
                                    return ArcArcConfig::CocircularOnePointOneArc0(
                                        res_point0, res_arc0,
                                    );
                                }
                            } else {
                                // A0 = B1
                                if arc1.a != arc0.b {
                                    // The intersection is a point {A0}
                                    // and an arc <A1,B0>.
                                    let res_point0 = arc0.a;
                                    let res_arc0 = arc(arc1.a, arc0.b, arc0.c, arc0.r);
                                    return ArcArcConfig::CocircularOnePointOneArc1(
                                        res_point0, res_arc0,
                                    );
                                } else {
                                    // The arcs shared endpoints, so the
                                    // union is a circle.
                                    let res_point0 = arc0.a;
                                    let res_point1 = arc0.b;
                                    return ArcArcConfig::CocircularTwoPoints(
                                        res_point0, res_point1,
                                    );
                                }
                            }
                        }
                    } else {
                        // Arc0 inside Arc1, <B0,A0,A1,B1>.
                        return ArcArcConfig::CocircularOneArc1(arc0.clone());
                    }
                } else {
                    if arc0.a != arc1.b {
                        // Arc0 and Arc1 overlap, <B0,A0,B1,A1>.
                        let res_arc0 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
                        return ArcArcConfig::CocircularOneArc2(res_arc0);
                    } else {
                        // Arc0 and arc1 share endpoint, <B0,A0,B1,A1>
                        // with A0 = B1.
                        let res_point0 = arc0.a;
                        return ArcArcConfig::CocircularOnePoint0(res_point0);
                    }
                }
            }
            if arc1.contains(arc0.b) {
                if arc0.b != arc1.a {
                    // Arc0 and arc1 overlap in a single arc,
                    // <A0,B0,A1,B1>.
                    let res_arc0 = arc(arc1.a, arc0.b, arc0.c, arc0.r);
                    return ArcArcConfig::CocircularOneArc3(res_arc0);
                } else {
                    // Arc0 and arc1 share endpoint, <A0,B0,A1,B1>
                    // with B0 = A1.
                    let res_point0 = arc1.a;
                    return ArcArcConfig::CocircularOnePoint1(res_point0);
                }
            }

            if arc0.contains(arc1.a) {
                // Arc1 inside Arc0, <A0,B0,B1,A1>.
                return ArcArcConfig::CocircularOneArc4(*arc1);
            } else {
                // Arcs do not overlap, <A0,A1,B0,B1>.
                return ArcArcConfig::NoIntersection();
            }
        }
        CircleCircleConfig::NoncocircularOnePoint(point0) => {
            // Test whether circle-circle intersection points are on the arcs.
            if arc0.contains(point0) && arc1.contains(point0) {
                if are_ends_towching(arc0, arc1) {
                    // The arcs are touching at one end.
                    return ArcArcConfig::NonCocircularOnePointTouching(point0);
                } else {
                    // The arcs intersect in a single point.
                    return ArcArcConfig::NonCocircularOnePoint(point0);
                }
            } else {
                return ArcArcConfig::NoIntersection();
            }
        }
        CircleCircleConfig::NoncocircularTwoPoints(point0, point1) => {
            let b0 = arc0.contains(point0) && arc1.contains(point0);
            let b1 = arc0.contains(point1) && arc1.contains(point1);

            if b0 && b1 {
                // If arcs are touching at both ends
                if are_both_ends_towching(arc0, arc1) {
                    return ArcArcConfig::NonCocircularTwoPointsTouching(point0, point1);
                }
                return ArcArcConfig::NonCocircularTwoPoints(point0, point1);
            }
            if b0 {
                if are_ends_towching(arc0, arc1) {
                    return ArcArcConfig::NonCocircularOnePointTouching(point0);
                }
                return ArcArcConfig::NonCocircularOnePoint(point0);
            }
            if b1 {
                if are_ends_towching(arc0, arc1) {
                    return ArcArcConfig::NonCocircularOnePointTouching(point1);
                }
                return ArcArcConfig::NonCocircularOnePoint(point1);
            }
            return ArcArcConfig::NoIntersection();
        }
    }
}

fn are_ends_towching(arc0: &Arc, arc1: &Arc) -> bool {
    if arc0.a == arc1.a || arc0.a == arc1.b || arc0.b == arc1.a || arc0.b == arc1.b {
        true
    } else {
        false
    }
}

fn are_both_ends_towching(arc0: &Arc, arc1: &Arc) -> bool {
    (arc0.a == arc1.a && arc0.b == arc1.b) || (arc0.b == arc1.a && arc0.a == arc1.b)
}

// If arcs are really intersecting, but not just touching at ends, return true
// In other words, do we need to split arcs further?
pub fn if_really_intersecting_arc_arc(arc0: &Arc, arc1: &Arc) -> bool {
    match int_arc_arc(arc0, arc1) {
        ArcArcConfig::NoIntersection() => false,
        ArcArcConfig::NonCocircularOnePoint(_) => true,
        ArcArcConfig::NonCocircularOnePointTouching(_) => false,
        ArcArcConfig::NonCocircularTwoPoints(_, _) => true,
        ArcArcConfig::NonCocircularTwoPointsTouching(_, _) => false,
        ArcArcConfig::CocircularOnePoint0(_) | ArcArcConfig::CocircularOnePoint1(_) => false,
        ArcArcConfig::CocircularTwoPoints(_, _) => false,
        ArcArcConfig::CocircularOnePointOneArc0(_, _)
        | ArcArcConfig::CocircularOnePointOneArc1(_, _) => true,
        ArcArcConfig::CocircularOneArc0(_)
        | ArcArcConfig::CocircularOneArc1(_)
        | ArcArcConfig::CocircularOneArc2(_)
        | ArcArcConfig::CocircularOneArc3(_)
        | ArcArcConfig::CocircularOneArc4(_)
        | ArcArcConfig::CocircularTwoArcs(_, _) => true,
    }
}

// Arc Arc Intersect
#[cfg(test)]
mod test_int_arc_arc {
    use super::*;
    use crate::arc::Arc;
    use crate::point::point;

    // short funtion
    fn i_arc(arc0: &Arc, arc1: &Arc) -> ArcArcConfig {
        int_arc_arc(arc0, arc1)
    }

    #[test]
    fn test_no_intersection() {
        let arc0 = arc(point(-2.0, 2.0), point(-2.0, 0.0), point(-2.0, 1.0), 1.0);
        let arc1 = arc(point(2.0, 0.0), point(2.0, 2.0), point(1.0, 1.0), 1.0);
        assert_eq!(i_arc(&arc0, &arc1), ArcArcConfig::NoIntersection());
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_arc0() {
        let arc0 = arc(point(2.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0);
        let arc1 = arc(point(2.0, 1.0), point(1.0, 0.0), point(1.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc0(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_arc0_2() {
        let arc0 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc0(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_two_arc() {
        let arc0 = arc(point(0.0, 0.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 2.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc00 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        let arc01 = arc(arc1.a, arc0.b, arc1.c, arc1.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularTwoArcs(arc00, arc01));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc0() {
        let arc0 = arc(point(0.0, 0.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(-1.0, 1.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let p0 = point(-1.0, 1.0);
        let arc00 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc0(p0, arc00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc1() {
        let arc0 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 2.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let p0 = point(1.0, 1.0);
        let arc00 = arc(arc1.a, arc0.b, arc0.c, arc0.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc1(p0, arc00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_two_points() {
        let arc0 = arc(point(1.0, 1.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 2.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let p0 = point(1.0, 1.0);
        let p1 = point(0.0, 2.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularTwoPoints(p0, p1));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_arc_1() {
        let arc0 = arc(point(1.0, 1.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc00 = arc(arc0.a, arc0.b, arc0.c, arc0.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc1(arc00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_arc_2() {
        let arc0 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);

        let arc00 = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc2(arc00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_0() {
        let arc0 = arc(point(0.0, 2.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);

        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePoint0(arc0.a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_arc_3() {
        let arc0 = arc(point(0.0, 0.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc00 = arc(arc1.a, arc0.b, arc0.c, arc0.r);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc3(arc00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_1() {
        let arc0 = arc(point(0.0, 0.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePoint1(arc1.a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_arc_4() {
        let arc0 = arc(point(0.0, 0.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc4(arc1));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_no_intersection() {
        let arc0 = arc(point(0.0, 0.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(0.0, 2.0), point(-1.0, 1.0), point(0.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NoIntersection());
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_one_point_01() {
        let arc0 = arc(point(0.0, 0.0), point(0.0, 2.0), point(0.0, 1.0), 1.0);
        let arc1 = arc(point(1.0, 1.0), point(2.0, 2.0), point(1.0, 2.0), 1.0);
        let point00 = point(1.0, 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePoint(point00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_noncircular_one_point_02() {
        let arc0 = arc(point(1.0, -1.0), point(-1.0, -1.0), point(0.0, -1.0), 1.0);
        let arc1 = arc(point(-1.0, 1.0), point(1.0, 1.0), point(0.0, 1.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePoint(point(0.0, 0.0)));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_noncircular_two_points_0() {
        let arc0 = arc(point(-0.5, -1.0), point(-0.5, 1.0), point(-0.5, 0.0), 1.0);
        let arc1 = arc(point(0.5, 1.0), point(0.5, -1.0), point(0.5, 0.0), 1.0);
        let point00 = point(0.0, 0.8660254037844386);
        let point01 = point(0.0, -0.8660254037844386);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularTwoPoints(point00, point01));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_noncircular_two_points_1() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.03, 0.03), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NoIntersection());
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_two_points_2() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.03), 1.0);
        let point00 = point(0.9998874936711629, 0.015);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePointTouching(point00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_two_points_2b() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.03), 1.0);
        let point00 = point(-0.9998874936711629, 0.015);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePointTouching(point00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_two_points_3() {
        let x = std::f64::consts::SQRT_2 / 2.0;
        let arc0 = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.5, 0.5), x);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(
            res,
            ArcArcConfig::NonCocircularTwoPointsTouching(point(0.0, 1.0), point(1.0, 0.0))
        );
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_two_points_3b() {
        let x = std::f64::consts::SQRT_2 / 2.0;
        let arc0 = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.5, 0.5), x);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(
            res,
            ArcArcConfig::NonCocircularTwoPointsTouching(point(0.0, 1.0), point(1.0, 0.0))
        );
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_noncircular_one_point_03() {
        let e = 1e-13;
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(
            point(1.0 + e, 0.0),
            point(-1.0 + e, 0.0),
            point(0.0 + e, 0.0),
            1.0,
        );
        let point00 = point(5e-14, 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePoint(point00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_noncircular_two_points_4() {
        let arc0 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.03), 1.0);
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let point00 = point(0.9998874936711629, 0.015);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NonCocircularOnePointTouching(point00));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    // Old tests
    /////////////////////////
    #[test]
    fn test_cocircular_one_arc() {
        // two half circle arcs
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        assert_eq!(i_arc(&arc0, &arc1), ArcArcConfig::CocircularOneArc0(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);

        // one half circle arc and one circle arc
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, -1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        assert_eq!(i_arc(&arc0, &arc1), ArcArcConfig::CocircularOneArc1(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);

        // Two circle arcs
        let arc0 = arc(point(0.0, -1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, -1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        assert_eq!(i_arc(&arc0, &arc1), ArcArcConfig::CocircularOneArc0(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_arc2() {
        // two half circle arcs, zero radius, degenerate case
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 0.0);
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 0.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOneArc0(arc0));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_no_intersection111() {
        // two half circle arcs, zero and infinite radius, degenerate case
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), f64::MAX);
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 0.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NoIntersection());
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_no_cocircular_two_arcs() {
        let arc0 = arc(point(1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        let arc_b = arc(arc1.a, arc0.b, arc1.c, arc1.r);
        assert_eq!(res, ArcArcConfig::CocircularTwoArcs(arc_a, arc_b));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc0(arc0.b, arc_a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc2() {
        let arc1 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc0 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc1.a, arc0.b, arc0.c, arc0.r);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc1(arc0.a, arc_a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc3() {
        let arc0 = arc(point(0.0, 1.0), point(0.0, -1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc1.a, arc0.b, arc0.c, arc0.r);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc1(arc0.a, arc_a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point_one_arc4() {
        let arc0 = arc(point(1.0, 0.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        assert_eq!(res, ArcArcConfig::CocircularOnePointOneArc0(arc0.b, arc_a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_two_points_02() {
        let arc0 = arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(
            res,
            ArcArcConfig::CocircularTwoPoints(point(0.0, 1.0), point(-1.0, 0.0))
        );
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_point() {
        let arc0 = arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePoint1(point(-1.0, 0.0)));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_cocircular_one_arc3() {
        let arc0 = arc(point(0.0, -1.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        let arc_a = arc(arc0.a, arc1.b, arc0.c, arc0.r);
        assert_eq!(res, ArcArcConfig::CocircularOneArc2(arc_a));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == true);
    }

    #[test]
    fn test_cocircular_one_point2() {
        let arc0 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        let arc1 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::CocircularOnePoint0(point(1.0, 0.0)));
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_one_point() {
        let arc0 = arc(point(0.0, 0.0), point(2.0, 0.0), point(1.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(-2.0, 0.0), point(-1.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(
            res,
            ArcArcConfig::NonCocircularOnePointTouching(point(0.0, 0.0))
        );
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    #[test]
    fn test_no_intersection2() {
        let arc0 = arc(point(1.0, -1.0), point(1.0, 1.0), point(1.0, 0.0), 1.0);
        let arc1 = arc(point(0.0, 0.0), point(-2.0, 0.0), point(-1.0, 0.0), 1.0);
        let res = i_arc(&arc0, &arc1);
        assert_eq!(res, ArcArcConfig::NoIntersection());
        assert!(if_really_intersecting_arc_arc(&arc0, &arc1) == false);
    }

    use crate::svg::svg;

    #[test]
    fn test_no_issue_01() {
        let mut svg = svg(200.0, 200.0);
        let arc0 = arc(
            point(88.0, 96.0),
            point(92.307692307692306, 61.538461538461533),
            point(100.0, 130.0),
            20.0,
        );
        let arc1 = arc(
            point(107.69230769230769, 118.46153846153847),
            point(42.307692307692307, 118.46153846153847),
            point(75.0, 40.0),
            85.0,
        );
        let res = int_arc_arc(&arc0, &arc1);
        svg.offset_segment(&arc0, "black");
        svg.offset_segment(&arc1, "black");
        svg.circle(&circle(arc0.c, 20.0), "blue");
        svg.circle(&circle(arc1.c, 85.0), "blue");
        let p = point(80.68522962987866, 124.80965843614482);
        svg.circle(&circle(p, 1.0), "red");
        svg.write();
        assert_eq!(res, ArcArcConfig::NonCocircularOnePoint(p));
        let inter = if_really_intersecting_arc_arc(&arc0, &arc1);
        assert!(inter);
    }
}

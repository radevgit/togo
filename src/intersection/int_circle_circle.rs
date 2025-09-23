#![allow(dead_code)]

use crate::point::point;
use crate::utils::diff_of_prod;
use crate::{circle::Circle, point::Point};

// #00019
/// Configuration for circle-circle intersection results.
#[derive(Debug, PartialEq)]
pub enum CircleCircleConfig {
    NoIntersection(),
    NoncocircularOnePoint(Point),         // point[0]
    NoncocircularTwoPoints(Point, Point), // point[0], point[1]
    SameCircles(),
}

const ZERO: f64 = 0f64;

/// Computes the intersection of two circles.
///
/// This function determines the intersection points of two circles defined by their centers and radii.
///
/// # Arguments
/// * `circle0` - The first circle.
/// * `circle1` - The second circle.
///
/// # Returns
/// Returns a `CircleCircleConfig` enum indicating the result of the intersection test.
///
/// # Circle Intersection Logic
/// The circles are defined by the equations:
/// /// - |X - C0| = R0
/// /// - |X - C1| = R1
/// /// The vector U = C1 - C0 is computed, and its squared length is used to determine the relationship between the circles.
///
/// # Conditions for Intersection
/// The circles can have the following relationships:
///     - **Same Circles**: If the centers and radii are identical, they are the same circle.
///     - **No Intersection**: If the distance between centers is greater than the sum of the radii or less than the absolute difference of the radii, they do not intersect.
///     - **Tangent Circles**: If the distance equals the sum or absolute difference of the radii, they touch at one point.
///     - **Intersecting Circles**: If the distance is strictly between the absolute difference and sum of the radii, they intersect at two points.
///
/// # Examples
/// ```
/// use togo::prelude::*;
///
/// let circle0 = circle(point(0.0, 0.0), 1.0);
/// let circle1 = circle(point(1.0, 0.0), 1.0);
/// let result = int_circle_circle(circle0, circle1);
/// // result should be CircleCircleConfig::NoncocircularTwoPoints(Point(0.5, 0.8660254037844386), Point(0.5, -0.8660254037844386))
/// let circle0 = circle(point(0.0, 0.0), 1.0);
/// let circle1 = circle(point(2.0, 0.0), 1.0);
/// let result = int_circle_circle(circle0, circle1);
/// // result should be CircleCircleConfig::NoIntersection()
/// ```
pub fn int_circle_circle(circle0: Circle, circle1: Circle) -> CircleCircleConfig {
    debug_assert!(circle0.r.is_finite());
    debug_assert!(circle1.r.is_finite());

    let u = circle1.c - circle0.c;
    let usqr_len = u.dot(u);
    let r0 = circle0.r;
    let r1 = circle1.r;
    let r0_m_r1 = r0 - r1;

    if usqr_len == ZERO && r0_m_r1 == ZERO {
        // Circles are the same
        return CircleCircleConfig::SameCircles();
    }

    let r0_m_r1_sqr = r0_m_r1 * r0_m_r1;
    if usqr_len < r0_m_r1_sqr {
        // The circles do not intersect.
        return CircleCircleConfig::NoIntersection();
    }

    let r0_p_r1 = r0 + r1;
    let r0_p_r1_sqr = r0_p_r1 * r0_p_r1;
    if usqr_len > r0_p_r1_sqr {
        // The circles do not intersect.
        return CircleCircleConfig::NoIntersection();
    }

    if usqr_len < r0_p_r1_sqr {
        if r0_m_r1_sqr < usqr_len {
            //let inv_usqr_len = 1.0 / usqr_len;
            // let s = 0.5 * ((r0 * r0 - r1 * r1) * inv_usqr_len + 1.0);
            let s = 0.5 * (diff_of_prod(r0, r0, r1, r1) / usqr_len + 1.0);
            //let s = 0.5 * (((r1 + r0) / (usqr_len / (r0 - r1))) + 1.0);
            //let s = (0.5 / (usqr_len / (r1 + r0))).mul_add(r0 - r1, 0.5);

            // In theory, discr is nonnegative.  However, numerical round-off
            // errors can make it slightly negative.  Clamp it to zero.
            //let mut discr = r0 * r0 * inv_usqr_len - s * s;
            let mut discr = diff_of_prod(r0 / usqr_len, r0, s, s);
            //println!("{:.40} {:.40}", discr, discr);
            if discr < ZERO {
                discr = ZERO;
            }
            let t = discr.sqrt();
            let v = point(u.y, -u.x);
            let tmp = circle0.c + u * s;
            let p0 = tmp - v * t;
            let p1 = tmp + v * t;
            if t > 0f64 {
                CircleCircleConfig::NoncocircularTwoPoints(p0, p1)
            } else {
                // t==0.0
                CircleCircleConfig::NoncocircularOnePoint(p0)
            }
        } else {
            // |U| = |R0-R1|, circles are tangent.
            let p0 = circle0.c + u * (r0 / r0_m_r1);
            CircleCircleConfig::NoncocircularOnePoint(p0)
        }
    } else {
        // |U| = |R0+R1|, circles are tangent.
        let p0 = circle0.c + u * (r0 / r0_p_r1);
        CircleCircleConfig::NoncocircularOnePoint(p0)
    }
}

/// Circle Circle intersect
#[cfg(test)]
mod tests_circle {
    use super::*;
    use crate::circle::circle;

    // short funtion
    fn ff(circle0: Circle, circle1: Circle) -> CircleCircleConfig {
        int_circle_circle(circle0, circle1)
    }

    #[test]
    fn test_same_circles_01() {
        let circle0 = circle(point(100.0, -100.0), 1.0);
        let circle1 = circle(point(100.0, -100.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::SameCircles());
    }

    #[test]
    fn test_same_non_intersection_01() {
        let circle0 = circle(point(1000.0, -1000.0), 1.01);
        let circle1 = circle(point(1000.0, -1000.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::NoIntersection());
    }

    #[test]
    fn test_same_non_intersection_02() {
        let circle0 = circle(point(1000.0, -1000.0), 1.0);
        let circle1 = circle(point(1002.0, -1002.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::NoIntersection());
    }

    #[test]
    fn test_noncircular_two_points() {
        let eps = f64::EPSILON * 10.0;
        let circle0 = circle(point(10.0, -10.0), 1.0);
        let circle1 = circle(point(10.0, -12.0 + eps), 1.0);
        let point0 = point(10.000000042146848, -11.0);
        let point1 = point(9.999999957853152, -11.0);
        let res = ff(circle0, circle1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularTwoPoints(point0, point1)
        );
    }

    #[test]
    fn test_noncircular_one_point_01() {
        let eps = f64::EPSILON * 2.0;
        let circle0 = circle(point(10.0, -10.0), 1.0);
        let circle1 = circle(point(10.0, -12.0 + eps), 1.0);
        let point0 = point(10.0, -11.0);
        let res = ff(circle0, circle1);
        assert_eq!(res, CircleCircleConfig::NoncocircularOnePoint(point0));
    }

    #[test]
    fn test_noncircular_one_point_02() {
        let circle0 = circle(point(10.0, -10.0), 1.0);
        let circle1 = circle(point(10.0, -10.5), 0.5);
        let point0 = point(10.0, -11.0);
        let res = ff(circle0, circle1);
        assert_eq!(res, CircleCircleConfig::NoncocircularOnePoint(point0));
    }

    #[test]
    fn test_noncircular_two_points_1() {
        let eps = f64::EPSILON * 5.0;
        let circle0 = circle(point(10.0, -10.0), 1.0);
        let circle1 = circle(point(10.0, -10.5 - eps), 0.5);
        let point0 = point(10.000000059604645, -10.999999999999998);
        let point1 = point(9.999999940395355, -10.999999999999998);
        let res = ff(circle0, circle1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularTwoPoints(point0, point1)
        );
    }

    #[test]
    // Test for numerical stability
    fn test_noncircular_one_point_03() {
        let eps = f64::EPSILON * 2.0;
        let circle0 = circle(point(1000.0, -1000.0), 100.0);
        let circle1 = circle(point(1000.0, -1200.0 + eps), 100.0);
        let point0 = point(1000.0, -1100.0);
        let res = ff(circle0, circle1);
        assert_eq!(res, CircleCircleConfig::NoncocircularOnePoint(point0));
    }
}

#[cfg(test)]
mod tests_circle_old {

    // Old tests
    ///////////////////////
    use super::*;
    use crate::{circle::circle, utils::perturbed_ulps_as_int};

    // short funtion
    fn ff(circle0: Circle, circle1: Circle) -> CircleCircleConfig {
        int_circle_circle(circle0, circle1)
    }

    #[test]
    fn test_same_circles01() {
        // Circles are the same
        let circle0 = circle(point(0.0, 0.0), 1.0);
        let circle1 = circle(point(0.0, 0.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::SameCircles());
    }
    #[test]
    fn test_same_circles02() {
        // Circles are the same, radius zero
        let circle0 = circle(point(0.0, 0.0), 0.0);
        let circle1 = circle(point(0.0, 0.0), 0.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::SameCircles());
    }
    #[test]
    fn test_same_circles03() {
        // Circles are the same, radius is large
        let circle0 = circle(point(0.0, 0.0), f64::MAX);
        let circle1 = circle(point(0.0, 0.0), f64::MAX);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::SameCircles());
    }
    #[test]
    fn test_same_circles04() {
        // Circles are the same, center is large
        let circle0 = circle(point(f64::MAX, f64::MAX), f64::MAX);
        let circle1 = circle(point(f64::MAX, f64::MAX), f64::MAX);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::SameCircles());
    }

    #[test]
    fn test_donot_intersect01() {
        // The circles do not intersect.
        let r = perturbed_ulps_as_int(1.0, -2);
        let circle0 = circle(point(-1.0, 0.0), r);
        let circle1 = circle(point(1.0, 0.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::NoIntersection());
    }
    #[test]
    fn test_donot_intersect02() {
        // The circles do not intersect.
        let x = perturbed_ulps_as_int(1.0, 2);
        let circle0 = circle(point(-1.0, 0.0), 1.0);
        let circle1 = circle(point(x, 0.0), 1.0);
        assert_eq!(ff(circle0, circle1), CircleCircleConfig::NoIntersection());
    }

    #[test]
    fn test_tangent01() {
        // The circles touch in one point.
        let x = perturbed_ulps_as_int(1.0, 1);
        let circle0 = circle(point(-1.0, 0.0), 1.0);
        let circle1 = circle(point(x, 0.0), 1.0);
        assert_eq!(
            ff(circle0, circle1),
            CircleCircleConfig::NoncocircularOnePoint(point(0.0, 0.0))
        );
    }

    #[test]
    fn test_tangent02() {
        // Circles with small shift intersect in two points
        let circle0 = circle(point(1.0, 0.0), 1.0);
        let circle1 = circle(point(1.0 + f64::EPSILON, 0.0), 1.0);
        let res = int_circle_circle(circle0, circle1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularTwoPoints(point(1.0, 1.0), point(1.0, -1.0))
        );
    }

    #[test]
    fn test_tangent03() {
        // Circles with very small shift are same (cocircular)
        let _0 = perturbed_ulps_as_int(0.0, 1);
        let _1 = perturbed_ulps_as_int(1.0, -1);
        let circle0 = circle(point(0.0, 0.0), 1.0);
        let circle1 = circle(point(_0, 0.0), 1.0);
        let res = ff(circle0, circle1);
        assert_eq!(res, CircleCircleConfig::SameCircles());
    }

    #[test]
    fn test_tangent04() {
        // Cocircular with small r difference
        let _1 = perturbed_ulps_as_int(1.0, -1);
        let circle0 = circle(point(0.0, 0.0), 1.0);
        let circle1 = circle(point(0.0, 0.0), _1);
        let res = ff(circle0, circle1);
        assert_eq!(res, CircleCircleConfig::NoIntersection());
    }

    #[test]
    fn test_tangent05() {
        // Small difference in center and radius
        let _1m = perturbed_ulps_as_int(1.0, -1);
        let _1p = perturbed_ulps_as_int(1.0, 1);
        let circle0 = circle(point(1.0, 0.0), 1.0);
        let circle1 = circle(point(_1p, 0.0), _1m);
        let res = ff(circle0, circle1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularTwoPoints(
                point(1.5, 0.8660254037844386),
                point(1.5, -0.8660254037844386)
            )
        );
    }

    #[test]
    fn test_tangent06() {
        // Small difference in center and smaller radius
        let _1m = perturbed_ulps_as_int(1.0, -2);
        let _1p = perturbed_ulps_as_int(1.0, 1);
        let circle0 = circle(point(1.0, 0.0), 1.0);
        let circle1 = circle(point(_1p, 0.0), _1m);
        let res = ff(circle0, circle1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularOnePoint(point(2.0, 0.0))
        );
    }

    #[test]
    fn test_no_intersection2() {
        let c0 = circle(point(0.5, 0.0), 0.5);
        let c1 = circle(point(-1.0, 0.0), 1.0);
        let res = ff(c0, c1);
        assert_eq!(
            res,
            CircleCircleConfig::NoncocircularOnePoint(point(0.0, 0.0))
        );
    }

    use crate::svg::svg;
    #[test]
    fn test_intersection_issue_01() {
        let mut svg = svg(150.0, 200.0);
        let c0 = circle(point(100.0, 130.0), 20.0);
        let c1 = circle(point(75.0, 40.0), 85.0);
        svg.circle(&c0, "red");
        svg.circle(&c1, "blue");
        let p0 = point(113.87064429562277, 115.59148769566033);
        let p1 = point(80.68522962987866, 124.80965843614482);

        svg.circle(&circle(p0, 1.0), "red");
        svg.write();
        let res = ff(c0, c1);
        assert_eq!(res, CircleCircleConfig::NoncocircularTwoPoints(p0, p1));
    }
}

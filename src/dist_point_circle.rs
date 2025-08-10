#![allow(dead_code)]

use crate::circle::Circle;
use crate::point::{Point, point};

const ZERO : f64 = 0.0;
// #00015
/// Computes the distance between a point and a circle.
/// 
/// This function finds the shortest distance from a point to a circle,
/// along with the closest point on the circle.
/// 
/// # Arguments
/// * `p` - The point to measure distance from
/// * `circle` - The circle to measure distance to
///
/// # Returns
/// A tuple containing:
/// * The minimum distance as a f64
/// * The closest point on the circle
/// * A boolean indicating if the point is at the center of the circle
///
/// # Algorithm
/// The algorithm:
/// 1. Compute the vector from the circle center to the point
/// 2. If the point is not at the center, normalize this vector and scale it by the circle radius
/// 3. Compute the distance as the absolute difference between the point's distance from the center
///    and the circle radius
/// 4. Return the closest point on the circle as the center plus the scaled vector
///
/// # Examples
/// ```
/// use basegeom::prelude::*;
/// let c = circle(point(1.0, 1.0), 1.0);
/// let p = point(3.0, 1.0);
/// let (dist, closest, equidistant) = dist_point_circle(&p, &c);
/// // dist = 1.0, closest = (2.0, 1.0), equidistant = false
/// ```
pub fn dist_point_circle(p: &Point, circle: &Circle) -> (f64, Point, bool) {
    let diff = p - circle.c;
    let length = diff.dot(diff);
    if length > ZERO {
        let length = length.sqrt();
        let diff = diff / length;
        ((length - circle.r).abs(), circle.c + diff * circle.r, false)
    } else {
        let unit = point(1.0, 0.0);
        (circle.r, circle.c + unit * circle.r, true)
    }
}

#[cfg(test)]
mod test_dist_point_circle {
    use crate::{circle::circle, point::point};


    #[test]
    fn test_point_outside_circle() {
        let c = circle(point(1.0, 1.0), 1.0);
        let p = point(3.0, 1.0);
        let (dist, closest, equidistant) = super::dist_point_circle(&p, &c);
        assert_eq!(dist, 1.0);
        assert_eq!(closest, point(2.0, 1.0));
        assert_eq!(equidistant, false);
    }

    #[test]
    fn test_point_on_circle() {
        let c = circle(point(1.0, 1.0), 1.0);
        let p = point(2.0, 1.0);
        let (dist, closest, equidistant) = super::dist_point_circle(&p, &c);
        assert_eq!(dist, 0.0);
        assert_eq!(closest, point(2.0, 1.0));
        assert_eq!(equidistant, false);
    }

    #[test]
    fn test_point_inside_circle() {
        let c = circle(point(1.0, 1.0), 1.0);
        let p = point(1.5, 1.0);
        let (dist, closest, equidistant) = super::dist_point_circle(&p, &c);
        assert_eq!(dist, 0.5);
        assert_eq!(closest, point(2.0, 1.0));
        assert_eq!(equidistant, false);
    }

    #[test]
    fn test_point_in_circle_center() {
        let c = circle(point(1.0, 1.0), 1.0);
        let p = point(1.0, 1.0);
        let (dist, closest, equidistant) = super::dist_point_circle(&p, &c);
        assert_eq!(dist, 1.0);
        assert_eq!(closest, point(2.0, 1.0));
        assert_eq!(equidistant, true);
    }
}

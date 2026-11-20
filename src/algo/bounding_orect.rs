#![allow(dead_code)]

//! Axis-aligned bounding rectangles of circles and oriented rectangles
//! (for use with the `aabb` broad phase).

use crate::circle::Circle;
use crate::orect::OrientedRect;
use crate::point::point;
use crate::rect::Rect;

/// Computes the axis-aligned bounding rectangle of a circle.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let b = circle_bounding_rect(&circle(point(1.0, 2.0), 0.5));
/// assert_eq!(b.p1, point(0.5, 1.5));
/// assert_eq!(b.p2, point(1.5, 2.5));
/// ```
pub fn circle_bounding_rect(circle: &Circle) -> Rect {
    Rect::new(
        point(circle.c.x - circle.r, circle.c.y - circle.r),
        point(circle.c.x + circle.r, circle.c.y + circle.r),
    )
}

/// Computes the tight axis-aligned bounding rectangle of an oriented rectangle.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let r = orect(point(5.0, 5.0), point(1.0, 0.0), 1.0, 0.5);
/// let b = orect_bounding_rect(&r);
/// assert_eq!(b.p1, point(4.0, 4.5));
/// assert_eq!(b.p2, point(6.0, 5.5));
/// ```
pub fn orect_bounding_rect(rect: &OrientedRect) -> Rect {
    let ex = rect.dir.x.abs() * rect.half_len + rect.dir.y.abs() * rect.half_wid;
    let ey = rect.dir.y.abs() * rect.half_len + rect.dir.x.abs() * rect.half_wid;
    Rect::new(
        point(rect.c.x - ex, rect.c.y - ey),
        point(rect.c.x + ex, rect.c.y + ey),
    )
}

#[cfg(test)]
mod test_bounding_orect {
    use super::*;
    use crate::circle::circle;
    use crate::orect::orect;
    use crate::point::Point;
    use std::f64::consts::FRAC_PI_4;

    fn close(a: Point, b: Point) -> bool {
        a.close_enough(b, 1e-12)
    }

    #[test]
    fn circle_basic() {
        let b = circle_bounding_rect(&circle(point(-1.0, 3.0), 2.0));
        assert_eq!(b.p1, point(-3.0, 1.0));
        assert_eq!(b.p2, point(1.0, 5.0));
    }

    #[test]
    fn circle_zero_radius() {
        let b = circle_bounding_rect(&circle(point(1.0, 1.0), 0.0));
        assert_eq!(b.p1, point(1.0, 1.0));
        assert_eq!(b.p2, point(1.0, 1.0));
    }

    #[test]
    fn orect_axis_aligned() {
        let r = orect(point(5.0, 5.0), point(1.0, 0.0), 1.0, 0.5);
        let b = orect_bounding_rect(&r);
        assert!(close(b.p1, point(4.0, 4.5)));
        assert!(close(b.p2, point(6.0, 5.5)));
    }

    #[test]
    fn orect_45_degrees() {
        let r = OrientedRect::from_angle(point(1.0, 2.0), FRAC_PI_4, 2.0, 1.0);
        let b = orect_bounding_rect(&r);
        let e = (2.0 + 1.0) * FRAC_PI_4.cos();
        assert!(close(b.p1, point(1.0 - e, 2.0 - e)));
        assert!(close(b.p2, point(1.0 + e, 2.0 + e)));
    }

    #[test]
    fn orect_tight_around_all_corners() {
        for i in 0..64 {
            let a = i as f64 * 0.1 - 3.0;
            let r = OrientedRect::from_angle(point(-1.5, 0.7), a, 1.3, 0.4);
            let b = orect_bounding_rect(&r);
            let (mut mnx, mut mny, mut mxx, mut mxy) = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
            for p in r.corners() {
                mnx = mnx.min(p.x);
                mny = mny.min(p.y);
                mxx = mxx.max(p.x);
                mxy = mxy.max(p.y);
            }
            // must be tight, not just containing
            assert!(close(b.p1, point(mnx, mny)));
            assert!(close(b.p2, point(mxx, mxy)));
        }
    }

    #[test]
    fn orect_degenerate_zero_size() {
        let r = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        let b = orect_bounding_rect(&r);
        assert!(close(b.p1, point(1.0, 1.0)) && close(b.p2, point(1.0, 1.0)));
    }
}

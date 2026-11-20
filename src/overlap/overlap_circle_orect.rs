#![allow(dead_code)]

use crate::circle::Circle;
use crate::orect::OrientedRect;

/// Tests whether a circle (a filled disc) and an oriented rectangle overlap.
///
/// Returns `true` if the closed disc and the closed rectangle share at least
/// one point, so **touching counts as overlap**: the test is
/// `distance(center, rectangle) <= radius`. A circle completely inside the
/// rectangle, or a rectangle completely inside the circle, also overlaps.
///
/// The circle centre is moved to the rectangle's local frame and clamped to
/// the rectangle; no square root is taken.
///
/// For a rotated rectangle, exact touching is subject to floating-point
/// rounding. For axis-aligned rectangles with exactly representable
/// coordinates it is exact. The radius must be non-negative.
///
/// # Arguments
///
/// * `circle` - The circle
/// * `rect` - The oriented rectangle; its `dir` must be a unit vector
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let r = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// // circle touching the right edge at (1, 0)
/// assert!(overlap_circle_orect(&circle(point(2.0, 0.0), 1.0), &r));
/// assert!(!overlap_circle_orect(&circle(point(2.1, 0.0), 1.0), &r));
/// ```
pub fn overlap_circle_orect(circle: &Circle, rect: &OrientedRect) -> bool {
    let q = rect.to_local(circle.c);
    // Distance from the centre to the rectangle along each local axis (0 if inside the slab).
    let dx = (q.x.abs() - rect.half_len).max(0.0);
    let dy = (q.y.abs() - rect.half_wid).max(0.0);
    dx * dx + dy * dy <= circle.r * circle.r
}

#[cfg(test)]
mod test_overlap_circle_orect {
    use super::overlap_circle_orect as overlap;
    use crate::circle::circle;
    use crate::distance::dist_point_segment;
    use crate::orect::{OrientedRect, orect};
    use crate::point::point;
    use crate::segment::segment;
    use std::f64::consts::FRAC_PI_4;

    fn unit_rect() -> OrientedRect {
        orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0)
    }

    #[test]
    fn far_apart() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), 0.6, 2.0, 1.0);
        assert!(!overlap(&circle(point(30.0, -20.0), 1.0), &r));
    }

    #[test]
    fn center_inside_and_circle_inside() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), 0.6, 2.0, 1.0);
        assert!(overlap(&circle(point(0.1, 0.1), 0.01), &r)); // small circle inside
        assert!(overlap(&circle(point(0.0, 0.0), 100.0), &r)); // rect inside big circle
        assert!(overlap(&circle(point(0.0, 0.0), 0.0), &r)); // point inside
    }

    #[test]
    fn touching_edge() {
        let r = unit_rect();
        assert!(overlap(&circle(point(2.0, 0.0), 1.0), &r));
        assert!(!overlap(&circle(point(2.0 + 1e-9, 0.0), 1.0), &r));
        assert!(overlap(&circle(point(0.3, -2.0), 1.0), &r));
        assert!(!overlap(&circle(point(0.3, -2.0 - 1e-9), 1.0), &r));
    }

    #[test]
    fn touching_corner() {
        let r = unit_rect();
        // corner (1,1); centre (4,5) is exactly 5 away (3-4-5 triangle)
        assert!(overlap(&circle(point(4.0, 5.0), 5.0), &r));
        assert!(!overlap(&circle(point(4.0, 5.0), 5.0 - 1e-9), &r));
        // circle in the corner region, close to the edges' extensions but outside the rect
        assert!(!overlap(&circle(point(1.8, 1.8), 1.0), &r)); // dist to corner 1.13 > 1
        assert!(overlap(&circle(point(1.6, 1.6), 1.0), &r)); // dist to corner 0.85 < 1
    }

    #[test]
    fn rotated_45_corner() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), FRAC_PI_4, 1.0, 1.0);
        // a diamond with a tip at (sqrt2, 0)
        let tip = 2.0_f64.sqrt();
        assert!(overlap(&circle(point(tip + 1.0 - 1e-9, 0.0), 1.0), &r));
        assert!(!overlap(&circle(point(tip + 1.0 + 1e-9, 0.0), 1.0), &r));
        // bounding box would say overlap, but circle is beyond the diamond's slanted edge
        assert!(!overlap(&circle(point(1.2, 1.2), 0.5), &r));
    }

    #[test]
    fn degenerate_rect() {
        // zero-size rectangle = a point
        let p = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        assert!(overlap(&circle(point(1.0, 2.0), 1.0), &p));
        assert!(!overlap(&circle(point(1.0, 2.0), 1.0 - 1e-9), &p));
        // zero-width segment
        let s = OrientedRect::from_angle(point(0.0, 0.0), 0.0, 2.0, 0.0);
        assert!(overlap(&circle(point(1.0, 0.5), 0.5), &s));
        assert!(!overlap(&circle(point(3.0, 0.5), 0.5), &s));
    }

    #[test]
    fn zero_radius_circle_on_boundary() {
        let r = unit_rect();
        for p in [point(1.0, 0.0), point(1.0, 1.0), point(-1.0, -1.0), point(0.0, 1.0)] {
            assert!(overlap(&circle(p, 0.0), &r), "p={p}");
        }
        assert!(!overlap(&circle(point(1.0 + 1e-9, 0.0), 0.0), &r));
        assert!(!overlap(&circle(point(1.0 + 1e-9, 1.0 + 1e-9), 0.0), &r));
    }

    #[test]
    fn center_at_rect_center() {
        let r = OrientedRect::from_angle(point(2.0, -1.0), 0.9, 1.0, 0.5);
        assert!(overlap(&circle(r.c, 0.0), &r));
        assert!(overlap(&circle(r.c, 1e-12), &r));
    }

    #[test]
    fn point_rect_vs_circle() {
        let p = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        assert!(overlap(&circle(point(1.0, 1.0), 0.0), &p));
        assert!(!overlap(&circle(point(1.0, 1.0 + 1e-9), 0.0), &p));
    }

    #[test]
    fn rotated_30_degrees_edge() {
        let r = OrientedRect::from_angle(point(1.0, 2.0), 0.5236, 2.0, 1.0);
        let at = |gap: f64| circle(r.c + r.normal() * (1.0 + 0.5 + gap), 0.5);
        assert!(overlap(&at(-0.01), &r));
        assert!(!overlap(&at(0.01), &r));
    }

    #[test]
    fn far_from_origin() {
        let o = 1.0e6;
        let r = orect(point(o, o), point(1.0, 0.0), 1.0, 1.0);
        assert!(overlap(&circle(point(o + 2.0, o), 1.0), &r)); // touching
        assert!(!overlap(&circle(point(o + 2.001, o), 1.0), &r));
    }

    #[test]
    fn direction_sign_does_not_matter() {
        let mut rng = Lcg(77);
        for _ in 0..2000 {
            let r = OrientedRect::from_angle(
                point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                rng.range(-3.2, 3.2),
                rng.range(0.1, 2.0),
                rng.range(0.1, 2.0),
            );
            let flipped = orect(r.c, -r.dir, r.half_len, r.half_wid);
            let c = circle(
                point(rng.range(-4.0, 4.0), rng.range(-4.0, 4.0)),
                rng.range(0.05, 2.0),
            );
            assert_eq!(overlap(&c, &r), overlap(&c, &flipped));
        }
    }

    // ---- brute-force reference: centre inside, or an edge within radius ----

    fn brute_overlap(c: &crate::circle::Circle, r: &OrientedRect) -> bool {
        if r.contains_point(c.c) {
            return true;
        }
        let k = r.corners();
        (0..4).any(|i| {
            let (d, _) = dist_point_segment(&c.c, &segment(k[i], k[(i + 1) % 4]));
            d <= c.r
        })
    }

    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self) -> f64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next()
        }
    }

    #[test]
    fn matches_brute_force_random() {
        let mut rng = Lcg(987654321);
        let (mut hits, mut misses) = (0, 0);
        for _ in 0..20000 {
            let r = OrientedRect::from_angle(
                point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                rng.range(-3.2, 3.2),
                rng.range(0.1, 2.0),
                rng.range(0.1, 2.0),
            );
            let c = circle(
                point(rng.range(-4.0, 4.0), rng.range(-4.0, 4.0)),
                rng.range(0.05, 2.0),
            );
            let expected = brute_overlap(&c, &r);
            assert_eq!(overlap(&c, &r), expected, "c={c} r={r}");
            if expected { hits += 1 } else { misses += 1 }
        }
        assert!(hits > 2000 && misses > 2000, "hits={hits} misses={misses}");
    }
}

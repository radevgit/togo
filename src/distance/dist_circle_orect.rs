#![allow(dead_code)]

use crate::circle::Circle;
use crate::orect::OrientedRect;

/// Computes the distance between a filled circle (disc) and a filled oriented rectangle.
///
/// Returns the length of the shortest segment between the two filled shapes,
/// or `0.0` if they touch or overlap (including one shape lying inside the other).
/// The result is never negative.
///
/// Note: unlike [`dist_point_circle`](crate::distance::dist_point_circle()) and
/// [`dist_segment_circle`](crate::distance::dist_segment_circle()), which treat the
/// circle as a curve, this function treats both shapes as **filled**, consistent
/// with [`overlap_circle_orect`](crate::overlap::overlap_circle_orect()):
/// `dist_circle_orect(c, r) == 0.0` exactly when the shapes overlap.
///
/// # Arguments
///
/// * `circle` - The circle; its radius must be non-negative
/// * `rect` - The oriented rectangle; its `dir` must be a unit vector
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let r = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// assert_eq!(dist_circle_orect(&circle(point(4.0, 0.0), 1.0), &r), 2.0);
/// assert_eq!(dist_circle_orect(&circle(point(0.5, 0.5), 0.1), &r), 0.0); // inside
/// ```
pub fn dist_circle_orect(circle: &Circle, rect: &OrientedRect) -> f64 {
    let q = rect.to_local(circle.c);
    let dx = (q.x.abs() - rect.half_len).max(0.0);
    let dy = (q.y.abs() - rect.half_wid).max(0.0);
    ((dx * dx + dy * dy).sqrt() - circle.r).max(0.0)
}

#[cfg(test)]
mod test_dist_circle_orect {
    use super::dist_circle_orect as dist;
    use crate::circle::circle;
    use crate::distance::dist_point_segment;
    use crate::orect::{OrientedRect, orect};
    use crate::overlap::overlap_circle_orect;
    use crate::point::point;
    use crate::segment::segment;
    use std::f64::consts::FRAC_PI_4;

    fn unit_rect() -> OrientedRect {
        orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0)
    }

    #[test]
    fn face_to_face() {
        let r = unit_rect();
        assert_eq!(dist(&circle(point(4.0, 0.0), 1.0), &r), 2.0);
        assert_eq!(dist(&circle(point(0.3, -3.5), 0.5), &r), 2.0);
    }

    #[test]
    fn corner_region() {
        let r = unit_rect();
        // corner (1,1); centre (4,5) is 5 away
        assert_eq!(dist(&circle(point(4.0, 5.0), 1.0), &r), 4.0);
    }

    #[test]
    fn touching_and_overlapping_is_zero() {
        let r = unit_rect();
        assert_eq!(dist(&circle(point(2.0, 0.0), 1.0), &r), 0.0); // touching
        assert_eq!(dist(&circle(point(1.5, 0.0), 1.0), &r), 0.0); // overlapping
        assert_eq!(dist(&circle(point(0.0, 0.0), 0.1), &r), 0.0); // circle inside
        assert_eq!(dist(&circle(point(0.0, 0.0), 50.0), &r), 0.0); // rect inside circle
    }

    #[test]
    fn rotated_45() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), FRAC_PI_4, 1.0, 1.0);
        let tip = 2.0_f64.sqrt();
        let d = dist(&circle(point(tip + 3.0, 0.0), 1.0), &r);
        assert!((d - 2.0).abs() < 1e-12, "d={d}");
    }

    #[test]
    fn degenerate() {
        let p = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        assert_eq!(dist(&circle(point(1.0, 4.0), 1.0), &p), 2.0);
        assert_eq!(dist(&circle(point(1.0, 1.0), 0.0), &p), 0.0);
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn zero_radius_circle() {
        let r = unit_rect();
        assert_eq!(dist(&circle(point(1.0, 0.0), 0.0), &r), 0.0); // on edge
        assert_eq!(dist(&circle(point(1.0, 1.0), 0.0), &r), 0.0); // on corner
        assert_eq!(dist(&circle(point(0.0, 0.0), 0.0), &r), 0.0); // centre
        assert!(close(dist(&circle(point(4.0, 5.0), 0.0), &r), 5.0));
    }

    #[test]
    fn point_rect_vs_circle() {
        let p = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        assert_eq!(dist(&circle(point(1.0, 1.0), 0.0), &p), 0.0);
        assert!(close(dist(&circle(point(4.0, 5.0), 0.0), &p), 5.0));
    }

    #[test]
    fn parallel_rotated_30_degrees() {
        let r = OrientedRect::from_angle(point(1.0, 2.0), 0.5236, 2.0, 1.0);
        let c = circle(r.c + r.normal() * (1.0 + 0.5 + 2.5), 0.5);
        assert!(close(dist(&c, &r), 2.5));
    }

    #[test]
    fn far_from_origin() {
        let o = 1.0e6;
        let r = orect(point(o, o), point(1.0, 0.0), 1.0, 1.0);
        assert!(close(dist(&circle(point(o + 4.0, o), 1.0), &r), 2.0));
    }

    #[test]
    fn direction_sign_does_not_matter() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), 0.4, 1.0, 0.5);
        let flipped = orect(r.c, -r.dir, r.half_len, r.half_wid);
        let c = circle(point(3.0, 2.0), 0.5);
        assert!(dist(&c, &r) > 0.0);
        assert!(close(dist(&c, &r), dist(&c, &flipped)));
    }

    fn brute_dist(c: &crate::circle::Circle, r: &OrientedRect) -> f64 {
        if r.contains_point(c.c) {
            return 0.0;
        }
        let k = r.corners();
        let d = (0..4)
            .map(|i| dist_point_segment(&c.c, &segment(k[i], k[(i + 1) % 4])).0)
            .fold(f64::MAX, f64::min);
        (d - c.r).max(0.0)
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
    fn matches_brute_force_and_overlap_random() {
        let mut rng = Lcg(555);
        let (mut zero, mut pos) = (0, 0);
        for _ in 0..20000 {
            let r = OrientedRect::from_angle(
                point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                rng.range(-3.2, 3.2),
                rng.range(0.1, 2.0),
                rng.range(0.1, 2.0),
            );
            let c = circle(
                point(rng.range(-5.0, 5.0), rng.range(-5.0, 5.0)),
                rng.range(0.05, 2.0),
            );
            let d = dist(&c, &r);
            assert!(d >= 0.0);
            assert!((d - brute_dist(&c, &r)).abs() < 1e-9, "c={c} r={r}");
            assert_eq!(d == 0.0, overlap_circle_orect(&c, &r), "c={c} r={r}");
            if d == 0.0 { zero += 1 } else { pos += 1 }
        }
        assert!(zero > 2000 && pos > 2000, "zero={zero} pos={pos}");
    }
}

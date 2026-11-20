#![allow(dead_code)]

use crate::distance::dist_point_segment::dist_point_segment;
use crate::orect::OrientedRect;
use crate::overlap::overlap_orect_orect;
use crate::segment::segment;

/// Computes the distance between two filled oriented rectangles.
///
/// Returns the length of the shortest segment between the two filled
/// rectangles, or `0.0` if they touch or overlap (including one rectangle lying
/// inside the other). The result is never negative.
///
/// Both rectangles are treated as **filled**, consistent with
/// [`overlap_orect_orect`](crate::overlap::overlap_orect_orect()):
/// `dist_orect_orect(a, b) == 0.0` exactly when the rectangles overlap.
///
/// # Algorithm
/// 1. If the rectangles overlap (separating-axis test), the distance is zero.
/// 2. Otherwise the closest points of two disjoint convex polygons always
///    involve a corner, so the distance is the minimum over the distances
///    from the 4 corners of each rectangle to the 4 edges of the other.
///
/// # Arguments
///
/// * `a`, `b` - The rectangles; `dir` of each must be a unit vector
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// let b = orect(point(5.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// assert_eq!(dist_orect_orect(&a, &b), 3.0);
/// assert_eq!(dist_orect_orect(&a, &a), 0.0);
/// ```
pub fn dist_orect_orect(a: &OrientedRect, b: &OrientedRect) -> f64 {
    if overlap_orect_orect(a, b) {
        return 0.0;
    }
    let ca = a.corners();
    let cb = b.corners();
    let mut best = f64::MAX;
    for i in 0..4 {
        let ea = segment(ca[i], ca[(i + 1) % 4]);
        let eb = segment(cb[i], cb[(i + 1) % 4]);
        for j in 0..4 {
            best = best.min(dist_point_segment(&cb[j], &ea).0);
            best = best.min(dist_point_segment(&ca[j], &eb).0);
        }
    }
    best
}

#[cfg(test)]
mod test_dist_orect_orect {
    use super::dist_orect_orect as dist;
    use crate::orect::{OrientedRect, orect};
    use crate::overlap::overlap_orect_orect;
    use crate::point::{Point, point};
    use std::f64::consts::FRAC_PI_4;

    fn unit_rect() -> OrientedRect {
        orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0)
    }

    #[test]
    fn face_to_face() {
        let a = unit_rect();
        let b = orect(point(5.0, 0.5), point(1.0, 0.0), 1.0, 1.0);
        assert_eq!(dist(&a, &b), 3.0);
        assert_eq!(dist(&b, &a), 3.0);
    }

    #[test]
    fn corner_to_corner() {
        let a = unit_rect();
        // corners (1,1) and (4,5): 3-4-5
        let b = orect(point(5.0, 6.0), point(1.0, 0.0), 1.0, 1.0);
        assert_eq!(dist(&a, &b), 5.0);
    }

    #[test]
    fn touching_overlapping_inside_is_zero() {
        let a = unit_rect();
        assert_eq!(dist(&a, &orect(point(2.0, 0.0), point(1.0, 0.0), 1.0, 1.0)), 0.0); // edge
        assert_eq!(dist(&a, &orect(point(2.0, 2.0), point(1.0, 0.0), 1.0, 1.0)), 0.0); // corner
        assert_eq!(dist(&a, &orect(point(1.0, 0.0), point(1.0, 0.0), 1.0, 1.0)), 0.0); // overlap
        let inner = OrientedRect::from_angle(point(0.0, 0.0), 0.5, 0.2, 0.1);
        assert_eq!(dist(&a, &inner), 0.0);
        assert_eq!(dist(&inner, &a), 0.0);
    }

    #[test]
    fn diamond_tip_to_edge() {
        let a = unit_rect();
        // 45° square (half-size 1) has its tip sqrt(2) from its centre
        let diag = 2.0_f64.sqrt();
        let b = OrientedRect::from_angle(point(1.0 + diag + 2.0, 0.0), FRAC_PI_4, 1.0, 1.0);
        let d = dist(&a, &b);
        assert!((d - 2.0).abs() < 1e-12, "d={d}");
    }

    #[test]
    fn degenerate_point_and_segment() {
        let a = unit_rect();
        let pt = orect(point(1.0, 4.0), point(0.6, 0.8), 0.0, 0.0);
        assert_eq!(dist(&a, &pt), 3.0);
        let seg = OrientedRect::from_angle(point(5.0, 0.0), 0.0, 1.0, 0.0);
        assert_eq!(dist(&a, &seg), 3.0);
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn point_vs_point() {
        let p = orect(point(0.0, 0.0), point(0.6, 0.8), 0.0, 0.0);
        let q = orect(point(3.0, 4.0), point(1.0, 0.0), 0.0, 0.0);
        assert!(close(dist(&p, &q), 5.0));
        assert_eq!(dist(&p, &p), 0.0);
    }

    #[test]
    fn segment_vs_segment() {
        let seg = |cx, cy, ang, h| OrientedRect::from_angle(point(cx, cy), ang, h, 0.0);
        let half_pi = std::f64::consts::FRAC_PI_2;
        let a = seg(0.0, 0.0, 0.0, 2.0); // x in [-2, 2] on y = 0
        assert_eq!(dist(&a, &seg(0.0, 0.0, half_pi, 2.0)), 0.0); // crossing
        assert!(close(dist(&a, &seg(0.0, 3.0, 0.0, 2.0)), 3.0)); // parallel, 3 apart
        assert!(close(dist(&a, &seg(5.0, 0.0, 0.0, 1.0)), 2.0)); // collinear, gap 2
        assert!(close(dist(&a, &seg(0.0, 4.0, half_pi, 1.0)), 3.0)); // vertical above, ends at y = 3
    }

    #[test]
    fn parallel_rotated_rects_at_30_degrees() {
        let a = OrientedRect::from_angle(point(1.0, 2.0), 0.5236, 1.5, 1.0);
        let b = orect(a.c + a.normal() * (2.0 + 2.5), a.dir, 1.5, 1.0);
        assert!(close(dist(&a, &b), 2.5));
    }

    #[test]
    fn far_from_origin() {
        let o = 1.0e6;
        let a = orect(point(o, o), point(1.0, 0.0), 1.0, 1.0);
        let b = orect(point(o + 5.0, o), point(1.0, 0.0), 1.0, 1.0);
        assert!(close(dist(&a, &b), 3.0));
    }

    #[test]
    fn direction_sign_does_not_matter() {
        let a = OrientedRect::from_angle(point(0.0, 0.0), 0.4, 1.0, 0.5);
        let b = OrientedRect::from_angle(point(4.0, 3.0), 1.3, 0.7, 0.3);
        let fb = orect(b.c, -b.dir, b.half_len, b.half_wid);
        assert!(dist(&a, &b) > 0.0);
        assert!(close(dist(&a, &b), dist(&a, &fb)));
    }

    // ---- brute-force reference: dense boundary sampling ----

    const SAMPLES_PER_EDGE: usize = 25;

    fn boundary(r: &OrientedRect) -> Vec<Point> {
        let k = r.corners();
        let mut v = Vec::new();
        for i in 0..4 {
            for s in 0..SAMPLES_PER_EDGE {
                v.push(k[i].lerp(k[(i + 1) % 4], s as f64 / SAMPLES_PER_EDGE as f64));
            }
        }
        v
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
    fn matches_sampling_and_overlap_random() {
        let mut rng = Lcg(31337);
        let (mut zero, mut pos) = (0, 0);
        for _ in 0..1500 {
            let mut mk = || {
                OrientedRect::from_angle(
                    point(rng.range(-4.0, 4.0), rng.range(-4.0, 4.0)),
                    rng.range(-3.2, 3.2),
                    rng.range(0.1, 2.0),
                    rng.range(0.1, 2.0),
                )
            };
            let (a, b) = (mk(), mk());
            let d = dist(&a, &b);
            assert!(d >= 0.0);
            assert_eq!(d, dist(&b, &a), "symmetry a={a} b={b}");
            assert_eq!(d == 0.0, overlap_orect_orect(&a, &b), "a={a} b={b}");
            if d == 0.0 {
                zero += 1;
                continue;
            }
            pos += 1;
            // sampled boundary distance is an upper bound, within half a sample spacing
            // of the truth (the closest pair always includes a sampled corner)
            let (pa, pb) = (boundary(&a), boundary(&b));
            let mut s = f64::MAX;
            for &p in &pa {
                for &q in &pb {
                    s = s.min((p - q).norm());
                }
            }
            let max_edge = 2.0 * 2.0 * 2.0_f64.sqrt(); // longest possible edge
            let tol = max_edge / SAMPLES_PER_EDGE as f64;
            assert!(s >= d - 1e-9, "sampled {s} < exact {d}; a={a} b={b}");
            assert!(s <= d + tol, "sampled {s} >> exact {d}; a={a} b={b}");
        }
        assert!(zero > 100 && pos > 100, "zero={zero} pos={pos}");
    }
}

#![allow(dead_code)]

use crate::orect::OrientedRect;

/// Tests whether two oriented rectangles overlap (separating-axis test).
///
/// Returns `true` if the closed rectangles share at least one point, so
/// **touching counts as overlap**: rectangles that share only an edge segment
/// or a single corner overlap. Only a strict gap on some axis separates them.
///
/// For rotated rectangles, "exactly touching" is subject to floating-point
/// rounding, so a contact closer than a few ulps may be reported either way.
/// For axis-aligned rectangles with exactly representable coordinates it is exact.
///
/// This is a yes/no test with no distance computation; it is the fast path
/// for collision checks.
///
/// # Arguments
///
/// * `a`, `b` - The rectangles; `dir` of each must be a unit vector.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// let b = orect(point(2.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// assert!(overlap_orect_orect(&a, &b)); // shared edge x = 1
/// let c = orect(point(2.1, 0.0), point(1.0, 0.0), 1.0, 1.0);
/// assert!(!overlap_orect_orect(&a, &c));
/// ```
pub fn overlap_orect_orect(a: &OrientedRect, b: &OrientedRect) -> bool {
    let d = b.c - a.c;
    let an = a.normal();
    let bn = b.normal();
    // The 4 candidate separating axes: edge normals of both rectangles.
    let axes = [a.dir, an, b.dir, bn];
    for axis in axes {
        let ra = a.half_len * a.dir.dot(axis).abs() + a.half_wid * an.dot(axis).abs();
        let rb = b.half_len * b.dir.dot(axis).abs() + b.half_wid * bn.dot(axis).abs();
        if d.dot(axis).abs() > ra + rb {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test_overlap_orect_orect {
    use super::overlap_orect_orect as overlap;
    use crate::algo::orect_bounding_rect;
    use crate::orect::{OrientedRect, orect};
    use crate::point::{Point, point};
    use std::f64::consts::FRAC_PI_4;

    fn unit(a: f64) -> Point {
        let (s, c) = a.sin_cos();
        point(c, s)
    }

    #[test]
    fn far_apart() {
        let a = OrientedRect::from_angle(point(0.0, 0.0), 0.3, 1.0, 0.5);
        let b = OrientedRect::from_angle(point(50.0, 40.0), 1.1, 1.0, 0.5);
        assert!(!overlap(&a, &b));
        assert!(!overlap(&b, &a));
    }

    #[test]
    fn one_inside_other() {
        let a = OrientedRect::from_angle(point(0.0, 0.0), 0.4, 5.0, 4.0);
        let b = OrientedRect::from_angle(point(0.5, -0.5), 1.0, 1.0, 0.5);
        assert!(overlap(&a, &b));
        assert!(overlap(&b, &a));
        assert!(overlap(&a, &a)); // identical
    }

    #[test]
    fn touching_edge() {
        let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
        let b = orect(point(2.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
        assert!(overlap(&a, &b));
        let b_gap = orect(point(2.0 + 1e-9, 0.0), point(1.0, 0.0), 1.0, 1.0);
        assert!(!overlap(&a, &b_gap));
        // sliding along the shared edge
        let b_slide = orect(point(2.0, 1.5), point(1.0, 0.0), 1.0, 1.0);
        assert!(overlap(&a, &b_slide));
    }

    #[test]
    fn touching_corner() {
        let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
        let b = orect(point(2.0, 2.0), point(1.0, 0.0), 1.0, 1.0);
        assert!(overlap(&a, &b)); // corner (1,1) shared
        let b_gap = orect(point(2.0 + 1e-9, 2.0), point(1.0, 0.0), 1.0, 1.0);
        assert!(!overlap(&a, &b_gap));
    }

    #[test]
    fn diamond_vs_square_45_degrees() {
        let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
        let h = 1.0;
        let diag = 2.0_f64.sqrt() * h;
        // diamond tip touching the square's right edge at (1, 0): center at (1 + diag, 0)
        let b = OrientedRect::from_angle(point(1.0 + diag + 1e-9, 0.0), FRAC_PI_4, h, h);
        assert!(!overlap(&a, &b));
        let b = OrientedRect::from_angle(point(1.0 + diag - 1e-9, 0.0), FRAC_PI_4, h, h);
        assert!(overlap(&a, &b));
        // bounding boxes overlap but shapes do not: diamond near a corner
        let b = OrientedRect::from_angle(point(2.2, 2.2), FRAC_PI_4, h, h);
        let (ba, bb) = (orect_bounding_rect(&a), orect_bounding_rect(&b));
        assert!(ba.p2.x > bb.p1.x && ba.p2.y > bb.p1.y);
        assert!(!overlap(&a, &b));
    }

    #[test]
    fn degenerate() {
        let a = orect(point(0.0, 0.0), point(1.0, 0.0), 1.0, 1.0);
        let pt_in = orect(point(0.5, 0.5), unit(0.7), 0.0, 0.0);
        let pt_edge = orect(point(1.0, 0.5), unit(0.7), 0.0, 0.0);
        let pt_out = orect(point(1.1, 0.5), unit(0.7), 0.0, 0.0);
        assert!(overlap(&a, &pt_in) && overlap(&pt_in, &a));
        assert!(overlap(&a, &pt_edge) && overlap(&pt_edge, &a));
        assert!(!overlap(&a, &pt_out) && !overlap(&pt_out, &a));
        // zero-width segment crossing the square
        let seg = OrientedRect::from_angle(point(0.0, 0.0), 0.5, 3.0, 0.0);
        assert!(overlap(&a, &seg));
        let seg_far = OrientedRect::from_angle(point(5.0, 5.0), 0.5, 1.0, 0.0);
        assert!(!overlap(&a, &seg_far));
    }

    #[test]
    fn point_vs_point() {
        let p = orect(point(1.0, 1.0), unit(0.3), 0.0, 0.0);
        let same = orect(point(1.0, 1.0), unit(1.2), 0.0, 0.0);
        assert!(overlap(&p, &same));
        for k in 0..16 {
            let a = k as f64 * 0.4;
            let off = point(1.0 + 1e-6 * a.cos(), 1.0 + 1e-6 * a.sin());
            assert!(!overlap(&p, &orect(off, unit(1.2), 0.0, 0.0)), "k={k}");
        }
    }

    #[test]
    fn segment_vs_segment() {
        let seg = |cx, cy, ang, h| OrientedRect::from_angle(point(cx, cy), ang, h, 0.0);
        let a = seg(0.0, 0.0, 0.0, 2.0); // x in [-2, 2] on y = 0
        let half_pi = std::f64::consts::FRAC_PI_2;
        assert!(overlap(&a, &seg(0.0, 0.0, half_pi, 2.0))); // X crossing
        assert!(overlap(&a, &seg(1.0, 0.0, half_pi, 2.0))); // off-centre crossing
        assert!(overlap(&a, &seg(0.0, 1.0, half_pi, 1.0))); // T: end touches the middle
        assert!(!overlap(&a, &seg(0.0, 1.5, half_pi, 1.0))); // vertical, stops short
        assert!(!overlap(&a, &seg(0.0, 0.001, 0.0, 2.0))); // parallel, offset
        assert!(overlap(&a, &seg(1.0, 0.0, 0.0, 2.0))); // collinear, overlapping
        assert!(overlap(&a, &seg(3.0, 0.0, 0.0, 1.0))); // collinear, touching at x = 2
        assert!(!overlap(&a, &seg(5.0, 0.0, 0.0, 1.0))); // collinear, disjoint
    }

    #[test]
    fn parallel_rotated_rects_at_30_degrees() {
        let a = OrientedRect::from_angle(point(1.0, 2.0), 0.5236, 1.5, 1.0);
        let shifted = |gap: f64| orect(a.c + a.normal() * (2.0 + gap), a.dir, 1.5, 1.0);
        assert!(!overlap(&a, &shifted(0.01)));
        assert!(overlap(&a, &shifted(-0.01)));
    }

    #[test]
    fn far_from_origin() {
        let o = 1.0e6;
        let a = orect(point(o, o), point(1.0, 0.0), 1.0, 1.0);
        assert!(overlap(&a, &orect(point(o + 2.0, o), point(1.0, 0.0), 1.0, 1.0))); // touching
        assert!(!overlap(&a, &orect(point(o + 2.001, o), point(1.0, 0.0), 1.0, 1.0)));
    }

    #[test]
    fn direction_sign_does_not_matter() {
        let mut rng = Lcg(4242);
        let flip = |r: &OrientedRect| orect(r.c, -r.dir, r.half_len, r.half_wid);
        for _ in 0..2000 {
            let mut mk = || {
                OrientedRect::from_angle(
                    point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                    rng.range(-3.2, 3.2),
                    rng.range(0.1, 2.0),
                    rng.range(0.1, 2.0),
                )
            };
            let (a, b) = (mk(), mk());
            let r = overlap(&a, &b);
            assert_eq!(r, overlap(&flip(&a), &b));
            assert_eq!(r, overlap(&a, &flip(&b)));
            assert_eq!(r, overlap(&flip(&a), &flip(&b)));
        }
    }

    // ---- brute-force reference: corner containment or edge crossing ----

    fn cross(o: Point, a: Point, b: Point) -> f64 {
        (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
    }

    fn segs_cross(p1: Point, p2: Point, q1: Point, q2: Point) -> bool {
        let d1 = cross(q1, q2, p1);
        let d2 = cross(q1, q2, p2);
        let d3 = cross(p1, p2, q1);
        let d4 = cross(p1, p2, q2);
        d1 * d2 < 0.0 && d3 * d4 < 0.0
    }

    fn brute_overlap(a: &OrientedRect, b: &OrientedRect) -> bool {
        let ca = a.corners();
        let cb = b.corners();
        if ca.iter().any(|&p| b.contains_point(p)) || cb.iter().any(|&p| a.contains_point(p)) {
            return true;
        }
        for i in 0..4 {
            for j in 0..4 {
                if segs_cross(ca[i], ca[(i + 1) % 4], cb[j], cb[(j + 1) % 4]) {
                    return true;
                }
            }
        }
        false
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
        let mut rng = Lcg(12345);
        let (mut hits, mut misses) = (0, 0);
        for _ in 0..20000 {
            let a = OrientedRect::from_angle(
                point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                rng.range(-3.2, 3.2),
                rng.range(0.1, 2.0),
                rng.range(0.1, 2.0),
            );
            let b = OrientedRect::from_angle(
                point(rng.range(-3.0, 3.0), rng.range(-3.0, 3.0)),
                rng.range(-3.2, 3.2),
                rng.range(0.1, 2.0),
                rng.range(0.1, 2.0),
            );
            let expected = brute_overlap(&a, &b);
            assert_eq!(overlap(&a, &b), expected, "a={a} b={b}");
            assert_eq!(overlap(&b, &a), expected, "symmetry a={a} b={b}");
            if expected { hits += 1 } else { misses += 1 }
        }
        // the random set must exercise both outcomes
        assert!(hits > 2000 && misses > 2000, "hits={hits} misses={misses}");
    }
}

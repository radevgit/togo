#![allow(dead_code)]





use std::ops::Range;

use crate::{utils::{close_enough, diff_of_prod}, bezierq::BezierQ, Point};

#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BezierC {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub(crate) enum BezierConfig {
    NoLoop,
    Cusip(f64, f64), // (t, u) parameters around the cusip point
    Loop(f64, f64),  // (t, u) parameters of self-intersection
}

impl BezierC {
    /// Create a new cubic Bézier segment.
    #[inline(always)]
    pub(crate) fn new<P: Into<Point>>(p0: P, p1: P, p2: P, p3: P) -> BezierC {
        BezierC {
            p0: p0.into(),
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
        }
    }

}

// #00013
#[doc(hidden)]
const CUSIP_THRESHOLD: f64 = 1e-4; // threshold for cusip detection
impl BezierC {
    pub(crate) fn self_intersection(&self) -> BezierConfig {
        // For cubic curves containing a loop, compute the parameters t, u where
        // B(t) = B(u), i.e. the parameters of self-intersection.
        // The method name is both a phonetic abbreviation ("c"elf "x")
        // and an ASCII depiction of a loopy curve.

        // As Bézier curve geometry is invariant under affine transformations,
        // we can restrict ourselves to some canonical form of the curve.
        // Then, consider the coordinate polynomials
        // x(t) = at³+bt²+ct+p
        // y(t) = dt³+et²+ft+q
        // and parameters of self-intersection as λ and μ. By definition,
        // x(λ) - x(μ) = a(λ³-μ³)+b(λ²-μ²)+c(λ-μ) = 0
        // y(λ) - y(μ) = d(λ³-μ³)+e(λ²-μ²)+f(λ-μ) = 0
        // Dividing by the trivial solution λ = μ and expanding we get
        // a(λ²+λμ+μ²)+b(λ+μ)+c = 0
        // d(λ²+λμ+μ²)+e(λ+μ)+f = 0
        // In the canonical form chosen here
        // (https://pomax.github.io/bezierinfo/#canonical) we have
        // (x-3)(λ²+λμ+μ²)+3(λ+μ) = 0
        // y(λ²+λμ+μ²)-3(λ+μ)+3 = 0
        // whereby eliminating λ²+λμ+μ² gives (-3-3y/(x-3))(λ+μ) + 3 = 0
        // or λ+μ = (x-3)/(x+y-3), followed by λμ = (λ+μ)²+3/(x+y-3).
        // λ and μ can now be found by Viète's formulas.
        let vx = self.p2 - self.p1;
        let vy = self.p1 - self.p0;
        let vz = self.p3 - self.p0;
        let a = vx.x;
        let b = vy.x;
        let c = vx.y;
        let d = vy.y;

        let z1 = vz.x;
        let z2 = vz.y;

        let res = solve2x2(a, b, c, d, z1, z2);
        if res.is_none() {
            // No solution found, the curve is not self-intersecting.
            return BezierConfig::NoLoop;
        }
        let (x, y) = res.unwrap();
        // Check if the solution is within the bounds of the Bezier curve
        // Comparisons come from the Canonical form of the cubic Bezier curve
        // https://pomax.github.io/BezierInfo-2/#canonical
        if (x > 1.0)
            || (4.0 * y > (x + 1.0) * (3.0 - x))
            || (x > 0.0 && (2.0 * y + x < (3.0 * x * (4.0 - x)).sqrt())
                || (3.0 * y < x * (3.0 - x)))
        {
            // https://github.com/linebender/kurbo/blob/main/kurbo/src/cubicbez.rs#L442 detectCusip() for comparison

            if close_enough(4.0 * y, (x + 1.0) * (3.0 - x), CUSIP_THRESHOLD) {
                // TODO: adjust tolerance
                let rs = (x - 3.0) / (x + y - 3.0);
                // let rp = rs * rs + 3.0 / (x + y - 3.0);
                // let zz = rs * rs - 4.0 * rp; // close to zero
                let x1 = rs / 2.0;
                // Special case: cusip, where the curve touches itself at a single point
                return BezierConfig::Cusip(x1 - CUSIP_THRESHOLD, x1 + CUSIP_THRESHOLD);
            }
            return BezierConfig::NoLoop;
        }

        let rs = (x - 3.0) / (x + y - 3.0);
        let rp = rs * rs + 3.0 / (x + y - 3.0);
        let x1 = (rs - (rs * rs - 4.0 * rp).sqrt()) / 2.0;
        // sorted
        if x1 < rp / x1 {
            return BezierConfig::Loop(x1, rp / x1);
        } else {
            return BezierConfig::Loop(rp / x1, x1);
        }
    }
}

// Solution by inverting a 2x2 matrix equation Ax = b
// https://www.google.com/search?q=rust+Solve+a+linear+2x2+matrix+equation
fn solve2x2(a: f64, b: f64, c: f64, d: f64, z1: f64, z2: f64) -> Option<(f64, f64)> {
    // Solve a linear 2x2 matrix equation
    //let det = a * d - b * c;
    let det = diff_of_prod(a, d, b, c);
    if close_enough(det, 0.0, 10e-10) {
        return None; // No solution
    }

    // Inverse of matrix
    let ainv = [d / det, -b / det, -c / det, a / det];
    // solution to the matrix equation \(Ax=b\) is \(x=A^{-1}b\).
    let x = ainv[0] * z1 + ainv[1] * z2;
    let y = ainv[2] * z1 + ainv[3] * z2;
    Some((x, y))
}

#[cfg(test)]
mod test_bezier {
    use super::*;

    #[test]
    fn test_selfintersect_01() {
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(11.0, 10.0),
            Point::new(-1.0, 10.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(
            res,
            BezierConfig::Loop(0.3194212203713461, 0.6805787796286539)
        );
    }

    #[test]
    fn test_nointersect_02() {
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(7.0, 10.0),
            Point::new(3.0, 10.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::NoLoop);
    }

    #[test]
    fn test_selfintersect_03a() {
        // edge case, loop
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::Loop(0.5, 0.5));
    }

    #[test]
    fn test_selfintersect_03b() {
        // edge case, cusip
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0 + 1e-6, 10.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(
            res,
            BezierConfig::Cusip(0.5 - CUSIP_THRESHOLD, 0.5 + CUSIP_THRESHOLD)
        );
    }

    #[test]
    fn test_selfintersect_04() {
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(1e-10, 10.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(
            res,
            BezierConfig::Cusip(0.5 - CUSIP_THRESHOLD, 0.5 + CUSIP_THRESHOLD)
        );
    }

    #[test]
    fn test_selfintersect_05() {
        // edge case, cusip
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(13.0 + 1.0 / 3.0, 0.0),
            Point::new(0.0, 4.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(
            res,
            BezierConfig::Loop(0.6666666666666666, 0.6666666666666666)
        );
    }

    #[test]
    fn test_nointersect_06() {
        // edge case
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(20.0, -4.0),
            Point::new(0.0, 4.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::NoLoop);
    }

    #[test]
    fn test_nointersect_zerodet_07() {
        // edge case
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(-2.0, 0.0),
            Point::new(12.0, 0.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::NoLoop);
    }

    #[test]
    fn test_nointersect_zerodet_08() {
        // edge case
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(-2.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::NoLoop);
    }

    #[test]
    fn test_nointersect_zerodet_09() {
        // edge case
        let b = BezierC::new(
            Point::new(2.0, 0.0),
            Point::new(12.0, 0.0),
            Point::new(8.0, 0.0),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        assert_eq!(res, BezierConfig::NoLoop);
    }

    #[test]
    fn test_cusip_10() {
        // edge case
        let b = BezierC::new(
            Point::new(0.0, 0.0),
            Point::new(20.0, -10.0),
            // Point::new(0.0, 12.07106),
            Point::new(0.0, 12.071),
            Point::new(10.0, 0.0),
        );
        let res = b.self_intersection();
        let x = 0.7734605377276669;
        assert_eq!(
            res,
            BezierConfig::Cusip(x - CUSIP_THRESHOLD, x + CUSIP_THRESHOLD)
        );
    }
}

impl BezierC {
    #[inline]
    pub fn eval(&self, t: f64) -> Point {
        let mt = 1.0 - t;
        let v = self.p0 * (mt * mt * mt)
            + (self.p1 * (mt * mt * 3.0)
                + (self.p2 * (mt * 3.0) + self.p3 * t) * t)
                * t;
        v
    }

    #[inline]
    pub fn deriv(&self) -> BezierQ {
        BezierQ::new(
            (self.p1 - self.p0) * 3.0,
            (self.p2 - self.p1) * 3.0,
            (self.p3 - self.p2) * 3.0,
        )
    }

    pub fn subsegment(&self, range: Range<f64>) -> BezierC {
        let (t0, t1) = (range.start, range.end);
        let p0 = self.eval(t0);
        let p3 = self.eval(t1);
        let d = self.deriv();
        let scale = (t1 - t0) * (1.0 / 3.0);
        let p1 = p0 + d.eval(t0) * scale;
        let p2 = p3 - d.eval(t1) * scale;
        BezierC { p0, p1, p2, p3 }
    }
}
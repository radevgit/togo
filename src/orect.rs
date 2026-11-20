#![allow(dead_code)]

use crate::point::{Point, point};
use std::fmt::Display;

/// An oriented (rotated) rectangle defined by a center, a unit direction and two half-sizes.
///
/// `dir` is the unit vector along the "length" axis (for a robot: its heading).
/// The "width" axis is `dir.perp`, i.e. `dir` rotated 90° counter-clockwise.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// // 2.0 long, 1.0 wide, centered at (5, 5), heading along +x
/// let r = OrientedRect::new(point(5.0, 5.0), point(1.0, 0.0), 1.0, 0.5);
/// let c = r.corners();
/// assert_eq!(c[0], point(4.0, 4.5));
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OrientedRect {
    /// Center of the rectangle
    pub c: Point,
    /// Unit vector along the length axis
    pub dir: Point,
    /// Half of the size along `dir`
    pub half_len: f64,
    /// Half of the size along the perpendicular of `dir`
    pub half_wid: f64,
}

impl Display for OrientedRect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[c={}, dir={}, half_len={}, half_wid={}]",
            self.c, self.dir, self.half_len, self.half_wid
        )
    }
}

impl OrientedRect {
    /// Creates an oriented rectangle from a center, a **unit** direction and half-sizes.
    ///
    /// The caller is responsible for `dir` being normalized; this is not checked.
    #[inline]
    pub fn new(c: Point, dir: Point, half_len: f64, half_wid: f64) -> Self {
        OrientedRect { c, dir, half_len, half_wid }
    }

    /// Creates an oriented rectangle from a center and a heading angle in radians
    /// (counter-clockwise from the +x axis).
    ///
    /// # Examples
    ///
    /// ```
    /// use togo::prelude::*;
    ///
    /// let r = OrientedRect::from_angle(point(0.0, 0.0), std::f64::consts::FRAC_PI_2, 1.0, 0.5);
    /// assert!((r.dir.x).abs() < 1e-15 && (r.dir.y - 1.0).abs() < 1e-15);
    /// ```
    #[inline]
    pub fn from_angle(c: Point, angle: f64, half_len: f64, half_wid: f64) -> Self {
        let (s, co) = angle.sin_cos();
        OrientedRect { c, dir: point(co, s), half_len, half_wid }
    }

    /// The unit vector along the width axis (`dir` rotated 90° counter-clockwise).
    #[inline]
    pub fn normal(&self) -> Point {
        point(-self.dir.y, self.dir.x)
    }

    /// The four corners in counter-clockwise order, starting at the
    /// (-length, -width) corner of the local frame.
    pub fn corners(&self) -> [Point; 4] {
        let u = self.dir * self.half_len;
        let n = self.normal() * self.half_wid;
        [
            self.c - u - n,
            self.c + u - n,
            self.c + u + n,
            self.c - u + n,
        ]
    }

    /// Converts a world point to the rectangle's local frame
    /// (origin at the center, x along `dir`, y along `normal()`).
    #[inline]
    pub fn to_local(&self, p: Point) -> Point {
        let d = p - self.c;
        point(d.dot(self.dir), d.dot(self.normal()))
    }

    /// Converts a point in the rectangle's local frame back to world coordinates.
    #[inline]
    pub fn from_local(&self, q: Point) -> Point {
        self.c + self.dir * q.x + self.normal() * q.y
    }

    /// Returns true if the point is inside the rectangle or on its boundary.
    ///
    /// Touching counts as inside (closed rectangle).
    ///
    /// # Examples
    ///
    /// ```
    /// use togo::prelude::*;
    ///
    /// let r = OrientedRect::new(point(0.0, 0.0), point(1.0, 0.0), 2.0, 1.0);
    /// assert!(r.contains_point(point(2.0, 1.0))); // corner
    /// assert!(!r.contains_point(point(2.1, 0.0)));
    /// ```
    #[inline]
    pub fn contains_point(&self, p: Point) -> bool {
        let q = self.to_local(p);
        q.x.abs() <= self.half_len && q.y.abs() <= self.half_wid
    }

}

/// Creates an oriented rectangle from a center, a unit direction and half-sizes.
///
/// Equivalent to `OrientedRect::new(c, dir, half_len, half_wid)`.
#[inline]
pub fn orect(c: Point, dir: Point, half_len: f64, half_wid: f64) -> OrientedRect {
    OrientedRect::new(c, dir, half_len, half_wid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

    fn close(a: Point, b: Point) -> bool {
        a.close_enough(b, 1e-12)
    }

    #[test]
    fn corners_axis_aligned() {
        let r = orect(point(5.0, 5.0), point(1.0, 0.0), 1.0, 0.5);
        let c = r.corners();
        assert!(close(c[0], point(4.0, 4.5)));
        assert!(close(c[1], point(6.0, 4.5)));
        assert!(close(c[2], point(6.0, 5.5)));
        assert!(close(c[3], point(4.0, 5.5)));
    }

    #[test]
    fn corners_rotated_90() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), FRAC_PI_2, 2.0, 1.0);
        let c = r.corners();
        // length now along +y, width along -x
        assert!(close(c[0], point(1.0, -2.0)));
        assert!(close(c[1], point(1.0, 2.0)));
        assert!(close(c[2], point(-1.0, 2.0)));
        assert!(close(c[3], point(-1.0, -2.0)));
    }

    #[test]
    fn local_frame_round_trip() {
        let r = OrientedRect::from_angle(point(3.0, -2.0), 0.7, 1.0, 0.5);
        for p in [point(0.0, 0.0), point(3.0, -2.0), point(-4.5, 8.25)] {
            assert!(close(r.from_local(r.to_local(p)), p));
        }
        // local axes map to dir / normal
        assert!(close(r.to_local(r.c + r.dir), point(1.0, 0.0)));
        assert!(close(r.to_local(r.c + r.normal()), point(0.0, 1.0)));
    }

    #[test]
    fn contains_point_axis_aligned() {
        let r = orect(point(0.0, 0.0), point(1.0, 0.0), 2.0, 1.0);
        assert!(r.contains_point(point(0.0, 0.0)));
        assert!(r.contains_point(point(1.9, 0.9)));
        assert!(r.contains_point(point(2.0, 0.0))); // edge
        assert!(r.contains_point(point(-2.0, -1.0))); // corner
        assert!(!r.contains_point(point(2.0001, 0.0)));
        assert!(!r.contains_point(point(0.0, -1.0001)));
    }

    #[test]
    fn contains_point_45_degrees() {
        let r = OrientedRect::from_angle(point(0.0, 0.0), FRAC_PI_4, 2.0, 1.0);
        // inside the bounding rect but outside the rotated rectangle
        let b = crate::algo::orect_bounding_rect(&r);
        assert!(!r.contains_point(point(b.p2.x - 0.01, b.p1.y + 0.01)));
        assert!(r.contains_point(point(0.0, 0.0)));
        // a point along dir inside, and just beyond
        assert!(r.contains_point(r.c + r.dir * 1.99));
        assert!(!r.contains_point(r.c + r.dir * 2.01));
        // every corner nudged toward the center is inside, nudged outward is outside
        for p in r.corners() {
            assert!(r.contains_point(r.c + (p - r.c) * 0.999));
            assert!(!r.contains_point(r.c + (p - r.c) * 1.001));
        }
    }

    #[test]
    fn contains_point_degenerate() {
        let r = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        assert!(r.contains_point(point(1.0, 1.0)));
        assert!(!r.contains_point(point(1.0, 1.001)));
    }

    #[test]
    fn degenerate_zero_size() {
        let r = orect(point(1.0, 1.0), point(0.6, 0.8), 0.0, 0.0);
        for p in r.corners() {
            assert!(close(p, point(1.0, 1.0)));
        }
    }
}

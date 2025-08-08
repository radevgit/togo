#![allow(dead_code)]
#![deny(unused_results)]

use robust::{Coord, orient2d};

pub use crate::utils::almost_equal_as_int;
use crate::utils::{diff_of_prod, sum_of_prod};
use std::fmt::Display;
use std::ops;
use std::ops::{Div, Mul, Neg};

const ZERO: f64 = 0f64;

/// A 2D point with double precision floating point coordinates.
///
/// This is a fundamental data type used throughout the offsetting library
/// to represent positions and vectors in 2D space.
///
/// # Examples
///
/// ```
/// use base_geom::prelude::*;
///
/// let p1 = point(3.0, 4.0);
/// let p2 = point(1.0, 2.0);
/// let sum = p1 + p2; // Point arithmetic is supported
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate  
    pub y: f64,
}

impl Point {
    #[doc(hidden)]
    /// Creates a new point with the given coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate
    /// * `y` - The y coordinate
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.x, 3.0);
    /// assert_eq!(p.y, 4.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

/// Creates a new point with the given coordinates.
///
/// This is a convenience function equivalent to `Point::new(x, y)`.
///
/// # Arguments
///
/// * `x` - The x coordinate
/// * `y` - The y coordinate
///
/// # Examples
///
/// ```
/// use base_geom::prelude::*;
///
/// let p = point(3.0, 4.0);
/// assert_eq!(p.x, 3.0);
/// assert_eq!(p.y, 4.0);
/// ```
#[inline]
pub fn point(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.20}, {:.20}]", self.x, self.y)
    }
}

// #00027
// This approach uses a macro to reduce code duplication.
macro_rules! ImplBinaryOp {
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl ops::$op_trait<Point> for Point {
            type Output = Point;
            #[inline]
            fn $op_func(self, rhs: Point) -> Self::Output {
                Point::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl ops::$op_trait<&Point> for Point {
            type Output = Point;
            #[inline]
            fn $op_func(self, rhs: &Point) -> Self::Output {
                Point::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl ops::$op_trait<Point> for &Point {
            type Output = Point;
            #[inline]
            fn $op_func(self, rhs: Point) -> Self::Output {
                Point::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl<'a, 'b> ops::$op_trait<&'b Point> for &'a Point {
            type Output = Point;
            #[inline]
            fn $op_func(self, _rhs: &'b Point) -> Self::Output {
                Point::new(self.x $op _rhs.x, self.y $op _rhs.y)
            }
        }

    };
}

ImplBinaryOp!(Add, add, +);
ImplBinaryOp!(Sub, sub, -);

impl Neg for Point {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    #[inline]
    fn mul(self, num: f64) -> Self::Output {
        Self {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;
    #[inline]
    fn div(self, num: f64) -> Self::Output {
        Self {
            x: self.x / num,
            y: self.y / num,
        }
    }
}

impl Point {
    #[doc(hidden)]
    /// Computes the dot product of this point with another point.
    ///
    /// Uses improved precision via the Kahan summation method.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point
    ///
    /// # Returns
    ///
    /// The dot product as a f64
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p1 = point(3.0, 4.0);
    /// let p2 = point(1.0, 2.0);
    /// let dot = p1.dot(p2); // 3*1 + 4*2 = 11.0
    /// ```
    #[inline]
    // improved dot using Kahan method
    pub fn dot(&self, other: Self) -> f64 {
        sum_of_prod(self.x, other.x, self.y, other.y)
    }

    // #[inline]
    // pub fn perp(&self, other: Self) -> f64 {
    //     self.x * other.y - self.y * other.x
    // }

    #[doc(hidden)]
    // Improved perp using Kahan method
    #[inline]
    pub fn perp(&self, other: Self) -> f64 {
        diff_of_prod(self.x, other.y, self.y, other.x)
    }

    #[doc(hidden)]
    /// Computes the Euclidean norm (magnitude) of this point when treated as a vector.
    ///
    /// # Returns
    ///
    /// The magnitude as a f64
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p = point(3.0, 4.0);
    /// let magnitude = p.norm(); // sqrt(3² + 4²) = 5.0
    /// ```
    #[inline]
    pub fn norm(&self) -> f64 {
        (self.dot(*self)).sqrt()
    }

    /// Normalizes this point to unit length and returns both the normalized point and original magnitude.
    ///
    /// The function uses robust computation to handle edge cases with very small or zero vectors.
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// * The normalized point (unit vector)
    /// * The original magnitude
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p = point(3.0, 4.0);
    /// let (normalized, magnitude) = p.normalize();
    /// // normalized will be approximately (0.6, 0.8)
    /// // magnitude will be 5.0
    /// ```
    #[inline]
    pub fn normalize(&self) -> (Point, f64) {
        let robust = false;
        if robust {
            let mut max_abs_comp = self.x.abs();
            let abs_comp = self.y.abs();
            if abs_comp > max_abs_comp {
                max_abs_comp = abs_comp;
            }

            let mut v = *self;
            if max_abs_comp > ZERO {
                v = v / max_abs_comp;
                let mut norm = v.norm();
                v = v / norm;
                norm = norm * max_abs_comp;
                (v, norm)
            } else {
                (point(ZERO, ZERO), ZERO)
            }
        } else {
            let norm = self.norm();
            let normalized = if norm > 0f64 {
                point(self.x / norm, self.y / norm)
            } else {
                point(0.0, 0.0)
            };
            (normalized, norm)
        }
    }

    /// Checks if this point is almost equal to another point within a given ULP tolerance.
    ///
    /// Uses ULP (Units in the Last Place) comparison for floating point equality.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to compare with
    /// * `ulp` - The ULP tolerance for comparison
    ///
    /// # Returns
    ///
    /// True if the points are almost equal within the tolerance
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p1 = point(1.0, 2.0);
    /// let p2 = point(1.0000001, 2.0000001);
    /// let almost_equal = p1.almost_eq(p2, 10);
    /// ```
    /// Almost equal comparison with another Point using `ulp` given.
    #[inline]
    pub fn almost_eq(&self, other: Self, ulp: i64) -> bool {
        almost_equal_as_int(self.x, other.x, ulp) && almost_equal_as_int(self.y, other.y, ulp)
    }

    /// Checks if this point is close enough to another point within an epsilon tolerance.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point to compare with  
    /// * `eps` - The epsilon tolerance for comparison
    ///
    /// # Returns
    ///
    /// True if the points are within epsilon distance in both x and y coordinates
    ///
    /// # Examples
    ///
    /// ```
    /// use base_geom::prelude::*;
    ///
    /// let p1 = point(1.0, 2.0);
    /// let p2 = point(1.001, 2.001);
    /// let close = p1.close_enough(p2, 0.01);
    /// ```
    #[inline]
    pub fn close_enough(&self, other: Self, eps: f64) -> bool {
        return (self.x - other.x).abs() < eps && (self.y - other.y).abs() < eps;
    }

    // diff_of_prod for points
    #[inline]
    pub fn diff_of_prod(&self, a: f64, other: Point, b: f64) -> Point {
        Point {
            x: diff_of_prod(self.x, a, other.x, b),
            y: diff_of_prod(self.y, a, other.y, b),
        }
    }

    // sum_of_prod for points
    #[inline]
    pub fn sum_of_prod(&self, a: f64, other: Point, b: f64) -> Point {
        Point {
            x: sum_of_prod(self.x, a, other.x, b),
            y: sum_of_prod(self.y, a, other.y, b),
        }
    }

    /// Linearly interpolate between two vectors.
    #[inline]
    pub fn lerp(self, other: Point, t: f64) -> Point {
        self + (other - self) * t
    }

    /// Sorts four points in parallel.
    ///
    /// This function sorts four points that are expected to be collinear.
    /// Sort 4 points on a line (from two segments), using sort networks
    /// Ascending or descending order is not important
    pub fn sort_parallel_points(
        a: Point,
        b: Point,
        c: Point,
        d: Point,
    ) -> (Point, Point, Point, Point) {
        let p0 = Coord { x: a.x, y: a.y };
        let p1 = Coord { x: b.x, y: b.y };
        let p2 = Coord { x: c.x, y: c.y };
        let p3 = Coord { x: d.x, y: d.y };
        let mut tt = (p0, p1, p2, p3);
        let diff0 = a - b;
        let diff1 = c - d;
        // create perpendicular segment to order points
        let perp = if diff0.dot(diff0).abs() >= diff1.dot(diff1).abs() {
            point(diff0.y, -diff0.x)
        } else {
            point(diff1.y, -diff1.x)
        };
        let t0 = Coord {
            x: perp.x,
            y: perp.y,
        };
        if orient2d(t0, tt.1, tt.3) < 0.0 {
            tt = (tt.0, tt.3, tt.2, tt.1)
        }
        if orient2d(t0, tt.0, tt.2) < 0.0 {
            tt = (tt.2, tt.1, tt.0, tt.3)
        }
        if orient2d(t0, tt.0, tt.1) < 0.0 {
            tt = (tt.1, tt.0, tt.2, tt.3)
        }
        if orient2d(t0, tt.2, tt.3) < 0.0 {
            tt = (tt.0, tt.1, tt.3, tt.2)
        }
        if orient2d(t0, tt.1, tt.2) < 0.0 {
            tt = (tt.0, tt.2, tt.1, tt.3)
        }
        let e = point(tt.0.x, tt.0.y);
        let f = point(tt.1.x, tt.1.y);
        let g = point(tt.2.x, tt.2.y);
        let h = point(tt.3.x, tt.3.y);
        (e, f, g, h)
    }
}

#[cfg(test)]
mod test_binary_op {
    use super::*;

    macro_rules! test_binary_op {
        ($v1:ident, $v2:ident, $op:tt, $expected:expr) => {
            assert!(($v1 $op $v2).almost_eq($expected, 10));
            assert!((&$v1 $op $v2).almost_eq($expected, 10));
            assert!(($v1 $op &$v2).almost_eq($expected, 10));
            assert!((&$v1 $op &$v2).almost_eq($expected, 10));
        };
    }

    macro_rules! test_num_op {
        ($v1:ident, $v2:ident, $op:tt, $expected:expr) => {
            assert!(($v1 $op $v2).almost_eq($expected, 10));
        };
    }

    #[test]
    fn test_ops() {
        let v1 = point(5.0, 5.0);
        let v2 = point(1.0, 2.0);
        let s = 2.0f64;
        test_binary_op!(v1, v2, +, point(6.0, 7.0));
        test_binary_op!(v1, v2, -, point(4.0, 3.0));
        test_num_op!(v1, s, *, point(10.0, 10.0));
        test_num_op!(v2, s, /, point(0.5, 1.0));
    }

    #[test]
    fn test_neg() {
        let p1 = point(1.0, 3.0);
        let p2 = point(-1.0, -3.0);
        assert_eq!(-p1, p2);
    }
}

#[cfg(test)]
mod test_point {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_new() {
        let point0 = Point::new(1.0, 2.0);
        let point1 = point(1.0, 2.0);
        assert_eq!(point0, point1);
    }

    #[test]
    fn test_norm() {
        let p = point(1.0, 1.0);
        let e = p.norm();
        assert_eq!(e, 1.4142135623730951);
    }

    #[test]
    fn test_display() {
        let p = point(1.0, 2.0);
        //print!("{}", p);
        assert_eq!(
            "[1.00000000000000000000, 2.00000000000000000000]",
            format!("{}", p)
        );
    }

    #[test]
    fn test_sort_parallel_points_01() {
        let a = point(1.0, 1.0);
        let b = point(3.0, 3.0);
        let c = point(2.0, 2.0);
        let d = point(4.0, 4.0);
        let (e, f, g, h) = Point::sort_parallel_points(a, b, c, d);
        assert_eq!(e, a);
        assert_eq!(f, c);
        assert_eq!(g, b);
        assert_eq!(h, d);
    }

    #[test]
    fn test_sort_parallel_points_02() {
        let a = point(1.0, 1.0);
        let b = point(3.0, 3.0);
        let c = point(4.0, 4.0);
        let d = point(2.0, 2.0);
        let (e, f, g, h) = Point::sort_parallel_points(a, b, c, d);
        assert_eq!(e, a);
        assert_eq!(f, d);
        assert_eq!(g, b);
        assert_eq!(h, c);
    }

    #[test]
    fn test_sort_parallel_points_03() {
        let a = point(1.0, 1.0);
        let b = point(2.0, 2.0);
        let c = point(4.0, 4.0);
        let d = point(-1.0, -1.0);
        let (e, f, g, h) = Point::sort_parallel_points(a, b, c, d);
        assert_eq!(e, c);
        assert_eq!(f, b);
        assert_eq!(g, a);
        assert_eq!(h, d);
    }
}

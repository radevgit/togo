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
/// This is a fundamental data type used throughout the basegeom library
/// to represent positions and vectors in 2D space.
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
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

pub type Pointline = Vec<Point>;

impl Point {
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
    /// use basegeom::prelude::*;
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
/// use basegeom::prelude::*;
///
/// let p = point(3.0, 4.0);
/// assert_eq!(p.x, 3.0);
/// assert_eq!(p.y, 4.0);
/// ```
#[inline]
pub fn point(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

/// Computes the 2D orientation test for three points.
///
/// This function determines the orientation of point `p` relative to the directed
/// line from point `a` to point `b`. It uses the robust `orient2d` predicate
/// which computes the sign of the determinant:
///
/// ```text
/// | ax  ay  1 |
/// | bx  by  1 |  
/// | px  py  1 |
/// ```
///
/// This is equivalent to twice the signed area of the triangle formed by the three points.
///
/// # Returns
///
/// * Positive value: `p` is to the left of the directed line from `a` to `b`
/// * Negative value: `p` is to the right of the directed line from `a` to `b`  
/// * Zero: The three points are collinear
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// let a = point(0.0, 0.0);
/// let b = point(1.0, 0.0);
///
/// // Point to the left of the line (positive orientation)
/// let p_left = point(0.5, 1.0);
/// assert!(points_order(a, b, p_left) > 0.0);
///
/// // Point to the right of the line (negative orientation)  
/// let p_right = point(0.5, -1.0);
/// assert!(points_order(a, b, p_right) < 0.0);
///
/// // Collinear point (zero orientation)
/// let p_collinear = point(0.5, 0.0);
/// assert_eq!(points_order(a, b, p_collinear), 0.0);
/// ```
pub fn points_order(a: Point, b: Point, p: Point) -> f64 {
    let pa = Coord { x: a.x, y: a.y };
    let pb = Coord { x: b.x, y: b.y };
    let pp = Coord { x: p.x, y: p.y };
    orient2d(pa, pb, pp)
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


/// Implements negation of a point.
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

/// Implements multiplication of a point by a scalar.
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

/// Implements division of a point by a scalar.
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
    /// use basegeom::prelude::*;
    ///
    /// let p1 = point(3.0, 4.0);
    /// let p2 = point(1.0, 2.0);
    /// let dot = p1.dot(p2); // 3*1 + 4*2 = 11.0
    /// ```
    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        sum_of_prod(self.x, other.x, self.y, other.y)
    }

    // #[inline]
    // pub fn perp(&self, other: Self) -> f64 {
    //     self.x * other.y - self.y * other.x
    // }

    /// Computes the perp product (cross product) of this point with another point.
    ///
    /// # Arguments
    ///
    /// * `other` - The other point
    ///
    /// # Returns
    ///
    /// The perp product as a f64
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    ///
    /// let p1 = point(3.0, 4.0);
    /// let p2 = point(1.0, 2.0);
    /// let perp = p1.perp(p2); // 3*2 - 4*1 = 2.0
    /// ```
    #[inline]
    pub fn perp(&self, other: Self) -> f64 {
        diff_of_prod(self.x, other.y, self.y, other.x)
    }

    /// Computes the Euclidean norm (magnitude) of this point when treated as a vector.
    ///
    /// # Returns
    ///
    /// The magnitude as a f64
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
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
    /// use basegeom::prelude::*;
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
    /// use basegeom::prelude::*;
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
    /// use basegeom::prelude::*;
    ///
    /// let p1 = point(1.0, 2.0);
    /// let p2 = point(1.001, 2.001);
    /// let close = p1.close_enough(p2, 0.01);
    /// ```
    #[inline]
    pub fn close_enough(&self, other: Self, eps: f64) -> bool {
        return (self.x - other.x).abs() <= eps && (self.y - other.y).abs() <= eps;
    }

    // /// diff_of_prod for points
    // #[inline]
    // pub fn diff_of_prod(&self, a: f64, other: Point, b: f64) -> Point {
    //     Point {
    //         x: diff_of_prod(self.x, a, other.x, b),
    //         y: diff_of_prod(self.y, a, other.y, b),
    //     }
    // }

    // /// sum_of_prod for points
    // #[inline]
    // pub fn sum_of_prod(&self, a: f64, other: Point, b: f64) -> Point {
    //     Point {
    //         x: sum_of_prod(self.x, a, other.x, b),
    //         y: sum_of_prod(self.y, a, other.y, b),
    //     }
    // }

    /// Linearly interpolate between two points.
    #[inline]
    pub fn lerp(self, other: Point, t: f64) -> Point {
        self + (other - self) * t
    }

    /// Sorts four collinear points.
    ///
    /// This function sorts four points that are expected to be collinear,
    /// and are usually a result of intersection of two collinear segments overlapping.
    /// 
    /// Sort using sort networks.
    /// Ascending or descending order is not important.
    pub fn sort_colinear_points(
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
        let (e, f, g, h) = Point::sort_colinear_points(a, b, c, d);
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
        let (e, f, g, h) = Point::sort_colinear_points(a, b, c, d);
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
        let (e, f, g, h) = Point::sort_colinear_points(a, b, c, d);
        assert_eq!(e, c);
        assert_eq!(f, b);
        assert_eq!(g, a);
        assert_eq!(h, d);
    }

    #[test]
    fn test_dot_product() {
        // Basic dot product
        let p1 = point(3.0, 4.0);
        let p2 = point(1.0, 2.0);
        assert_eq!(p1.dot(p2), 11.0); // 3*1 + 4*2 = 11

        // Orthogonal vectors (dot product should be 0)
        let p3 = point(1.0, 0.0);
        let p4 = point(0.0, 1.0);
        assert_eq!(p3.dot(p4), 0.0);

        // Dot product with self (magnitude squared)
        let p5 = point(3.0, 4.0);
        assert_eq!(p5.dot(p5), 25.0); // 3² + 4² = 25

        // Zero vector
        let zero = point(0.0, 0.0);
        let p6 = point(5.0, 7.0);
        assert_eq!(zero.dot(p6), 0.0);
        assert_eq!(p6.dot(zero), 0.0);

        // Negative values
        let p7 = point(-2.0, 3.0);
        let p8 = point(4.0, -1.0);
        assert_eq!(p7.dot(p8), -11.0); // -2*4 + 3*(-1) = -8 - 3 = -11
    }

    #[test]
    fn test_perp_product() {
        // Basic perpendicular product (cross product in 2D)
        let p1 = point(3.0, 4.0);
        let p2 = point(1.0, 2.0);
        assert_eq!(p1.perp(p2), 2.0); // 3*2 - 4*1 = 6 - 4 = 2

        // Parallel vectors (perp product should be 0)
        let p3 = point(2.0, 4.0);
        let p4 = point(1.0, 2.0);
        assert_eq!(p3.perp(p4), 0.0);

        // Anti-parallel vectors
        let p5 = point(1.0, 2.0);
        let p6 = point(-2.0, -4.0);
        assert_eq!(p5.perp(p6), 0.0);

        // Perpendicular vectors
        let p7 = point(1.0, 0.0);
        let p8 = point(0.0, 1.0);
        assert_eq!(p7.perp(p8), 1.0);
        assert_eq!(p8.perp(p7), -1.0);

        // Zero vector
        let zero = point(0.0, 0.0);
        let p9 = point(5.0, 7.0);
        assert_eq!(zero.perp(p9), 0.0);
        assert_eq!(p9.perp(zero), 0.0);
    }

    #[test]
    fn test_norm_magnitude() {
        // Basic magnitude calculation
        let p1 = point(3.0, 4.0);
        assert_eq!(p1.norm(), 5.0); // sqrt(3² + 4²) = 5

        // Unit vectors
        let unit_x = point(1.0, 0.0);
        let unit_y = point(0.0, 1.0);
        assert_eq!(unit_x.norm(), 1.0);
        assert_eq!(unit_y.norm(), 1.0);

        // Zero vector
        let zero = point(0.0, 0.0);
        assert_eq!(zero.norm(), 0.0);

        // Negative coordinates
        let p2 = point(-3.0, -4.0);
        assert_eq!(p2.norm(), 5.0);

        // Mixed sign coordinates
        let p3 = point(-3.0, 4.0);
        let p4 = point(3.0, -4.0);
        assert_eq!(p3.norm(), 5.0);
        assert_eq!(p4.norm(), 5.0);

        // Very small values
        let tiny = point(1e-10, 1e-10);
        assert!((tiny.norm() - std::f64::consts::SQRT_2 * 1e-10).abs() < 1e-15);

        // Very large values
        let large = point(1e10, 1e10);
        assert!((large.norm() - std::f64::consts::SQRT_2 * 1e10).abs() < 1e5);
    }

    #[test]
    fn test_normalize() {
        // Basic normalization
        let p1 = point(3.0, 4.0);
        let (normalized, magnitude) = p1.normalize();
        assert_eq!(magnitude, 5.0);
        assert!((normalized.norm() - 1.0).abs() < 1e-15);
        assert!((normalized.x - 0.6).abs() < 1e-15);
        assert!((normalized.y - 0.8).abs() < 1e-15);

        // Unit vector should remain unit
        let unit = point(1.0, 0.0);
        let (norm_unit, mag) = unit.normalize();
        assert_eq!(mag, 1.0);
        assert_eq!(norm_unit, unit);

        // Zero vector edge case
        let zero = point(0.0, 0.0);
        let (norm_zero, mag_zero) = zero.normalize();
        assert_eq!(mag_zero, 0.0);
        // Normalized zero should be zero (implementation detail)
        assert!(norm_zero.x.is_finite());
        assert!(norm_zero.y.is_finite());

        // Very small vector
        let tiny = point(1e-100, 1e-100);
        let (norm_tiny, mag_tiny) = tiny.normalize();
        assert!((mag_tiny - std::f64::consts::SQRT_2 * 1e-100).abs() < 1e-115);
        assert!((norm_tiny.norm() - 1.0).abs() < 1e-10); // May have some numerical error

        // Negative values
        let p2 = point(-6.0, -8.0);
        let (norm_p2, mag_p2) = p2.normalize();
        assert_eq!(mag_p2, 10.0);
        assert!((norm_p2.x - (-0.6)).abs() < 1e-15);
        assert!((norm_p2.y - (-0.8)).abs() < 1e-15);
    }

    #[test]
    fn test_almost_eq() {
        // Exactly equal points
        let p1 = point(1.0, 2.0);
        let p2 = point(1.0, 2.0);
        assert!(p1.almost_eq(p2, 0));

        // Very close points (should need some ULP tolerance)
        let p3 = point(1.0, 2.0);
        let p4 = point(1.0 + f64::EPSILON, 2.0 + f64::EPSILON); // Much smaller difference
        assert!(p3.almost_eq(p4, 1)); // 1 ULP should be enough for EPSILON difference

        // Different points
        let p5 = point(1.0, 2.0);
        let p6 = point(1.1, 2.1);
        assert!(!p5.almost_eq(p6, 10));

        // Zero points
        let zero1 = point(0.0, 0.0);
        let zero2 = point(0.0, 0.0);
        assert!(zero1.almost_eq(zero2, 0));

        // Negative zero vs positive zero
        let neg_zero = point(-0.0, -0.0);
        let pos_zero = point(0.0, 0.0);
        assert!(neg_zero.almost_eq(pos_zero, 0));

        // Large values with minimal difference
        let big1 = point(1e10, 1e10);
        let big2 = point(1e10 * (1.0 + f64::EPSILON), 1e10 * (1.0 + f64::EPSILON));
        assert!(big1.almost_eq(big2, 100)); // ULP tolerance needed for large values

        // One coordinate different by EPSILON
        let p7 = point(1.0, 2.0);
        let p8 = point(1.0 + f64::EPSILON, 2.0);
        assert!(p7.almost_eq(p8, 1));

        let p9 = point(1.0, 2.0);
        let p10 = point(1.0, 2.0 + f64::EPSILON);
        assert!(p9.almost_eq(p10, 1));

        // Test different signs - should only be equal if both are zero
        let pos = point(1.0, 1.0);
        let neg = point(-1.0, -1.0);
        assert!(!pos.almost_eq(neg, 1000));
    }

    #[test]
    fn test_close_enough() {
        // Exactly equal points
        let p1 = point(1.0, 2.0);
        let p2 = point(1.0, 2.0);
        assert!(p1.close_enough(p2, 0.0));

        // Points within epsilon (both coordinates must be within epsilon)
        let p3 = point(1.0, 2.0);
        let p4 = point(1.005, 2.005); // Both coordinates are 0.005 different
        assert!(p3.close_enough(p4, 0.006)); // eps > 0.005
        assert!(!p3.close_enough(p4, 0.004)); // eps < 0.005

        // Points outside epsilon
        let p5 = point(1.0, 2.0);
        let p6 = point(1.1, 2.1); // Both coordinates are 0.1 different
        assert!(!p5.close_enough(p6, 0.05));
        assert!(p5.close_enough(p6, 0.15));

        // Zero tolerance - requires exact equality
        let p7 = point(1.0, 2.0);
        let p8 = point(1.0, 2.0);
        assert!(p7.close_enough(p8, f64::EPSILON)); // Use smallest epsilon instead of 0.0
        
        let p9 = point(1.0, 2.0);
        let p10 = point(1.001, 2.001);
        assert!(!p9.close_enough(p10, 0.0));

        // Negative coordinates
        let p11 = point(-1.0, -2.0);
        let p12 = point(-1.005, -2.005);
        assert!(p11.close_enough(p12, 0.01));

        // Mixed signs
        let p13 = point(1.0, -2.0);
        let p14 = point(1.005, -2.005);
        assert!(p13.close_enough(p14, 0.01));

        // Large epsilon
        let p15 = point(0.0, 0.0);
        let p16 = point(5.0, 5.0);
        assert!(p15.close_enough(p16, 10.0));

        // One coordinate within, one outside - should fail
        let p17 = point(1.0, 2.0);
        let p18 = point(1.005, 2.05); // x within 0.01, y outside 0.01
        assert!(!p17.close_enough(p18, 0.01));

        // Boundary case - exactly at epsilon
        let p19 = point(1.0, 2.0);
        let p20 = point(1.01, 2.01); // exactly 0.01 difference
        assert!(!p19.close_enough(p20, 0.01)); // < epsilon, not <= epsilon
        assert!(p19.close_enough(p20, 0.011)); // > epsilon
    }

    // #[test]
    // fn test_diff_of_prod() {
    //     let p1 = point(2.0, 3.0);
    //     let p2 = point(4.0, 5.0);
    //     let a = 2.0;
    //     let b = 3.0;
        
    //     let result = p1.diff_of_prod(a, p2, b);
    //     // Expected: Point(p1.x * a - p2.x * b, p1.y * a - p2.y * b)
    //     // = Point(2.0 * 2.0 - 4.0 * 3.0, 3.0 * 2.0 - 5.0 * 3.0)
    //     // = Point(4.0 - 12.0, 6.0 - 15.0)
    //     // = Point(-8.0, -9.0)
    //     assert_eq!(result, point(-8.0, -9.0));

    //     // Test with zero
    //     let zero = point(0.0, 0.0);
    //     let p3 = point(1.0, 1.0);
    //     let result_zero = zero.diff_of_prod(1.0, p3, 1.0);
    //     assert_eq!(result_zero, point(-1.0, -1.0));

    //     // Test with identity
    //     let p4 = point(5.0, 7.0);
    //     let result_identity = p4.diff_of_prod(1.0, zero, 0.0);
    //     assert_eq!(result_identity, p4);
    // }

    // #[test]
    // fn test_sum_of_prod() {
    //     let p1 = point(2.0, 3.0);
    //     let p2 = point(4.0, 5.0);
    //     let a = 2.0;
    //     let b = 3.0;
        
    //     let result = p1.sum_of_prod(a, p2, b);
    //     // Expected: Point(p1.x * a + p2.x * b, p1.y * a + p2.y * b)
    //     // = Point(2.0 * 2.0 + 4.0 * 3.0, 3.0 * 2.0 + 5.0 * 3.0)
    //     // = Point(4.0 + 12.0, 6.0 + 15.0)
    //     // = Point(16.0, 21.0)
    //     assert_eq!(result, point(16.0, 21.0));

    //     // Test with zero
    //     let zero = point(0.0, 0.0);
    //     let p3 = point(1.0, 1.0);
    //     let result_zero = zero.sum_of_prod(1.0, p3, 1.0);
    //     assert_eq!(result_zero, point(1.0, 1.0));

    //     // Test with identity
    //     let p4 = point(5.0, 7.0);
    //     let result_identity = p4.sum_of_prod(1.0, zero, 0.0);
    //     assert_eq!(result_identity, p4);

    //     // Test weighted average-like operation
    //     let p5 = point(0.0, 0.0);
    //     let p6 = point(10.0, 20.0);
    //     let result_weighted = p5.sum_of_prod(0.3, p6, 0.7);
    //     assert_eq!(result_weighted, point(7.0, 14.0));
    // }

    #[test]
    fn test_lerp() {
        // Basic interpolation
        let p1 = point(0.0, 0.0);
        let p2 = point(10.0, 20.0);
        
        // At t=0, should return p1
        assert_eq!(p1.lerp(p2, 0.0), p1);
        
        // At t=1, should return p2
        assert_eq!(p1.lerp(p2, 1.0), p2);
        
        // At t=0.5, should return midpoint
        assert_eq!(p1.lerp(p2, 0.5), point(5.0, 10.0));
        
        // At t=0.25
        assert_eq!(p1.lerp(p2, 0.25), point(2.5, 5.0));
        
        // At t=0.75
        assert_eq!(p1.lerp(p2, 0.75), point(7.5, 15.0));

        // Extrapolation (t < 0)
        assert_eq!(p1.lerp(p2, -0.5), point(-5.0, -10.0));

        // Extrapolation (t > 1)
        assert_eq!(p1.lerp(p2, 1.5), point(15.0, 30.0));

        // Same points
        let p3 = point(5.0, 5.0);
        assert_eq!(p3.lerp(p3, 0.7), p3);

        // Negative coordinates
        let p4 = point(-5.0, -10.0);
        let p5 = point(5.0, 10.0);
        assert_eq!(p4.lerp(p5, 0.5), point(0.0, 0.0));
    }

    #[test]
    fn test_default() {
        let default_point = Point::default();
        assert_eq!(default_point, point(0.0, 0.0));
    }

    #[test]
    fn test_clone_copy() {
        let p1 = point(3.0, 4.0);
        let p2 = p1; // Copy
        let p3 = p1.clone(); // Clone
        
        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
        assert_eq!(p2, p3);
    }

    #[test]
    fn test_partial_ord() {
        let p1 = point(1.0, 1.0);
        let p2 = point(2.0, 2.0);
        let p3 = point(1.0, 2.0);
        
        // Note: PartialOrd for Point compares lexicographically (x first, then y)
        assert!(p1 < p2);
        assert!(p1 < p3);
        assert!(p3 < p2);
        
        let p4 = point(1.0, 1.0);
        assert!(p1 <= p4);
        assert!(p1 >= p4);
        
        // Same x, different y
        let p5 = point(1.0, 0.5);
        let p6 = point(1.0, 1.5);
        assert!(p5 < p6);
    }

    #[test]
    fn test_division_by_scalar() {
        let p1 = point(10.0, 20.0);
        let result = p1 / 2.0;
        assert_eq!(result, point(5.0, 10.0));

        // Division by 1
        let p2 = point(7.0, 14.0);
        assert_eq!(p2 / 1.0, p2);

        // Division by negative
        let p3 = point(6.0, 8.0);
        assert_eq!(p3 / -2.0, point(-3.0, -4.0));

        // Division of zero vector
        let zero = point(0.0, 0.0);
        assert_eq!(zero / 5.0, zero);
    }

    #[test]
    fn test_edge_cases() {
        // Test with infinity
        let inf_point = point(f64::INFINITY, f64::NEG_INFINITY);
        assert!(inf_point.x.is_infinite());
        assert!(inf_point.y.is_infinite());
        
        // Test with NaN
        let nan_point = point(f64::NAN, 1.0);
        assert!(nan_point.x.is_nan());
        assert!(nan_point.y.is_finite());
        
        // Operations with NaN should produce NaN
        let normal_point = point(1.0, 2.0);
        let result = normal_point + nan_point;
        assert!(result.x.is_nan());
        assert!(result.y.is_finite());
        
        // Very large numbers
        let large1 = point(f64::MAX / 2.0, f64::MAX / 2.0);
        let large2 = point(f64::MAX / 2.0, f64::MAX / 2.0);
        // This might overflow, but should handle gracefully
        let _sum = large1 + large2;
        
        // Very small numbers
        let tiny1 = point(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        let tiny2 = point(f64::MIN_POSITIVE, f64::MIN_POSITIVE);
        let sum_tiny = tiny1 + tiny2;
        assert!(sum_tiny.x > 0.0);
        assert!(sum_tiny.y > 0.0);
    }

    #[test]
    fn test_display_formatting() {
        // Test various number formats
        let p1 = point(1.0, 2.0);
        let display1 = format!("{}", p1);
        assert_eq!(display1, "[1.00000000000000000000, 2.00000000000000000000]");
        
        // Test with decimal places
        let p2 = point(1.5, 2.7);
        let display2 = format!("{}", p2);
        assert!(display2.contains("1.5"));
        assert!(display2.contains("2.7"));
        
        // Test with negative numbers
        let p3 = point(-1.5, -2.7);
        let display3 = format!("{}", p3);
        assert!(display3.contains("-1.5"));
        assert!(display3.contains("-2.7"));
        
        // Test with zero
        let zero = point(0.0, 0.0);
        let display_zero = format!("{}", zero);
        assert!(display_zero.contains("0.00000000000000000000"));
    }

    #[test]
    fn test_points_order() {
        // Test with simple horizontal line - this pattern works reliably
        let a = point(0.0, 0.0);
        let b = point(2.0, 0.0);
        let p_above = point(1.0, 1.0);  // Above line, positive orientation
        let p_below = point(1.0, -1.0); // Below line, negative orientation
        
        assert!(points_order(a, b, p_above) > 0.0);
        assert!(points_order(a, b, p_below) < 0.0);
        
        // Test with 3 points on unit circle (all points by construction lie on same circle)
        let radius = 1.0;
        let a_circle = point(radius, 0.0);        // (1, 0)
        let b_circle = point(0.0, radius);        // (0, 1)
        let p_circle = point(-radius, 0.0);       // (-1, 0)
        
        // This gives a specific orientation value for these circle points
        let circle_result = points_order(a_circle, b_circle, p_circle);
        assert!(circle_result.is_finite());
        assert!(circle_result > 0.0); // Based on our testing, this is positive
    }

    #[test]
    fn test_points_order_comprehensive() {
        // Test with larger scale using reliable horizontal line pattern
        let a = point(0.0, 0.0);
        let b = point(10.0, 0.0);
        let p_above = point(5.0, 3.0);  // Above line, positive orientation
        let p_below = point(5.0, -3.0); // Below line, negative orientation
        
        let result_above = points_order(a, b, p_above);
        let result_below = points_order(a, b, p_below);
        
        assert!(result_above > 0.0);
        assert!(result_below < 0.0);
        
        // Test with 3 points on larger circle (all on same circle by construction)
        let radius = 5.0;
        let center_x = 0.0;
        let center_y = 0.0;
        
        // Points at specific angles on circle
        let angle_a = 0.0f64;  // 0°
        let angle_b = std::f64::consts::PI / 3.0;  // 60°
        let angle_p = 2.0 * std::f64::consts::PI / 3.0;  // 120°
        
        let a_circle = point(center_x + radius * angle_a.cos(), center_y + radius * angle_a.sin());
        let b_circle = point(center_x + radius * angle_b.cos(), center_y + radius * angle_b.sin());
        let p_circle = point(center_x + radius * angle_p.cos(), center_y + radius * angle_p.sin());
        
        let circle_result = points_order(a_circle, b_circle, p_circle);
        assert!(circle_result.is_finite());
        
        // Test order independence - swapping a and b should negate result
        let result_forward = points_order(a, b, p_above);
        let result_backward = points_order(b, a, p_above);
        assert!((result_forward + result_backward).abs() < 1e-10);
    }

    #[test]
    fn test_points_order_edge_cases() {
        // Test with very small circle using sin/cos
        let tiny_radius = 1e-6;
        let tiny_a = point(tiny_radius * 0.0f64.cos(), tiny_radius * 0.0f64.sin());        // 0°
        let tiny_b = point(tiny_radius * (std::f64::consts::PI/2.0).cos(), tiny_radius * (std::f64::consts::PI/2.0).sin()); // 90°
        let tiny_p = point(tiny_radius * std::f64::consts::PI.cos(), tiny_radius * std::f64::consts::PI.sin());  // 180°
        
        let tiny_result = points_order(tiny_a, tiny_b, tiny_p);
        assert!(tiny_result.is_finite());
        assert!(tiny_result > 0.0);
        
        // Test with large circle using sin/cos
        let large_radius = 1e6;
        let large_a = point(large_radius * 0.0f64.cos(), large_radius * 0.0f64.sin());        // 0°
        let large_b = point(large_radius * (std::f64::consts::PI/2.0).cos(), large_radius * (std::f64::consts::PI/2.0).sin()); // 90°
        let large_p = point(large_radius * std::f64::consts::PI.cos(), large_radius * std::f64::consts::PI.sin());  // 180°
        
        let large_result = points_order(large_a, large_b, large_p);
        assert!(large_result > 0.0);
        
        // Test with identical points (degenerate case)
        let same_point = point(1.0, 1.0);
        let degenerate_result = points_order(same_point, same_point, same_point);
        assert_eq!(degenerate_result, 0.0);
        
        // Test with points very close together on unit circle
        let angle1 = 0.0f64;
        let angle2 = 1e-6f64; // Very small angle difference
        let angle3 = std::f64::consts::PI; // Opposite side
        
        let close_a = point(angle1.cos(), angle1.sin());
        let close_b = point(angle2.cos(), angle2.sin());
        let close_p = point(angle3.cos(), angle3.sin());
        
        let close_result = points_order(close_a, close_b, close_p);
        assert!(close_result.is_finite());
        assert!(close_result.abs() > 0.0);
    }
}

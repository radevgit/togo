#![allow(dead_code)]

use crate::point::Point;
use std::fmt::Display;

/// A circle defined by a center point and radius.
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let circle = Circle::new(point(0.0, 0.0), 5.0);
/// ```
// #00008
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    /// Center of the circle
    pub c: Point,
    /// Radius of the circle
    pub r: f64,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {:.20}]", self.c, self.r)
    }
}

impl Circle {
    /// Creates a new circle with the specified center and radius.
    ///
    /// # Arguments
    ///
    /// * `c` - Center point of the circle
    /// * `r` - Radius of the circle (must be non-negative)
    ///
    /// # Examples
    ///
    /// ```
    /// use togo::prelude::*;
    ///
    /// let circle = Circle::new(point(3.0, 4.0), 5.0);
    /// ```
    #[inline]
    pub fn new(c: Point, r: f64) -> Self {
        Circle { c, r }
    }
}

/// Creates a new circle with the specified center and radius.
///
/// This is a convenience function equivalent to `Circle::new(c, r)`.
///
/// # Arguments
///
/// * `c` - Center point of the circle
/// * `r` - Radius of the circle (must be non-negative)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let circle = circle(point(3.0, 4.0), 5.0);
/// ```
#[inline]
pub fn circle(c: Point, r: f64) -> Circle {
    Circle::new(c, r)
}

#[cfg(test)]
mod test_circle {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_new() {
        let c0 = Circle::new(point(1.0, 1.0), 2.0);
        let c1 = circle(point(1.0, 1.0), 2.0);
        assert_eq!(c0, c1);
    }

    #[test]
    fn test_display() {
        let c0 = circle(point(1.0, 1.0), 2.0);
        assert_eq!(
            "[[1.00000000000000000000, 1.00000000000000000000], 2.00000000000000000000]",
            format!("{}", c0)
        );
    }
}

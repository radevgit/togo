#![allow(dead_code)]

use crate::point::Point;
use std::fmt::Display;

/// A rectangle defined by a left-bottom point and right-top point.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    /// Left-bottom point of the rectangle
    pub p1: Point,
    /// Right-top point of the rectangle
    pub p2: Point,
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.p1, self.p2)
    }
}

impl Rect {
    /// Creates a new rectangle with the specified corners.
    ///
    /// # Arguments
    ///
    /// * `p1` - Left-bottom point of the rectangle
    /// * `p2` - Right-top point of the rectangle
    ///
    /// # Examples
    ///
    /// ```
    /// use basegeom::prelude::*;
    ///
    /// let rect = Rect::new(point(0.0, 0.0), point(1.0, 1.0));
    /// ```
    #[inline]
    pub fn new(p1: Point, p2: Point) -> Self {
        Rect { p1, p2 }
    }
}

/// Creates a new rectangle with the specified corners.
///
/// This is a convenience function equivalent to `Rect::new(p1, p2)`.
///
/// # Arguments
///
/// * `p1` - Left-bottom point of the rectangle
/// * `p2` - Right-top point of the rectangle
///
/// # Examples
///
/// ```
/// use basegeom::prelude::*;
///
/// let rect = rect(point(3.0, 4.0), point(5.0, 6.0));
/// ```
#[inline]
pub fn rect(p1: Point, p2: Point) -> Rect {
    Rect::new(p1, p2)
}

#[cfg(test)]
mod test_rect {
    use super::*;
    use crate::point::point;

}

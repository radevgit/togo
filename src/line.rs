#![allow(dead_code)]

use crate::point::Point;
use std::fmt::Display;

/// Line is defined by origin and direction
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub origin: Point,
    pub dir: Point,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.origin, self.dir)
    }
}

impl Line {
    /// Creates a new line with the specified origin and direction.
    ///  
    /// # Arguments
    /// * `origin` - Origin point of the line
    /// * `dir` - Direction vector of the line
    ///
    /// # Examples
    /// ```
    /// use basegeom::prelude::*;
    /// let line = Line::new(point(1.0, 2.0), point(3.0, 4.0));
    /// ```
    #[inline]
    pub fn new(origin: Point, dir: Point) -> Self {
        Line { origin, dir }
    }

    /// Returns a unit direction vector of the line.
    /// This normalizes the direction vector to have a length of 1.
    ///
    /// # Examples
    /// ```
    /// use basegeom::prelude::*;
    /// let line = Line::new(point(1.0, 2.0), point(3.0, 4.0));
    /// let unit_line = line.unitdir();
    /// assert_eq!(unit_line.dir.norm(), 1.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn unitdir(&self) -> Self {
        let (dir, _) = self.dir.normalize(false);
        Line {
            origin: self.origin,
            dir,
        }
    }
}

/// Creates a new line with the specified origin and direction.
///
/// This is a convenience function equivalent to `Line::new(origin, dir)`.
#[inline]
#[must_use]
pub fn line(origin: Point, dir: Point) -> Line {
    Line::new(origin, dir)
}

#[cfg(test)]
mod test_line {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_new() {
        let l0 = Line::new(point(1.0, 2.0), point(3.0, 4.0));
        let l1 = line(point(1.0, 2.0), point(3.0, 4.0));
        assert_eq!(l0, l1);
    }

    #[test]
    fn test_display() {
        let s0 = Line::new(point(1.0, 2.0), point(3.0, 4.0));
        assert_eq!(
            "[[1.00000000000000000000, 2.00000000000000000000], [3.00000000000000000000, 4.00000000000000000000]]",
            format!("{}", s0)
        );
    }

    #[test]
    fn test_unitdir() {
        // Test unitdir with a simple vector
        let l = Line::new(point(0.0, 0.0), point(3.0, 4.0)); // 3-4-5 triangle
        let unit_l = l.unitdir();
        assert!((unit_l.dir.norm() - 1.0).abs() < 1e-15);
        assert_eq!(unit_l.origin, point(0.0, 0.0));

        // Test with already unit vector
        let unit_input = Line::new(point(1.0, 1.0), point(1.0, 0.0));
        let unit_output = unit_input.unitdir();
        assert!((unit_output.dir.norm() - 1.0).abs() < 1e-15);
        assert_eq!(unit_output.dir, point(1.0, 0.0));
    }

    #[test]
    fn test_unitdir_zero_vector() {
        // Edge case: zero direction vector should handle gracefully
        let l = Line::new(point(0.0, 0.0), point(0.0, 0.0));
        let unit_l = l.unitdir();
        // The normalize function should handle this case
        // Check that we don't panic and get a valid result
        assert!(unit_l.dir.x.is_nan() || unit_l.dir.x == 0.0);
    }
}

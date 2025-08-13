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

    #[test]
    fn test_rect_new() {
        // Test basic rectangle creation with Rect::new()
        let p1 = point(1.0, 2.0);
        let p2 = point(4.0, 5.0);
        let rectangle = Rect::new(p1, p2);
        
        assert_eq!(rectangle.p1, p1);
        assert_eq!(rectangle.p2, p2);
        assert_eq!(rectangle.p1.x, 1.0);
        assert_eq!(rectangle.p1.y, 2.0);
        assert_eq!(rectangle.p2.x, 4.0);
        assert_eq!(rectangle.p2.y, 5.0);
    }

    #[test]
    fn test_rect_convenience_function() {
        // Test the rect() convenience function
        let p1 = point(3.0, 4.0);
        let p2 = point(5.0, 6.0);
        let rectangle = rect(p1, p2);
        
        assert_eq!(rectangle.p1, p1);
        assert_eq!(rectangle.p2, p2);
        
        // Should be equivalent to Rect::new()
        let rectangle_new = Rect::new(p1, p2);
        assert_eq!(rectangle, rectangle_new);
    }

    #[test]
    fn test_rect_equality() {
        // Test PartialEq implementation
        let rect1 = rect(point(0.0, 0.0), point(1.0, 1.0));
        let rect2 = rect(point(0.0, 0.0), point(1.0, 1.0));
        let rect3 = rect(point(0.0, 0.0), point(2.0, 2.0));
        
        assert_eq!(rect1, rect2);
        assert_ne!(rect1, rect3);
    }

    #[test]
    fn test_rect_clone() {
        // Test Clone implementation
        let original = rect(point(10.0, 20.0), point(30.0, 40.0));
        let cloned = original.clone();
        
        assert_eq!(original, cloned);
        assert_eq!(original.p1, cloned.p1);
        assert_eq!(original.p2, cloned.p2);
    }

    #[test]
    fn test_rect_copy() {
        // Test Copy implementation
        let original = rect(point(10.0, 20.0), point(30.0, 40.0));
        let copied = original; // Should copy, not move
        
        assert_eq!(original, copied);
        // Original should still be usable (proves it was copied, not moved)
        assert_eq!(original.p1.x, 10.0);
    }

    #[test]
    fn test_rect_display() {
        // Test Display implementation
        let rectangle = rect(point(1.0, 2.0), point(3.0, 4.0));
        let display_string = format!("{}", rectangle);
        
        // Should display as "[p1, p2]" format
        assert_eq!(display_string, "[[1.00000000000000000000, 2.00000000000000000000], [3.00000000000000000000, 4.00000000000000000000]]");
    }

    #[test]
    fn test_rect_debug() {
        // Test Debug implementation
        let rectangle = rect(point(1.5, 2.5), point(3.5, 4.5));
        let debug_string = format!("{:?}", rectangle);
        
        // Should contain the struct name and field values
        assert!(debug_string.contains("Rect"));
        assert!(debug_string.contains("p1"));
        assert!(debug_string.contains("p2"));
        assert!(debug_string.contains("1.5"));
        assert!(debug_string.contains("2.5"));
        assert!(debug_string.contains("3.5"));
        assert!(debug_string.contains("4.5"));
    }

    #[test]
    fn test_rect_with_negative_coordinates() {
        // Test rectangle with negative coordinates
        let p1 = point(-5.0, -3.0);
        let p2 = point(-1.0, -1.0);
        let rectangle = rect(p1, p2);
        
        assert_eq!(rectangle.p1.x, -5.0);
        assert_eq!(rectangle.p1.y, -3.0);
        assert_eq!(rectangle.p2.x, -1.0);
        assert_eq!(rectangle.p2.y, -1.0);
    }

    #[test]
    fn test_rect_with_zero_coordinates() {
        // Test rectangle with zero coordinates
        let p1 = point(0.0, 0.0);
        let p2 = point(0.0, 0.0);
        let rectangle = rect(p1, p2);
        
        assert_eq!(rectangle.p1, point(0.0, 0.0));
        assert_eq!(rectangle.p2, point(0.0, 0.0));
    }

    #[test]
    fn test_rect_with_floating_point_precision() {
        // Test rectangle with floating point values that test precision
        let p1 = point(1.0/3.0, 2.0/3.0);
        let p2 = point(4.0/3.0, 5.0/3.0);
        let rectangle = rect(p1, p2);
        
        assert_eq!(rectangle.p1.x, 1.0/3.0);
        assert_eq!(rectangle.p1.y, 2.0/3.0);
        assert_eq!(rectangle.p2.x, 4.0/3.0);
        assert_eq!(rectangle.p2.y, 5.0/3.0);
    }

    #[test]
    fn test_rect_field_access() {
        // Test direct field access
        let mut rectangle = rect(point(1.0, 2.0), point(3.0, 4.0));
        
        // Test reading fields
        assert_eq!(rectangle.p1.x, 1.0);
        assert_eq!(rectangle.p1.y, 2.0);
        assert_eq!(rectangle.p2.x, 3.0);
        assert_eq!(rectangle.p2.y, 4.0);
        
        // Test modifying fields
        rectangle.p1 = point(10.0, 20.0);
        rectangle.p2 = point(30.0, 40.0);
        
        assert_eq!(rectangle.p1, point(10.0, 20.0));
        assert_eq!(rectangle.p2, point(30.0, 40.0));
    }
}

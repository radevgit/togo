#![allow(dead_code)]

use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct OffsetRaw {
    pub arc: Arc,
    pub orig: Point, // original point p0
    pub g: f64,
}

impl Display for OffsetRaw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.arc, self.orig, self.g)
    }
}

impl OffsetRaw {
    #[inline]
    pub fn new(arc: Arc, orig: Point, g: f64) -> Self {
        OffsetRaw { arc, orig, g }
    }
}

#[inline]
pub fn offsetraw(arc: Arc, orig: Point, g: f64) -> OffsetRaw {
    OffsetRaw::new(arc, orig, g)
}

#[cfg(test)]
mod test_offset_raw {
    use crate::prelude::*;

    #[test]
    fn test_new_and_display() {
        let a = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.0), 1.0);
        let o = point(0.0, 0.0);
        let off = offsetraw(a, o, 0.25);
        assert_eq!(off.arc, a);
        assert_eq!(off.orig, o);
        assert_eq!(off.g, 0.25);
        // Check Display has brackets and three components
        let s = format!("{}", off);
        assert!(s.starts_with("["));
        assert!(s.contains("], ["));
        assert!(s.ends_with("]"));
    }
}
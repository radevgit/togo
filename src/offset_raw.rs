#![allow(dead_code)]

use std::fmt::Display;

use crate::Arc;
use crate::Point;

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
    fn new(arc: Arc, orig: Point, g: f64) -> Self {
        OffsetRaw { arc, orig, g }
    }
}

#[inline]
pub(crate) fn offsetraw(arc: Arc, orig: Point, g: f64) -> OffsetRaw {
    OffsetRaw::new(arc, orig, g)
}
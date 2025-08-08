#![allow(dead_code)]



// use std::{fmt::Display, ops::Range};

// //use kurbo::{offset::CubicOffset, BezPath, CubicBez, Point as KurboPoint};

// use crate::{
//     line::Line, bezierc::BezierC, Point
// };

// #[doc(hidden)]
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub(crate) struct BezierQ {
//     pub p0: Point,
//     pub p1: Point,
//     pub p2: Point,
// }

// impl Display for BezierQ {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[{}, {}, {}]", self.p0, self.p1, self.p2)
//     }
// }

// impl BezierQ {
//     #[inline]
//     pub(crate) fn new<V: Into<Point>>(p0: V, p1: V, p2: V) -> BezierQ {
//         BezierQ {
//             p0: p0.into(),
//             p1: p1.into(),
//             p2: p2.into(),
//         }
//     }

//     #[inline]
//     pub(crate) fn eval(&self, t: f64) -> Point {
//         let mt = 1.0 - t;
//         self.p0 * (mt * mt) + (self.p1 * (mt * 2.0) + self.p2 * t) * t
//     }

//     /// Returns a cubic Bézier segment that exactly represents this quadratic.
//     #[inline]
//     pub(crate) fn raise(&self) -> BezierC {
//         BezierC::new(
//             self.p0,
//             self.p0 + (self.p1 - self.p0) * (2.0 / 3.0),
//             self.p2 + (self.p1 - self.p2) * (2.0 / 3.0),
//             self.p2,
//         )
//     }

//     fn subsegment(&self, range: Range<f64>) -> BezierQ {
//         let (t0, t1) = (range.start, range.end);
//         let p0 = self.eval(t0);
//         let p2 = self.eval(t1);
//         let p1 = p0 + (self.p1 - self.p0).lerp(self.p2 - self.p1, t0) * (t1 - t0);
//         BezierQ { p0, p1, p2 }
//     }

//     /// Subdivide into halves, using de Casteljau.
//     #[inline]
//     fn subdivide(&self) -> (BezierQ, BezierQ) {
//         let pm = self.eval(0.5);
//         (
//             BezierQ::new(self.p0, (self.p0 + self.p1) / 2.0, pm),
//             BezierQ::new(pm, (self.p1 + self.p2) / 2.0, self.p2),
//         )
//     }

//     #[inline]
//     fn deriv(&self) -> Line {
//         Line::new(
//             (self.p1 - self.p0) * 2.0,
//             (self.p2 - self.p1) * 2.0,
//         )
//     }
// }

#![allow(dead_code)]

// #00014

// use basegeom::prelude::*;

// This path can contain lines, quadratics ([`QuadBez`]), cubics ([`CubicBez`])
// and arcs ([`Arc`]) and is used to represent a Bezier path.
// 
// PathEl - path element is more close to the SVG path element.
// PathSeg - path segment is a segment more close to mathematical aspects.

// Element of a Bezier path.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub(crate) enum PathEl {
//     MoveTo(Point),
//     LineTo(Point),
//     QuadTo(Point, Point, Point),
//     CurveTo(Point, Point, Point, Point),
//     ArcTo(Point, f64, bool, bool, Point), // TODO: more parameters for arc
//     ClosePath,
// }

// // Segment of a Bezier path.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum PathSeg {
//     Line(Segment),
//     Quad(BezierQ),
//     Cubic(BezierQ),
//     Arc(Arc),
// }




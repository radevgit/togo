use basegeom::prelude::*;

fn main() {
    // Writes SVG to standard output
    let mut svg = SVG::new(200.0, 200.0, None);
    let mut arc = arc_circle_parametrization(point(70.0, 10.0), point(10.0, 70.0), 0.7);
    arc.translate(point(25.0, 25.0));
    let bounding_circle = arc_bounding_circle(&arc);
    svg.circle(&bounding_circle, "green");
    svg.arc(&arc, "blue");
    svg.circle(&circle(arc.a, 1.0), "red");
    svg.circle(&circle(arc.b, 1.0), "red");


    let mut arc = arc_circle_parametrization(point(70.0, 10.0), point(10.0, 70.0), 0.7);
    arc.translate(point(95.0, 95.0));
    let bounding_rect = arc_bounding_rect(&arc);
    svg.rect(&bounding_rect, "green");
    svg.arc(&arc, "blue");
    svg.circle(&circle(arc.a, 1.0), "red");
    svg.circle(&circle(arc.b, 1.0), "red");
    svg.write_stroke_width(1.0);
}
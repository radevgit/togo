use basegeom::prelude::*;

fn main() {
    // Writes SVG to standard output
    let mut svg = SVG::new(1280.0, 640.0, None);
    let mut arc = arc_circle_parametrization(point(70.0, 10.0), point(10.0, 70.0), 0.7);
    arc.scale(5.0);
    arc.translate(point(120.0, 120.0));
    let bounding_circle = arc_bounding_circle(&arc);
    svg.circle(&bounding_circle, "green");
    svg.arc(&arc, "blue");
    svg.circle(&circle(arc.a, 5.0), "red");
    svg.circle(&circle(arc.b, 5.0), "red");


    let mut arc = arc_circle_parametrization(point(70.0, 10.0), point(10.0, 70.0), 0.7);
    arc.scale(5.0);
    arc.translate(point(700.0, 120.0));
    let bounding_rect = arc_bounding_rect(&arc);
    svg.rect(&bounding_rect, "green");
    svg.arc(&arc, "blue");
    svg.circle(&circle(arc.a, 5.0), "red");
    svg.circle(&circle(arc.b, 5.0), "red");
    svg.write_stroke_width(5.0);
}
use basegeom::prelude::*;

fn main() {
    // Writes SVG file in selected location
    let mut svg = SVG::new(120.0, 120.0, "/tmp/arc_segment_intersect.svg");
    let seg1 = segment(point(10.0, 10.0), point(110.0, 110.0));
    let seg2 = segment(point(10.0, 30.0), point(110.0, 30.0));
    let arc = arc(
        point(10.0, 60.0),
        point(60.0, 110.0),
        point(60.0, 60.0),
        50.0,
    );

    let mut points = Vec::new();
    points.extend(segarc_intersect(&arc, &seg1));
    points.extend(segarc_intersect(&arc, &seg2));
    points.extend(segseg_intersect(&seg1, &seg2));

    svg.segment(&seg1, "blue");
    svg.segment(&seg2, "blue");
    svg.arc(&arc, "blue");
    for point in points {
        svg.circle(&circle(point, 1.0), "red");
    }

    svg.write_stroke_width(1.0);
}

fn segarc_intersect(arc1: &Arc, seg2: &Segment) -> Vec<Point> {
    let res = int_segment_arc(seg2, arc1);
    match res {
        SegmentArcConfig::OnePoint(point, _) => vec![point],
        SegmentArcConfig::TwoPoints(point, point1, _, _) => vec![point, point1],
        _ => Vec::new(),
    }
}

fn segseg_intersect(seg1: &Segment, seg2: &Segment) -> Vec<Point> {
    let res = int_segment_segment(seg1, seg2);
    match res {
        SegmentSegmentConfig::OnePoint(point, _, _) => vec![point],
        SegmentSegmentConfig::TwoPoints(point, point1, _, _) => vec![point, point1],
        _ => Vec::new(),
    }
}

use crate::poly::arcline200;
use super::*;

// ===== Basic Convex Hull Tests =====

#[test]
fn test_arcline_convex_hull_empty() {
    let arcs: Arcline = vec![];
    let hull = arcline_convex_hull(&arcs);
    assert!(hull.is_empty());
}

#[test]
fn test_arcline_convex_hull_single_arc() {
    // A single arc cannot form a closed polygon - this is an invalid input
    // The algorithm should handle it gracefully (likely return empty or the arc itself)
    let arcs = vec![arc(
        point(0.0, 0.0),
        point(1.0, 0.0),
        point(0.5, 0.0),
        f64::INFINITY,
    )];
    let hull = arcline_convex_hull(&arcs);
    assert_eq!(hull.len(), 0);
}

#[test]
fn test_arcline_convex_hull_square() {
    // Square made of 4 line segments
    let arcs = vec![
        arcseg(point(0.0, 0.0), point(1.0, 0.0)),
        arcseg(point(1.0, 0.0), point(1.0, 1.0)),
        arcseg(point(1.0, 1.0), point(0.0, 1.0)),
        arcseg(point(0.0, 1.0), point(0.0, 0.0)),
    ];
    let hull = arcline_convex_hull(&arcs);
    assert_eq!(hull.len(), 4); // All segments should be on hull

    // Write SVG for visualization
    use crate::svg::SVG;
    let arcs2 = arcline_scale(&arcs, 100.0);
    let hull2 = arcline_scale(&hull, 100.0);
    let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
    let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
    let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
    svg.arcline(&arcs3, "blue");
    svg.arcline(&hull3, "red");
    svg.write_stroke_width(0.1);
}

#[test]
fn test_arcline_convex_hull_circle() {
    // Circle made of 4 quarter-circle arcs (all convex)
    // Center at origin, radius 1.0
    let arcs = vec![
        arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0), // Right to Top
        arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0), // Top to Left
        arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0), // Left to Bottom
        arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0), // Bottom to Right
    ];

    let hull = arcline_convex_hull(&arcs);

    // Hull should include all 4 cardinal points (endpoints)
    assert_eq!(hull.len(), 4);

    // Verify that all cardinal points are in the hull
    let hull_points: Vec<Point> = hull.iter().map(|arc| arc.a).collect();
    assert!(hull_points.contains(&point(1.0, 0.0))); // Right
    assert!(hull_points.contains(&point(0.0, 1.0))); // Top
    assert!(hull_points.contains(&point(-1.0, 0.0))); // Left
    assert!(hull_points.contains(&point(0.0, -1.0))); // Bottom

    // Write SVG for visualization
    use crate::svg::SVG;
    let arcs2 = arcline_scale(&arcs, 100.0);
    let hull2 = arcline_scale(&hull, 100.0);
    let arcs3 = arcline_translate(&arcs2, point(50.0, 50.0));
    let hull3 = arcline_translate(&hull2, point(50.0, 50.0));
    let mut svg = SVG::new(600.0, 600.0, Some("/tmp/test.svg"));
    svg.arcline(&arcs3, "blue");
    svg.arcline(&hull3, "red");
    svg.write_stroke_width(0.1);
}

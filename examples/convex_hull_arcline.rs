use rand::{Rng, SeedableRng, rngs::StdRng};
use togo::prelude::*;
use std::f64::consts::PI;

/// Example: Compute convex hull of a wavy arc-based polygon
///
/// This example generates a non-convex arc-based polygon using two concentric circles:
/// - Inner circle: radius ~100, with points at regular angles
/// - Outer circle: radius ~150, with points at phase-shifted angles
///
/// Points from the two circles are connected alternately with bulge arcs:
/// - Positive bulge: arc curves outward (convex)
/// - Negative bulge: arc curves inward (concave)
///
/// The resulting polygon has a wavy/star pattern. The convex hull should:
/// 1. Keep the positive bulge arcs (convex, on the hull)
/// 2. Remove the negative bulge arcs (concave, inside the hull)
/// 3. Replace concave sections with straight bridges
fn main() {
    // Configuration
    const N_POINTS: usize = 12;
    const CENTER_X: f64 = 300.0;
    const CENTER_Y: f64 = 300.0;
    const INNER_RADIUS: f64 = 100.0;
    const OUTER_RADIUS: f64 = 180.0;
    const PHASE_SHIFT: f64 = 0.3; // Phase shift for outer circle (in radians)

    // Use fixed seed for reproducible results
    let mut rng = StdRng::seed_from_u64(42);
    let center = point(CENTER_X, CENTER_Y);

    // Generate points on two circles
    let mut inner_points: Vec<Point> = Vec::new();
    let mut outer_points: Vec<Point> = Vec::new();

    for i in 0..N_POINTS {
        let angle = 2.0 * PI * (i as f64) / (N_POINTS as f64);

        // Inner circle: points at regular angles
        let inner_x = CENTER_X + INNER_RADIUS * angle.cos();
        let inner_y = CENTER_Y + INNER_RADIUS * angle.sin();
        inner_points.push(point(inner_x, inner_y));

        // Outer circle: points at phase-shifted angles with random radius variation
        let phase_angle = angle + PHASE_SHIFT;
        let radius_variation = OUTER_RADIUS + rng.random::<f64>() * 30.0 - 15.0;
        let outer_x = CENTER_X + radius_variation * phase_angle.cos();
        let outer_y = CENTER_Y + radius_variation * phase_angle.sin();
        outer_points.push(point(outer_x, outer_y));
    }

    // Build the arc-based polygon by connecting inner and outer points
    // Create arcs by computing center and radius for bulge
    // Note: For inward (concave) arcs, we swap endpoints and use positive bulge
    // so that arc_from_bulge always creates CCW arcs
    let mut arcs: Arcline = Vec::new();

    for i in 0..N_POINTS {
        let inner_current = inner_points[i];
        let outer_current = outer_points[i];
        let inner_next = inner_points[(i + 1) % N_POINTS];

        // Connect inner[i] to outer[i] with outward (convex) arc
        let bulge_pos = 0.35;
        let arc_out = arc_from_bulge(inner_current, outer_current, bulge_pos);
        arcs.push(arc_out);

        // Connect outer[i] to inner[i+1] with inward (concave) arc
        // For inward arcs, swap the endpoints so arc_from_bulge processes them correctly
        let bulge_pos = 0.35;
        let arc_in = arc_from_bulge(inner_next, outer_current, bulge_pos);
        arcs.push(arc_in);
    }

    println!("=== Wavy Arc-Based Polygon ===");
    println!("Generated from two concentric circles with phase shift");
    println!("Configuration:");
    println!("  N_POINTS: {}", N_POINTS);
    println!("  Inner radius: {}", INNER_RADIUS);
    println!("  Outer radius: {} (with ±15 variation)", OUTER_RADIUS);
    println!("  Phase shift: {:.2} radians", PHASE_SHIFT);
    println!("  Center: ({}, {})", CENTER_X, CENTER_Y);

    println!("\n=== Input Arc-Based Polygon ===");
    println!("Total elements: {}", arcs.len());
    
    let mut convex_count = 0;
    let mut concave_count = 0;
    for (i, elem) in arcs.iter().enumerate() {
        if i % 2 == 0 {
            convex_count += 1;
            println!("  [{}] Arc (positive bulge, outward): {} → {}", i, elem.a, elem.b);
        } else {
            concave_count += 1;
            println!("  [{}] Arc (negative bulge, inward): {} → {}", i, elem.a, elem.b);
        }
    }
    println!("\nBreakdown: {} total arcs ({} outward/convex, {} inward/concave)",
             arcs.len(), convex_count, concave_count);

    // Check validity
    let validation = arcline_is_valid(&arcs);
    println!("\nArcline validation: {:?}", validation);

    // Compute convex hull
    println!("\n=== Computing Convex Hull ===");
    let hull = arcline_convex_hull(&arcs);

    println!("\nConvex hull result:");
    println!("Total elements: {} (reduced from {})", hull.len(), arcs.len());
    
    let mut hull_arcs = 0;
    let mut hull_lines = 0;
    for (i, elem) in hull.iter().enumerate() {
        if elem.is_arc() {
            hull_arcs += 1;
            println!("  [{}] Arc: {} → {}", i, elem.a, elem.b);
        } else {
            hull_lines += 1;
            println!("  [{}] Line: {} → {}", i, elem.a, elem.b);
        }
    }
    println!("\nHull composition: {} arcs + {} line segments", hull_arcs, hull_lines);

    // Visualize
    println!("\n=== Visualization ===");
    let mut svg = SVG::new(700.0, 700.0, Some("/tmp/hull_arcline.svg"));

    // Draw the input polygon in blue
    svg.arcline(&arcs, "blue");

    // Draw the convex hull in red (thicker)
    svg.arcline(&hull, "red");

    // Mark the center and circles for reference
    svg.circle(&Circle { c: center, r: 3.0 }, "black");

    // Draw circle guides (very thin, for reference)
    svg.circle(&Circle { c: center, r: INNER_RADIUS }, "lightgray");
    svg.circle(&Circle { c: center, r: OUTER_RADIUS }, "lightgray");

    svg.write_stroke_width(1.0);

    println!("SVG saved to: /tmp/hull_arcline.svg");
    println!("\nVisualization key:");
    println!("  • Blue: Input wavy polygon (outward + inward arcs)");
    println!("  • Red: Convex hull (inward arcs removed, outward arcs preserved)");
    println!("  • Black dot: Center point");
    println!("  • Light gray circles: Inner and outer radius guides");
    
    println!("\nExpected algorithm behavior:");
    println!("  ✓ Outward (positive bulge) arcs preserved on hull");
    println!("  ✓ Inward (negative bulge) arcs removed from hull");
    println!("  ✓ New line segments added where concave regions bridged");
    println!("  ✓ Final hull should have approximately {} elements", N_POINTS);
}

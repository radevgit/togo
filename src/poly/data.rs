use crate::prelude::*;

/// Generated ~1000-arc double spiral polyline spiraling outward
/// Two identical spirals rotated 180 degrees relative to each other
/// Spiral 1: starts at angle 0°, radius 50, spirals outward
/// Spiral 2: starts at angle 180°, radius 50, spirals outward, ends at angle 0° (half revolution extended)
/// Both spirals have synchronized wave patterns (alternating bulges)
/// Connected at start and end points
/// Total approximately 1020 arcs (~500 per spiral + ~20 extra for spiral 2 + 2 connections)
/// For benchmarking and testing spatial algorithms
pub fn arcline1000() -> Arcline {
    let mut arcs = Vec::with_capacity(1025);

    let num_arcs = 500;
    let center_x: f64 = 400.0;
    let center_y: f64 = 400.0;
    let inner_radius: f64 = 10.0; // Start from inner radius
    let spiral_increment: f64 = 0.58; // Increase radius per arc (spiral outward)
    let angular_step: f64 = std::f64::consts::PI / 20.0;

    // SPIRAL 1: starts at angle 0°, spirals outward
    let mut angle1: f64 = 0.0;
    let mut radius1: f64 = inner_radius;
    let spiral1_start_angle = angle1;
    let spiral1_start_radius = radius1;

    for i in 0..num_arcs {
        let start_x = center_x + radius1 * angle1.cos();
        let start_y = center_y + radius1 * angle1.sin();

        angle1 += angular_step;
        radius1 += spiral_increment;

        let end_x = center_x + radius1 * angle1.cos();
        let end_y = center_y + radius1 * angle1.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        arcs.push(arc);
    }

    let spiral1_end_angle = angle1;
    let spiral1_end_radius = radius1;

    // SPIRAL 2: starts at angle 180°, spirals outward, extended to reach angle 0°
    // Generate spiral2, then reverse it
    let mut spiral2_arcs = Vec::new();
    let spiral2_start_angle = std::f64::consts::PI; // 180 degrees
    let spiral2_start_radius = inner_radius;

    let mut angle2: f64 = spiral2_start_angle;
    let mut radius2: f64 = spiral2_start_radius;

    let extra_arcs = (std::f64::consts::PI / angular_step).ceil() as usize;
    let num_arcs_spiral2 = num_arcs + extra_arcs;

    for i in 0..num_arcs_spiral2 {
        let start_x = center_x + radius2 * angle2.cos();
        let start_y = center_y + radius2 * angle2.sin();

        angle2 += angular_step;
        radius2 += spiral_increment;

        let end_x = center_x + radius2 * angle2.cos();
        let end_y = center_y + radius2 * angle2.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        spiral2_arcs.push(arc);
    }

    let spiral2_end_angle = angle2;
    let spiral2_end_radius = radius2;

    // Reverse spiral2 so it traverses in opposite direction
    let spiral2_reversed = arcline_reverse(&spiral2_arcs);

    // Get the last arc of reversed spiral2
    let last_reversed_arc = spiral2_reversed[spiral2_reversed.len() - 1];

    // CONNECTION 1: Connect spiral 1 end to spiral 2 start (which is spiral2_end_angle after reversal)
    let connection1 = arc_from_bulge(
        Point::new(
            center_x + spiral1_end_radius * spiral1_end_angle.cos(),
            center_y + spiral1_end_radius * spiral1_end_angle.sin(),
        ),
        Point::new(
            center_x + spiral2_end_radius * spiral2_end_angle.cos(),
            center_y + spiral2_end_radius * spiral2_end_angle.sin(),
        ),
        0.0,
    );
    arcs.push(connection1);

    // Add reversed spiral2
    arcs.extend(spiral2_reversed);

    // CONNECTION 2: Connect from where spiral2_reversed ends back to spiral 1 start to close the loop
    let connection2 = arc_from_bulge(
        last_reversed_arc.a,
        Point::new(
            center_x + spiral1_start_radius * spiral1_start_angle.cos(),
            center_y + spiral1_start_radius * spiral1_start_angle.sin(),
        ),
        -0.3, // Use arc with negative bulge to bow inward
    );
    arcs.push(connection2);

    arcs
}

/// Generated ~500-arc double spiral polyline spiraling outward
/// Same structure as arcline1000 but with 250 arcs per spiral
/// Approximately 500-520 total arcs
/// Scaled to reach same outer radius as arcline1000
pub fn arcline500() -> Arcline {
    let mut arcs = Vec::with_capacity(520);

    let num_arcs = 250;
    let center_x: f64 = 400.0;
    let center_y: f64 = 400.0;
    let inner_radius: f64 = 10.0;
    let spiral_increment: f64 = 1.16; // 2x spiral_increment to reach same size in half the arcs
    let angular_step: f64 = std::f64::consts::PI / 20.0;

    // SPIRAL 1
    let mut angle1: f64 = 0.0;
    let mut radius1: f64 = inner_radius;
    let spiral1_start_angle = angle1;
    let spiral1_start_radius = radius1;

    for i in 0..num_arcs {
        let start_x = center_x + radius1 * angle1.cos();
        let start_y = center_y + radius1 * angle1.sin();

        angle1 += angular_step;
        radius1 += spiral_increment;

        let end_x = center_x + radius1 * angle1.cos();
        let end_y = center_y + radius1 * angle1.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        arcs.push(arc);
    }

    let spiral1_end_angle = angle1;
    let spiral1_end_radius = radius1;

    // SPIRAL 2: starts at angle 180°, spirals outward, extended to reach angle 0°
    // Generate spiral2 first, then reverse it
    let mut spiral2_arcs = Vec::new();
    let spiral2_start_angle = std::f64::consts::PI; // 180 degrees
    let spiral2_start_radius = inner_radius;

    let mut angle2: f64 = spiral2_start_angle;
    let mut radius2: f64 = spiral2_start_radius;

    // Extra arcs for spiral 2 to complete half revolution (π radians) to reach 0°
    let extra_arcs = (std::f64::consts::PI / angular_step).ceil() as usize;
    let num_arcs_spiral2 = num_arcs + extra_arcs;

    for i in 0..num_arcs_spiral2 {
        let start_x = center_x + radius2 * angle2.cos();
        let start_y = center_y + radius2 * angle2.sin();

        angle2 += angular_step;
        radius2 += spiral_increment;

        let end_x = center_x + radius2 * angle2.cos();
        let end_y = center_y + radius2 * angle2.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        spiral2_arcs.push(arc);
    }

    let spiral2_end_angle = angle2;
    let spiral2_end_radius = radius2;

    // Reverse spiral2 so it traverses in opposite direction
    let spiral2_reversed = arcline_reverse(&spiral2_arcs);

    // Get the last arc of reversed spiral2
    let last_reversed_arc = spiral2_reversed[spiral2_reversed.len() - 1];

    // CONNECTION 1: Connect spiral 1 end to spiral 2 start (which is spiral2_end_angle after reversal)
    let connection1 = arc_from_bulge(
        Point::new(
            center_x + spiral1_end_radius * spiral1_end_angle.cos(),
            center_y + spiral1_end_radius * spiral1_end_angle.sin(),
        ),
        Point::new(
            center_x + spiral2_end_radius * spiral2_end_angle.cos(),
            center_y + spiral2_end_radius * spiral2_end_angle.sin(),
        ),
        0.0,
    );
    arcs.push(connection1);

    // Add reversed spiral2
    arcs.extend(spiral2_reversed);

    // CONNECTION 2: Connect spiral 2 end back to spiral 1 start to close the loop
    let connection2 = arc_from_bulge(
        last_reversed_arc.a,
        Point::new(
            center_x + spiral1_start_radius * spiral1_start_angle.cos(),
            center_y + spiral1_start_radius * spiral1_start_angle.sin(),
        ),
        -0.3, // Use arc with negative bulge to bow inward
    );
    arcs.push(connection2);

    arcs
}

/// Generated ~200-arc double spiral polyline spiraling outward
/// Same structure as arcline1000 but with 100 arcs per spiral
/// Approximately 200-220 total arcs
/// Scaled to reach same outer radius as arcline1000
pub fn arcline200() -> Arcline {
    let mut arcs = Vec::with_capacity(220);

    let num_arcs = 100;
    let center_x: f64 = 400.0;
    let center_y: f64 = 400.0;
    let inner_radius: f64 = 10.0;
    let spiral_increment: f64 = 2.9; // 5x spiral_increment to reach same size in 1/5 the arcs
    let angular_step: f64 = std::f64::consts::PI / 20.0;

    // SPIRAL 1
    let mut angle1: f64 = 0.0;
    let mut radius1: f64 = inner_radius;
    let spiral1_start_angle = angle1;
    let spiral1_start_radius = radius1;

    for i in 0..num_arcs {
        let start_x = center_x + radius1 * angle1.cos();
        let start_y = center_y + radius1 * angle1.sin();

        angle1 += angular_step;
        radius1 += spiral_increment;

        let end_x = center_x + radius1 * angle1.cos();
        let end_y = center_y + radius1 * angle1.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        arcs.push(arc);
    }

    let spiral1_end_angle = angle1;
    let spiral1_end_radius = radius1;

    // SPIRAL 2: Generate spiral2 first, then reverse it
    let mut spiral2_arcs = Vec::new();
    let spiral2_start_angle = std::f64::consts::PI;
    let spiral2_start_radius = inner_radius;

    let mut angle2: f64 = spiral2_start_angle;
    let mut radius2: f64 = spiral2_start_radius;
    let extra_arcs = (std::f64::consts::PI / angular_step).ceil() as usize;
    let num_arcs_spiral2 = num_arcs + extra_arcs;

    for i in 0..num_arcs_spiral2 {
        let start_x = center_x + radius2 * angle2.cos();
        let start_y = center_y + radius2 * angle2.sin();

        angle2 += angular_step;
        radius2 += spiral_increment;

        let end_x = center_x + radius2 * angle2.cos();
        let end_y = center_y + radius2 * angle2.sin();

        let bulge = if i % 2 == 0 { 0.3 } else { -0.3 };

        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );

        spiral2_arcs.push(arc);
    }

    let spiral2_end_angle = angle2;
    let spiral2_end_radius = radius2;

    // Reverse spiral2 so it traverses in opposite direction
    let spiral2_reversed = arcline_reverse(&spiral2_arcs);

    // Get the last arc of spiral2_reversed - its .a endpoint is where spiral2_reversed ends
    let last_reversed_arc = spiral2_reversed[spiral2_reversed.len() - 1];

    // CONNECTION 1: Connect spiral 1 end to spiral 2 start (which is spiral2_end_angle after reversal)
    let connection1 = arc_from_bulge(
        Point::new(
            center_x + spiral1_end_radius * spiral1_end_angle.cos(),
            center_y + spiral1_end_radius * spiral1_end_angle.sin(),
        ),
        Point::new(
            center_x + spiral2_end_radius * spiral2_end_angle.cos(),
            center_y + spiral2_end_radius * spiral2_end_angle.sin(),
        ),
        0.0,
    );
    arcs.push(connection1);

    // Add reversed spiral2
    arcs.extend(spiral2_reversed);

    // CONNECTION 2: Connect from where spiral2_reversed ends back to spiral 1 start to close the loop
    // spiral2_reversed ends at last_reversed_arc.a (start of the last arc in reversed sequence)
    let connection2 = arc_from_bulge(
        last_reversed_arc.a,
        Point::new(
            center_x + spiral1_start_radius * spiral1_start_angle.cos(),
            center_y + spiral1_start_radius * spiral1_start_angle.sin(),
        ),
        -0.3, // Use arc with negative bulge to bow inward
    );
    arcs.push(connection2);

    arcs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arcline1000_len() {
        let arcline = arcline1000();
        // ~500 + (500 + 20 for half revolution) + 2 connections = ~1022 arcs
        assert!(
            arcline.len() >= 1020 && arcline.len() <= 1025,
            "Expected ~1022 arcs, got {}",
            arcline.len()
        );
    }

    #[test]
    fn test_arcline500_len() {
        let arcline = arcline500();
        // ~250 + (250 + 20 for half revolution) + 2 connections = ~522 arcs
        assert!(
            arcline.len() >= 520 && arcline.len() <= 525,
            "Expected ~522 arcs, got {}",
            arcline.len()
        );
    }

    #[test]
    fn test_arcline200_len() {
        let arcline = arcline200();
        // ~100 + (100 + 20 for half revolution) + 2 connections = ~222 arcs
        assert!(
            arcline.len() >= 220 && arcline.len() <= 225,
            "Expected ~222 arcs, got {}",
            arcline.len()
        );
    }

    #[test]
    fn test_arcline1000_svg() {
        let arcline = arcline1000();
        let mut svg = SVG::new(800.0, 800.0, Some("/tmp/arcline1000.svg"));
        svg.arcline(&arcline, "red");
        svg.write_stroke_width(0.1);

        println!("SVG written for arcline1000 inspection");
    }

    #[test]
    fn test_arcline500_svg() {
        let arcline = arcline500();
        let mut svg = SVG::new(800.0, 800.0, Some("/tmp/arcline500.svg"));
        svg.arcline(&arcline, "red");
        svg.write_stroke_width(0.1);

        println!("SVG written for arcline500 inspection");
    }

    #[test]
    fn test_arcline200_svg() {
        let arcline = arcline200();
        let mut svg = SVG::new(800.0, 800.0, Some("/tmp/arcline200.svg"));
        svg.arcline(&arcline, "red");
        svg.write_stroke_width(0.1);

        println!("SVG written for arcline200 inspection");
    }
}

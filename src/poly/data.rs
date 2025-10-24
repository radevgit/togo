use crate::prelude::*;

/// Generated 1000-arc non-intersecting spiral polyline (clean spiral outward)
/// Alternating bulge signs create a wave-like pattern
/// For benchmarking and testing spatial algorithms
pub fn poly1000() -> Arcline {
    let mut arcs = Vec::with_capacity(1000);
    
    let num_arcs = 1000;
    let center_x: f64 = 0.0;
    let center_y: f64 = 0.0;
    let base_radius: f64 = 10.0;
    let spiral_increment: f64 = 0.02;  // Increased to ensure no overlap
    let angular_step: f64 = std::f64::consts::PI / 20.0;
    
    let mut current_angle: f64 = 0.0;
    let mut current_radius: f64 = base_radius;
    
    // Pure outward spiral - no inward part to avoid intersections
    for i in 0..num_arcs {
        let start_x = center_x + current_radius * current_angle.cos();
        let start_y = center_y + current_radius * current_angle.sin();
        
        current_angle += angular_step;
        current_radius += spiral_increment;
        
        let end_x = center_x + current_radius * current_angle.cos();
        let end_y = center_y + current_radius * current_angle.sin();
        
        // Alternate bulge sign: even arcs have negative, odd have positive
        let bulge = if i % 2 == 0 { -0.3 } else { 0.3 };
        
        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );
        
        arcs.push(arc);
    }
    
    arcs
}

/// Creates a non-intersecting circular spiral polyline with 1000 arcs
pub fn poly1000_circular() -> Arcline {
    let mut arcs = Vec::with_capacity(1000);
    
    let num_arcs = 1000;
    let center_x: f64 = 500.0;
    let center_y: f64 = 500.0;
    let base_radius: f64 = 50.0;
    let spiral_radius_increment: f64 = 0.05;
    let angle_per_arc: f64 = 2.0 * std::f64::consts::PI / 10.0;
    
    let mut angle: f64 = 0.0;
    let mut radius: f64 = base_radius;
    
    for _ in 0..num_arcs {
        let start_x = center_x + radius * angle.cos();
        let start_y = center_y + radius * angle.sin();
        
        angle += angle_per_arc;
        radius += spiral_radius_increment;
        
        let end_x = center_x + radius * angle.cos();
        let end_y = center_y + radius * angle.sin();
        
        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            -0.2,
        );
        
        arcs.push(arc);
    }
    
    arcs
}

/// Creates a non-intersecting grid polyline with 1000 arcs
pub fn poly1000_grid() -> Arcline {
    let mut arcs = Vec::with_capacity(1000);
    
    let num_arcs = 1000;
    let cell_size: f64 = 5.0;
    
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut direction = 0;
    let mut steps_in_direction = 1;
    let mut steps_taken = 0;
    let mut direction_changes = 0;
    
    for _ in 0..num_arcs {
        let start_x = x;
        let start_y = y;
        
        match direction {
            0 => x += cell_size,
            1 => y += cell_size,
            2 => x -= cell_size,
            3 => y -= cell_size,
            _ => {}
        }
        
        let end_x = x;
        let end_y = y;
        
        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            0.0,
        );
        
        arcs.push(arc);
        
        steps_taken += 1;
        if steps_taken >= steps_in_direction {
            steps_taken = 0;
            direction = (direction + 1) % 4;
            direction_changes += 1;
            if direction_changes % 2 == 0 {
                steps_in_direction += 1;
            }
        }
    }
    
    arcs
}

/// Create a custom-sized non-intersecting spiral for benchmarking
/// num_arcs: number of arcs to generate
pub fn double_spiral(num_arcs: usize) -> Arcline {
    let mut arcs = Vec::with_capacity(num_arcs);
    
    let center_x: f64 = 0.0;
    let center_y: f64 = 0.0;
    let base_radius: f64 = 10.0;
    let spiral_increment: f64 = 0.02;  // Increased to ensure no overlap
    let angular_step: f64 = std::f64::consts::PI / 20.0;
    
    let mut current_angle: f64 = 0.0;
    let mut current_radius: f64 = base_radius;
    
    // Pure outward spiral - no inward part to avoid intersections
    for i in 0..num_arcs {
        let start_x = center_x + current_radius * current_angle.cos();
        let start_y = center_y + current_radius * current_angle.sin();
        
        current_angle += angular_step;
        current_radius += spiral_increment;
        
        let end_x = center_x + current_radius * current_angle.cos();
        let end_y = center_y + current_radius * current_angle.sin();
        
        let bulge = if i % 2 == 0 { -0.3 } else { 0.3 };
        
        let arc = arc_from_bulge(
            Point::new(start_x, start_y),
            Point::new(end_x, end_y),
            bulge,
        );
        
        arcs.push(arc);
    }
    
    arcs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_poly1000_len() {
        let poly = poly1000();
        assert_eq!(poly.len(), 1000);
    }
    
    #[test]
    fn test_poly1000_no_self_intersections() {
        let poly = poly1000();
        assert!(!arcline_has_self_intersection(&poly));
    }
    
    #[test]
    fn test_poly1000_circular_len() {
        let poly = poly1000_circular();
        assert_eq!(poly.len(), 1000);
    }
    
    #[test]
    fn test_poly1000_grid_len() {
        let poly = poly1000_grid();
        assert_eq!(poly.len(), 1000);
    }
}

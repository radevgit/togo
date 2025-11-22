use std::iter;

use rand::{Rng, SeedableRng, rngs::StdRng};
use togo::prelude::*;

/// Configurable number of points
const N_POINTS: usize = 10000;

fn main() {
    // Use fixed seed for reproducible results
    let mut rng = StdRng::seed_from_u64(42);
    
    // Generate random points once with fixed seed
    let points: Pointline = (0..N_POINTS)
        .map(|i| {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (N_POINTS as f64);
            let radius = 10.0 + rng.random::<f64>() * 200.0;
            point(
                300.0 + radius * angle.cos(),
                300.0 + radius * angle.sin(),
            )
        })
        .collect();

    // Compute convex hull
    let mut hull = Vec::new();
    for _ in 0..1 {
        hull = points_convex_hull(&points);
    }

    // Use the result to prevent optimization
    println!("Hull size: {}", hull.len());
    
    let mut svg = SVG::new(600.0, 600.0, Some("/tmp/hull_points.svg"));
    for point in points.iter() {
        let circle = Circle::new(*point, 0.5);
        svg.circle(&circle, "black");
    }
    svg.pointline(&hull, "red");
    svg.write_stroke_width(0.1);
}
/* 
samply record cargo run --release --example convex_hull_points

*/

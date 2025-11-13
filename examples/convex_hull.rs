use rand::{Rng, SeedableRng, rngs::StdRng};
use togo::prelude::*;

/// Configurable number of points
const N_POINTS: usize = 10000;

fn main() {
    // Use fixed seed for reproducible results
    let mut rng = StdRng::seed_from_u64(42);
    
    // Generate random points once with fixed seed
    let points: Pointline = (0..N_POINTS)
        .map(|_| point(
            rng.random::<f64>() * 10000.0,
            rng.random::<f64>() * 10000.0,
        ))
        .collect();

    // Compute convex hull
    let mut hull = Vec::new();
    for _ in 0..500 {
        hull = points_convex_hull(&points);
    }

    // Use the result to prevent optimization
    println!("Hull size: {}", hull.len());
}
/* 
samply record cargo run --release --example convex_hull

*/

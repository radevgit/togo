use std::time::Instant;
use rand::{Rng, SeedableRng, rngs::StdRng};
use togo::prelude::*;

fn generate_points(seed: u64, count: usize) -> Pointline {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| point(
            rng.random::<f64>() * 10000.0,
            rng.random::<f64>() * 10000.0,
        ))
        .collect()
}

fn benchmark_convex_hull(name: &str, points: &Pointline, iterations: usize) {
    let start = Instant::now();
    let mut hull = Vec::new();
    for _ in 0..iterations {
        hull = pointline_convex_hull(points);
    }
    let elapsed = start.elapsed();
    
    let avg_time = elapsed.as_micros() as f64 / iterations as f64;
    println!(
        "{}: {} iterations, total: {:?}, avg: {:.2}µs, hull size: {}",
        name, iterations, elapsed, avg_time, hull.len()
    );
}

fn main() {
    println!("Convex Hull Benchmark\n");

    // Benchmark with 10,000 points
    println!("=== 10,000 points ===");
    let points_10k = generate_points(42, 10_000);
    benchmark_convex_hull("10k points", &points_10k, 5000);

    println!();

    // Benchmark with 100,000 points
    println!("=== 100,000 points ===");
    let points_100k = generate_points(42, 100_000);
    benchmark_convex_hull("100k points", &points_100k, 500);

    println!();

    // Benchmark with 1,000,000 points
    println!("=== 1,000,000 points ===");
    let points_1m = generate_points(42, 1_000_000);
    benchmark_convex_hull("100k points", &points_1m, 50);
}

/*
cargo bench --bench hull_bench

Convex Hull Benchmark

=== 10,000 points ===
10k points: 5000 iterations, total: 5.385497557s, avg: 1077.10µs, hull size: 23

=== 100,000 points ===
100k points: 500 iterations, total: 7.258992381s, avg: 14517.98µs, hull size: 31

=== 1,000,000 points ===
100k points: 50 iterations, total: 8.532761668s, avg: 170655.22µs, hull size: 33
________________________________________________________________________________
Perp optimization.
=== 10,000 points ===
10k points: 5000 iterations, total: 2.713066241s, avg: 542.61µs, hull size: 23

=== 100,000 points ===
100k points: 500 iterations, total: 3.52370469s, avg: 7047.41µs, hull size: 31

=== 1,000,000 points ===
100k points: 50 iterations, total: 4.128193554s, avg: 82563.86µs, hull size: 33
________________________________________________________________________________

*/

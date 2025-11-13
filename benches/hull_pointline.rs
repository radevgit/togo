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
    println!("Convex Hull Pointline \n");


    // Benchmark with 1,000 points
    println!("=== 1,000 points ===");
    let points_1000 = generate_points(42, 1_000);
    benchmark_convex_hull("1,000 points", &points_1000, 500000);

    println!();

    // Benchmark with 10,000 points
    println!("=== 10,000 points ===");
    let points_10k = generate_points(42, 10_000);
    benchmark_convex_hull("10k points", &points_10k, 50000);

    println!();

    // Benchmark with 100,000 points
    println!("=== 100,000 points ===");
    let points_100k = generate_points(42, 100_000);
    benchmark_convex_hull("100k points", &points_100k, 5000);

    println!();

    // Benchmark with 1,000,000 points
    println!("=== 1,000,000 points ===");
    let points_1m = generate_points(42, 1_000_000);
    benchmark_convex_hull("1m points", &points_1m, 500);
}

/*
cargo bench --bench hull_pointline

Convex Hull Pointline 

=== 1,000 points ===
1,000 points: 500000 iterations, total: 1.24657293s, avg: 2.49µs, hull size: 496

=== 10,000 points ===
10k points: 50000 iterations, total: 2.882304454s, avg: 57.65µs, hull size: 4981

=== 100,000 points ===
100k points: 5000 iterations, total: 3.173074294s, avg: 634.61µs, hull size: 50176

=== 1,000,000 points ===
100k points: 500 iterations, total: 3.331625309s, avg: 6663.25µs, hull size: 499295
___________________________________________________________________________________

*/

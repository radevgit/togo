use std::time::Instant;
use togo::prelude::*;

fn main() {
    println!("\n========================================");
    println!("Self-Intersection Check Benchmark");
    println!("========================================\n");
    
    let poly = togo::poly::data::arcline1000();
    println!("Spiral polyline: {} arcs", poly.len());
    println!("Total arc pairs to check: {}", poly.len() * (poly.len() - 1) / 2);
    
    // Warmup
    let _ = arcline_has_self_intersection(&poly);
    
    // Actual benchmark
    let start1000 = Instant::now();
    let mut has_intersection1000 = true;
    for _ in 0..1000 {
        has_intersection1000 = arcline_has_self_intersection(&poly);
    }
    let elapsed1000 = start1000.elapsed();

    let start200 = Instant::now();
    let mut has_intersection200 = true;
    for _ in 0..10000 {
        has_intersection200 = arcline_has_self_intersection(&poly);
    }
    let elapsed200 = start200.elapsed();
    
    // ASSERT: poly1000 should have no self-intersections
    assert!(!has_intersection1000 && !has_intersection200, "poly1000 must have no self-intersections");
    
    println!("Time for arcline1000 (1_000 it): {:.4} ms ({:.0} µs)", 
             elapsed1000.as_secs_f64() * 1000.0 / 1000.0,
             elapsed1000.as_secs_f64() * 1_000_000.0 / 1000.0);
    println!("Time for arcline200 (10_000 it): {:.4} ms ({:.0} µs)\n", 
             elapsed200.as_secs_f64() * 1000.0 / 1000.0,
             elapsed200.as_secs_f64() * 1_000_000.0 / 10000.0);
    println!("========================================");
}

/*
cargo bench --bench bench_self_intersection


Spiral polyline: 1022 arcs
Total arc pairs to check: 521731


Time: 4.4790 ms (4479 µs) Base

Time: 1.4383 ms (1438 µs) Hilbert

Time: 0.9664 ms (966 µs) AABB v0.5

Time: 0.9530 ms (953 µs) AABB v0.6.6

Time: 0.8865 ms (886 µs) AABB v0.6.7

*/

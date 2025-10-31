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
    let start = Instant::now();
    let mut has_intersection = true;
    for _ in 0..1000 {
        has_intersection = arcline_has_self_intersection(&poly);
    }
    let elapsed = start.elapsed();
    
    // ASSERT: poly1000 should have no self-intersections
    assert!(!has_intersection, "poly1000 must have no self-intersections");
    
    println!("\nResult: {} (no self-intersections verified)", if has_intersection { "INTERSECTS" } else { "CLEAN" });
    println!("Time: {:.4} ms ({:.0} µs)\n", 
             elapsed.as_secs_f64() * 1000.0 / 1000.0,
             elapsed.as_secs_f64() * 1_000_000.0 / 1000.0);
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

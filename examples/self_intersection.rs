//! Self-Intersection Detection Example
//!
//! This example demonstrates how to check for self-intersections in complex
//! polylines using the `arcline_has_self_intersection` function. It showcases
//! the performance of the spatial indexing optimization for large polylines.
//!
//! Run with:
//! ```
//! cargo run --example self_intersection
//! ```

use std::time::Instant;
use togo::prelude::*;

fn main() {
    println!("\n========================================");
    println!("Self-Intersection Check Example");
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
cargo build --release --example perf
./target/release/examples/perf
samply record cargo run --release --example perf

*/
use std::hint::black_box;
use std::time::Instant;
use togo::prelude::*;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

const AREA: f64 = 4.0;

fn random_orect(rng: &mut Lcg) -> OrientedRect {
    // robot-like sizes (metres) in an AREA x AREA area
    OrientedRect::from_angle(
        point(rng.range(0.0, AREA), rng.range(0.0, AREA)),
        rng.range(-3.15, 3.15),
        rng.range(0.4, 1.2),
        rng.range(0.3, 0.8),
    )
}

fn main() {
    const PAIRS: usize = 1000;
    const ITERS: usize = 10_000;

    let mut rng = Lcg(2024);
    let rects_a: Vec<OrientedRect> = (0..PAIRS).map(|_| random_orect(&mut rng)).collect();
    let rects_b: Vec<OrientedRect> = (0..PAIRS).map(|_| random_orect(&mut rng)).collect();
    let circles: Vec<Circle> = (0..PAIRS)
        .map(|_| circle(point(rng.range(0.0, AREA), rng.range(0.0, AREA)), rng.range(0.3, 1.0)))
        .collect();

    println!("\n========================================");
    println!("Overlap Benchmark ({PAIRS} random pairs x {ITERS} iterations)");
    println!("========================================\n");

    // Warmup + hit counts (so the work is not optimised away and the mix is visible)
    let hits_rr = (0..PAIRS).filter(|&i| overlap_orect_orect(&rects_a[i], &rects_b[i])).count();
    let hits_cr = (0..PAIRS).filter(|&i| overlap_circle_orect(&circles[i], &rects_b[i])).count();

    let start = Instant::now();
    let mut count = 0usize;
    for _ in 0..ITERS {
        for i in 0..PAIRS {
            count += overlap_orect_orect(black_box(&rects_a[i]), black_box(&rects_b[i])) as usize;
        }
    }
    let t_rr = start.elapsed();
    assert_eq!(count, hits_rr * ITERS);

    let start = Instant::now();
    let mut count = 0usize;
    for _ in 0..ITERS {
        for i in 0..PAIRS {
            count += overlap_circle_orect(black_box(&circles[i]), black_box(&rects_b[i])) as usize;
        }
    }
    let t_cr = start.elapsed();
    assert_eq!(count, hits_cr * ITERS);

    let total = (PAIRS * ITERS) as f64;
    println!(
        "overlap_orect_orect : {:6.1} ns/pair  ({:.3} ms per {PAIRS} pairs, {} overlapping)",
        t_rr.as_secs_f64() * 1e9 / total,
        t_rr.as_secs_f64() * 1e3 / ITERS as f64,
        hits_rr
    );
    println!(
        "overlap_circle_orect: {:6.1} ns/pair  ({:.3} ms per {PAIRS} pairs, {} overlapping)",
        t_cr.as_secs_f64() * 1e9 / total,
        t_cr.as_secs_f64() * 1e3 / ITERS as f64,
        hits_cr
    );
    println!("========================================");
}

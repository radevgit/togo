use basegeom::prelude::*;
use basegeom::algo::arc_bounding_rect;

fn main() {
    // Test the quarter circle case
    println!("=== Quarter Circle Test ===");
    let center = point(0.0, 0.0);
    let radius = 1.0;
    let start = point(1.0, 0.0);   // 0°
    let end = point(0.0, 1.0);     // 90°
    
    let quarter_circle = arc(start, end, center, radius);
    let bounding = arc_bounding_rect(&quarter_circle);
    
    println!("Arc: start={:?}, end={:?}, center={:?}, radius={}", start, end, center, radius);
    println!("Bounding rect: p1={:?}, p2={:?}", bounding.p1, bounding.p2);
    println!("Width: {}, Height: {}", bounding.p2.x - bounding.p1.x, bounding.p2.y - bounding.p1.y);
    
    println!("\n=== Line Segment Test ===");
    let line = arcseg(point(1.0, 2.0), point(4.0, 6.0));
    let line_bounding = arc_bounding_rect(&line);
    
    println!("Line: start={:?}, end={:?}", line.a, line.b);
    println!("Bounding rect: p1={:?}, p2={:?}", line_bounding.p1, line_bounding.p2);
    println!("Width: {}, Height: {}", line_bounding.p2.x - line_bounding.p1.x, line_bounding.p2.y - line_bounding.p1.y);
}

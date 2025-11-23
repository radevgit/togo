//! Convex hull algorithm for Arcline (arc-based polygons).
//!
//! This module implements convex hull computation for polygons defined by arcs and line segments,
//! preserving the arc structure where possible.

use crate::intersection::tangent::{external_tangents_between_circles, tangent_point_to_circle};
use crate::prelude::*;

#[cfg(test)]
mod tests;

/// Finds the starting arc index for convex hull construction.
fn find_starting_arc(arcs: &Arcline) -> usize {
    let mut idx = usize::MAX;
    let mut min_y = f64::INFINITY;
    for (i, arc) in arcs.iter().enumerate() {
        let l = left_point(*arc);
        if l < min_y {
            min_y = l;
            idx = i;
        }
    }
    idx
}

/// Finds most left point of an arc
fn left_point(arc: Arc) -> f64 {
    if arc.is_seg() {
        return arc.a.x.min(arc.b.x);
    }
    let left = point(arc.c.x - arc.r, arc.c.y);
    if arc.contains(left) {
        return left.x;
    }
    return arc.a.x.min(arc.b.x);
}

fn find_tangent_point_to_circle(from: Point, c: Circle) -> Option<Point> {
    match tangent_point_to_circle(from, c) {
        Some((t1, t2)) => {
            let orient = (t1 - from).perp(t2 - from);
            if orient > 0.0 { Some(t1) } else { Some(t2) }
        }
        None => None,
    }
}

fn find_tangent_circle_to_circle(c1: Circle, c2: Circle) -> Option<(Point, Point)> {
    match external_tangents_between_circles(c1, c2) {
        Some((t1_c1, t1_c2, t2_c1, t2_c2)) => {
            // find the right side points
            let orient1 = (c2.c - c1.c).perp(t1_c1 - c1.c);
            let rc1 = if orient1 > 0.0 { t1_c1 } else { t2_c1 };
            let orient2 = (c2.c - c1.c).perp(t1_c2 - c2.c);
            let rc2 = if orient2 > 0.0 { t1_c2 } else { t2_c2 };
            Some((rc1, rc2))
        }
        None => None,
    }
}

/// Compare two connections and return true if conn1 is "better" (more to the right) than conn2
/// 
/// For convex hull, the rightmost connection is determined by:
/// 1. Connection that starts closer to current_arc.a (beginning of current arc)
/// 2. Among those, the one that ends further along the target arc (closer to its end)
/// 
/// This ensures we wrap tightly around the polygon for the convex hull.
fn is_better_connection(conn1: &[Arc], conn2: &[Arc], current_arc: Arc) -> bool {
    if conn1.is_empty() {
        return false;
    }
    if conn2.is_empty() {
        return true;
    }
    
    // For gift-wrapping, we want the connection that turns most to the right
    // from the current arc's direction. This means the smallest left turn.
    
    // Get the direction vector of current_arc
    let current_dir = current_arc.b - current_arc.a;
    
    // Get the direction vector of each connection's first segment
    // This represents the direction we'd be heading after leaving current_arc
    let conn1_dir = conn1[0].b - conn1[0].a;
    let conn2_dir = conn2[0].b - conn2[0].a;
    
    let epsilon = 1e-9;
    
    if current_dir.norm() < epsilon || conn1_dir.norm() < epsilon || conn2_dir.norm() < epsilon {
        // Degenerate case
        return conn1_dir.norm() > conn2_dir.norm();
    }
    
    // Cross product: current_dir × conn_dir
    // Negative cross means clockwise turn (right turn), positive means CCW turn (left turn)
    // We want the MOST NEGATIVE (most right turn) or LEAST POSITIVE (least left turn)
    let cross1 = current_dir.perp(conn1_dir);
    let cross2 = current_dir.perp(conn2_dir);
    
    // Prefer the connection with smaller cross product (more right / less left)
    cross1 < cross2
}

/// Returns true if dir1 is more to the right (more clockwise) than dir2
/// when both directions start from the same point.
/// 
/// Uses cross product to determine relative orientation:
/// - cross < 0: dir1 is clockwise from dir2 (dir1 is to the right)
/// - cross > 0: dir1 is counter-clockwise from dir2 (dir1 is to the left)
fn is_better_direction(dir1: Point, dir2: Point) -> bool {
    let cross = dir1.perp(dir2);
    
    if cross.abs() < 1e-9 {
        // Directions are collinear, prefer the one that reaches further
        return dir1.norm() > dir2.norm();
    }
    
    // dir1 is better if it's more clockwise (to the right), i.e., cross < 0
    cross < 0.0
}

#[must_use]
fn new_convex_hull(arcs: &Arcline) -> Arcline {
    let mut segs = arcs.clone();
    
    if arcs.is_empty() {
        return Arcline::new();
    }

    let n = arcs.len();
    let mut convex = Vec::with_capacity(n);
    convex.resize(n, false);
    
    // Convert concave to properly oriented arcseg
    for i in 0..n {
        if arcs[i].is_seg() {
            convex[i] = false;
            continue;
        }
        if is_arc_convex(arcs, i) {
            convex[i] = true;
        } else {
            segs[i] = arcseg(segs[i].b, segs[i].a);
            convex[i] = false;
        }
    }
    
    let start_idx = find_starting_arc(&segs);
    let mut hull = Arcline::new();
    let mut current_idx = start_idx;
    let mut iterations = 0;
    
    // Gift wrapping: from current arc, find best connection to any other arc
    loop {
        let current_arc = segs[current_idx];
        
        let mut best_connection: Option<Vec<Arc>> = None;
        let mut best_next_idx: Option<usize> = None;
        
        // Try connecting to all other arcs (including wrapping back to start)
        for i in 1..n {
            let next_idx = (current_idx + i) % n;
            
            let next_arc = segs[next_idx];
            
            // Compute hull connection based on types
            let connection = if current_arc.is_seg() && next_arc.is_seg() {
                vec![hull_seg_seg(current_arc, next_arc)]
            } else if current_arc.is_seg() && !next_arc.is_seg() {
                hull_seg_arc(current_arc, next_arc)
            } else if !current_arc.is_seg() && next_arc.is_seg() {
                hull_arc_seg(current_arc, next_arc)
            } else {
                hull_arc_arc(current_arc, next_arc)
            };
            
            // Compare with current best
            if best_connection.is_none() || is_better_connection(&connection, best_connection.as_ref().unwrap(), current_arc) {
                best_connection = Some(connection);
                best_next_idx = Some(next_idx);
            }
        }
        
        // Also check connection back to self (for wrapping, e.g., large arc containing everything)
        let self_connection = if current_arc.is_seg() {
            vec![hull_seg_seg(current_arc, current_arc)]
        } else {
            hull_arc_arc(current_arc, current_arc)
        };
        
        if best_connection.is_none() || is_better_connection(&self_connection, best_connection.as_ref().unwrap(), current_arc) {
            best_connection = Some(self_connection);
            best_next_idx = Some(current_idx);
        }
        
        // Add the best connection to hull
        if let Some(connection) = best_connection {
            for arc in connection {
                hull.push(arc);
            }
        }
        
        // Move to next arc
        if let Some(next_idx) = best_next_idx {
            current_idx = next_idx;
            iterations += 1;
            
            // Check if we've completed the loop back to start
            if current_idx == start_idx && iterations > 1 {
                // We've wrapped back to start after processing at least one arc
                break;
            }
            
            // Safety check to prevent infinite loops
            if iterations > n {
                break;
            }
        } else {
            break; // No valid connection found
        }
    }
    
    hull
}

fn hull_seg_seg(seg1: Arc, seg2: Arc) -> Arc {
    // If segments are already connected (seg1.b == seg2.a), return seg1
    if seg1.b == seg2.a {
        return seg1;
    }
    
    // Four possible connections between segment endpoints:
    // 1. seg1.a -> seg2.a
    // 2. seg1.a -> seg2.b
    // 3. seg1.b -> seg2.a
    // 4. seg1.b -> seg2.b
    
    let candidates = [
        (seg1.a, seg2.a),
        (seg1.a, seg2.b),
        (seg1.b, seg2.a),
        (seg1.b, seg2.b),
    ];
    
    let mut best_connection: Option<(Point, Point)> = None;
    let mut best_angle = f64::NEG_INFINITY;
    
    for &(p1, p2) in &candidates {
        let connection_dir = p2 - p1;
        
        if connection_dir.norm() < 1e-9 {
            continue; // Skip zero-length connections
        }
        
        let mut valid = true;
        
        // Check if the other endpoint of seg1 is on the right side (or on the line)
        let other_seg1 = if p1 == seg1.a { seg1.b } else { seg1.a };
        let to_other1 = other_seg1 - p1;
        let cross1 = connection_dir.perp(to_other1);
        
        // For CCW hull, the other point should not be on the left (cross > 0 is invalid)
        if cross1 > 1e-9 {
            valid = false;
        }
        
        // Check if the other endpoint of seg2 is on the right side (or on the line)
        if valid {
            let other_seg2 = if p2 == seg2.a { seg2.b } else { seg2.a };
            let to_other2 = other_seg2 - p1;
            let cross2 = connection_dir.perp(to_other2);
            
            // For CCW hull, the other point should not be on the left
            if cross2 > 1e-9 {
                valid = false;
            }
        }
        
        // Among valid connections, choose the one that makes the most CCW turn
        // We can use the angle or just compare cross products with a reference direction
        if valid {
            // Use the angle of the connection direction as a tie-breaker
            // For CCW hull, we want the connection that turns most to the left
            let angle = connection_dir.y.atan2(connection_dir.x);
            
            if best_connection.is_none() || angle > best_angle {
                best_angle = angle;
                best_connection = Some((p1, p2));
            }
        }
    }
    
    // Return the best connection, or default to seg1.b -> seg2.a if none found
    let (p1, p2) = best_connection.unwrap_or((seg1.b, seg2.a));
    arcseg(p1, p2)
}

fn hull_arc_arc(arc1: Arc, arc2: Arc) -> Vec<Arc> {
    let mut result = Vec::new();
    
    // Get the circles that the arcs belong to
    let c1 = Circle { c: arc1.c, r: arc1.r };
    let c2 = Circle { c: arc2.c, r: arc2.r };
    
    // Find the external tangent between the two circles (right-side tangent)
    let tangent_opt = find_tangent_circle_to_circle(c1, c2);
    
    if tangent_opt.is_none() {
        // No external tangent exists (concentric or nested circles)
        // Try to find the best connection between arc endpoints
        
        // Four possible connections
        let candidates = [
            (arc1.a, arc2.a),
            (arc1.a, arc2.b),
            (arc1.b, arc2.a),
            (arc1.b, arc2.b),
        ];
        
        // Check each connection to see if it crosses the arcs
        let mut best_connection: Option<(Point, Point)> = None;
        
        for &(p1, p2) in &candidates {
            let connection_dir = p2 - p1;
            
            if connection_dir.norm() < 1e-9 {
                continue;
            }
            
            // Check if this connection crosses arc1
            // For an arc, we need to check if intermediate points would be on the wrong side
            let other1 = if p1 == arc1.a { arc1.b } else { arc1.a };
            let to_other1 = other1 - p1;
            let cross1 = connection_dir.perp(to_other1);
            
            // Check if this connection crosses arc2
            let other2 = if p2 == arc2.a { arc2.b } else { arc2.a };
            let to_other2 = other2 - p1;
            let cross2 = connection_dir.perp(to_other2);
            
            // If both checks pass (other endpoints not on left), this is a valid direct connection
            if cross1 <= 1e-9 && cross2 <= 1e-9 {
                best_connection = Some((p1, p2));
                break;
            }
        }
        
        if let Some((p1, p2)) = best_connection {
            // Direct connection is valid
            if !p1.close_enough(p2, 1e-9) {
                result.push(arcseg(p1, p2));
            }
        } else {
            // No direct connection works, need to use point-to-arc tangent
            // Try tangent from arc1 endpoints to arc2
            let tangent_from_arc1_b = find_tangent_point_to_circle(arc1.b, c2);
            
            if let Some(t2) = tangent_from_arc1_b {
                if arc2.contains(t2) {
                    // Tangent from arc1.b to t2 on arc2
                    result.push(arcseg(arc1.b, t2));
                    
                    // Add remaining portion of arc2
                    if !t2.close_enough(arc2.b, 1e-9) {
                        result.push(arc(t2, arc2.b, arc2.c, arc2.r));
                    }
                    return result;
                }
            }
            
            // Try tangent from arc2.a to arc1
            let tangent_from_arc2_a = find_tangent_point_to_circle(arc2.a, c1);
            
            if let Some(t1) = tangent_from_arc2_a {
                if arc1.contains(t1) {
                    // Add portion of arc1 to tangent point
                    if !arc1.a.close_enough(t1, 1e-9) {
                        result.push(arc(arc1.a, t1, arc1.c, arc1.r));
                    }
                    
                    // Tangent from t1 on arc1 to arc2.a
                    result.push(arcseg(t1, arc2.a));
                    return result;
                }
            }
            
            // Fall back to endpoint connection if nothing else works
            result.push(arcseg(arc1.b, arc2.a));
        }
        
        return result;
    }
    
    let (t1, t2) = tangent_opt.unwrap();
    
    // Check if tangent points are contained within the arcs
    let t1_in_arc1 = arc1.contains(t1);
    let t2_in_arc2 = arc2.contains(t2);
    
    match (t1_in_arc1, t2_in_arc2) {
        (true, true) => {
            // Both tangent points are on the arcs
            // Hull: part of arc1 (from arc1.a to t1), tangent line, part of arc2 (from t2 to arc2.b)
            
            // Add the portion of arc1 from arc1.a to t1
            if !arc1.a.close_enough(t1, 1e-9) {
                result.push(arc(arc1.a, t1, arc1.c, arc1.r));
            }
            
            // Add the tangent line segment
            if !t1.close_enough(t2, 1e-9) {
                result.push(arcseg(t1, t2));
            }
            
            // Add the portion of arc2 from t2 to arc2.b
            if !t2.close_enough(arc2.b, 1e-9) {
                result.push(arc(t2, arc2.b, arc2.c, arc2.r));
            }
        }
        
        (false, true) => {
            // t1 is outside arc1, t2 is inside arc2
            // Need to determine which endpoint of arc1 to use
            
            // The tangent should start from whichever end of arc1 is "closer" to t1
            // in the sense of being on the convex hull
            let start_point = if arc1.b.close_enough(t1, 1e-9) {
                arc1.b
            } else {
                // Check which endpoint makes a valid hull connection
                // Use arc1.b as it's the natural continuation point
                arc1.b
            };
            
            // Add tangent line from arc1 endpoint to t2
            if !start_point.close_enough(t2, 1e-9) {
                result.push(arcseg(start_point, t2));
            }
            
            // Add the portion of arc2 from t2 to arc2.b
            if !t2.close_enough(arc2.b, 1e-9) {
                result.push(arc(t2, arc2.b, arc2.c, arc2.r));
            }
        }
        
        (true, false) => {
            // t1 is inside arc1, t2 is outside arc2
            // Need to determine which endpoint of arc2 to use
            
            // Add the portion of arc1 from arc1.a to t1
            if !arc1.a.close_enough(t1, 1e-9) {
                result.push(arc(arc1.a, t1, arc1.c, arc1.r));
            }
            
            // The tangent should end at whichever end of arc2 is "closer" to t2
            let end_point = if arc2.a.close_enough(t2, 1e-9) {
                arc2.a
            } else {
                // Use arc2.a as it's the natural starting point
                arc2.a
            };
            
            // Add tangent line from t1 to arc2 endpoint
            if !t1.close_enough(end_point, 1e-9) {
                result.push(arcseg(t1, end_point));
            }
        }
        
        (false, false) => {
            // Both tangent points are outside the arcs
            // Connect arc1 endpoint to arc2 endpoint
            
            // Use arc1.b and arc2.a as the natural connection points
            let start_point = arc1.b;
            let end_point = arc2.a;
            
            if !start_point.close_enough(end_point, 1e-9) {
                result.push(arcseg(start_point, end_point));
            }
        }
    }
    
    result
}

fn hull_seg_arc(seg1: Arc, arc2: Arc) -> Vec<Arc> {
    let mut result = Vec::new();
    
    // Get the circle that arc2 belongs to
    let c2 = Circle { c: arc2.c, r: arc2.r };
    
    // Try to find tangent from each endpoint of seg1 to the circle of arc2
    let tangent_from_a = find_tangent_point_to_circle(seg1.a, c2);
    let tangent_from_b = find_tangent_point_to_circle(seg1.b, c2);
    
    // Check tangent from seg1.b first (as it's the natural continuation point)
    if let Some(t_b) = tangent_from_b {
        // Check if tangent point is on arc2
        if arc2.contains(t_b) {
            // Valid tangent: seg1.b -> t_b (tangent line), then arc from t_b to arc2.b
            if !seg1.b.close_enough(t_b, 1e-9) {
                result.push(arcseg(seg1.b, t_b));
            }
            
            // Add remaining portion of arc2 from tangent point to end
            if !t_b.close_enough(arc2.b, 1e-9) {
                result.push(arc(t_b, arc2.b, arc2.c, arc2.r));
            }
            
            return result;
        }
    }
    
    // Check tangent from seg1.a 
    if let Some(t_a) = tangent_from_a {
        // Check if tangent point is on arc2
        if arc2.contains(t_a) {
            // Valid tangent: seg1.a -> t_a (tangent line), then arc from t_a to arc2.b
            if !seg1.a.close_enough(t_a, 1e-9) {
                result.push(arcseg(seg1.a, t_a));
            }
            
            // Add remaining portion of arc2 from tangent point to end
            if !t_a.close_enough(arc2.b, 1e-9) {
                result.push(arc(t_a, arc2.b, arc2.c, arc2.r));
            }
            
            return result;
        }
    }
    
    // No tangent point on the arc, try direct connections to arc endpoints
    let candidates = [
        (seg1.b, arc2.a),
        (seg1.a, arc2.a),
        (seg1.b, arc2.b),
        (seg1.a, arc2.b),
    ];
    
    for &(p1, p2) in &candidates {
        let connection_dir = p2 - p1;
        if connection_dir.norm() < 1e-9 {
            continue;
        }
        
        let other_seg = if p1 == seg1.a { seg1.b } else { seg1.a };
        let to_other = other_seg - p1;
        let cross = connection_dir.perp(to_other);
        
        if cross <= 1e-9 {
            // Valid connection
            result.push(arcseg(p1, p2));
            
            // If connected to arc2.a, include the full arc
            if p2 == arc2.a {
                result.push(arc2);
            }
            
            return result;
        }
    }
    
    // Fallback: direct connection from seg1.b to arc2.a
    if !seg1.b.close_enough(arc2.a, 1e-9) {
        result.push(arcseg(seg1.b, arc2.a));
    }
    result.push(arc2);
    
    result
}

fn hull_arc_seg(arc1: Arc, seg2: Arc) -> Vec<Arc> {
    let mut result = Vec::new();
    
    // Get the circle that arc1 belongs to
    let c1 = Circle { c: arc1.c, r: arc1.r };
    
    // Try to find tangent from arc1's circle to each endpoint of seg2
    let tangent_to_a = find_tangent_point_to_circle(seg2.a, c1);
    let tangent_to_b = find_tangent_point_to_circle(seg2.b, c1);
    
    // Check tangent to seg2.a first (as it's the natural continuation point)
    if let Some(t_a) = tangent_to_a {
        // Check if tangent point is on arc1
        if arc1.contains(t_a) {
            // Valid tangent: arc portion from arc1.a to t_a, then tangent line t_a -> seg2.a
            if !arc1.a.close_enough(t_a, 1e-9) {
                result.push(arc(arc1.a, t_a, arc1.c, arc1.r));
            }
            
            // Add tangent line from arc tangent point to seg2 start
            if !t_a.close_enough(seg2.a, 1e-9) {
                result.push(arcseg(t_a, seg2.a));
            }
            
            return result;
        }
    }
    
    // Check tangent to seg2.b
    if let Some(t_b) = tangent_to_b {
        // Check if tangent point is on arc1
        if arc1.contains(t_b) {
            // Valid tangent: arc portion from arc1.a to t_b, then tangent line t_b -> seg2.b
            if !arc1.a.close_enough(t_b, 1e-9) {
                result.push(arc(arc1.a, t_b, arc1.c, arc1.r));
            }
            
            // Add tangent line from arc tangent point to seg2 end
            if !t_b.close_enough(seg2.b, 1e-9) {
                result.push(arcseg(t_b, seg2.b));
            }
            
            return result;
        }
    }
    
    // No tangent point on the arc, try direct connections from arc endpoints to seg2
    let candidates = [
        (arc1.b, seg2.a),
        (arc1.a, seg2.a),
        (arc1.b, seg2.b),
        (arc1.a, seg2.b),
    ];
    
    for &(p1, p2) in &candidates {
        let connection_dir = p2 - p1;
        if connection_dir.norm() < 1e-9 {
            continue;
        }
        
        let other_arc = if p1 == arc1.a { arc1.b } else { arc1.a };
        let to_other = other_arc - p1;
        let cross = connection_dir.perp(to_other);
        
        if cross <= 1e-9 {
            // Valid connection
            // If connected from arc1.a, don't include arc portion
            if p1 != arc1.a && !arc1.a.close_enough(p1, 1e-9) {
                result.push(arc(arc1.a, p1, arc1.c, arc1.r));
            }
            
            result.push(arcseg(p1, p2));
            
            return result;
        }
    }
    
    // Fallback: direct connection from arc1.b to seg2.a
    if !arc1.a.close_enough(arc1.b, 1e-9) {
        result.push(arc(arc1.a, arc1.b, arc1.c, arc1.r));
    }
    if !arc1.b.close_enough(seg2.a, 1e-9) {
        result.push(arcseg(arc1.b, seg2.a));
    }
    
    result
}

/// Computes the convex hull of an arcline (arc-based polygon).
///
/// This function computes the convex hull while attempting to preserve arc segments
/// where they lie on the convex boundary. Concave arcs are replaced with line segments
/// or tangent connections.
///
/// # Algorithm Overview
///
/// 1. **Extract all candidate points**: Arc endpoints + cardinal extrema for curved arcs
/// 2. **Compute point-based hull**: Use gift wrapping on all candidates
/// 3. **Convert to arcline**: Create line segments connecting hull vertices
///
/// Future enhancements will preserve original arc segments where they lie on the hull.
///
/// # Arguments
///
/// * `arcs` - The input arcline (closed, non-self-intersecting, CCW)
///
/// # Returns
///
/// A new `Arcline` representing the convex hull
///
/// # Examples
///
/// ```ignore
/// use togo::prelude::*;
/// use togo::algo::convex_hull_arcs::arcline_convex_hull;
///
/// // Circle made of 4 quarter-circle arcs
/// let arcs = vec![
///     arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0),
///     arc(point(0.0, 1.0), point(-1.0, 0.0), point(0.0, 0.0), 1.0),
///     arc(point(-1.0, 0.0), point(0.0, -1.0), point(0.0, 0.0), 1.0),
///     arc(point(0.0, -1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0),
/// ];
/// let hull = arcline_convex_hull(&arcs);
/// assert_eq!(hull.len(), 4); // All arcs are convex
/// ```
#[must_use]
pub fn arcline_convex_hull(arcs: &Arcline) -> Arcline {
    new_convex_hull(arcs)
}

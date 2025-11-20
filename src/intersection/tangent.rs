//! Tangent computation functions for convex hull construction.
//!
//! This module provides geometric algorithms to compute tangent lines from points to circles
//! and between circles, without using trigonometric functions (sin, cos, atan, etc.).
//!
//! All algorithms use pure geometric approaches based on vector operations and the Pythagorean theorem.

use crate::prelude::*;

/// Computes the two tangent points from an external point to a circle.
///
/// Given a point P outside a circle with center C and radius r, there are exactly two
/// tangent lines from P to the circle. This function returns the two tangent points T1 and T2
/// on the circle where these tangent lines touch.
///
/// # Algorithm (Geometric, No Trigonometry)
///
/// The key insight is that the tangent point T, center C, and external point P form a right triangle,
/// where the angle at T is 90 degrees (radius perpendicular to tangent).
///
/// 1. Let d = distance from P to C
/// 2. Let h = distance from P to tangent point T
/// 3. By Pythagorean theorem: h² + r² = d²
/// 4. Therefore: h = sqrt(d² - r²)
///
/// To find the tangent points without angles:
/// 1. Create vector PC from P to C
/// 2. Find the perpendicular vector to PC (rotate 90°)
/// 3. Scale vectors appropriately using the distances computed above
/// 4. Tangent points are: C + (scaled vectors)
///
/// # Arguments
///
/// * `point` - External point from which tangents are drawn
/// * `circle` - Circle to which tangents are drawn
///
/// # Returns
///
/// `Some((t1, t2))` - Two tangent points on the circle, ordered such that:
/// - `t1` is the "left" tangent (CCW from the point's perspective looking toward center)
/// - `t2` is the "right" tangent (CW from the point's perspective looking toward center)
///
/// Returns `None` if:
/// - Point is inside or on the circle (no external tangents exist)
/// - Circle has zero or negative radius
/// - Numerical computation fails (e.g., distances too small)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// use togo::intersection::tangent::tangent_point_to_circle;
///
/// // Point to the right of a unit circle at origin
/// let p = point(3.0, 0.0);
/// let c = circle(point(0.0, 0.0), 1.0);
///
/// let tangents = tangent_point_to_circle(p, c);
/// assert!(tangents.is_some());
///
/// let (t1, t2) = tangents.unwrap();
/// // Tangent points should be on the circle
/// assert!(((t1 - c.c).norm() - c.r).abs() < 1e-10);
/// assert!(((t2 - c.c).norm() - c.r).abs() < 1e-10);
/// ```
#[must_use]
pub fn tangent_point_to_circle(ext_point: Point, circle: Circle) -> Option<(Point, Point)> {
    // Validate inputs
    if circle.r <= 0.0 {
        return None;
    }

    // Vector from point to center
    let pc = circle.c - ext_point;
    let d_sq = pc.dot(pc);
    let r_sq = circle.r * circle.r;

    // Check if point is outside the circle (d > r)
    if d_sq <= r_sq {
        return None; // Point is inside or on circle
    }

    let d = d_sq.sqrt();

    // Check for degenerate case: point coincides with center
    if d < 1e-10 {
        return None; // Point is at the center, no well-defined tangent
    }

    // Using the right triangle formed by P, C, and tangent point T:
    // PT² + r² = d²  (Pythagorean theorem)
    // PT = sqrt(d² - r²)
    let pt_dist = (d_sq - r_sq).sqrt();

    // Normalize the PC vector
    let pc_norm = pc / d;

    // Find the perpendicular vector (rotate 90° CCW)
    let perp = point(-pc_norm.y, pc_norm.x);

    // The tangent points lie on a circle centered at P with radius pt_dist
    // They also lie on the original circle
    // The key is to project correctly from the center C
    
    // Distance from C to the tangent line (the perpendicular distance)
    // This is just the radius r (since tangent is perpendicular to radius)
    
    // Using similar triangles:
    // The projection of the tangent point onto the PC line from C is:
    // distance = r² / d
    let proj_dist = r_sq / d;
    
    // The perpendicular distance from this projection to the tangent point is:
    // perp_dist = r * sqrt(1 - (r/d)²) = r * sqrt((d² - r²)/d²)
    let perp_dist = circle.r * pt_dist / d;

    // The tangent points are found by:
    // 1. Starting at center C
    // 2. Moving proj_dist along PC direction (toward P)
    // 3. Moving ± perp_dist perpendicular to PC
    let base = circle.c - pc_norm * proj_dist;
    
    let t1 = base + perp * perp_dist;  // Left tangent (CCW)
    let t2 = base - perp * perp_dist;  // Right tangent (CW)

    Some((t1, t2))
}

/// Computes the two external tangent lines between two circles.
///
/// Given two circles, there are up to 4 tangent lines: 2 external and 2 internal.
/// This function computes only the **external tangents** - the ones that don't cross
/// between the circles.
///
/// Each external tangent is defined by two points: one on each circle.
///
/// # Algorithm (Geometric, No Trigonometry)
///
/// The algorithm uses the concept of homothety (similarity transformation):
///
/// 1. For equal radii: tangents are parallel to the line connecting centers
/// 2. For different radii: tangents meet at a homothety center H
///    - External homothety center: H = (r2*C1 - r1*C2) / (r2 - r1)
/// 3. Find tangents from H to a helper circle (radius difference)
/// 4. Project these tangents onto both original circles
///
/// The geometric approach avoids angles entirely by using:
/// - Vector arithmetic
/// - Pythagorean theorem
/// - Similar triangles
///
/// # Arguments
///
/// * `c1` - First circle
/// * `c2` - Second circle
///
/// # Returns
///
/// `Some((t1_c1, t1_c2, t2_c1, t2_c2))` where:
/// - `(t1_c1, t1_c2)` is the first external tangent (one point per circle)
/// - `(t2_c1, t2_c2)` is the second external tangent (one point per circle)
///
/// The tangents are ordered such that when traversing CCW around the circles:
/// - First tangent is the "upper" or "left" tangent
/// - Second tangent is the "lower" or "right" tangent
///
/// Returns `None` if:
/// - Either circle has zero or negative radius
/// - Circles are too close (one contains the other)
/// - Numerical computation fails
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
/// use togo::intersection::tangent::external_tangents_between_circles;
///
/// // Two circles side by side
/// let c1 = circle(point(0.0, 0.0), 1.0);
/// let c2 = circle(point(5.0, 0.0), 1.0);
///
/// let tangents = external_tangents_between_circles(c1, c2);
/// assert!(tangents.is_some());
/// ```
#[must_use]
pub fn external_tangents_between_circles(
    c1: Circle,
    c2: Circle,
) -> Option<(Point, Point, Point, Point)> {
    // Validate inputs
    if c1.r <= 0.0 || c2.r <= 0.0 {
        return None;
    }

    // Vector from c1 to c2
    let c1c2 = c2.c - c1.c;
    let d_sq = c1c2.dot(c1c2);
    let d = d_sq.sqrt();

    // Check for degenerate case: circles have the same center
    if d < 1e-10 {
        return None; // Concentric circles, no well-defined external tangents
    }

    // Check if circles are too close (one might contain the other)
    let r_diff = (c1.r - c2.r).abs();
    
    if d < r_diff {
        return None; // One circle contains the other
    }

    // Special case: equal radii - tangents are parallel to line between centers
    if (c1.r - c2.r).abs() < 1e-10 {
        let c1c2_norm = c1c2 / d;
        let perp = Point::new(-c1c2_norm.y, c1c2_norm.x);
        
        let t1_c1 = c1.c + perp * c1.r;
        let t1_c2 = c2.c + perp * c2.r;
        let t2_c1 = c1.c - perp * c1.r;
        let t2_c2 = c2.c - perp * c2.r;
        
        return Some((t1_c1, t1_c2, t2_c1, t2_c2));
    }

    // General case: different radii
    // Find the external homothety center H
    // H divides the line C1-C2 externally in the ratio r1:r2
    // H = (r2*C1 - r1*C2) / (r2 - r1)
    
    let r_diff_signed = c2.r - c1.r;
    let h = (c2.c * c1.r - c1.c * c2.r) / r_diff_signed;

    // Create a helper circle: center at c1, radius = |r1 - r2|
    let helper_circle = circle(c1.c, r_diff.abs());
    
    // Find tangents from H to the helper circle
    let tangents_to_helper = tangent_point_to_circle(h, helper_circle)?;
    
    // For each tangent to the helper circle, project to both original circles
    // The direction from H through the tangent point on helper circle
    // gives the direction of the tangent line
    
    let (helper_t1, helper_t2) = tangents_to_helper;
    
    // Tangent 1: from H through helper_t1
    let (dir1, _) = (helper_t1 - h).normalize(false);
    
    // Tangent 2: from H through helper_t2
    let (dir2, _) = (helper_t2 - h).normalize(false);
    
    // Project onto circles using perpendicular to direction
    let perp1 = point(-dir1.y, dir1.x);
    let perp2 = point(-dir2.y, dir2.x);
    
    // For each circle, find where the tangent line touches
    // The tangent point is: center + radius * perpendicular_direction
    // But we need to determine the sign based on which side of the line
    
    // Since we're looking for external tangents, both circles should be on the same side
    // Determine the correct perpendicular direction based on the geometry
    
    // Distance from c1 center to the tangent line (should be r1)
    // The tangent line passes through H with direction dir1
    // Point on circle c1: c1.c + perp1 * c1.r * sign
    
    // Use the fact that tangent from H to helper circle defines the tangent line
    // The tangent points on original circles lie along this line
    
    // For circle c1: move from center in perpendicular direction
    // Sign is determined by: which side of the H-tangent line is the center on?
    
    // Actually, simpler approach:
    // The tangent line has direction dir1 (or dir2)
    // For circle c1: tangent point = c1.c + perp * r1 (where perp ⊥ dir)
    // We need the correct sign for perp
    
    // Check: H to tangent point should be perpendicular to radius
    // Let's use the signed distance approach
    
    let sign1_c1 = if (c1.c - h).perp(dir1) > 0.0 { 1.0 } else { -1.0 };
    let sign1_c2 = if (c2.c - h).perp(dir1) > 0.0 { 1.0 } else { -1.0 };
    let sign2_c1 = if (c1.c - h).perp(dir2) > 0.0 { 1.0 } else { -1.0 };
    let sign2_c2 = if (c2.c - h).perp(dir2) > 0.0 { 1.0 } else { -1.0 };
    
    let t1_c1 = c1.c + perp1 * (c1.r * sign1_c1);
    let t1_c2 = c2.c + perp1 * (c2.r * sign1_c2);
    let t2_c1 = c1.c + perp2 * (c1.r * sign2_c1);
    let t2_c2 = c2.c + perp2 * (c2.r * sign2_c2);

    Some((t1_c1, t1_c2, t2_c1, t2_c2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangent_point_to_circle_basic() {
        // Point to the right of unit circle at origin
        let p = point(3.0, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);

        let result = tangent_point_to_circle(p, c);
        assert!(result.is_some());

        let (t1, t2) = result.unwrap();
        
        // Tangent points should be on the circle
        let dist1 = (t1 - c.c).norm();
        let dist2 = (t2 - c.c).norm();
        assert!((dist1 - c.r).abs() < 1e-10, "t1 not on circle: dist={}", dist1);
        assert!((dist2 - c.r).abs() < 1e-10, "t2 not on circle: dist={}", dist2);

        // Tangent lines should be perpendicular to radius at tangent point
        let radius1 = t1 - c.c;
        let tangent1 = t1 - p;
        let dot1 = radius1.dot(tangent1);
        assert!(dot1.abs() < 1e-9, "Tangent 1 not perpendicular: dot={}", dot1);

        let radius2 = t2 - c.c;
        let tangent2 = t2 - p;
        let dot2 = radius2.dot(tangent2);
        assert!(dot2.abs() < 1e-9, "Tangent 2 not perpendicular: dot={}", dot2);
    }

    #[test]
    fn test_tangent_point_to_circle_point_inside() {
        // Point inside circle - no external tangents
        let p = point(0.5, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);

        let result = tangent_point_to_circle(p, c);
        assert!(result.is_none());
    }

    #[test]
    fn test_tangent_point_to_circle_point_on_circle() {
        // Point on circle - no external tangents
        let p = point(1.0, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);

        let result = tangent_point_to_circle(p, c);
        assert!(result.is_none());
    }

    #[test]
    fn test_tangent_point_to_circle_point_at_center() {
        // Point at center - degenerate case
        let p = point(0.0, 0.0);
        let c = circle(point(0.0, 0.0), 1.0);

        let result = tangent_point_to_circle(p, c);
        assert!(result.is_none());
    }

    #[test]
    fn test_external_tangents_equal_radii() {
        // Two circles with equal radii
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(5.0, 0.0), 1.0);

        let result = external_tangents_between_circles(c1, c2);
        assert!(result.is_some());

        let (t1_c1, t1_c2, t2_c1, t2_c2) = result.unwrap();

        // Tangent points should be on their respective circles
        assert!(((t1_c1 - c1.c).norm() - c1.r).abs() < 1e-10);
        assert!(((t1_c2 - c2.c).norm() - c2.r).abs() < 1e-10);
        assert!(((t2_c1 - c1.c).norm() - c1.r).abs() < 1e-10);
        assert!(((t2_c2 - c2.c).norm() - c2.r).abs() < 1e-10);

        // For equal radii, tangent lines should be parallel
        let (tangent_dir1, _) = (t1_c2 - t1_c1).normalize(false);
        let (tangent_dir2, _) = (t2_c2 - t2_c1).normalize(false);
        
        // Parallel means cross product ≈ 0
        let cross = tangent_dir1.perp(tangent_dir2);
        assert!(cross.abs() < 1e-9, "Tangents not parallel for equal radii");
    }

    #[test]
    fn test_external_tangents_different_radii() {
        // Two circles with different radii
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(10.0, 0.0), 2.0);

        let result = external_tangents_between_circles(c1, c2);
        assert!(result.is_some());

        let (t1_c1, t1_c2, t2_c1, t2_c2) = result.unwrap();

        // Tangent points should be on their respective circles
        assert!(((t1_c1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t1_c2 - c2.c).norm() - c2.r).abs() < 1e-9);
        assert!(((t2_c1 - c1.c).norm() - c1.r).abs() < 1e-9);
        assert!(((t2_c2 - c2.c).norm() - c2.r).abs() < 1e-9);
    }

    #[test]
    fn test_external_tangents_concentric_circles() {
        // Concentric circles - degenerate case
        let c1 = circle(point(0.0, 0.0), 1.0);
        let c2 = circle(point(0.0, 0.0), 2.0);

        let result = external_tangents_between_circles(c1, c2);
        assert!(result.is_none());
    }

    #[test]
    fn test_external_tangents_one_contains_other() {
        // One circle contains the other
        let c1 = circle(point(0.0, 0.0), 5.0);
        let c2 = circle(point(1.0, 0.0), 1.0);

        let result = external_tangents_between_circles(c1, c2);
        assert!(result.is_none());
    }
}

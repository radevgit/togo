


use crate::prelude::*;

#[derive(Debug, PartialEq)]
pub enum TangentArcArc {
    OneArc(Arc),
    TwoArcs(Arc, Arc),
    ThreeArcs(Arc, Arc, Arc),
}


/// Given two arcs, computes the external tangent (CCW) points between them.
/// Returns new parts of the original arcs and connecting tangent part.
// pub fn tangent_arc_arc(arc1: &Arc, arc2: &Arc) -> TangentArcArc {

// }


#[cfg(test)]
mod test_tangent_arc_arc {
    //use super::*;

    // #[test]
    // fn test_tangent_arc_arc_two_lines() {
    //     // Two line segments
    //     let line1 = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    //     let line2 = arcseg(point(2.0, 0.0), point(3.0, 0.0));
        
    //     let result = tangent_arc_arc(&line1, &line2);
        
    //     match result {
    //         TangentArcArc::OneArc(arc) => {
    //             assert!(arc.is_line());
    //             assert_eq!(arc.a, point(1.0, 0.0)); // End of first line
    //             assert_eq!(arc.b, point(2.0, 0.0)); // Start of second line
    //         }
    //         _ => panic!("Expected OneArc for two line segments"),
    //     }
    // }

    // #[test]
    // fn test_tangent_line_to_arc() {
    //     // Line segment to circular arc
    //     let line = arcseg(point(0.0, 0.0), point(1.0, 0.0));
    //     let arc = arc(point(3.0, 0.0), point(2.0, 1.0), point(2.0, 0.0), 1.0);
        
    //     let result = tangent_arc_arc(&line, &arc);
        
    //     match result {
    //         TangentArcArc::OneArc(connecting_arc) => {
    //             assert!(connecting_arc.is_line());
    //             assert_eq!(connecting_arc.a, point(1.0, 0.0)); // End of line
    //             assert_eq!(connecting_arc.b, point(3.0, 0.0)); // Start of arc
    //         }
    //         _ => panic!("Expected OneArc for line to arc"),
    //     }
    // }

    // #[test]
    // fn test_tangent_arc_to_line() {
    //     // Circular arc to line segment
    //     let arc = arc(point(0.0, 1.0), point(1.0, 0.0), point(0.0, 0.0), 1.0);
    //     let line = arcseg(point(2.0, 0.0), point(3.0, 0.0));
        
    //     let result = tangent_arc_arc(&arc, &line);
        
    //     match result {
    //         TangentArcArc::OneArc(connecting_arc) => {
    //             assert!(connecting_arc.is_line());
    //             assert_eq!(connecting_arc.a, point(1.0, 0.0)); // End of arc
    //             assert_eq!(connecting_arc.b, point(2.0, 0.0)); // Start of line
    //         }
    //         _ => panic!("Expected OneArc for arc to line"),
    //     }
    // }

    // #[test]
    // fn test_tangent_arc_to_arc_simple() {
    //     // Two circular arcs that are well separated
    //     let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
    //     let arc2 = arc(point(4.0, 0.0), point(3.0, 1.0), point(3.0, 0.0), 1.0);
        
    //     let result = tangent_arc_arc(&arc1, &arc2);
        
    //     // Should get either OneArc, TwoArcs, or ThreeArcs
    //     match result {
    //         TangentArcArc::OneArc(_) => {
    //             // Simple connection is acceptable
    //         }
    //         TangentArcArc::TwoArcs(arc_a, arc_b) => {
    //             // More sophisticated tangent connection
    //             assert!(arc_a.is_line() || arc_a.is_arc());
    //             assert!(arc_b.is_line() || arc_b.is_arc());
    //         }
    //         TangentArcArc::ThreeArcs(arc_a, arc_b, arc_c) => {
    //             // Most sophisticated smooth connection
    //             assert!(arc_a.is_line() || arc_a.is_arc());
    //             assert!(arc_b.is_line() || arc_b.is_arc());
    //             assert!(arc_c.is_line() || arc_c.is_arc());
    //         }
    //     }
    // }

    // #[test]
    // fn test_tangent_arc_to_arc_overlapping() {
    //     // Two overlapping circular arcs
    //     let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
    //     let arc2 = arc(point(1.5, 0.0), point(0.5, 1.0), point(0.5, 0.0), 1.0);
        
    //     let result = tangent_arc_arc(&arc1, &arc2);
        
    //     // Should handle overlapping case gracefully
    //     match result {
    //         TangentArcArc::OneArc(arc) => {
    //             // Fallback to simple connection is expected for overlapping arcs
    //             assert!(arc.is_line());
    //         }
    //         _ => {
    //             // Other solutions are also acceptable
    //         }
    //     }
    // }

    // #[test]
    // fn test_tangent_arc_to_arc_different_sizes() {
    //     // Arcs with different radii
    //     let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);   // radius 1
    //     let arc2 = arc(point(5.0, 0.0), point(3.0, 2.0), point(3.0, 0.0), 2.0);   // radius 2
        
    //     let result = tangent_arc_arc(&arc1, &arc2);
        
    //     // Should handle different radii
    //     match result {
    //         TangentArcArc::OneArc(_) => {}
    //         TangentArcArc::TwoArcs(_, _) => {}
    //         TangentArcArc::ThreeArcs(_, _, _) => {}
    //     }
    // }

    // #[test]
    // fn test_tangent_arc_to_arc_concentric() {
    //     // Two concentric arcs (same center, different radii)
    //     let arc1 = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
    //     let arc2 = arc(point(2.0, 0.0), point(0.0, 2.0), point(0.0, 0.0), 2.0);
        
    //     let result = tangent_arc_arc(&arc1, &arc2);
        
    //     // Should handle concentric case (fallback to simple connection expected)
    //     match result {
    //         TangentArcArc::OneArc(arc) => {
    //             assert!(arc.is_line());
    //         }
    //         _ => {
    //             // Other solutions might be possible depending on implementation
    //         }
    //     }
    // }

    // #[test]
    // fn test_compute_external_tangent_points() {
    //     // Test the helper function directly
    //     let c1 = point(0.0, 0.0);
    //     let r1 = 1.0;
    //     let c2 = point(4.0, 0.0);
    //     let r2 = 1.0;
        
    //     let result = compute_external_tangent_points(c1, r1, c2, r2);
        
    //     assert!(result.is_some());
    //     if let Some((t1, t2)) = result {
    //         // Tangent points should be on the circle boundaries
    //         let dist1 = (t1 - c1).norm();
    //         let dist2 = (t2 - c2).norm();
            
    //         assert!((dist1 - r1).abs() < 1e-10, "t1 not on first circle: {} vs {}", dist1, r1);
    //         assert!((dist2 - r2).abs() < 1e-10, "t2 not on second circle: {} vs {}", dist2, r2);
    //     }
    // }

    // #[test]
    // fn test_compute_external_tangent_points_different_radii() {
    //     // Test with different radii
    //     let c1 = point(0.0, 0.0);
    //     let r1 = 1.0;
    //     let c2 = point(5.0, 0.0);
    //     let r2 = 2.0;
        
    //     let result = compute_external_tangent_points(c1, r1, c2, r2);
        
    //     assert!(result.is_some());
    //     if let Some((t1, t2)) = result {
    //         // Verify tangent points are on circles
    //         let dist1 = (t1 - c1).norm();
    //         let dist2 = (t2 - c2).norm();
            
    //         assert!((dist1 - r1).abs() < 1e-10);
    //         assert!((dist2 - r2).abs() < 1e-10);
    //     }
    // }

    // #[test]
    // fn test_compute_external_tangent_points_impossible() {
    //     // Test case where external tangent is impossible (one circle inside other)
    //     let c1 = point(0.0, 0.0);
    //     let r1 = 1.0;
    //     let c2 = point(0.5, 0.0);  // Very close
    //     let r2 = 2.0;             // Large radius
        
    //     let result = compute_external_tangent_points(c1, r1, c2, r2);
        
    //     // Should return None for impossible case
    //     assert!(result.is_none());
    // }

    // #[test]
    // fn test_angle_computation() {
    //     // Test angle computation functions
    //     let arc = arc(point(1.0, 0.0), point(0.0, 1.0), point(0.0, 0.0), 1.0);
        
    //     let start_angle = compute_arc_start_angle(&arc);
    //     let end_angle = compute_arc_end_angle(&arc);
        
    //     // Start should be at 0 radians (positive x-axis)
    //     assert!((start_angle - 0.0).abs() < 1e-10, "Start angle: {}", start_angle);
        
    //     // End should be at π/2 radians (positive y-axis)
    //     assert!((end_angle - std::f64::consts::PI/2.0).abs() < 1e-10, "End angle: {}", end_angle);
    // }

    // #[test]
    // fn test_angle_computation_line() {
    //     // Test angle computation for line segments
    //     let line = arcseg(point(0.0, 0.0), point(1.0, 1.0));
        
    //     let start_angle = compute_arc_start_angle(&line);
    //     let end_angle = compute_arc_end_angle(&line);
        
    //     // For lines, both should give the direction angle (45 degrees = π/4)
    //     let expected_angle = std::f64::consts::PI / 4.0;
    //     assert!((start_angle - expected_angle).abs() < 1e-10);
    //     assert!((end_angle - expected_angle).abs() < 1e-10);
    // }
}

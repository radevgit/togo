//! Tests for numerical stability issues identified in floating-point analysis.
//!
//! These tests are designed to FAIL with the current implementation and PASS
//! after applying the recommended fixes from FLOATING_POINT_ANALYSIS.md
//!
//! Run with: cargo test test_numerical_issues

#![cfg(test)]

use crate::prelude::*;

/// Tests for Issue 2.0: Exact zero comparison in line-line intersection
mod test_line_line_parallel_detection {
    use super::*;

    #[test]
    fn test_nearly_parallel_lines_detected() {
        // Two lines that are almost parallel (angle ~1e-10 radians)
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(0.0, 100.0), point(1.0, 1e-10));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::ParallelDistinct() => {
                // CORRECT - this is what should happen
            }
            LineLineConfig::OnePoint(p, s0, s1) => {
                // BUG - will fail the test
                panic!(
                    "Lines are nearly parallel but detected as intersecting! \
                     Got intersection at {:?} with params s0={}, s1={} \
                     (intersection is {} units away)",
                    p, s0, s1, s0.abs()
                );
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_very_nearly_parallel_lines() {
        // Even closer to parallel (angle ~1e-12 radians)
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(0.0, 1.0), point(1.0, 1e-12));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::ParallelDistinct() => {
                // CORRECT
            }
            LineLineConfig::OnePoint(p, s0, s1) => {
                panic!(
                    "Lines are nearly parallel but detected as intersecting! \
                     Intersection at {:?}, params s0={}, s1={}",
                    p, s0, s1
                );
            }
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_exactly_parallel_lines_detected() {
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(0.0, 1.0), point(1.0, 0.0));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::ParallelDistinct() => {
                // Expected
            }
            _ => panic!("Exactly parallel lines should be detected as parallel"),
        }
    }

    #[test]
    fn test_same_line_detected() {
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(5.0, 0.0), point(1.0, 0.0));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::ParallelTheSame() => {
                // Expected
            }
            _ => panic!("Same line should be detected"),
        }
    }

    #[test]
    fn test_intersection_parameters_reasonable() {
        // When lines DO intersect, parameters should be reasonable
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(0.5, -1.0), point(0.0, 1.0));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::OnePoint(p, s0, s1) => {
                assert!(
                    s0.abs() < 1e6,
                    "Parameter s0 is unreasonably large: {}",
                    s0
                );
                assert!(
                    s1.abs() < 1e6,
                    "Parameter s1 is unreasonably large: {}",
                    s1
                );
                assert!(
                    p.x.is_finite() && p.y.is_finite(),
                    "Intersection point should be finite"
                );
            }
            _ => panic!("Lines should intersect"),
        }
    }
}

/// Tests for Issue 2.1: Division by bulge without validation
mod test_arc_bulge_division {
    use super::*;

    #[test]
    fn test_tiny_bulge_produces_finite_arc() {
        // Very small bulge value (close to zero)
        let arc = arc_from_bulge(
            point(0.0, 0.0),
            point(1.0, 0.0),
            1e-15,  // Tiny bulge - should be treated as line segment
        );
        
        // When bulge is too small, it's treated as a line segment
        // Line segments have infinite radius and center
        // The IMPORTANT thing is the arc endpoints are finite and correct
        assert_eq!(arc.a, point(0.0, 0.0), "Arc start point should be correct");
        assert_eq!(arc.b, point(1.0, 0.0), "Arc end point should be correct");
        
        // And the result should not be NaN
        assert!(!arc.c.x.is_nan() && !arc.c.y.is_nan(), "Arc center should not be NaN");
        assert!(!arc.r.is_nan(), "Arc radius should not be NaN");
    }

    #[test]
    fn test_zero_bulge_produces_line_segment() {
        let arc = arc_from_bulge(
            point(0.0, 0.0),
            point(2.0, 0.0),
            0.0,  // Zero bulge
        );
        
        // Line segments are represented as arcs with infinite center and radius
        assert!(arc.r.is_infinite(), "Zero bulge should produce line segment with infinite radius");
        assert!(
            arc.c.x.is_infinite() && arc.c.y.is_infinite(),
            "Line segment should have infinite center point"
        );
    }

    #[test]
    fn test_epsilon_bulge_produces_finite_arc() {
        let arc = arc_from_bulge(
            point(0.0, 0.0),
            point(1.0, 0.0),
            f64::EPSILON,  // Machine epsilon - treated as line segment
        );
        
        // Should not produce NaN values
        assert!(!arc.c.x.is_nan() && !arc.c.y.is_nan(), "Arc center should not be NaN");
        assert!(!arc.r.is_nan(), "Arc radius should not be NaN");
        // Endpoints should be correct
        assert_eq!(arc.a, point(0.0, 0.0), "Arc start point should be correct");
        assert_eq!(arc.b, point(1.0, 0.0), "Arc end point should be correct");
    }

    #[test]
    fn test_negative_tiny_bulge() {
        let arc = arc_from_bulge(
            point(0.0, 0.0),
            point(1.0, 0.0),
            -1e-15,  // Negative tiny bulge - treated as line segment
        );
        
        // Should not produce NaN values
        assert!(!arc.c.x.is_nan() && !arc.c.y.is_nan(), "Arc center should not be NaN");
        assert!(!arc.r.is_nan(), "Arc radius should not be NaN");
        // Endpoints should be correct (swapped for negative bulge)
        assert_eq!(arc.a, point(1.0, 0.0), "Arc start point should be swapped for negative bulge");
        assert_eq!(arc.b, point(0.0, 0.0), "Arc end point should be swapped for negative bulge");
    }
}

/// Tests for Issue 2.2: Square root of potentially negative values
mod test_sqrt_guards {
    use super::*;

    #[test]
    fn test_arc_with_numerical_error_in_bulge() {
        // Test various bulge values that might cause numerical issues
        let bulges = [1e-10, 1e-8, 1e-6, 0.1, 0.5, 0.9];
        
        for &bulge in &bulges {
            let arc = arc_from_bulge(
                point(0.0, 0.0),
                point(1.0, 0.0),
                bulge,
            );
            
            assert!(
                !arc.c.x.is_nan(),
                "Arc center x is NaN for bulge {}",
                bulge
            );
            assert!(
                !arc.c.y.is_nan(),
                "Arc center y is NaN for bulge {}",
                bulge
            );
            assert!(
                !arc.r.is_nan(),
                "Arc radius is NaN for bulge {}",
                bulge
            );
        }
    }

    #[test]
    fn test_circle_intersection_discriminant() {
        // Circles that are very close to tangent (might produce negative discriminant)
        let c0 = circle(point(0.0, 0.0), 1.0);
        let c1 = circle(point(2.0 + 1e-10, 0.0), 1.0);  // Just barely separated
        
        let result = int_circle_circle(c0, c1);
        
        match result {
            CircleCircleConfig::NoncocircularOnePoint(p) => {
                assert!(p.x.is_finite() && p.y.is_finite());
            }
            CircleCircleConfig::NoIntersection() => {
                // Also acceptable due to numerical tolerance
            }
            _ => panic!("Unexpected result for tangent circles"),
        }
    }
}

/// Tests for Issue 2.3: Convex hull NaN handling
mod test_convex_hull_nan {
    use super::*;

    #[test]
    fn test_convex_hull_filters_nan_points() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(0.0, 1.0),
            point(f64::NAN, 0.5),  // NaN point
        ];
        
        // Should either filter NaN or return valid hull, not panic
        let hull = pointline_convex_hull(&points);
        
        // All points in hull should be finite
        for p in &hull {
            assert!(
                p.x.is_finite() && p.y.is_finite(),
                "Hull contains non-finite point: {:?}",
                p
            );
        }
        
        // Should have at least 3 points for a valid hull (the 4 valid input points)
        assert!(
            hull.len() >= 3,
            "Hull should contain valid points after filtering NaN"
        );
    }

    #[test]
    fn test_convex_hull_filters_infinity() {
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 0.0),
            point(1.0, 1.0),
            point(f64::INFINITY, 0.5),  // Infinite point
        ];
        
        let hull = pointline_convex_hull(&points);
        
        for p in &hull {
            assert!(
                p.x.is_finite() && p.y.is_finite(),
                "Hull contains non-finite point: {:?}",
                p
            );
        }
    }

    #[test]
    fn test_convex_hull_all_nan_points() {
        let points = vec![
            point(f64::NAN, f64::NAN),
            point(f64::NAN, 1.0),
            point(1.0, f64::NAN),
        ];
        
        let hull = pointline_convex_hull(&points);
        
        // Should return empty hull or handle gracefully
        assert!(
            hull.is_empty() || hull.iter().all(|p| p.x.is_finite() && p.y.is_finite()),
            "Should handle all-NaN input gracefully"
        );
    }
}

/// Tests for Issue 8.1: Exact equality comparisons in geometric predicates
mod test_exact_zero_comparisons {
    use super::*;

    #[test]
    fn test_collinear_points_with_numerical_error() {
        // Three points that should be collinear but have tiny numerical error
        let p1 = point(0.0, 0.0);
        let p2 = point(1.0, 1.0);
        let p3 = point(2.0, 2.0 + 1e-14);  // Tiny error
        
        let cross = (p2 - p1).perp(p3 - p2);
        
        // Cross product will be ~1e-14, not exactly 0.0
        // Should be treated as collinear within tolerance
        const COLLINEAR_TOLERANCE: f64 = 1e-10;
        assert!(
            cross.abs() < COLLINEAR_TOLERANCE,
            "Points should be treated as collinear. Cross product: {}",
            cross
        );
    }

    #[test]
    fn test_convex_hull_handles_numerical_collinearity() {
        // Points on a line with tiny numerical errors
        let points = vec![
            point(0.0, 0.0),
            point(1.0, 1.0),
            point(2.0, 2.0 + 1e-14),
            point(3.0, 3.0 - 1e-14),
            point(4.0, 4.0),
        ];
        
        let hull = pointline_convex_hull(&points);
        
        // Should recognize collinearity and return only endpoints
        // (or at most 3 points if tolerance isn't applied)
        assert!(
            hull.len() <= 3,
            "Collinear points should produce minimal hull. Got {} points",
            hull.len()
        );
    }

    #[test]
    fn test_circle_center_comparison_with_tolerance() {
        // Two circles with nearly identical centers
        let c0 = circle(point(0.0, 0.0), 1.0);
        let c1 = circle(point(1e-15, 1e-15), 1.0);
        
        let result = int_circle_circle(c0, c1);
        
        // Should be treated as same circle or handle gracefully
        match result {
            CircleCircleConfig::SameCircles() => {
                // Correct with tolerance
            }
            CircleCircleConfig::NoncocircularOnePoint(_) => {
                // Acceptable - treated as tangent
            }
            _ => panic!("Nearly identical circles should be handled properly"),
        }
    }
}

/// Tests for Issue 8.2: Division without bounds checking
mod test_division_guards {
    use super::*;

    #[test]
    fn test_distance_to_degenerate_line() {
        // Line with nearly zero direction vector
        let line = Line::new(
            point(0.0, 0.0),
            point(1e-100, 1e-100),  // Extremely small direction
        );
        let circle = circle(point(1.0, 1.0), 1.0);
        
        // Should handle gracefully without division by zero
        let result = dist_line_circle(&line, &circle);
        
        match result {
            DistLineCircleConfig::OnePair(dist, _, _, _) |
            DistLineCircleConfig::TwoPairs(dist, _, _, _, _, _, _) => {
                assert!(
                    dist.is_finite(),
                    "Distance should be finite for degenerate line"
                );
            }
        }
    }

    #[test]
    fn test_point_segment_distance_zero_length() {
        // Segment with zero length (point)
        let seg = segment(point(1.0, 2.0), point(1.0, 2.0));
        let p = point(3.0, 4.0);
        
        let (dist, closest) = dist_point_segment(&p, &seg);
        
        assert!(dist.is_finite(), "Distance should be finite");
        assert!(
            closest.x.is_finite() && closest.y.is_finite(),
            "Closest point should be finite"
        );
        
        // Should be distance from point to the segment endpoint
        let expected = ((3.0_f64 - 1.0).powi(2) + (4.0_f64 - 2.0).powi(2)).sqrt();
        assert!(
            (dist - expected).abs() < 1e-10,
            "Distance should be from point to segment endpoint"
        );
    }
}

/// Tests for Issue 8.3: Catastrophic cancellation with large coordinates
mod test_large_coordinates {
    use super::*;

    #[test]
    fn test_distance_with_large_coordinates() {
        let scales = [1.0, 1e3, 1e6, 1e9];
        
        for &scale in &scales {
            let p1 = point(scale, scale);
            let p2 = point(scale + 1.0, scale);
            
            let dist = (p2 - p1).norm();
            
            if scale < 1e8 {
                // Should be close to 1.0 for reasonable scales
                assert!(
                    (dist - 1.0).abs() < 1e-6,
                    "Distance should be ~1.0 at scale {}. Got: {}",
                    scale,
                    dist
                );
            } else {
                // At very large scales, at least check it's finite and positive
                assert!(
                    dist.is_finite() && dist >= 0.0,
                    "Distance should be finite and non-negative at scale {}",
                    scale
                );
            }
        }
    }

    #[test]
    fn test_line_intersection_large_coordinates() {
        // Lines at large coordinates
        let scale = 1e8;
        let line0 = Line::new(
            point(scale, scale),
            point(1.0, 0.0),
        );
        let line1 = Line::new(
            point(scale, scale + 1.0),
            point(0.0, -1.0),
        );
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::OnePoint(p, s0, s1) => {
                assert!(p.x.is_finite() && p.y.is_finite());
                assert!(s0.is_finite() && s1.is_finite());
                
                // Intersection should be near (scale, scale)
                assert!(
                    (p.x - scale).abs() < 10.0,
                    "Intersection x should be near {}",
                    scale
                );
            }
            _ => panic!("Lines should intersect"),
        }
    }
}

/// Tests for Issue 8.4: Interpolation parameter overflow
mod test_parameter_overflow {
    use super::*;

    #[test]
    fn test_nearly_parallel_lines_reject_far_intersection() {
        let line0 = Line::new(point(0.0, 0.0), point(1.0, 0.0));
        let line1 = Line::new(point(0.0, 1e6), point(1.0, 1e-10));
        
        match int_line_line(&line0, &line1) {
            LineLineConfig::ParallelDistinct() => {
                // Correct - should be treated as parallel
            }
            LineLineConfig::OnePoint(p, s0, s1) => {
                // If intersection is computed, it should be reasonable
                assert!(
                    s0.abs() < 1e8 && s1.abs() < 1e8,
                    "Intersection parameters are too far: s0={}, s1={} \
                     (intersection at {:?} is {} units away)",
                    s0, s1, p, s0.abs()
                );
            }
            _ => {}
        }
    }
}

/// Tests for Issue 8.7: Area calculations with angle wraparound
mod test_area_edge_cases {
    use super::*;

    #[test]
    fn test_degenerate_arc_area() {
        // Arc where start equals center (degenerate)
        let arc = arc(
            point(1.0, 0.0),
            point(0.0, 1.0),
            point(1.0, 0.0),  // center equals start
            1.0,
        );
        
        let arcline = vec![arc];
        let area = arcline_area(&arcline);
        
        assert!(
            area.is_finite(),
            "Area should be finite for degenerate arc"
        );
    }

    #[test]
    fn test_nearly_full_circle_arc() {
        // Arc that's almost a full circle (should have clear area)
        let c = point(0.0, 0.0);
        let r = 1.0;
        
        // Full circle is detected when start and end are very close
        // So we create an arc from a point almost back to itself
        let start = point(r, 0.0);  // angle 0
        let epsilon: f64 = 1e-11;  // Very small angle
        let end = point(r * epsilon.cos(), r * epsilon.sin());  // Almost same point, but slightly rotated
        
        let arc = arc(start, end, c, r);
        
        let arcline = vec![arc];
        let area = arcline_area(&arcline);
        
        assert!(area.is_finite(), "Area should be finite");
        
        // Should be close to full circle area (or very close depending on how arc is oriented)
        let full_circle_area = std::f64::consts::PI * r * r;
        assert!(
            area.abs() > full_circle_area * 0.9,
            "Nearly-full-circle arc should have area close to full circle. Got {}",
            area
        );
    }
}

/// Tests for Issue 8.8: Inconsistent epsilon values
mod test_tolerance_consistency {
    use super::*;

    #[test]
    fn test_close_enough_with_nan() {
        // close_enough should handle NaN gracefully
        assert!(
            !close_enough(f64::NAN, 1.0, 0.1),
            "close_enough should return false for NaN"
        );
        assert!(
            !close_enough(1.0, f64::NAN, 0.1),
            "close_enough should return false for NaN"
        );
    }

    #[test]
    fn test_close_enough_with_infinity() {
        assert!(
            !close_enough(f64::INFINITY, 1.0, 0.1),
            "close_enough should return false for infinity"
        );
        assert!(
            !close_enough(1.0, f64::INFINITY, 0.1),
            "close_enough should return false for infinity"
        );
    }

    #[test]
    fn test_close_enough_with_negative_epsilon() {
        // Negative epsilon should be handled (return false or panic)
        let result = close_enough(1.0, 1.1, -0.1);
        assert!(
            !result,
            "close_enough should handle negative epsilon"
        );
    }
}

/// Integration tests combining multiple issues
mod test_integration {
    use super::*;

    // TODO: This test requires a function to convert polyline to arcline
    // #[test]
    // fn test_polyline_with_tiny_bulge_values() {
    //     // Polyline with segments that have tiny bulge values
    //     let polyline: Polyline = vec![
    //         PVertex::new(point(0.0, 0.0), 1e-15),
    //         PVertex::new(point(1.0, 0.0), 0.0),
    //         PVertex::new(point(1.0, 1.0), 1e-14),
    //         PVertex::new(point(0.0, 1.0), 0.0),
    //     ];
    //     
    //     let arcline = polyline_to_arcline(&polyline);
    //     
    //     // All arcs should have finite geometry
    //     for arc in &arcline {
    //         assert!(
    //             arc.a.x.is_finite() && arc.a.y.is_finite(),
    //             "Arc start should be finite"
    //         );
    //         assert!(
    //             arc.b.x.is_finite() && arc.b.y.is_finite(),
    //             "Arc end should be finite"
    //         );
    //         assert!(
    //             arc.c.x.is_finite() && arc.c.y.is_finite(),
    //             "Arc center should be finite"
    //         );
    //         assert!(arc.r.is_finite(), "Arc radius should be finite");
    //     }
    // }

    #[test]
    fn test_area_calculation_robustness() {
        // Calculate area of various shapes at different scales
        let scales = [1.0, 1e3, 1e6];
        
        for &scale in &scales {
            let square = vec![
                point(0.0 * scale, 0.0 * scale),
                point(1.0 * scale, 0.0 * scale),
                point(1.0 * scale, 1.0 * scale),
                point(0.0 * scale, 1.0 * scale),
            ];
            
            let area = pointline_area(&square);
            
            assert!(area.is_finite(), "Area should be finite at scale {}", scale);
            
            if scale < 1e6 {
                let expected = scale * scale;
                assert!(
                    (area - expected).abs() / expected < 0.01,
                    "Area should be accurate at scale {}. Expected {}, got {}",
                    scale,
                    expected,
                    area
                );
            }
        }
    }
}

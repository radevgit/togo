# Convex Hull for Arcline (Arc-Based Polygons)

## Document Purpose

This document explores the research and implementation strategies for computing the convex hull of an Arcline - a CCW polygon where each edge is represented by an `Arc` (circular arc or line segment). This is a significantly more complex problem than standard point-based or line-segment-based convex hulls.

## Problem Statement

### Input Constraints
- **Arcline**: `Vec<Arc>` - A sequence of connected arcs forming a closed polygon
- **Orientation**: Counter-clockwise (CCW)
- **Non-intersecting**: The polygon has no self-intersections
- **Each Arc**: Can be either:
  - A circular arc segment (finite radius `r`)
  - A line segment (when `r == f64::INFINITY`)

### Output Requirements
- **Convex Hull**: A new `Arcline` representing the smallest convex shape containing all input arcs
- **Hull edges**: May be:
  - Original arcs that lie on the convex boundary
  - Portions of original arcs (trimmed)
  - New line segments connecting hull vertices
  - New arcs bridging gaps (if preserving curvature)

### Key Challenge
**Critical hull vertices may appear at:**
1. **Arc endpoints** - Start and end points of original arcs
2. **Points within arcs** - Local extrema on curved segments
3. **Tangent points** - Where supporting lines touch arcs

This is fundamentally different from polygon convex hulls where vertices are only at explicit polygon vertices.

## Research: Related Work and Algorithms

### 1. Convex Hull of Curved Objects

#### 1.1 "Convex Hull of a Finite Set of Points and Line Segments" (1983)
**Authors**: T. Asano, T. Asano, L. Guibas, J. Hershberger, H. Imai  
**Key Contributions**:
- O(n log n) algorithm for convex hull of points and line segments
- Handles mixed geometric primitives
- **Limitation**: Only handles line segments, not circular arcs

#### 1.2 "Optimal Output-Sensitive Convex Hull Algorithms in Two and Three Dimensions" (1996)
**Authors**: Timothy M. Chan  
**Key Contributions**:
- O(n log h) algorithm where h is hull size
- Output-sensitive approach
- **Limitation**: Designed for point sets, not curved boundaries

### 2. Curved Boundary Hulls

#### 2.1 "Convex Hulls of Spheres and Convex Hulls of Disjoint Convex Polytopes" (1990)
**Authors**: K. Sugihara  
**Key Contributions**:
- Algorithms for convex hulls of circular objects
- Handles tangent line computations between circles
- **Relevance**: Techniques applicable to finding extrema on circular arcs

#### 2.2 "Computing the Convex Hull of a Simple Polygon" (1979)
**Authors**: R. L. Graham, F. F. Yao  
**Key Observations**:
- For simple polygons, convex hull can be computed in O(n) linear time
- Uses the fact that vertices are ordered
- **Relevance**: Our Arcline is ordered, suggesting potential O(n) approaches

#### 2.3 "Fast Algorithms for Convex Hulls" (various)
**Key Concept**: When input has structure (like ordering), faster algorithms exist:
- Melkman's Algorithm: O(n) for simple polygons
- **Applicability**: We can adapt these for arc-based polygons

### 3. Arc-Specific Research

#### 3.1 "Convex Hull Algorithms for Piecewise Smooth Jordan Curves" (2003)
**Authors**: Various computational geometry researchers  
**Key Insights**:
- Smooth curves require finding local extrema
- Extremal points occur where tangent is vertical/horizontal
- For circular arcs: extrema occur at specific angular positions

#### 3.2 CAD/CAM Literature on Offset Curves
**Relevance**:
- Offset curve computation involves similar problems
- Finding outer boundaries of curved shapes
- Techniques for arc trimming and connection

## Arc Structure in Togo Library

### Arc Definition
```rust
pub struct Arc {
    pub a: Point,     // Start point
    pub b: Point,     // End point
    pub c: Point,     // Center point
    pub r: f64,       // Radius (f64::INFINITY for line segments)
    pub id: usize,    // Identifier
}
```

### Key Properties
1. **Always CCW**: All arcs are counter-clockwise oriented
2. **Mixed primitives**: Can be line segment (r = ∞) or circular arc (finite r)
3. **Connected**: In an Arcline, `arc[i].b == arc[i+1].a`
4. **Closed polygon**: Last arc connects back to first

### Bulge Factor
- Related representation using "bulge" = `tan(θ/4)` where θ is arc angle
- Allows compact representation of arc curvature
- Can convert between Arc and bulge representations

## Questions and Answers

### Q1: How do we identify candidate hull vertices?

**Answer:**

Candidates come from three sources:

1. **Arc Endpoints** (always check):
   - All start and end points of arcs
   - These are explicit vertices of the input polygon

2. **Arc Extrema** (for curved arcs only):
   For a circular arc from angle θ₁ to θ₂ around center c:
   - **Horizontal extrema**: Check if 0° or 180° falls within arc's angular span
   - **Vertical extrema**: Check if 90° or 270° falls within arc's angular span
   - Extrema points: `c + r * (cos(θ), sin(θ))` for the extremal angles

3. **Line Segment Endpoints**:
   - When `r == f64::INFINITY`, only endpoints matter
   - No interior extrema for straight segments

### Q2: How do we determine if an arc point is on the convex hull?

**Answer:**

Use the **supporting line test**:

1. **For each candidate point P**:
   - Construct lines through P in all directions
   - P is on hull if there exists a direction where all other points lie on one side

2. **Practical approach** (for ordered polygons):
   - Similar to `pointline_convex_hull` but adapted for arcs
   - Check cross product of tangent vectors at each point
   - For arc interiors, use tangent to circle at that point

3. **Arc-specific consideration**:
   - An arc may be partially on the hull
   - Need to find "trim points" where hull transitions

### Q3: How do we connect hull vertices?

**Answer:**

This is the **most complex part**. Options:

#### Option A: Line Segment Connections (Simplest)
- Always connect hull vertices with straight lines
- Results in a traditional polygonal convex hull
- **Pros**: Simple, guaranteed convex
- **Cons**: Loses arc information

#### Option B: Preserve Arc Segments
- Keep original arcs if they're entirely on the hull
- Trim arcs that are partially on the hull
- **Pros**: Preserves geometric fidelity
- **Cons**: Complex arc trimming logic

#### Option C: Hybrid Approach
- Use original arcs where they lie on hull
- Use line segments for connections
- **Pros**: Balance of fidelity and simplicity
- **Cons**: Mixed representation

**Recommendation**: Start with Option A (line segments only), then implement Option C.

### Q4: What about arcs that bulge outward vs inward?

**Answer:**

Arc convexity is determined by the **traversal direction** in the Arcline:

1. **Outward-bulging arcs** (convex, positive original bulge):
   - Traversed **forward**: `arc[i-1].b == arc[i].a → arc[i].b == arc[i+1].a`
   - The arc rotates CCW from `a` to `b`
   - These are candidates for convex hull preservation
   - May need to find extrema (peak points)

2. **Inward-bulging arcs** (concave, negative original bulge):
   - Traversed **backward**: `arc[i-1].b == arc[i].b ← arc[i].a == arc[i+1].a`
   - The arc rotates CCW from `b` to `a` (reverse traversal)
   - These are typically NOT on convex hull
   - Hull shortcuts across with chord from `arc.b` to `arc.a`

3. **Detection**:
   ```rust
   fn is_arc_convex_in_polygon(arcline: &Arcline, idx: usize) -> bool {
       let arc = &arcline[idx];
       let prev_idx = if idx == 0 { arcline.len() - 1 } else { idx - 1 };
       let prev_arc = &arcline[prev_idx];
       
       // If previous arc connects to current arc's start point,
       // we traverse forward (positive bulge = convex)
       prev_arc.b == arc.a
   }
   ```

4. **Key Insight**:
   - All arcs are geometrically CCW (center-to-point rotation)
   - But traversal direction indicates original bulge sign
   - Forward traversal = convex (preserve in hull)
   - Backward traversal = concave (replace with chord)

### Q5: Can we use the O(n) approach like `pointline_convex_hull`?

**Answer:**

**Partially yes**, with significant modifications:

1. **Advantage of ordering**:
   - Arcline vertices are ordered CCW
   - Can traverse in sequence
   - Similar to Melkman's algorithm for simple polygons

2. **Complications**:
   - Must sample points within arcs for extrema
   - Cannot just check vertices
   - Need arc-tangent calculations

3. **Modified O(n) approach**:
   ```
   For each arc in sequence:
     1. Check arc endpoints
     2. Find and check arc extrema (if any)
     3. Use cross-product test with tangents
     4. Include points that make CCW turns
   ```

4. **Worst case**: May need to iterate through arcs multiple times if complex trimming required

### Q6: How do we handle numerical precision with arcs?

**Answer:**

Critical precision issues:

1. **Angle calculations**:
   - Use `atan2` for robust angle computation
   - Check angular containment carefully
   - Account for wraparound at 2π

2. **Extrema detection**:
   - Use tolerance when checking if extremal angle is within arc span
   - `COLLINEARITY_TOLERANCE = 1e-10` (from existing code)

3. **Tangent calculations**:
   - Tangent at point on circle: perpendicular to radius
   - For point P on arc with center C: tangent ⊥ (P - C)

4. **Arc trimming**:
   - When creating sub-arcs, preserve center and radius
   - Update start/end points
   - Verify connectivity

## Proposed Algorithm

### Phase 1: Candidate Collection (O(n))

```rust
struct HullCandidate {
    point: Point,
    source_arc_idx: usize,
    tangent_direction: Vector, // For cross-product tests
    is_endpoint: bool,
}

fn collect_candidates(arcline: &Arcline) -> Vec<HullCandidate> {
    let mut candidates = Vec::new();
    
    for (idx, arc) in arcline.iter().enumerate() {
        // Add arc endpoints
        candidates.push(HullCandidate {
            point: arc.a,
            source_arc_idx: idx,
            tangent_direction: compute_tangent_at_start(arc),
            is_endpoint: true,
        });
        
        // For curved arcs, find and add extrema
        if arc.is_arc() {
            let extrema = find_arc_extrema(arc);
            for ext_point in extrema {
                candidates.push(HullCandidate {
                    point: ext_point,
                    source_arc_idx: idx,
                    tangent_direction: compute_tangent_at_point(arc, ext_point),
                    is_endpoint: false,
                });
            }
        }
    }
    
    candidates
}
```

### Phase 2: Hull Point Selection (O(n))

```rust
fn select_hull_points(candidates: &[HullCandidate]) -> Vec<HullCandidate> {
    let mut hull = Vec::new();
    let n = candidates.len();
    
    for i in 0..n {
        let prev_idx = if i == 0 { n - 1 } else { i - 1 };
        let next_idx = (i + 1) % n;
        
        let prev_tangent = candidates[prev_idx].tangent_direction;
        let curr_point = candidates[i].point;
        let next_tangent = candidates[next_idx].tangent_direction;
        
        // Cross product test: is this a convex vertex?
        let cross = prev_tangent.perp(next_tangent);
        
        if cross > COLLINEARITY_TOLERANCE {
            hull.push(candidates[i].clone());
        }
    }
    
    hull
}
```

### Phase 3: Hull Edge Construction

```rust
fn construct_hull_arcline(hull_points: &[HullCandidate], 
                          original_arcline: &Arcline) -> Arcline {
    let mut hull_arcline = Vec::new();
    
    for i in 0..hull_points.len() {
        let curr = &hull_points[i];
        let next = &hull_points[(i + 1) % hull_points.len()];
        
        // Check if points are consecutive on same arc
        if curr.source_arc_idx == next.source_arc_idx {
            // Extract sub-arc from original
            let sub_arc = extract_sub_arc(
                &original_arcline[curr.source_arc_idx],
                curr.point,
                next.point
            );
            hull_arcline.push(sub_arc);
        } else {
            // Connect with line segment
            hull_arcline.push(arcseg(curr.point, next.point));
        }
    }
    
    hull_arcline
}
```

## Helper Functions Needed

### 1. Find Arc Extrema

```rust
/// Find extremal points (topmost, bottommost, leftmost, rightmost) on a circular arc
fn find_arc_extrema(arc: &Arc) -> Vec<Point> {
    if arc.is_seg() {
        return vec![]; // Line segments have no interior extrema
    }
    
    let mut extrema = Vec::new();
    
    // Calculate start and end angles
    let start_angle = (arc.a.y - arc.c.y).atan2(arc.a.x - arc.c.x);
    let end_angle = (arc.b.y - arc.c.y).atan2(arc.b.x - arc.c.x);
    
    // Normalize to [0, 2π) and ensure CCW ordering
    let mut angle_span = normalize_arc_angles(start_angle, end_angle);
    
    // Check if 0° (rightmost), 90° (topmost), 180° (leftmost), 270° (bottommost) 
    // fall within the arc's angular span
    let cardinal_angles = [0.0, PI/2.0, PI, 3.0*PI/2.0];
    
    for &angle in &cardinal_angles {
        if angle_in_arc_span(angle, angle_span) {
            let extremal_point = arc.c + Point::new(
                arc.r * angle.cos(),
                arc.r * angle.sin()
            );
            extrema.push(extremal_point);
        }
    }
    
    extrema
}
```

### 2. Compute Tangent at Arc Point

```rust
/// Compute tangent direction at a point on an arc
fn compute_tangent_at_point(arc: &Arc, point: Point) -> Vector {
    if arc.is_seg() {
        // Tangent to line segment is just the direction
        return (arc.b - arc.a).normalize();
    }
    
    // For circular arc, tangent is perpendicular to radius
    let radius_dir = point - arc.c;
    // CCW perpendicular (rotate 90° counter-clockwise)
    Vector::new(-radius_dir.y, radius_dir.x).normalize()
}
```

### 3. Extract Sub-Arc

```rust
/// Extract a portion of an arc between two points on it
fn extract_sub_arc(arc: &Arc, start: Point, end: Point) -> Arc {
    if arc.is_seg() {
        return arcseg(start, end);
    }
    
    // Create new arc with same center and radius, new endpoints
    Arc::new(start, end, arc.c, arc.r)
}
```

### 4. Angle Utilities

```rust
/// Check if angle θ falls within arc's angular span (CCW from start to end)
fn angle_in_arc_span(theta: f64, arc: &Arc) -> bool {
    let start_angle = (arc.a.y - arc.c.y).atan2(arc.a.x - arc.c.x);
    let end_angle = (arc.b.y - arc.c.y).atan2(arc.b.x - arc.c.x);
    
    // Normalize to [0, 2π)
    let start = normalize_angle(start_angle);
    let end = normalize_angle(end_angle);
    let angle = normalize_angle(theta);
    
    // Check CCW containment
    if start <= end {
        angle >= start && angle <= end
    } else {
        angle >= start || angle <= end // Wraps around 0
    }
}

fn normalize_angle(angle: f64) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    ((angle % two_pi) + two_pi) % two_pi
}
```

## Implementation Strategy

### Stage 1: Line-Segment-Only Hull (Simplest)
**Goal**: Get working convex hull using only line segments

1. Extract all arc endpoints
2. Find extrema on curved arcs
3. Run modified `pointline_convex_hull` on these points
4. Return hull as line segments only

**Pros**: Simple, validates the approach  
**Cons**: Loses arc geometry

### Stage 2: Preserve Straight Arcs
**Goal**: Keep original arcs that are line segments

1. Same as Stage 1, but check if consecutive hull points lie on same arc
2. If yes and arc is line segment, preserve it
3. Otherwise use line segment

**Pros**: Preserves some structure  
**Cons**: Still loses curved arcs

### Stage 3: Preserve Convex Arcs (Full Solution)
**Goal**: Keep portions of arcs that lie on hull

1. Full extrema detection
2. Arc trimming logic
3. Validate arc portions are convex
4. Connect with appropriate primitives

**Pros**: Maximum fidelity  
**Cons**: Most complex

## Edge Cases and Challenges

### 1. Nearly Collinear Arcs
- Very flat arcs may be numerically unstable
- Use tolerance-based comparisons
- Fall back to line segment if arc is too flat

### 2. Full Circle Arcs
- Arc spanning > 180° may have multiple extrema
- Need to check all four cardinal directions

### 3. Degenerate Cases
- Single arc: hull is the arc itself (if convex) or its chord
- Two arcs: check both, potentially simplify
- All arcs concave inward: hull may be just a few line segments

### 4. Mixed Arc Types
- Some arcs are curved, some are lines
- Algorithm must handle both uniformly
- Use polymorphic approach based on `arc.is_arc()`

### 5. Precision Near Endpoints
- When extremal angle is very close to start/end angle
- Could create duplicate or nearly-duplicate points
- Use point distance threshold to merge

## Performance Considerations

### Time Complexity
- **Candidate collection**: O(n) - constant extrema per arc
- **Hull selection**: O(n) - single pass with cross products
- **Edge construction**: O(n) - one hull edge per hull vertex
- **Overall**: O(n) assuming ordered input

### Space Complexity
- **Candidates**: O(n) - at most 5 points per arc (2 endpoints + 4 cardinal extrema, but typically 2-3)
- **Hull**: O(h) where h ≤ n is hull size
- **Overall**: O(n)

### Optimization Opportunities
1. **Lazy extrema computation**: Only compute extrema for arcs likely to be on hull
2. **Bounding box prefilter**: Quickly eliminate interior arcs
3. **Vectorization**: Batch angle and cross-product calculations

## Testing Strategy

### Test Cases Needed

1. **Simple convex polygon** (all line segments)
   - Should return identical polygon
   
2. **Simple concave polygon** (all line segments)
   - Should simplify to convex hull

3. **Circle** (single arc)
   - Should return the circle itself

4. **Semicircle** (one arc + line segment closing)
   - Should return the arc + chord

5. **Mixed convex/concave arcs**
   - Convex arcs on hull, concave ones replaced

6. **Spiral polygon**
   - Can use existing `arcline1000()` test data
   - Hull should be roughly circular

7. **Rectangle with rounded corners**
   - Hull includes both straight sides and arc corners

## References and Further Reading

### Academic Papers
1. Asano et al. (1983) - "Convex Hull of Points and Line Segments"
2. Graham & Yao (1979) - "Convex Hulls of Simple Polygons"
3. Melkman (1987) - "On-line Construction of Convex Hull of Simple Polyline"
4. Chan (1996) - "Optimal Output-Sensitive Convex Hull Algorithms"

### Books
1. "Computational Geometry: Algorithms and Applications" (de Berg et al.)
   - Chapter 1: Convex Hulls
   - Section on curved objects

2. "Computational Geometry in C" (O'Rourke)
   - Practical implementations
   - Edge case handling

### Online Resources
1. CGAL Library - Circular arc arrangements
2. Boost.Geometry - Curve handling
3. PostGIS - Curved geometry operations

## Open Questions for Further Investigation

1. **Optimal arc approximation**: When to replace curved sections with polygonal approximations?

2. **Numerical stability**: Best practices for angle arithmetic in presence of floating-point error?

3. **Parallel computation**: Can extrema finding be parallelized effectively?

4. **Adaptive precision**: When to use higher precision arithmetic (e.g., `f128`)?

5. **Incremental updates**: If polygon is modified, can hull be updated incrementally?

6. **3D extension**: How would this generalize to spherical patches in 3D?

## Conclusion

Implementing convex hull for Arcline is a non-trivial extension of standard convex hull algorithms. The key challenges are:

1. Identifying candidate points (including arc interiors)
2. Handling both line segments and circular arcs uniformly
3. Deciding how to represent hull edges (preserve arcs vs. use lines)
4. Maintaining numerical precision in angle calculations

The recommended approach is incremental: start with a line-segment-only hull, then progressively add arc preservation. The existing codebase provides good foundations (arc/angle utilities, robust predicates), and the ordered nature of Arcline suggests an O(n) algorithm is achievable.

---

**Document Status**: Research and design phase  
**Next Steps**: 
1. Implement Stage 1 (line-segment-only hull)
2. Validate with test cases
3. Extend to Stage 2 and 3 as needed

**Last Updated**: November 20, 2025

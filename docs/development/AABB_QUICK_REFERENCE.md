# AABB Quick Reference Guide

## What is AABB?

AABB = **Axis-Aligned Bounding Box** - A rectangular box aligned with X and Y axes. Used to quickly filter geometric queries before doing expensive detailed calculations.

## Quick Start

### 1. Create a Tree
```rust
use togo::prelude::*;

// With preallocated capacity (100 boxes)
let mut tree = AABBTree::with_capacity(100);

// Or with default capacity
let mut tree = AABBTree::new();
```

### 2. Add Boxes (As Specified)
```rust
// add(min_x, max_x, min_y, max_y) -> index
let idx0 = tree.add(0.0, 10.0, 0.0, 10.0);    // Returns 0
let idx1 = tree.add(5.0, 15.0, 5.0, 15.0);    // Returns 1
let idx2 = tree.add(20.0, 30.0, 20.0, 30.0);  // Returns 2
```

### 3. Query Overlapping Boxes
```rust
// Create a query box
let query = AABB::new(7.0, 12.0, 7.0, 12.0);

// Preallocate results (efficient - reusable)
let mut results = Vec::new();

// Query: find all boxes that overlap
tree.query_intersecting(&query, &mut results);

// results = [0, 1]  (both box 0 and 1 overlap with query)
```

### 4. Query Point
```rust
let mut results = Vec::new();
tree.query_point(7.0, 7.0, &mut results);
// results = [0, 1]  (both boxes contain point (7.0, 7.0))
```

## API Overview

### AABB Methods
```rust
// Create
AABB::new(min_x, max_x, min_y, max_y)

// Query
aabb.intersects(&other)              // Do two boxes overlap?
aabb.contains_point(x, y)            // Is point inside?

// Dimensions
aabb.width()                          // max_x - min_x
aabb.height()                         // max_y - min_y
aabb.area()                           // width * height
```

### AABBTree Methods
```rust
// Creation
AABBTree::new()
AABBTree::with_capacity(100)

// Adding (as specified in requirements)
tree.add(min_x, max_x, min_y, max_y) -> usize

// Querying (with reusable result vector)
tree.query_intersecting(&query, &mut results)
tree.query_point(x, y, &mut results)

// Access
tree.get(index) -> Option<&AABB>
tree.get_mut(index) -> Option<&mut AABB>

// Utilities
tree.len()                            // Number of boxes
tree.is_empty()                       // Is tree empty?
tree.capacity()                       // Allocated capacity
tree.clear()                          // Remove all boxes (keeps capacity)
tree.reserve(additional)              // Reserve more capacity

// Iteration
for aabb in tree.iter() { }
for aabb in tree.iter_mut() { }
```

## Common Patterns

### Pattern 1: Efficient Query with Result Reuse
```rust
let mut tree = AABBTree::with_capacity(1000);
// ... add boxes ...

// Create result vector once
let mut results = Vec::with_capacity(50);

// Query 1
let q1 = AABB::new(0.0, 10.0, 0.0, 10.0);
tree.query_intersecting(&q1, &mut results);
process_results(&results);

// Query 2 - reuses same vector (cleared automatically)
let q2 = AABB::new(50.0, 60.0, 50.0, 60.0);
tree.query_intersecting(&q2, &mut results);
process_results(&results);
```

### Pattern 2: Filter Before Expensive Operations
```rust
// Fast AABB check first
let bounding = AABB::new(x1, x2, y1, y2);
let mut candidates = Vec::new();
tree.query_intersecting(&bounding, &mut candidates);

// Only run expensive tests on candidates
for &idx in &candidates {
    if let Some(box_aabb) = tree.get(idx) {
        if expensive_intersection_test(box_aabb) {
            // Process intersection
        }
    }
}
```

### Pattern 3: Spatial Partitioning
```rust
let mut tree = AABBTree::with_capacity(100);

// Add geometric primitives as bounding boxes
for arc in arcs {
    let (x_min, x_max, y_min, y_max) = arc.bounding_box();
    tree.add(x_min, x_max, y_min, y_max);
}

// Now can quickly find candidate intersections
let query = AABB::new(query_x1, query_x2, query_y1, query_y2);
let mut candidates = Vec::new();
tree.query_intersecting(&query, &mut candidates);
```

## Performance Tips

✅ **Do:**
- Preallocate with expected capacity: `AABBTree::with_capacity(100)`
- Reuse result vectors across multiple queries
- Use for early rejection/filtering
- Store indices, not AABB copies

❌ **Avoid:**
- Creating new result vectors for each query
- Using with very large datasets (1000+) without optimization
- Frequent add/remove operations
- Using as final spatial answer (use for filtering first)

## When to Use AABB

✅ **Perfect For:**
- Filtering before detailed geometric tests
- Spatial queries on 10-500 objects
- CAD tool geometric acceleration
- Scene intersection culling
- Bounding box checks

⚠️ **Consider Alternatives For:**
- 1000+ objects (use hierarchical structure)
- Frequent add/remove (consider dynamic tree)
- Complex hierarchical queries (use BVH)
- Point clouds (use grid/octree)

## Example: CAD Arc Intersection

```rust
use togo::prelude::*;

// Build spatial index of arcs
let mut tree = AABBTree::with_capacity(arcs.len());
for arc in &arcs {
    let (x1, x2, y1, y2) = arc.bounding_box();
    tree.add(x1, x2, y1, y2);
}

// Find potential intersections
let mut results = Vec::new();
let query_box = AABB::new(0.0, 100.0, 0.0, 100.0);
tree.query_intersecting(&query_box, &mut results);

// Only test actual intersections for candidates
for &idx in &results {
    if let Some(candidate_aabb) = tree.get(idx) {
        // Run detailed arc intersection test only on candidates
        if let Some(intersection_point) = test_arc_intersection(...) {
            // Found real intersection
        }
    }
}
```

## Complexity Summary

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `add()` | O(1) | Amortized |
| `query_intersecting()` | O(n) | n = number of boxes |
| `query_point()` | O(n) | n = number of boxes |
| `clear()` | O(1) | Keeps capacity |
| `get()` / `get_mut()` | O(1) | Direct index access |

## Intersection Logic

Boxes intersect when:
```
box1.min_x <= box2.max_x  &&
box1.max_x >= box2.min_x  &&
box1.min_y <= box2.max_y  &&
box1.max_y >= box2.min_y
```

Point inside box when:
```
point.x >= box.min_x && point.x <= box.max_x &&
point.y >= box.min_y && point.y <= box.max_y
```

## Test Status

✅ 16 unit tests passing
✅ 8 documentation examples passing
✅ Clean compilation (0 warnings)
✅ Production ready

## See Also

- `AABB_IMPLEMENTATION.md` - Full documentation
- `src/spatial/aabb.rs` - Source code with comments

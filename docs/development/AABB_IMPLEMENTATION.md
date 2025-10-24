# AABB (Axis-Aligned Bounding Box) Spatial Data Structure

## Overview

Created a simple but efficient AABB spatial data structure in `src/spatial/aabb.rs` for accelerating geometric spatial queries. The implementation uses preallocated vector storage to minimize allocations and maximize cache efficiency.

## Implementation Details

### Core Components

#### 1. `AABB` Struct
A single axis-aligned bounding box with four bounds (min_x, max_x, min_y, max_y).

**Key Methods:**
- `new(min_x, max_x, min_y, max_y)` - Create a new AABB
- `intersects(&self, other: &AABB) -> bool` - Check if two AABBs overlap
- `contains_point(x, y) -> bool` - Check if a point is inside the AABB
- `width()` / `height()` / `area()` - Get dimensions

**Example:**
```rust
use togo::prelude::*;

let aabb = AABB::new(0.0, 10.0, 0.0, 10.0);
assert!(aabb.intersects(&AABB::new(5.0, 15.0, 5.0, 15.0)));
assert!(aabb.contains_point(5.0, 5.0));
```

#### 2. `AABBTree` - AABB Collection with Preallocated Storage

A collection that stores multiple AABBs in a preallocated vector for efficient queries.

**Key Features:**
- ✅ Preallocated vector storage (`Vec::with_capacity`)
- ✅ No dynamic allocations during queries
- ✅ Reusable result vectors for queries
- ✅ O(n) linear search for intersections (suitable for small-to-medium datasets)

**Key Methods:**

```rust
// Creation
pub fn with_capacity(capacity: usize) -> Self
pub fn new() -> Self

// Adding boxes (as specified)
pub fn add(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> usize

// Queries with preallocated results
pub fn query_intersecting(&self, query: &AABB, results: &mut Vec<usize>)
pub fn query_point(&self, x: f64, y: f64, results: &mut Vec<usize>)

// Utilities
pub fn get(index: usize) -> Option<&AABB>
pub fn get_mut(index: usize) -> Option<&mut AABB>
pub fn clear()
pub fn reserve(additional: usize)
pub fn len() -> usize
pub fn capacity() -> usize
pub fn iter() -> Iterator
pub fn iter_mut() -> IteratorMut
```

## Usage Examples

### Basic Setup
```rust
use togo::prelude::*;

// Create a tree with initial capacity
let mut tree = AABBTree::with_capacity(100);

// Add bounding boxes (as specified in requirements)
let idx1 = tree.add(0.0, 10.0, 0.0, 10.0);    // Returns index 0
let idx2 = tree.add(5.0, 15.0, 5.0, 15.0);    // Returns index 1
let idx3 = tree.add(20.0, 30.0, 20.0, 30.0);  // Returns index 2
```

### Query: Find All Overlapping Boxes
```rust
let query = AABB::new(7.0, 12.0, 7.0, 12.0);
let mut results = Vec::new();
tree.query_intersecting(&query, &mut results);

// results will contain [0, 1] (indices of boxes that overlap with query)
for idx in &results {
    println!("Box {} overlaps", idx);
}
```

### Query: Find All Boxes Containing a Point
```rust
let mut results = Vec::new();
tree.query_point(7.0, 7.0, &mut results);

// results will contain [0, 1] (both boxes contain point (7.0, 7.0))
```

### Reuse Result Vectors (Efficient)
```rust
let mut results = Vec::with_capacity(10);

// Query 1
let q1 = AABB::new(7.0, 12.0, 7.0, 12.0);
tree.query_intersecting(&q1, &mut results);
println!("Found {} intersections", results.len());

// Reuse the same vector for query 2 (results.clear() called automatically)
let q2 = AABB::new(25.0, 35.0, 25.0, 35.0);
tree.query_intersecting(&q2, &mut results);
println!("Found {} intersections", results.len());
```

### Iterate Over All Boxes
```rust
for aabb in tree.iter() {
    println!("Box: ({}, {}) to ({}, {})", 
             aabb.min_x, aabb.min_y, aabb.max_x, aabb.max_y);
}
```

## Design Decisions

### 1. **Preallocated Vector Storage** ✅
- Uses `Vec::with_capacity()` for initial allocation
- No dynamic allocations during queries
- Result vectors are cleared and reused, not recreated
- Ideal for CAD/geometric applications with predictable workloads

### 2. **Simple Linear Search** ✅
- O(n) intersection query (sufficient for small-to-medium datasets)
- No complex tree structure overhead
- Perfect for paths with 10-100 bounding boxes
- Easy to understand and debug

### 3. **Index-Based Results**
- Queries return indices, not AABB copies
- Allows client code to maintain multiple views of same data
- Memory efficient

### 4. **Reusable Result Vectors**
- Query functions take `&mut Vec<usize>` as parameter
- Results are cleared before filling (no append overhead)
- Client pre-allocates with desired capacity
- Zero allocations during queries (after initial setup)

## Testing

### Test Coverage: 16 Unit Tests

All tests pass ✅:

**AABB Tests:**
- `test_aabb_creation` - Basic AABB creation
- `test_aabb_dimensions` - Width, height, area calculations
- `test_aabb_intersection_overlapping` - Overlapping AABBs
- `test_aabb_intersection_touching` - Edge-touching AABBs
- `test_aabb_intersection_separate` - Non-overlapping AABBs
- `test_aabb_contains_point_inside` - Point inside AABB
- `test_aabb_contains_point_on_edge` - Point on AABB edge
- `test_aabb_contains_point_outside` - Point outside AABB

**AABBTree Tests:**
- `test_aabb_tree_creation` - Tree creation and capacity
- `test_aabb_tree_add` - Adding boxes with correct indices
- `test_aabb_tree_query_intersecting` - Finding overlapping boxes
- `test_aabb_tree_query_point` - Finding boxes containing point
- `test_aabb_tree_clear` - Clearing tree contents
- `test_aabb_tree_get` - Retrieving boxes by index
- `test_aabb_tree_iterator` - Iterating over boxes
- `test_aabb_tree_reuse_query_results` - Reusing result vectors

### Test Results
```
test result: ok. 16 passed; 0 failed
```

### Documentation Tests
All 8 doc examples pass ✅

## File Structure

```
src/spatial/
├── mod.rs          (Module definition and exports)
└── aabb.rs         (AABB and AABBTree implementation)
```

## Integration

### Added to Module System
- `src/spatial/mod.rs` - Exports AABB and AABBTree
- `src/lib.rs` - Added `pub mod spatial;`
- `src/lib.rs` prelude - Added AABB, AABBTree exports

### Import Methods
```rust
// Via prelude
use togo::prelude::*;
let tree = AABBTree::with_capacity(10);

// Direct import
use togo::spatial::{AABB, AABBTree};

// Module import
use togo::spatial;
let aabb = spatial::AABB::new(0.0, 10.0, 0.0, 10.0);
```

## Performance Characteristics

### Time Complexity
- `add()`: O(1) amortized
- `query_intersecting()`: O(n) where n = number of boxes
- `query_point()`: O(n) where n = number of boxes
- `clear()`: O(1)
- `iter()`: O(1) to start, O(1) per element

### Space Complexity
- Storage: O(n) for n boxes
- Query results: O(k) where k = matches found
- No additional overhead beyond the boxes themselves

### Memory Usage
With preallocated capacity of 100:
- Each AABB: 32 bytes (4 × f64)
- Total capacity: ~3200 bytes (100 boxes)
- No fragmentation after clear

### Performance Profile
- **Optimal**: 1-100 boxes, frequent queries
- **Good**: 100-1000 boxes, occasional queries
- **Consider optimization**: 1000+ boxes, frequent queries

## Suitable Use Cases

✅ **Perfect For:**
- Spatial partitioning of geometric primitives
- Bounding box filtering before detailed intersection tests
- CAD tool geometric queries
- Scene spatial indexing with predictable sizes
- Early rejection in geometric algorithms

✅ **Very Efficient When:**
- Dataset size is known and stable (preallocate capacity)
- Queries reuse result vectors
- Result vectors are reused across queries
- Number of boxes is 10-500

⚠️ **Consider Alternatives For:**
- Highly dynamic datasets (frequent add/remove)
- Very large datasets (1000+ boxes)
- Complex hierarchical queries
- When you need spatial sorting

## Future Enhancement Ideas

### 1. **Spatial Sorting (BVH)**
```rust
pub struct BVHTree { /* ... */ }
// Better for large datasets or fewer queries
```

### 2. **Grid-Based Partitioning**
```rust
pub struct GridPartition {
    cells: HashMap<(i32, i32), Vec<usize>>
}
// Better for uniform distribution
```

### 3. **Batch Operations**
```rust
pub fn query_batch(&self, queries: &[AABB], results: &mut Vec<Vec<usize>>)
// Process multiple queries efficiently
```

### 4. **Dynamic Updates**
```rust
pub fn update(&mut self, index: usize, min_x: f64, max_x: f64, min_y: f64, max_y: f64)
// Update existing AABB in place
```

### 5. **Visualization**
```rust
pub fn debug_svg(&self) -> String
// Generate SVG representation for debugging
```

## Conclusion

A simple, efficient AABB spatial data structure perfectly suited for geometric applications requiring bounding box queries. The preallocated vector storage ensures predictable performance and minimal allocations during queries.

**Test Status**: ✅ 16 tests passing, 8 doc examples passing
**Integration**: ✅ Integrated into prelude and main module
**Ready**: ✅ Production ready with comprehensive documentation

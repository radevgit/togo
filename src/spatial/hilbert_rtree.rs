#![allow(dead_code)]

/// Simple Hilbert R-tree for spatial queries
///
/// Stores bounding boxes in a flat collection sorted by Hilbert curve order.
/// Efficient for static or infrequently-modified spatial data.
///
/// # Examples
/// ```
/// use togo::spatial::HilbertRTree;
///
/// let mut tree = HilbertRTree::new();
/// tree.add(0.0, 1.0, 0.0, 1.0);
/// tree.add(0.5, 1.5, 0.5, 1.5);
/// tree.build();
/// 
/// let mut results = Vec::new();
/// tree.query_intersecting(0.7, 1.3, 0.7, 1.3, &mut results);
/// assert_eq!(results.len(), 2); // Both boxes intersect the query
/// ```
#[derive(Clone)]
pub struct HilbertRTree {
    /// Flat storage: (min_x, max_x, min_y, max_y) for each box
    boxes: Vec<(f64, f64, f64, f64)>,
    /// Hilbert indices for sorting
    hilbert_indices: Vec<u64>,
    /// Sort order: mapping from Hilbert-sorted position to original index
    sorted_order: Vec<usize>,
    /// Whether the tree has been built
    built: bool,
}

impl HilbertRTree {
    /// Creates a new empty Hilbert R-tree
    pub fn new() -> Self {
        HilbertRTree::with_capacity(0)
    }

    /// Creates a new Hilbert R-tree with preallocated capacity
    ///
    /// # Arguments
    /// * `capacity` - Expected number of bounding boxes to be added
    pub fn with_capacity(capacity: usize) -> Self {
        HilbertRTree {
            boxes: Vec::with_capacity(capacity),
            hilbert_indices: Vec::with_capacity(capacity),
            sorted_order: Vec::with_capacity(capacity),
            built: false,
        }
    }

    /// Adds a bounding box to the tree
    ///
    /// Must call `build()` after adding all boxes before querying
    pub fn add(&mut self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) {
        self.boxes.push((min_x, max_x, min_y, max_y));
        self.built = false;
    }

    /// Builds the Hilbert R-tree index
    ///
    /// Call this after adding all boxes and before querying
    pub fn build(&mut self) {
        if self.boxes.is_empty() {
            return;
        }

        // Compute Hilbert indices for all boxes (by center)
        self.hilbert_indices.clear();
        for &(min_x, max_x, min_y, max_y) in &self.boxes {
            let center_x = (min_x + max_x) / 2.0;
            let center_y = (min_y + max_y) / 2.0;
            let h_idx = hilbert_index(center_x, center_y, 16);
            self.hilbert_indices.push(h_idx);
        }

        // Create sort order
        self.sorted_order = (0..self.boxes.len()).collect();
        self.sorted_order.sort_by_key(|&idx| self.hilbert_indices[idx]);

        self.built = true;
    }

    /// Queries for all boxes intersecting the given bounding box
    ///
    /// Results are appended to the output vector (not cleared first)
    pub fn query_intersecting(
        &self,
        query_min_x: f64,
        query_max_x: f64,
        query_min_y: f64,
        query_max_y: f64,
        results: &mut Vec<usize>,
    ) {
        for &idx in &self.sorted_order {
            let (min_x, max_x, min_y, max_y) = self.boxes[idx];
            
            // AABB intersection test
            if min_x <= query_max_x && max_x >= query_min_x &&
               min_y <= query_max_y && max_y >= query_min_y {
                results.push(idx);
            }
        }
    }

    /// Returns the number of boxes in the tree
    pub fn len(&self) -> usize {
        self.boxes.len()
    }

    /// Returns whether the tree is empty
    pub fn is_empty(&self) -> bool {
        self.boxes.is_empty()
    }
}

impl Default for HilbertRTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute Hilbert curve index for a point
fn hilbert_index(x: f64, y: f64, max_level: u32) -> u64 {
    let xi = (x.clamp(0.0, 1.0) * ((1u64 << max_level) as f64)) as u64;
    let yi = (y.clamp(0.0, 1.0) * ((1u64 << max_level) as f64)) as u64;
    xy_to_hilbert(xi, yi, max_level)
}

/// Convert 2D coordinates to Hilbert curve index
fn xy_to_hilbert(x: u64, y: u64, order: u32) -> u64 {
    if order == 0 {
        return 0;
    }

    let mut d = 0u64;
    let mut s = 1u64 << (order.saturating_sub(1).min(63));

    let mut x = x;
    let mut y = y;

    while s > 0 {
        let rx = ((x & s) > 0) as u64;
        let ry = ((y & s) > 0) as u64;
        d = d.saturating_add(s.saturating_mul(s).saturating_mul((3 * rx) ^ ry));

        // Rotate
        if ry == 0 {
            if rx == 1 {
                x = s.saturating_sub(1).saturating_sub(x);
                y = s.saturating_sub(1).saturating_sub(y);
            }
            std::mem::swap(&mut x, &mut y);
        }

        s >>= 1;
    }

    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hilbert_rtree_new() {
        let tree = HilbertRTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_hilbert_rtree_add() {
        let mut tree = HilbertRTree::new();
        tree.add(0.0, 1.0, 0.0, 1.0);
        tree.add(2.0, 3.0, 2.0, 3.0);
        
        assert_eq!(tree.len(), 2);
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_hilbert_rtree_build() {
        let mut tree = HilbertRTree::new();
        tree.add(0.0, 1.0, 0.0, 1.0);
        tree.add(2.0, 3.0, 2.0, 3.0);
        tree.build();
        
        assert!(tree.built);
    }

    #[test]
    fn test_hilbert_rtree_query_intersecting() {
        let mut tree = HilbertRTree::new();
        tree.add(0.0, 1.0, 0.0, 1.0);
        tree.add(2.0, 3.0, 2.0, 3.0);
        tree.add(0.5, 1.5, 0.5, 1.5);
        tree.build();
        
        let mut results = Vec::new();
        tree.query_intersecting(0.7, 1.3, 0.7, 1.3, &mut results);
        
        // Should find boxes 0 and 2 (both intersect the query)
        assert_eq!(results.len(), 2);
        assert!(results.contains(&0));
        assert!(results.contains(&2));
    }

    #[test]
    fn test_hilbert_rtree_query_no_intersections() {
        let mut tree = HilbertRTree::new();
        tree.add(0.0, 1.0, 0.0, 1.0);
        tree.add(5.0, 6.0, 5.0, 6.0);
        tree.build();
        
        let mut results = Vec::new();
        tree.query_intersecting(2.0, 3.0, 2.0, 3.0, &mut results);
        
        // No boxes intersect the query
        assert_eq!(results.len(), 0);
    }
}

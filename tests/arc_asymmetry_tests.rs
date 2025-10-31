use togo::prelude::*;

#[test]
fn test_arcseg_arc_asymmetry() {
    let seg = arcseg(point(0.5, 0.5), point(0.5, -1.0));
    let arc1 = arc(point(0.0, 0.0), point(1.0, 0.0), point(0.5, 0.5), 1.0);
    let ab = is_really_intersecting(&arc1, &seg);
    let ba = is_really_intersecting(&seg, &arc1);
    assert_eq!(ab, ba, "is_really_intersecting not symmetric for arc/seg");
}

#[test]
fn test_arc_arc_asymmetry() {
    let arc1 = arc(point(-1.0, 0.0), point(1.0, 0.0), point(0.0, 1.0), 1.0);
    let arc2 = arc(point(0.0, -1.0), point(0.0, 1.0), point(1.0, 0.0), 1.0);
    let ab = is_really_intersecting(&arc1, &arc2);
    let ba = is_really_intersecting(&arc2, &arc1);
    assert_eq!(ab, ba, "is_really_intersecting not symmetric for arc/arc");
}

#[test]
fn test_arcseg_arcseg_asymmetry() {
    let seg1 = arcseg(point(0.0, 0.0), point(1.0, 1.0));
    let seg2 = arcseg(point(0.0, 1.0), point(1.0, 0.0));
    let ab = is_really_intersecting(&seg1, &seg2);
    let ba = is_really_intersecting(&seg2, &seg1);
    assert_eq!(ab, ba, "is_really_intersecting not symmetric for seg/seg");
}

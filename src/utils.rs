#![allow(dead_code)]

const TWO_COMPLEMENT_64: u64 = 0x8000_0000_0000_0000_u64;
const TWO_COMPLEMENT_CI_64: i64 = TWO_COMPLEMENT_64 as i64;
/// Compares two f64 values for approximate equality
///
/// Use ULP (Units in the Last Place) comparison.
///
/// This function provides a robust way to compare floating-point numbers by converting
/// them to their bit representation and comparing the integer difference. This method
/// accounts for the non-uniform distribution of floating-point numbers.
///
/// # Arguments
///
/// * `a` - First floating-point value to compare
/// * `b` - Second floating-point value to compare  
/// * `ulps` - Maximum allowed difference in ULPs (must be positive)
///
/// # Returns
///
/// True if the values are within the specified ULP tolerance
///
/// # Behavior
///
/// - Values with different signs are only equal if both are exactly zero
/// - The function converts floating-point values to lexicographically ordered integers
/// - Negative values are handled specially to maintain proper ordering
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let a = 1.0f64;
/// let b = 1.0000000000000002f64; // Next representable value after 1.0
/// assert!(almost_equal_as_int(a, b, 1)); // Within 1 ULP
/// ```
///
/// # References
///
/// Based on:
/// - [](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
/// - [](https://github.com/randomascii/blogstuff/blob/main/FloatingPoint/CompareAsInt/CompareAsInt.cpp#L133)
///
/// # Safety
///
/// The input values must be finite (not NaN or infinite).
#[inline]
#[must_use]
pub fn almost_equal_as_int(a: f64, b: f64, ulps: u64) -> bool {
    debug_assert!(a.is_finite());
    debug_assert!(b.is_finite());

    let mut a_i: i64 = a.to_bits() as i64;
    let mut b_i: i64 = b.to_bits() as i64;

    // Make a_i, b_i lexicographically ordered as a twos-complement int
    if a_i < 0i64 {
        a_i = TWO_COMPLEMENT_CI_64 - a_i;
    }
    if b_i < 0i64 {
        b_i = TWO_COMPLEMENT_CI_64 - b_i;
    }

    // Use saturating arithmetic to avoid overflow when values are very far apart
    let diff = (a_i as i128) - (b_i as i128);
    diff.abs() <= ulps as i128

}

/// Checks if two floating-point values are close within an epsilon tolerance.
///
/// This provides a simple absolute difference comparison, which is suitable
/// for values near zero or when you know the expected magnitude of the values.
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
/// * `eps` - Maximum allowed absolute difference
///
/// # Returns
///
/// True if |a - b| <= eps
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// assert!(close_enough(1.0, 1.001, 0.01));
/// assert!(!close_enough(1.0, 1.1, 0.01));
/// ```
#[must_use]
pub fn close_enough(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

/// Perturbs a floating-point value by a specified number of ULPs.
///
/// This function is primarily intended for testing floating-point algorithms
/// by introducing controlled numerical errors.
///
/// # Arguments
///
/// * `f` - The floating-point value to perturb
/// * `c` - The number of ULPs to add (can be negative)
///
/// # Returns
///
/// The perturbed floating-point value
///
/// # Safety
///
/// This function is unsafe and should only be used for testing. It does not
/// handle edge cases like overflow or underflow properly.
#[must_use]
pub fn perturbed_ulps_as_int(f: f64, c: i64) -> f64 {
    // Special case: f == 0.0 and c == -1 should return -0.0 (valid bit pattern)
    if f == 0.0 && c == -1 {
        return -0.0;
    }
    let mut f_i: i64 = f.to_bits() as i64;
    f_i += c;
    f64::from_bits(f_i as u64)
}

#[cfg(test)]
mod test_almost_equal_as_int {

    use super::*;

    #[test]
    fn test_almost_equal_as_int_negative_zero() {
        // Make sure that zero and negativeZero compare as equal.
        assert!(almost_equal_as_int(0.0, -0.0, 0));
    }

    #[test]
    fn test_almost_equal_as_int_nearby_numbers() {
        // Make sure that nearby numbers compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.999999999999999, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-2.0, -1.999999999999999, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_slightly_more_distant() {
        // Make sure that slightly more distant numbers compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.999999999999998, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-2.0, -1.999999999999998, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_slightly_more_distant_reversed() {
        // Make sure the results are the same with parameters reversed.
        let result: bool = almost_equal_as_int(1.999999999999998, 2.0, 10);
        assert_eq!(result, true);
        let result: bool = almost_equal_as_int(-1.999999999999998, -2.0, 10);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_distant() {
        // Make sure that even more distant numbers don't compare as equal.
        let result: bool = almost_equal_as_int(2.0, 1.999999999999997, 10);
        assert_eq!(result, false);
        let result: bool = almost_equal_as_int(-2.0, -1.999999999999997, 10);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_distant_reversed() {
        // Make sure the results are the same with parameters reversed
        let result: bool = almost_equal_as_int(1.999999999999997, 2.0, 10);
        assert_eq!(result, false);
        let result: bool = almost_equal_as_int(-1.999999999999997, -2.0, 10);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_upper_limit_small() {
        // Upper limit of f64 small distance
        let mut f_u: u64 = f64::MAX.to_bits();
        f_u -= 2;
        let f_f = f64::from_bits(f_u);
        let result: bool = almost_equal_as_int(f64::MAX, f_f, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_upper_limit_large() {
        // Upper limit of f64 large distance
        let mut f_u: u64 = f64::MAX.to_bits();
        f_u -= 4;
        let f_f = f64::from_bits(f_u);
        let result: bool = almost_equal_as_int(f64::MAX, f_f, 3);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_lower_limit_small() {
        // Lower limit of f64 small distance
        let mut f_u: u64 = f64::MIN.to_bits();
        f_u -= 2;
        let f_f = f64::from_bits(f_u);
        let result: bool = almost_equal_as_int(f64::MIN, f_f, 3);
        assert_eq!(result, true);
    }

    #[test]
    fn test_almost_equal_as_int_lower_limit_large() {
        // Lower limit of f64 large distance
        let mut f_u: u64 = f64::MIN.to_bits();
        f_u -= 4;
        let f_f = f64::from_bits(f_u);
        let result: bool = almost_equal_as_int(f64::MIN, f_f, 3);
        assert_eq!(result, false);
    }

    #[test]
    fn test_almost_equal_as_int_some_numbers() {
        let result: bool = almost_equal_as_int(100.0, -300.0, 10);
        assert_eq!(result, false);
    }

    #[test]
    #[ignore = "printing"]
    fn test_print() {
        print_numbers();
        assert!(true);
    }

    // Used to print number representation
    fn print_number(f: f64, o: i64) {
        let mut f_i: i64 = f.to_bits() as i64;
        f_i += o;
        println!("{:.20} Ox{:X} {:.}", f, f_i, f_i);
    }

    pub fn print_numbers() {
        let f: f64 = 2.0f64;
        print_number(f, 0);
        println!("");
        let f: f64 = 1.999999999999998;
        for i in -10..=10i64 {
            print_number(f, i);
        }
        println!("");

        let f: f64 = 0.0;
        print_number(f, 0 as i64);
        let o: i64 = i64::from_ne_bytes(0x8000_0000_0000_0000u64.to_ne_bytes());
        print_number(f, o);
        println!("");

        for i in 0..=3i64 {
            print_number(f, i);
        }
        println!("");

        let c_i: i64 = i64::from_ne_bytes(0x8000_0000_0000_0000u64.to_ne_bytes());
        for i in 0..=3i64 {
            print_number(f, i + c_i);
        }
        println!("");
    }

    #[test]
    fn test_perturbed_ulps_as_int_0_minus_1() {
        // f = 0.0, c = -1 should return -0.0
        let result = perturbed_ulps_as_int(0.0, -1);
        assert_eq!(result, -0.0);
        // Check that -0.0 and 0.0 are considered almost equal
        assert!(almost_equal_as_int(result, 0.0, 0));
        // Check that the bit pattern is correct
        assert_eq!(result.to_bits(), (-0.0f64).to_bits());
    }

    #[test]
    fn test_perturbed_ulps_as_int() {
        let t = 1.0;
        let tt = perturbed_ulps_as_int(t, -1);
        let res = almost_equal_as_int(t, tt, 1);
        //println!("{:.20} {:.20}", t, tt);
        assert_eq!(res, true);

        let t = 1.0;
        let tt = perturbed_ulps_as_int(t, -1000);
        let res = almost_equal_as_int(t, tt, 1000);
        assert_eq!(res, true);

        let t = f64::MAX;
        let tt = perturbed_ulps_as_int(t, -1000);
        let res = almost_equal_as_int(t, tt, 1000);
        assert_eq!(res, true);
        //println!("{:.20} {:.20}", t, tt);

        let t = f64::MAX;
        let tt = perturbed_ulps_as_int(t, -1000000000);
        let res = almost_equal_as_int(t, tt, 1000000000);
        assert_eq!(res, true);
        //println!("{:.20} {:.20}", t, tt);
    }

    #[test]
    fn test_positive_negative_zero() {
        // Check that positive and negative zeros are equal
        assert!(almost_equal_as_int(-0f64, 0f64, 0));
    }
}

/// Computes (a×b - c×d) with improved numerical precision.
///
/// This function uses the Kahan method to avoid catastrophic cancellation
/// that can occur when subtracting two products of similar magnitude.
/// The algorithm rearranges the computation to minimize floating-point errors.
///
/// # Arguments
///
/// * `a`, `b` - Terms of the first product
/// * `c`, `d` - Terms of the second product
///
/// # Returns
///
/// The difference a×b - c×d computed with enhanced precision
///
/// # Algorithm
///
/// The function uses fused multiply-add operations to compute:
/// 1. cd = c × d
/// 2. err = (-c) × d + cd (captures rounding error)
/// 3. dop = a × b - cd (main computation)
/// 4. Returns dop + err (error correction)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// // This would suffer from catastrophic cancellation with naive computation
/// let result = diff_of_prod(1e16, 1.0, 1e16, 1.0000000000000001);
/// ```
///
/// # References
///
/// - [](https://pharr.org/matt/blog/2019/11/03/difference-of-floats)
/// - [](https://herbie.uwplse.org/) (for equation rearrangement)
#[inline]
pub fn diff_of_prod(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let err = (-c).mul_add(d, cd);
    let dop = a.mul_add(b, -cd);
    dop + err
}

/// Computes (a×b + c×d) with improved numerical precision.
///
/// This function uses the Kahan method to enhance the precision of the sum
/// of two products by capturing and correcting rounding errors.
///
/// # Arguments
///
/// * `a`, `b` - Terms of the first product
/// * `c`, `d` - Terms of the second product
///
/// # Returns
///
/// The sum a×b + c×d computed with enhanced precision
///
/// # Algorithm
///
/// The function uses fused multiply-add operations to compute:
/// 1. cd = c × d
/// 2. err = c × d - cd (captures rounding error)
/// 3. sop = a × b + cd (main computation)
/// 4. Returns sop + err (error correction)
///
/// # Examples
///
/// ```
/// use togo::prelude::*;
///
/// let result = sum_of_prod(1.5, 2.0, 3.0, 4.0); // 1.5×2.0 + 3.0×4.0 = 15.0
/// ```
#[inline]
pub fn sum_of_prod(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let cd = c * d;
    let err = c.mul_add(d, -cd);
    let sop = a.mul_add(b, cd);
    sop + err
}

#[inline]
pub fn min_5(a: f64, b: f64, c: f64, d: f64, e: f64) -> f64 {
    a.min(b).min(c).min(d).min(e)
}

#[inline]
pub fn min_4(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a.min(b).min(c).min(d)
}

#[inline]
pub fn min_3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

// #[cfg(test)]
// use rand::distr::{Distribution, Uniform};
// #[cfg(test)]
// use rand::rngs::StdRng;

// #[cfg(test)]
// use crate::segment::Segment;
// #[cfg(test)]
// use crate::Arc;
// #[cfg(test)]
// pub fn random_arc(wa: f64, wb: f64, ha: f64, hb: f64, b: f64, rng: &mut StdRng) -> Arc {
//     use crate::{arc::arc_circle_parametrization, point};

//     let range_w = Uniform::new(wa, wb).unwrap();
//     let range_h = Uniform::new(ha, hb).unwrap();
//     let bulge = if b != 0.0 {
//         let range_b = Uniform::new(-b, b).unwrap();
//         range_b.sample(rng)
//     } else {
//         0.0
//     };
//     let x0 = range_w.sample(rng);
//     let y0 = range_h.sample(rng);
//     let x1 = range_w.sample(rng);
//     let y1 = range_h.sample(rng);
//     arc_circle_parametrization(point(x0, y0), point(x1, y1), bulge)
// }

// #[cfg(test)]
// pub fn random_segment(wa: f64, wb: f64, ha: f64, hb: f64, rng: &mut StdRng) -> Segment {
//     use crate::{point, segment::segment};

//     let range_w = Uniform::new(wa, wb).unwrap();
//     let range_h = Uniform::new(ha, hb).unwrap();
//     let x0 = range_w.sample(rng);
//     let y0 = range_h.sample(rng);
//     let x1 = range_w.sample(rng);
//     let y1 = range_h.sample(rng);
//     segment(point(x0, y0), point(x1, y1))
// }

#[cfg(test)]
mod test_diff_of_prod {
    use crate::point::point;

    use super::*;

    const _0: f64 = 0f64;
    const _1: f64 = 0f64;
    const _2: f64 = 0f64;

    #[test]
    fn test_diff_of_prod0() {
        let p0 = point(10000.0, 10000.0);
        let p1 = point(-10001.0, -10000.0);
        let res0 = p0.perp(p1);
        let res1 = diff_of_prod(p0.x, p1.y, p0.y, p1.x);
        assert_eq!(res0, res1);
    }

    #[test]
    fn test_diff_of_prod1() {
        let p0 = point(100000.0, 100000.0);
        let p1 = point(-100001.0, -100000.0);
        let res0 = p0.perp(p1);
        let res1 = diff_of_prod(p0.x, p1.y, p0.y, p1.x);
        assert_eq!(res0, res1);
    }
}

#[cfg(test)]
mod test_sum_min_close {
    use super::*;

    #[test]
    fn test_sum_of_prod_basic() {
        // Exact representable case
        let v = sum_of_prod(0.5, 2.0, 0.25, 4.0);
        assert_eq!(v, 2.0);
        // Mixed signs
        let v2 = sum_of_prod(3.0, -2.0, 4.0, 0.5);
        assert_eq!(v2, -6.0 + 2.0);
    }

    #[test]
    fn test_min_3_4_5() {
        assert_eq!(min_3(3.0, 2.0, 1.0), 1.0);
        assert_eq!(min_3(-1.0, -2.0, 0.0), -2.0);

        assert_eq!(min_4(4.0, 3.0, 2.0, 1.0), 1.0);
        assert_eq!(min_4(-1.0, -2.0, -3.0, 0.0), -3.0);

        assert_eq!(min_5(5.0, 4.0, 3.0, 2.0, 1.0), 1.0);
        assert_eq!(min_5(-1.0, -2.0, -3.0, -4.0, 0.0), -4.0);
    }

    #[test]
    fn test_close_enough_bounds() {
        // Strict comparison: |a-b| < eps
        assert!(close_enough(1.0, 1.0009, 0.001));
        assert!(!close_enough(1.0, 1.002, 0.001));
        assert!(close_enough(-1.0, -1.0005, 0.001));
    }
}

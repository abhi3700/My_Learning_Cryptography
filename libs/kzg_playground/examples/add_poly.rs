//! Add 2 polynomials of same/different len with positive coefficients over Fp
//! where p = 18446744073709551557 is a large prime number closest to
//! [`u64::MAX`] This means all the sum result ∈ [0,p) i.e. {0, ..., p-1} i.e.
//! all the `limb_t` values of `l` would follow this range.
//!
//! Defined like this:
//! ```
//! type limb_t = u64
//!
//! struct blst_fr {
//!     l: [limb_t; 4usize],
//! }
//!
//! struct FsFr(pub blst_fr);
//! ```
//!
//! p1 = f(x) = x^3 + 3x + 7
//! p2 = g(x) = x^5 + 34x^4 + 4x^3 + 5x
//!
//! p3 = (p1 + p2) mod p

use kzg::types::fr::FsFr;
use kzg::types::poly::FsPoly;
use kzg_traits::eip_4844::blst_fr;
use kzg_traits::{Fr, Poly};

fn main() {
    // represent the polynomials as vec of coefficients
    let mut poly1_coeffs: Vec<FsFr> =
        vec![7, 3, 0, 1].into_iter().map(|c| FsFr::from_u64(c)).collect();
    let mut poly2_coeffs: Vec<FsFr> =
        vec![0, 5, 0, 4, 34, 1].into_iter().map(|c| FsFr::from_u64(c)).collect();

    // choose a large prime number closest to `u64::MAX`
    let prime_num = 18446744073709551557_u64;

    // just for representing as `FsPoly` type as defined in `blst` crate.
    let poly1 = FsPoly { coeffs: poly1_coeffs.clone() };
    let poly2 = FsPoly { coeffs: poly2_coeffs.clone() };

    // read the
    let p1_len = poly1.len();
    let p2_len = poly2.len();

    // resize the small poly zero padding if unequal len
    let max_len = if p1_len > p2_len {
        poly2_coeffs.resize(p1_len, FsFr::zero());
        p1_len
    } else if p1_len < p2_len {
        poly1_coeffs.resize(p2_len, FsFr::zero());
        p2_len
    } else {
        p1_len
    };

    // ensure polys are of equal len with new vec size.
    assert!(poly1_coeffs.len() == poly2_coeffs.len(), "unequal polys even after zero padding");

    // add the coefficients
    let poly3_coeffs: Vec<FsFr> = poly1_coeffs
        .iter()
        .zip(poly2_coeffs.iter())
        .map(|(a, b)| add_fsfr_coeffs(a, b, prime_num))
        .collect();

    assert_eq!(poly3_coeffs.len(), max_len);
    dbg!(poly3_coeffs.clone());

    // Just for representing poly3
    let _poly3 = FsPoly { coeffs: poly3_coeffs };
}

/// Add FsFr coefficients over a Fp
fn add_fsfr_coeffs(c1: &FsFr, c2: &FsFr, prime_num: u64) -> FsFr {
    let l1 = c1.0.l;
    let l2 = c2.0.l;

    let mut l_c1c2: Vec<u64> =
        l1.iter().zip(l2.iter()).map(|(a, b)| a.wrapping_add(*b) % prime_num).collect();

    // Ensure the vector has exactly 4 elements
    l_c1c2.resize(4, 0);
    // get the array out of slice.
    let l_c1c2: [u64; 4] =
        l_c1c2.as_slice().try_into().expect("Unable to get sized array from slice");

    let sum_c1c2 = FsFr(blst_fr { l: l_c1c2 });

    sum_c1c2
}

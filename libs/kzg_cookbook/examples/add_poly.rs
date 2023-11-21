//! Add 2 polynomials of same/different len with coefficients of
//! signed type (both positive & negative no.s included).
//!
//! p1 = f(x) = x^3 + 3x + 7
//! p2 = g(x) = x^5 + 34x^4 + 4x^3 + 5x

use kzg::types::{fr::FsFr, poly::FsPoly};
use kzg_traits::{eip_4844::blst_fr, Fr, Poly};

fn main() {
    // represent the polys in coefficients
    let mut poly1_coeffs = vec![FsFr::from_u64(7), FsFr::from_u64(3), FsFr::zero(), FsFr::one()];
    let mut poly2_coeffs = vec![
        FsFr::zero(),
        FsFr::from_u64(5),
        FsFr::zero(),
        FsFr::from_u64(4),
        FsFr::from_u64(34),
        FsFr::one(),
    ];

    // just for representing as FsPoly
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
    let poly3_coeffs: Vec<FsFr> =
        poly1_coeffs.iter().zip(poly2_coeffs.iter()).map(|(a, b)| add_fsfr_coeffs(a, b)).collect();

    assert_eq!(poly3_coeffs.len(), max_len);

    // Just for representing poly3
    let poly3 = FsPoly { coeffs: poly3_coeffs };
}

/// FIXME: Add FsFr coefficients
/// Error: `Overflow in checked add`
fn add_fsfr_coeffs(c1: &FsFr, c2: &FsFr) -> FsFr {
    let l1 = c1.0.l;
    let l2 = c2.0.l;

    let mut l_c1c2: Vec<u64> = l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| {
            dbg!(a, b);
            a.checked_add(*b).expect("Overflow in checked_add")
        })
        .collect();
    l_c1c2.resize(4, 0);
    let l_c1c2: [u64; 4] =
        l_c1c2.as_slice().try_into().expect("Unable to get sized array from slice");

    let sum_c1c2 = FsFr(blst_fr { l: l_c1c2 });

    sum_c1c2
}

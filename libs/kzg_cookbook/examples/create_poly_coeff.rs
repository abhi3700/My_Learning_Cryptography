//! Create polynomial of len 4 (= order + 1) from given coefficients.
//!
//! no. of coefficients = polynomial order + 1
//!
//! E.g. f(x) = x^3 + 3x + 7
//! Here, coefficients are represented as [7, 3, 0, 1]. Another way of
//! representing polynomial.
//!
//! A coefficient is represented as [u64; 4] because of BLST crate.
//! Technically it's possible to represent as [U256; 4], but the
//! computers are good in 64-bit arithmetic by default. So, it
//! would be easy to do the arithmetic operations if considered as
//! [u64; 4].
//! Moreover, kzg crate would be difficult to interface with BLST lib.

use kzg::types::fr::FsFr;
use kzg::types::poly::FsPoly;
use kzg_traits::{Fr, Poly};

fn main() {
    let mut poly = FsPoly::new(4).unwrap();
    poly.set_coeff_at(0, &FsFr::from_u64(7));
    poly.set_coeff_at(1, &FsFr::from_u64(3));
    poly.set_coeff_at(2, &FsFr::zero()); // OPTIONAL as already set as zero
    poly.set_coeff_at(3, &FsFr::one());

    // coefficients ordered from [c0, c1, c2, c3]
    dbg!(poly.clone());

    let c = poly.get_coeffs();
    println!("Coefficients: {:?}", c);
    let c0 = poly.get_coeff_at(0);
    println!("Coeff. c0: {:?}", c0.0.l); // --> [64424509425, 1721329240476523535, 18418692815241631664, 3824455624000121028]
    assert_eq!(poly.len(), 4);
}

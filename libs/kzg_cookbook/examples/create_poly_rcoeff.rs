//! Create polynomial of len 5 with random coefficients.
//! len = 5 means no. of coefficients = 5.
//! One way of representing polynomial is to represent as vector of coefficients.

use kzg::types::{fr::FsFr, poly::FsPoly};
use kzg_traits::{Fr, Poly};

fn main() {
    let mut poly = FsPoly::new(5).unwrap();
    // set random coefficients as u64 to determine the Fr poly coefficients
    for i in 0..5 {
        poly.set_coeff_at(i, &FsFr::rand())
    }

    dbg!(poly.clone());
    assert_eq!(poly.len(), 5);
}

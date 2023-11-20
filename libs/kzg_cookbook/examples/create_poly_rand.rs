//! Create polynomial of order 5 with random coefficients.

use kzg::types::poly::FsPoly;
use kzg_traits::Poly;

fn main() {
    let poly = FsPoly::new(5).unwrap();
    // TODO: set random coefficients as u64 to determine the Fr poly coefficients

    dbg!(poly.clone());
    assert_eq!(poly.len(), 5);
}

//! Create polynomial of order 5 with zero coefficients.

use kzg::types::poly::FsPoly;
use kzg_traits::Poly;

fn main() {
    let poly = FsPoly::new(5).unwrap();
    dbg!(poly.clone());
    assert_eq!(poly.len(), 5);
}

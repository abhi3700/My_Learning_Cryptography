//! Elliptic curve maths with real numbers
//!
//! In maths, we can perform operations like P + Q with real numbers.
//! But in cryptography, it's done within [`Finite Field (FF)`] with a
//! large prime number and that's the assumption we make in cryptography.
//!
//! When done with large prime number, then the operations like add, div
//! follows [`modulo arithmetic`].
//!
//! [`Objective`]: Determine the R such that `P + Q + R = 0`, a property of ECC.
//!
//! Then, consider the different cases for these 2 points (P, Q) that are falling
//! on a straight line where,
//! 1. either P or Q is zero
//! 2. if P = Q (are same)
//! 3. if P ≠ Q (are different)
//!
//! --P-------Q-------R-----
//! Basically, there is a straight line that is crossing the curves at 3 points (P, Q, R) on
//! one side of x-axis (preferably upper side).
//! Then, the reflection of the 3rd point on x-axis is `-R`.

// TODO: define a struct for Point on a EC
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: Option<i32>,
    y: Option<i32>,
}

// TODO: define the methods like `add` (modulo), `inverse` for the point,
impl Point {
    fn new(x: Option<i32>, y: Option<i32>) -> Self {
        Self { x, y }
    }

    fn add(p1: Self, p2: Self) -> Result<Self, &'static str> {
        let r: Self = todo!();
        // if P == Q
        // P != Q
        // if P or Q == 0

        Ok(r)
    }
}

fn main() {
    let p = Point {
        x: Some(3),
        y: Some(4),
    };
    let q = Point {
        x: Some(3),
        y: Some(4),
    };

    let r = Point::add(p, q).expect("Unable to determine point \'R\'");
    println!("R: {:?}", r);
}

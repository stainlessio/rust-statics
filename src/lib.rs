#![doc(issue_tracker_base_url = "https://github.com/stainlessio/statics/issues/")]
#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic))]

//! statics is a library that provides types for calculating static forces on free bodies.
//!
//! Most of this code is a result of reading and working through the information in
//! [Structural Design for the Stage](https://amzn.to/2LecGiS).

mod angle;
mod mass;
mod vector;

pub use angle::Angle;
pub use mass::Mass;
pub use vector::Vector;

#[cfg(test)]
extern crate extra_asserts;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

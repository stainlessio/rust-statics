#![doc(issue_tracker_base_url = "https://github.com/stainlessio/statics/issues/")]

//! statics is a library that provides types for calculating static forces on free bodies.
//! 
//! Most of this code is a result of reading and working through the information in
//! [Structural Design for the Stage](https://amzn.to/2LecGiS).

mod angle;
mod vector;

pub use vector::Vector;
pub use angle::Angle;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

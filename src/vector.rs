use angle::Angle;
use mass::Mass;

/// Type that implements a vector for force calculations.
///
/// Internally, it's represented as a magnitude and angle.
/// You can consider it as a right triangle similar to the below:
/// ```ignore
///          /|
///         / |
///        /  |
///       /___|
/// ```
/// The Vector also provides the ability to get the component forces (vertical and horizontal) which is
/// useful for calculating composite forces.
pub struct Vector {
    mag: Mass,
    angle: Angle,
}

impl Vector {
    /// A vector of magnitude 1, with angle 0deg
    pub fn unit() -> Self {
        Self {
            mag: Mass::Lb(1.0),
            angle: Angle::Deg(0.0),
        }
    }

    /// Factor based on a magnitude and angle
    pub fn from_mag_and_angle(mag: Mass, angle: Angle) -> Self {
        Self {
            mag,
            angle,
        }
    }

    pub fn mag(&self) -> Mass {
        self.mag
    }

    pub fn angle(&self) -> &Angle {
        &self.angle
    }

    /// The vertical component of the Vector
    pub fn vertical(&self) -> Self {
        Self {
            mag: self.mag * f64::from(self.angle.as_rad()).sin(),
            angle: Angle::Deg(0.0),
        }
    }

    /// The horizontal component of the Vector
    pub fn horizontal(&self) -> Self {
        Self {
            mag: self.mag * f64::from(self.angle.as_rad()).cos(),
            angle: Angle::Deg(90.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use extra_asserts::*;

    #[test]
    fn vector_unit_has_magnitude_of_one_and_angle_of_zero() {
        let vector = Vector::unit();
        assert_eq!(vector.mag(), Mass::Lb(1.0));
        assert_eq!(vector.angle(), &Angle::Deg(0.0));
    }

    #[test]
    fn vector_has_magnitude() {
        let vector = Vector::from_mag_and_angle(Mass::Lb(5.0), Angle::Deg(0.0));
        assert_eq!(vector.mag(), Mass::Lb(5.0));
    }

    #[test]
    fn vector_has_angle() {
        let vector = Vector::from_mag_and_angle(Mass::Lb(1.0), Angle::Deg(90.0));
        assert_eq!(vector.angle(), &Angle::Deg(90.0));
    }

    #[test]
    fn vector_has_vertical_component() {
        let vector = Vector::from_mag_and_angle(Mass::Lb(500.0), Angle::Deg(20.0));
        let vert = vector.vertical();
        assert_approx_eq(vert.mag(), Mass::Lb(171.0), Mass::Lb(1e-1));
        assert_eq!(f64::from(vert.angle().as_deg()), 0.0);
    }

    #[test]
    fn vector_has_horizontal_component() {
        let vector = Vector::from_mag_and_angle(Mass::Lb(500.0), Angle::Deg(20.0));
        let horiz = vector.horizontal();
        assert_approx_eq(horiz.mag(), Mass::Lb(469.8), Mass::Lb(1e-1));
        assert_eq!(f64::from(horiz.angle().as_deg()), 90.0);
    }

    // #[test]
    // fn vector_addition_yields_new_vector() {
    //     assert!(false, "TODO");
    // }
}

use angle::Angle;

pub struct Vector {
    _mag : f64,
    _angle: Angle,
}

impl Vector {
    pub fn unit() -> Self {
        Self { 
            _mag: 1.0,
            _angle: Angle::Deg(0.0),
        }
    }

    pub fn from_mag_and_angle(mag : f64, angle: Angle) -> Self {
        Self {
            _mag: mag,
            _angle: angle,
        }
    }

    pub fn mag(&self) -> f64 {
        self._mag
    }

    pub fn angle(&self) -> &Angle {
        &self._angle
    }

    pub fn vertical(&self) -> Vector {
        Vector {
            _mag: self._mag * f64::from(self._angle.as_rad()).sin(),
            _angle: Angle::Deg(0.0),
        }
    }

    pub fn horizontal(&self) -> Vector {
        Vector {
            _mag: self._mag * f64::from(self._angle.as_rad()).cos(),
            _angle: Angle::Deg(90.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;
    use std::fmt::Debug;

    fn assert_approx_eq<T>(l : T, r : T, sigma : T::Output)
    where T : Sub + PartialOrd + Debug + Copy,
    T::Output : Debug + PartialOrd {
        let diff = if l < r {
            r - l 
        } else {
            l - r
        };
        println!("Diff {:?}", diff);
        assert!(diff < sigma, format!("{:?} != {:?}", l, r));
    }

    #[test]
    fn vector_unit_has_magnitude_of_one_and_angle_of_zero() {
        let vector = Vector::unit();
        assert_eq!(vector.mag(), 1.0);
        assert_eq!(vector.angle(), &Angle::Deg(0.0));
    }

    #[test]
    fn vector_has_magnitude() {
        let vector = Vector::from_mag_and_angle(5.0, Angle::Deg(0.0));
        assert_eq!(vector.mag(), 5.0);
    }

    #[test]
    fn vector_has_angle() {
        let vector = Vector::from_mag_and_angle(1.0, Angle::Deg(90.0));
        assert_eq!(vector.angle(), &Angle::Deg(90.0));
    }

    #[test]
    fn vector_has_vertical_component() {
        let vector = Vector::from_mag_and_angle(500.0, Angle::Deg(20.0));
        let vert = vector.vertical();
        assert_approx_eq(vert.mag(), 171.0, 1e-1);
        assert_eq!(f64::from(vert.angle().as_deg()), 0.0);
    }

    #[test]
    fn vector_has_horizontal_component() {
        let vector = Vector::from_mag_and_angle(500.0, Angle::Deg(20.0));
        let horiz = vector.horizontal();
        assert_approx_eq(horiz.mag(), 469.8, 1e-1);
        assert_eq!(f64::from(horiz.angle().as_deg()), 90.0);
    }

    #[test]
    fn vector_addition_yields_new_vector() {
        assert!(false, "TODO");
    }
}
/// Type that represents an angle in either Degrees or Radians.
///
/// Conversion methods are provided.
/// ```rust
/// use statics::Angle;
/// use std::f64::consts::PI;
/// let deg_angle = Angle::Deg(180.0);
/// assert_eq!(f64::from(deg_angle.as_rad()), PI);
/// ```
#[derive(Debug)]
pub enum Angle {
    /// Radians
    Rad(f64),
    /// Degrees
    Deg(f64),
}

impl Angle {
    /// returns the angle as a radian angle
    pub fn as_rad(&self) -> Self {
        match self {
            Angle::Rad(r) => Angle::Rad(*r),
            Angle::Deg(d) => Angle::Rad(d.to_radians()),
        }
    }

    /// returns the angle as a degree angle
    pub fn as_deg(&self) -> Self {
        match self {
            Angle::Deg(d) => Angle::Deg(*d),
            Angle::Rad(r) => Angle::Deg(r.to_degrees()),
        }
    }
}

impl From<Angle> for f64 {
    fn from(t: Angle) -> Self {
        match t {
            Angle::Rad(r) => r,
            Angle::Deg(d) => d,
        }
    }
}

impl PartialEq<Angle> for Angle {
    fn eq(&self, rhs: &Self) -> bool {
        fn is_approx_eq(l: &f64, r: &f64) -> bool {
            (l - r).abs() < 1e-10
        }

        match self {
            Angle::Rad(l) => match rhs {
                Angle::Rad(r) => is_approx_eq(l, r),
                Angle::Deg(_) => is_approx_eq(l, &f64::from(rhs.as_rad())),
            },
            Angle::Deg(l) => match rhs {
                Angle::Deg(r) => is_approx_eq(l, r),
                Angle::Rad(_) => is_approx_eq(l, &f64::from(rhs.as_deg())),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn angle_1deg_is_pi_by_180() {
        let actual = Angle::Deg(1.0);
        assert_eq!(f64::from(actual.as_rad()), PI / 180.0);
    }

    #[test]
    fn angle_pi_is_180deg() {
        let actual = Angle::Rad(PI);
        assert_eq!(f64::from(actual.as_deg()), 180.0);
    }
}

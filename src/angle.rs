use std::f64::consts::PI;

#[derive(Debug)]
pub enum Angle {
    Rad(f64),
    Deg(f64)
}

impl Angle {
    pub fn as_rad(&self) -> Self {
        match self {
            Angle::Rad(r) => Angle::Rad(*r),
            Angle::Deg(d) => Angle::Rad(d * PI / 180.0)
        }
    }

    pub fn as_deg(&self) -> Self {
        match self {
            Angle::Deg(d) => Angle::Deg(*d),
            Angle::Rad(r) => Angle::Deg(r / PI * 180.0)
        }
    }
}

impl From<Angle> for f64 {
    fn from(t: Angle) -> f64 {
        match t {
            Angle::Rad(r) => r,
            Angle::Deg(d) => d
        }
    }
}

impl PartialEq<Angle> for Angle {
    fn eq(&self, rhs : &Angle) -> bool {
        match self {
            Angle::Rad(l) => match rhs {
                Angle::Rad(r) => l == r,
                Angle::Deg(_) => *l == f64::from(rhs.as_rad())
            },
            Angle::Deg(l) => match rhs {
                Angle::Deg(r) => l == r,
                Angle::Rad(_) => *l == f64::from(rhs.as_deg())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
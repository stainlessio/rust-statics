use std::ops::{Mul, Sub};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub enum Mass {
    Lb(f64),
    G(f64),
}

impl Mass {
    pub fn as_gram(&self) -> Mass {
        Mass::G(match self {
            Mass::Lb(l) => *l * 453.59237,
            Mass::G(g) => *g,
        })
    }

    pub fn as_lb(&self) -> Mass {
        Mass::Lb(match self {
            Mass::Lb(l) => *l,
            Mass::G(g) => *g * 0.002_204_62,
        })
    }
}

impl Mul<f64> for Mass {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        match self {
            Mass::Lb(lhs) => Mass::Lb(lhs * rhs),
            Mass::G(lhs) => Mass::G(lhs * rhs),
        }
    }
}

impl Sub for Mass {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self {
            Mass::Lb(lhs) => Mass::Lb(lhs - f64::from(rhs.as_lb())),
            Mass::G(lhs) => Mass::G(lhs - f64::from(rhs.as_gram())),
        }
    }
}

impl From<Mass> for f64 {
    fn from(m: Mass) -> f64 {
        match m {
            Mass::G(g) => g,
            Mass::Lb(l) => l,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use extra_asserts::*;

    #[test]
    fn one_lb_is_about_453_grams() {
        let mass = Mass::Lb(1.0);
        assert_approx_eq(f64::from(mass.as_gram()), 453.0, 1.0);
    }

    #[test]
    fn one_kg_is_about_2_2_pounds() {
        let mass = Mass::G(1000.0);
        assert_approx_eq(f64::from(mass.as_lb()), 2.2, 1e-1);
    }
}

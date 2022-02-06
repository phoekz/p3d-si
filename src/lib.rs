#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ops::*;

// Dimension of any quantity Q is written in the form of a dimensional product:
// > dim Q = LENGTH^a, MASS^b, TIME^c, ...
// where the exponents a,b,c are signed integers.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Quantity<const LENGTH: i64, const MASS: i64, const TIME: i64>(f64);

// Base units
pub type Dimensionless = Quantity<0, 0, 0>;
pub type Length = Quantity<1, 0, 0>;
pub type Mass = Quantity<0, 1, 0>;
pub type Time = Quantity<0, 0, 1>;

// Derived units
pub type Area = Quantity<2, 0, 0>;
pub type Volume = Quantity<3, 0, 0>;
pub type Velocity = Quantity<1, 0, -1>;
pub type Acceleration = Quantity<1, 0, -2>;
pub type Force = Quantity<1, 1, -2>;
pub type Frequency = Quantity<0, 0, -1>;
pub type Pressure = Quantity<-1, 1, -2>;
pub type Energy = Quantity<2, 1, -2>;
pub type Power = Quantity<2, 1, -3>;

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> Quantity<LENGTH, MASS, TIME> {
    pub fn format_units(self) -> String {
        let value = self.0;
        format!("{value:0.1} m^{LENGTH} kg^{MASS} s^{TIME}")
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> From<f64>
    for Quantity<LENGTH, MASS, TIME>
{
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> Add for Quantity<LENGTH, MASS, TIME> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> Sub for Quantity<LENGTH, MASS, TIME> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> AddAssign
    for Quantity<LENGTH, MASS, TIME>
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> SubAssign
    for Quantity<LENGTH, MASS, TIME>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<const LENGTH: i64, const MASS: i64, const TIME: i64> Neg for Quantity<LENGTH, MASS, TIME> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<
        const LHS_LENGTH: i64,
        const LHS_MASS: i64,
        const LHS_TIME: i64,
        const RHS_LENGTH: i64,
        const RHS_MASS: i64,
        const RHS_TIME: i64,
    > Mul<Quantity<RHS_LENGTH, RHS_MASS, RHS_TIME>> for Quantity<LHS_LENGTH, LHS_MASS, LHS_TIME>
where
    Quantity<{ LHS_LENGTH + RHS_LENGTH }, { LHS_MASS + RHS_MASS }, { LHS_TIME + RHS_TIME }>: Sized,
{
    type Output =
        Quantity<{ LHS_LENGTH + RHS_LENGTH }, { LHS_MASS + RHS_MASS }, { LHS_TIME + RHS_TIME }>;

    fn mul(self, rhs: Quantity<RHS_LENGTH, RHS_MASS, RHS_TIME>) -> Self::Output {
        Quantity(self.0 * rhs.0)
    }
}

impl<
        const LHS_LENGTH: i64,
        const LHS_MASS: i64,
        const LHS_TIME: i64,
        const RHS_LENGTH: i64,
        const RHS_MASS: i64,
        const RHS_TIME: i64,
    > Div<Quantity<RHS_LENGTH, RHS_MASS, RHS_TIME>> for Quantity<LHS_LENGTH, LHS_MASS, LHS_TIME>
where
    Quantity<{ LHS_LENGTH - RHS_LENGTH }, { LHS_MASS - RHS_MASS }, { LHS_TIME - RHS_TIME }>: Sized,
{
    type Output =
        Quantity<{ LHS_LENGTH - RHS_LENGTH }, { LHS_MASS - RHS_MASS }, { LHS_TIME - RHS_TIME }>;

    fn div(self, rhs: Quantity<RHS_LENGTH, RHS_MASS, RHS_TIME>) -> Self::Output {
        Quantity(self.0 / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<f64>(), std::mem::size_of::<Length>());
    }

    #[test]
    fn arithmetic() {
        let dimensionless = Dimensionless::from(1.0);
        assert_eq!(dimensionless + dimensionless, Dimensionless::from(2.0));
        assert_eq!(dimensionless - dimensionless, Dimensionless::from(0.0));

        let mut dimensionless = Dimensionless::from(0.0);
        dimensionless += Dimensionless::from(1.0);
        assert_eq!(dimensionless, Dimensionless::from(1.0));

        let mut dimensionless = Dimensionless::from(0.0);
        dimensionless -= Dimensionless::from(1.0);
        assert_eq!(dimensionless, Dimensionless::from(-1.0));

        let dimensionless = Dimensionless::from(1.0);
        assert_eq!(-dimensionless, Dimensionless::from(-1.0));
    }

    #[test]
    fn formatting() {
        let length = Length::from(1.0);
        let format = length.format_units();
        assert_eq!("1.0 m^1 kg^0 s^0", format);
    }

    #[test]
    fn length() {
        let length = Length::from(1.0);
        let dimensionless = length / length;
        assert_eq!(dimensionless, Dimensionless::from(1.0));
    }

    #[test]
    fn area() {
        let length = Length::from(1.0);
        let area = length * length;
        assert_eq!(area, Area::from(1.0));
    }

    #[test]
    fn volume() {
        let length = Length::from(1.0);
        let volume = length * length * length;
        assert_eq!(volume, Volume::from(1.0));
    }

    #[test]
    fn velocity() {
        let length = Length::from(1.0);
        let time = Time::from(1.0);
        let velocity = length / time;
        assert_eq!(velocity, Velocity::from(1.0));
    }

    #[test]
    fn acceleration() {
        let length = Length::from(1.0);
        let time = Time::from(1.0);
        let acceleration = length / (time * time);
        assert_eq!(acceleration, Acceleration::from(1.0));
    }

    #[test]
    fn force() {
        let length = Length::from(1.0);
        let mass = Mass::from(1.0);
        let time = Time::from(1.0);
        let acceleration = length / (time * time);
        let force = mass * acceleration;
        assert_eq!(force, Force::from(1.0));
    }

    #[test]
    fn frequency() {
        let dimensionless = Dimensionless::from(1.0);
        let time = Time::from(1.0);
        let frequency = dimensionless / time;
        assert_eq!(frequency, Frequency::from(1.0));
    }

    #[test]
    fn pressure() {
        let dimensionless = Dimensionless::from(1.0);
        let length = Length::from(1.0);
        let mass = Mass::from(1.0);
        let time = Time::from(1.0);
        let pressure = (dimensionless / length) * mass / (time * time);
        assert_eq!(pressure, Pressure::from(1.0));
    }

    #[test]
    fn energy() {
        let length = Length::from(1.0);
        let mass = Mass::from(1.0);
        let time = Time::from(1.0);
        let energy = (length * length) * mass / (time * time);
        assert_eq!(energy, Energy::from(1.0));
    }

    #[test]
    fn power() {
        let length = Length::from(1.0);
        let mass = Mass::from(1.0);
        let time = Time::from(1.0);
        let energy = (length * length) * mass / (time * time);
        let power = energy / time;
        assert_eq!(power, Power::from(1.0));
    }
}

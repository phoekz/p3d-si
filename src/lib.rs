#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use std::ops::*;

// Dimension of any quantity Q is written in the form of a dimensional product:
//   dim Q = length^a, mass^b, time^c, ...
// where the exponents a,b,c are signed integers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {
    length: i64,
    mass: i64,
    time: i64,
}

impl Unit {
    pub const fn add(self, rhs: Self) -> Self {
        Self {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
        }
    }

    pub const fn sub(self, rhs: Self) -> Self {
        Self {
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            time: self.time - rhs.time,
        }
    }

    pub const fn neg(self) -> Self {
        Self {
            length: -self.length,
            mass: -self.mass,
            time: -self.time,
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Quantity<const UNIT: Unit>(f64);

macro_rules! quantity {
    ($name: ident, $length:literal, $mass: literal, $time: literal) => {
        pub type $name = Quantity<
            {
                Unit {
                    length: $length,
                    mass: $mass,
                    time: $time,
                }
            },
        >;
    };
}

// Base units
quantity!(Dimensionless, 0, 0, 0);
quantity!(Length, 1, 0, 0);
quantity!(Mass, 0, 1, 0);
quantity!(Time, 0, 0, 1);

// Derived units
quantity!(Area, 2, 0, 0);
quantity!(Volume, 3, 0, 0);
quantity!(Velocity, 1, 0, -1);
quantity!(Acceleration, 1, 0, -2);
quantity!(Force, 1, 1, -2);
quantity!(Frequency, 0, 0, -1);
quantity!(Pressure, -1, 1, -2);
quantity!(Energy, 2, 1, -2);
quantity!(Power, 2, 1, -3);

impl<const UNIT: Unit> Quantity<UNIT> {
    pub fn format_units(self) -> String {
        let value = self.0;
        let length = UNIT.length;
        let mass = UNIT.mass;
        let time = UNIT.time;
        format!("{value:0.1} m^{length} kg^{mass} s^{time}")
    }
}

impl<const UNIT: Unit> From<f64> for Quantity<UNIT> {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl<const UNIT: Unit> Add for Quantity<UNIT> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<const UNIT: Unit> Sub for Quantity<UNIT> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<const UNIT: Unit> AddAssign for Quantity<UNIT> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl<const UNIT: Unit> SubAssign for Quantity<UNIT> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl<const UNIT: Unit> Neg for Quantity<UNIT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<const LHS_UNIT: Unit, const RHS_UNIT: Unit> Mul<Quantity<RHS_UNIT>> for Quantity<LHS_UNIT>
where
    Quantity<{ LHS_UNIT.add(RHS_UNIT) }>: Sized,
{
    type Output = Quantity<{ LHS_UNIT.add(RHS_UNIT) }>;

    fn mul(self, rhs: Quantity<RHS_UNIT>) -> Self::Output {
        Quantity(self.0 * rhs.0)
    }
}

impl<const LHS_UNIT: Unit, const RHS_UNIT: Unit> Div<Quantity<RHS_UNIT>> for Quantity<LHS_UNIT>
where
    Quantity<{ LHS_UNIT.sub(RHS_UNIT) }>: Sized,
{
    type Output = Quantity<{ LHS_UNIT.sub(RHS_UNIT) }>;

    fn div(self, rhs: Quantity<RHS_UNIT>) -> Self::Output {
        Quantity(self.0 / rhs.0)
    }
}

impl<const UNIT: Unit> Div<Quantity<UNIT>> for f64
where
    Quantity<{ UNIT.neg() }>: Sized,
{
    type Output = Quantity<{ UNIT.neg() }>;

    fn div(self, rhs: Quantity<UNIT>) -> Self::Output {
        Quantity(self / rhs.0)
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
        let time = Time::from(1.0);
        let frequency = 1.0 / time;
        assert_eq!(frequency, Frequency::from(1.0));
    }

    #[test]
    fn pressure() {
        let length = Length::from(1.0);
        let mass = Mass::from(1.0);
        let time = Time::from(1.0);
        let pressure = (1.0 / length) * mass / (time * time);
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

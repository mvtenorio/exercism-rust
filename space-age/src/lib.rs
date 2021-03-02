use planet_derive::Planet;

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

const EARTH_ORBITAL_PERIOD: f64 = 31_557_600.0;

macro_rules! def_planet {
    ($x: ident, $v: literal) => {
        #[derive(Planet)]
        pub struct $x;
        impl $x {
            const ORBITAL_PERIOD: f64 = EARTH_ORBITAL_PERIOD * ($v);
        }
    };
}

def_planet!(Earth, 1.0);
def_planet!(Mercury, 0.2408467);
def_planet!(Venus, 0.61519726);
def_planet!(Mars, 1.8808158);
def_planet!(Jupiter, 11.862615);
def_planet!(Saturn, 29.447498);
def_planet!(Uranus, 84.016846);
def_planet!(Neptune, 164.79132);

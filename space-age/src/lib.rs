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

#[derive(Planet)]
pub struct Earth;
impl Earth {
    const ORBITAL_PERIOD: f64 = 31_557_600.0;
}

#[derive(Planet)]
pub struct Mercury;
impl Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Venus;
impl Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Mars;
impl Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Jupiter;
impl Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Saturn;
impl Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Uranus;
impl Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846 * Earth::ORBITAL_PERIOD;
}

#[derive(Planet)]
pub struct Neptune;
impl Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132 * Earth::ORBITAL_PERIOD;
}

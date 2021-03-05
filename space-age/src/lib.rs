// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    const EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Earth::YEAR_IN_SEC / Self::EARTH_YEARS
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Earth {
    pub const YEAR_IN_SEC: f64 = 60.0*60.0*24.0*365.25;
}


impl Planet for Mercury { const EARTH_YEARS: f64 = 0.2408467; }
impl Planet for Venus   { const EARTH_YEARS: f64 = 0.61519726; }
impl Planet for Earth   { const EARTH_YEARS: f64 = 1.0; }
impl Planet for Mars    { const EARTH_YEARS: f64 = 1.8808158; }
impl Planet for Jupiter { const EARTH_YEARS: f64 = 11.862615; }
impl Planet for Saturn  { const EARTH_YEARS: f64 = 29.447498; }
impl Planet for Uranus  { const EARTH_YEARS: f64 = 84.01684; }
impl Planet for Neptune { const EARTH_YEARS: f64 = 164.79132; }

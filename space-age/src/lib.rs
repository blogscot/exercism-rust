#[derive(Debug)]
pub struct Duration {
  earth_orbits: f64,
}

impl From<u64> for Duration {
  fn from(s: u64) -> Self {
    let earth_orbits = s as f64 / 31_557_600.0;
    Duration { earth_orbits }
  }
}

pub trait Planet {
  fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 0.240_846_7
  }
}
impl Planet for Venus {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 0.615_197_26
  }
}

impl Planet for Earth {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits
  }
}
impl Planet for Mars {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 1.880_815_8
  }
}
impl Planet for Jupiter {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 11.862_615
  }
}
impl Planet for Saturn {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 29.447_498
  }
}
impl Planet for Uranus {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 84.016_846
  }
}
impl Planet for Neptune {
  fn years_during(d: &Duration) -> f64 {
    d.earth_orbits / 164.791_32
  }
}

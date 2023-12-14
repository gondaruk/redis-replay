use derive_more::{Add, Sub};
use std::fmt::{Debug, Display, Formatter};
use std::time::{Duration, SystemTime};

#[derive(Add, Sub, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Micros(i64);

impl Micros {
    pub fn now() -> Self {
        let micros_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        Self(micros_now as i64)
    }

    pub fn from_seconds(secs: f64) -> Self {
        let micros = 1e6f64 * secs;
        Self(micros as i64)
    }

    pub fn since(m: Micros) -> Self {
        let now = Self::now();
        now - m
    }

    pub fn is_positive(&self) -> bool {
        return self.0 > 0;
    }

    pub fn is_zero(&self) -> bool {
        return self.0 == 0;
    }

    pub fn is_negative(&self) -> bool {
        return self.0 < 0;
    }

    pub fn duration_since(&self, m: Micros) -> Option<Duration> {
        let micros = self.0 - m.0;
        if micros.is_negative() {
            None
        } else {
            Some(Duration::from_micros(micros as u64))
        }
    }
}

impl Display for Micros {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}μs", self.0)
    }
}

impl Debug for Micros {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}μs", self.0)
    }
}

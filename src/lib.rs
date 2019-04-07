//! Provides a way of representing [unix time](https://en.wikipedia.org/wiki/Unix_time)
//! in terms of seconds with fractional subseconds
//!
//! # Features
//!
//! ## serde
//!
//! Adds ability to serialize and deserialize seconds with serde. This is
//! enabled by default. To turn if off add the following to your `Cargo.toml`
//! file
//!
//! ```toml
//! [dependencies.unisecs]
//!  version = "..."
//!  default-features = false
//! ```
#[cfg(feature = "serde")]
use serde::{de, ser, Serializer};

use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Represents fractional seconds since the epoch
/// These can be derived from std::time::Duration and be converted
/// too std::time::Duration
///
/// A `Default` implementation is provided which yields the number of seconds since the epoch from
/// the system time's `now` value
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Seconds(pub(crate) f64);

impl Seconds {
    /// return the current time in seconds since the unix epoch (1-1-1970 midnight)
    pub fn now() -> Self {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .into()
    }

    /// truncate epoc time to remove fractional seconds
    pub fn trunc(&self) -> u64 {
        self.0.trunc() as u64
    }
}

impl Default for Seconds {
    fn default() -> Self {
        Seconds::now()
    }
}

impl From<Duration> for Seconds {
    fn from(d: Duration) -> Self {
        Seconds(d.as_secs() as f64 + (f64::from(d.subsec_nanos()) / 1.0e9))
    }
}

impl Into<Duration> for Seconds {
    fn into(self) -> Duration {
        let Seconds(secs) = self;
        Duration::new(secs.trunc() as u64, (secs.fract() * 1.0e9) as u32)
    }
}

#[cfg(feature = "serde")]
struct SecondsVisitor;

#[cfg(feature = "serde")]
impl<'de> de::Visitor<'de> for SecondsVisitor {
    type Value = Seconds;

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        formatter.write_str("a string value")
    }
    fn visit_f64<E>(
        self,
        value: f64,
    ) -> Result<Seconds, E>
    where
        E: de::Error,
    {
        Ok(Seconds(value))
    }
}

#[cfg(feature = "serde")]
impl ser::Serialize for Seconds {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Seconds(seconds) = self;
        serializer.serialize_f64(*seconds)
    }
}

#[cfg(feature = "serde")]
impl<'de> de::Deserialize<'de> for Seconds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_f64(SecondsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::Seconds;
    use std::time::Duration;

    #[test]
    fn seconds_default() {
        let (now, default) = (Seconds::default(), Seconds::now());
        assert_eq!(now.trunc(), default.trunc());
    }

    #[test]
    fn seconds_duration_interop() {
        let secs = Seconds(1_545_136_342.711_932);
        let duration: Duration = secs.into();
        let plus_one = duration + Duration::from_secs(1);
        assert_eq!(Seconds::from(plus_one), Seconds(1_545_136_343.711_932));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn seconds_serialize() {
        assert_eq!(
            serde_json::to_string(&Seconds(1_545_136_342.711_932)).expect("failed to serialize"),
            "1545136342.711932"
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn seconds_deserialize() {
        assert_eq!(
            serde_json::from_slice::<Seconds>(b"1545136342.711932").expect("failed to serialize"),
            Seconds(1_545_136_342.711_932)
        );
    }
}

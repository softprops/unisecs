//! Provides a way of representing [unix time](https://en.wikipedia.org/wiki/Unix_time)
//! in terms of seconds with fractional subseconds.
//!
//! # Examples
//!
//! The following is roughly equivalent with `date -v+1S +%s`
//!
//! ```rust
//! use std::time::Duration;
//! use unisecs::Seconds;
//!
//! fn main() {
//!   println!(
//!     "{}",
//!     Seconds::now() + Duration::from_secs(5)
//!   );
//! }
//! ```
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
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Represents fractional seconds since the [unix epoch](https://en.wikipedia.org/wiki/Unix_time)
/// These can be derived from [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) and be converted
/// into [`std::time::Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html)
///
/// A `Default` implementation is provided which yields the number of seconds since the epoch from
/// the system time's `now` value
///
/// You can also and and subtract durations from Seconds.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Seconds(f64);

impl fmt::Display for Seconds {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Seconds {
    /// return the current time in seconds since the unix epoch (1-1-1970 midnight)
    pub fn now() -> Self {
        Self::from_duration(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default(),
        )
    }

    /// truncate epoc time to remove fractional seconds
    pub fn trunc(self) -> Self {
        Self(self.0.trunc())
    }

    /// transformation is kept private as we can make no guarantees
    /// about whether a provided duration is anchored in any way to
    /// unix time
    fn from_duration(dur: Duration) -> Self {
        Seconds(dur.as_secs() as f64 + (f64::from(dur.subsec_nanos()) / 1.0e9))
    }
}

impl Default for Seconds {
    fn default() -> Self {
        Seconds::now()
    }
}

/// Similar to `date -v+1S +%s`
impl Add<Duration> for Seconds {
    type Output = Seconds;
    fn add(
        self,
        rhs: Duration,
    ) -> Self::Output {
        let lhs: Duration = self.into();
        Seconds::from_duration(lhs + rhs)
    }
}

/// Similar to `date -v-1S +%s`
impl Sub<Duration> for Seconds {
    type Output = Seconds;
    fn sub(
        self,
        rhs: Duration,
    ) -> Self::Output {
        let lhs: Duration = self.into();
        Seconds::from_duration(lhs - rhs)
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
        formatter.write_str("floating point seconds")
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
    fn seconds_display() {
        let secs = Seconds(1_545_136_342.711_932);
        assert_eq!(format!("{}", secs), "1545136342.711932");
    }

    #[test]
    fn seconds_duration_interop() {
        let secs = Seconds(1_545_136_342.711_932);
        let duration: Duration = secs.into();
        assert_eq!(duration.as_secs(), 1_545_136_342);
    }

    #[test]
    fn seconds_add_duration() {
        let secs = Seconds(1_545_136_342.711_932);
        assert_eq!(
            secs + Duration::from_secs(1),
            Seconds(1_545_136_343.711_932)
        );
    }

    #[test]
    fn seconds_sub_duration() {
        let secs = Seconds(1_545_136_342.711_932);
        assert_eq!(
            secs - Duration::from_secs(1),
            Seconds(1_545_136_341.711_932)
        );
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
    fn seconds_deserialize_floats() {
        assert_eq!(
            serde_json::from_slice::<Seconds>(b"1545136342.711932").expect("failed to serialize"),
            Seconds(1_545_136_342.711_932)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn seconds_fails_to_deserialize() {
        match serde_json::from_slice::<Seconds>(b"{\"foo\":\"bar\"}") {
            Err(err) => assert_eq!(
                format!("{}", err),
                "invalid type: map, expected floating point seconds at line 1 column 0"
            ),
            Ok(other) => panic!("unexpected result {}", other),
        }
    }
}

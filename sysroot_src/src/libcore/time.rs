#![stable(feature = "duration_core", since = "1.25.0")]


use crate::{fmt, u64};
use crate::iter::Sum;
use crate::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

const NANOS_PER_SEC: u32 = 1_000_000_000;
const NANOS_PER_MILLI: u32 = 1_000_000;
const NANOS_PER_MICRO: u32 = 1_000;
const MILLIS_PER_SEC: u64 = 1_000;
const MICROS_PER_SEC: u64 = 1_000_000;

#[stable(feature = "duration", since = "1.3.0")]
// #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Duration {
    secs: u64,
    nanos: u32, // Always 0 <= nanos < NANOS_PER_SEC
}

impl Duration {
    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const SECOND: Duration = Duration::from_secs(1);

    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const MILLISECOND: Duration = Duration::from_millis(1);

    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const MICROSECOND: Duration = Duration::from_micros(1);

    #[unstable(feature = "duration_constants", issue = "57391")]
    pub const NANOSECOND: Duration = Duration::from_nanos(1);

    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    pub fn new(secs: u64, nanos: u32) -> Duration { loop { } }

    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn from_secs(secs: u64) -> Duration {
        Duration { secs, nanos: 0 }
    }

    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn from_millis(millis: u64) -> Duration {
        Duration {
            secs: millis / MILLIS_PER_SEC,
            nanos: ((millis % MILLIS_PER_SEC) as u32) * NANOS_PER_MILLI,
        }
    }

    #[stable(feature = "duration_from_micros", since = "1.27.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn from_micros(micros: u64) -> Duration {
        Duration {
            secs: micros / MICROS_PER_SEC,
            nanos: ((micros % MICROS_PER_SEC) as u32) * NANOS_PER_MICRO,
        }
    }

    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn from_nanos(nanos: u64) -> Duration {
        Duration {
            secs: nanos / (NANOS_PER_SEC as u64),
            nanos: (nanos % (NANOS_PER_SEC as u64)) as u32,
        }
    }

    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    pub const fn as_secs(&self) -> u64 { self.secs }

    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[inline]
    pub const fn subsec_millis(&self) -> u32 { self.nanos / NANOS_PER_MILLI }

    #[stable(feature = "duration_extras", since = "1.27.0")]
    #[inline]
    pub const fn subsec_micros(&self) -> u32 { self.nanos / NANOS_PER_MICRO }

    #[stable(feature = "duration", since = "1.3.0")]
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 { self.nanos }

    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.secs as u128 * MILLIS_PER_SEC as u128 + (self.nanos / NANOS_PER_MILLI) as u128
    }

    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.secs as u128 * MICROS_PER_SEC as u128 + (self.nanos / NANOS_PER_MICRO) as u128
    }

    #[stable(feature = "duration_as_u128", since = "1.33.0")]
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.secs as u128 * NANOS_PER_SEC as u128 + self.nanos as u128
    }

    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[inline]
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> { loop { } }

    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[inline]
    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> { loop { } }

    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Duration> { loop { } }

    #[stable(feature = "duration_checked_ops", since = "1.16.0")]
    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Duration> { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn as_secs_f64(&self) -> f64 { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn as_secs_f32(&self) -> f32 { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn from_secs_f64(secs: f64) -> Duration { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn from_secs_f32(secs: f32) -> Duration { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn mul_f64(self, rhs: f64) -> Duration { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn mul_f32(self, rhs: f32) -> Duration { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn div_f64(self, rhs: f64) -> Duration { loop { } }

    #[stable(feature = "duration_float", since = "1.38.0")]
    #[inline]
    pub fn div_f32(self, rhs: f32) -> Duration { loop { } }

    #[unstable(feature = "div_duration", issue = "63139")]
    #[inline]
    pub fn div_duration_f64(self, rhs: Duration) -> f64 { loop { } }

    #[unstable(feature = "div_duration", issue = "63139")]
    #[inline]
    pub fn div_duration_f32(self, rhs: Duration) -> f32 { loop { } }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Duration) -> Duration { loop { } }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Duration) { loop { } }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Duration) -> Duration { loop { } }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Duration) { loop { } }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, rhs: u32) -> Duration { loop { } }
}

#[stable(feature = "symmetric_u32_duration_mul", since = "1.31.0")]
impl Mul<Duration> for u32 {
    type Output = Duration;

    fn mul(self, rhs: Duration) -> Duration { loop { } }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl MulAssign<u32> for Duration {
    fn mul_assign(&mut self, rhs: u32) { loop { } }
}

#[stable(feature = "duration", since = "1.3.0")]
impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, rhs: u32) -> Duration { loop { } }
}

#[stable(feature = "time_augmented_assignment", since = "1.9.0")]
impl DivAssign<u32> for Duration {
    fn div_assign(&mut self, rhs: u32) { loop { } }
}

macro_rules! sum_durations {
    ($iter:expr) => {{
        let mut total_secs: u64 = 0;
        let mut total_nanos: u64 = 0;

        for entry in $iter {
            total_secs = total_secs
                .checked_add(entry.secs)
                .expect("overflow in iter::sum over durations");
            total_nanos = match total_nanos.checked_add(entry.nanos as u64) {
                Some(n) => n,
                None => {
                    total_secs = total_secs
                        .checked_add(total_nanos / NANOS_PER_SEC as u64)
                        .expect("overflow in iter::sum over durations");
                    (total_nanos % NANOS_PER_SEC as u64) + entry.nanos as u64
                }
            };
        }
        total_secs = total_secs
            .checked_add(total_nanos / NANOS_PER_SEC as u64)
            .expect("overflow in iter::sum over durations");
        total_nanos = total_nanos % NANOS_PER_SEC as u64;
        Duration {
            secs: total_secs,
            nanos: total_nanos as u32,
        }
    }};
}

#[stable(feature = "duration_sum", since = "1.16.0")]
impl Sum for Duration {
    fn sum<I: Iterator<Item=Duration>>(iter: I) -> Duration { loop { } }
}

#[stable(feature = "duration_sum", since = "1.16.0")]
impl<'a> Sum<&'a Duration> for Duration {
    fn sum<I: Iterator<Item=&'a Duration>>(iter: I) -> Duration { loop { } }
}

#[stable(feature = "duration_debug_impl", since = "1.27.0")]
impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { loop { } }
}

use std::str::FromStr;

/// This module contains functions related to time manipulation.
/// It provides conversions between different time representations and utilities for calculating time intervals.
/// The functions in this module are used in the Beacon Standard Library.
use hifitime::Duration;

/// Converts Julian Ephemeris Date (JDE) to Unix timestamp.
///
/// # Arguments
///
/// * `jde` - The Julian Ephemeris Date to convert.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given JDE.
#[inline]
pub fn jde_to_unix(jde: f64) -> anyhow::Result<f64> {
    let epoch = hifitime::Epoch::from_jde_utc(jde);
    Ok(epoch.to_unix_seconds())
}

/// Converts seconds since a given epoch to Unix timestamp.
///
/// # Arguments
///
/// * `seconds` - The number of seconds since the epoch.
/// * `epoch` - The epoch to use as a reference.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given number of seconds since the epoch.
#[inline]
pub fn seconds_since_epoch_to_unix(seconds: f64, epoch: &str) -> anyhow::Result<f64> {
    let epoch = hifitime::Epoch::from_str(epoch)?;
    Ok((epoch + Duration::from_seconds(seconds)).to_unix_seconds())
}

/// Converts days since a given epoch to Unix timestamp.
///
/// # Arguments
///
/// * `days` - The number of days since the epoch.
/// * `epoch` - The epoch to use for the conversion.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given number of days since the epoch.
#[inline]
pub fn days_since_epoch_to_unix(days: f64, epoch: &str) -> anyhow::Result<f64> {
    let epoch = hifitime::Epoch::from_str(epoch)?;
    Ok((epoch + Duration::from_days(days)).to_unix_seconds())
}

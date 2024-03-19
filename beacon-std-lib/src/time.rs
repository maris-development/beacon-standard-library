/// This module contains functions related to time manipulation.
/// It provides conversions between different time representations and utilities for calculating time intervals.
/// The functions in this module are used in the Beacon Standard Library.
use hifitime::Duration;
use lazy_static::lazy_static;
use std::{collections::HashMap, ops::Deref, str::FromStr};

use crate::args::Args;

/// A method that takes a single argument of type `T` and an `Args` struct and returns a result of type `anyhow::Result<T>`.
#[derive(Clone, Copy)]
pub struct ArgsMethod<T>(pub fn(T, Args) -> anyhow::Result<T>);

impl Deref for ArgsMethod<f64> {
    type Target = fn(f64, Args) -> anyhow::Result<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Converts Julian Ephemeris Date (JDE) to Unix timestamp.
///
/// # Arguments
///
/// * `jde` - The Julian Ephemeris Date to convert.
/// * `_` - An `Args` struct that is not used in this function.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given JDE.
#[inline]
pub fn jde_to_unix(jde: f64, _: Args) -> anyhow::Result<f64> {
    let epoch = hifitime::Epoch::from_jde_utc(jde);
    Ok(epoch.to_unix_seconds())
}

/// Converts seconds since a given epoch to Unix timestamp.
///
/// # Arguments
///
/// * `value` - The number of seconds since the epoch.
/// * `args` - An `Args` struct that contains the epoch argument.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given number of seconds since the epoch.
pub fn seconds_since_epoch_to_unix(value: f64, args: Args) -> anyhow::Result<f64> {
    let epoch_arg = args.arg("epoch")?;
    let epoch = hifitime::Epoch::from_str(epoch_arg.as_str()?)?;
    Ok((epoch + Duration::from_seconds(value)).to_unix_seconds())
}

/// Converts days since a given epoch to Unix timestamp.
///
/// # Arguments
///
/// * `value` - The number of days since the epoch.
/// * `args` - An `Args` struct that contains the epoch argument.
///
/// # Returns
///
/// The Unix timestamp corresponding to the given number of days since the epoch.
pub fn days_since_epoch_to_unix(value: f64, args: Args) -> anyhow::Result<f64> {
    let epoch_arg = args.arg("epoch")?;
    let epoch = hifitime::Epoch::from_str(epoch_arg.as_str()?)?;
    Ok((epoch + Duration::from_days(value)).to_unix_seconds())
}

macro_rules! register_args_function {
    ($($name:ident => $func:expr),*) => {
    lazy_static! {
            pub static ref ARGS_FUNCTIONS:  HashMap<&'static str, ArgsMethod<f64>> = {
                let mut m = HashMap::new();

                $(
                    m.insert(concat!(stringify!($name)), ArgsMethod ($func as fn(f64, Args) -> anyhow::Result<f64>));
                )*

                m
            };
        }
    };
}

register_args_function!(
    jde_to_unix => jde_to_unix,
    seconds_since_epoch_to_unix => seconds_since_epoch_to_unix,
    days_since_epoch_to_unix => days_since_epoch_to_unix
);

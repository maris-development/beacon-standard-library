use std::collections::HashMap;

use lazy_static::lazy_static;

macro_rules! register_std_lib_conversions {
    ($path:ident,$($name:ident => $func:expr , $func_inverse: expr),*) => {
    lazy_static! {
            pub static ref STD_CONVERSIONS:  HashMap<&'static str, Conversion<f64>> = {
                let mut m = HashMap::new();

                $(
                    m.insert(concat!("std::",stringify!($path),"::",stringify!($name)), Conversion { conversion: $func as fn(f64) -> f64 , inverse_conversion: $func_inverse as fn(f64) -> f64 });
                )*

                m
            };
        }
    };
}

pub struct Conversion<T> {
    conversion: fn(T) -> T,
    inverse_conversion: fn(T) -> T,
}

impl<T> Conversion<T> {
    pub fn conversion_fn(&self) -> fn(T) -> T {
        self.conversion
    }

    pub fn inverse_conversion_fn(&self) -> fn(T) -> T {
        self.inverse_conversion
    }
}

register_std_lib_conversions! {
    conversions, 
    
    kelvin_to_celsius => kelvin_to_celsius, celsius_to_kelvin, 
    
    celsius_to_kelvin => celsius_to_kelvin, kelvin_to_celsius,
    
    kelvin_to_fahrenheit => kelvin_to_fahrenheit, fahrenheit_to_kelvin,

    fahrenheit_to_kelvin => fahrenheit_to_kelvin, kelvin_to_fahrenheit, 
    
    celsius_to_fahrenheit => celsius_to_fahrenheit , fahrenheit_to_celsius,

    fahrenheit_to_celsius => fahrenheit_to_celsius, celsius_to_fahrenheit,

    empty_conversion => empty_conversion, empty_conversion
}

/// This function returns the input value `x` unchanged.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::empty_conversion`
///
/// # Arguments
///
/// * `x` - A floating point number.
///
/// # Example
///
/// ```
/// use beacon_std_lib::conversions::empty_conversion;
///
/// let x = 3.14;
/// assert_eq!(empty_conversion(x), x);
/// ```
pub fn empty_conversion(x: f64) -> f64 {
    x
}

/// Converts temperature in Celsius to Kelvin.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::celsius_to_kelvin`
///
/// # Arguments
///
/// * `x` - The temperature in Celsius.
///
/// # Example
///
/// ```
/// use beacon_std_lib::conversions::celsius_to_kelvin;
///
/// let celsius = 25.0;
/// let kelvin = celsius_to_kelvin(celsius);
/// assert_eq!(kelvin, 298.15);
/// ```
/// 
pub fn celsius_to_kelvin(x: f64) -> f64 {
    x + 273.15
}

/// Converts temperature in Celsius to Fahrenheit.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::celsius_to_fahrenheit`
///
/// # Arguments
///
/// * `x` - The temperature in Celsius to be converted to Fahrenheit.
///
/// # Example
///
/// ```
/// use beacon_std_lib::conversions::celsius_to_fahrenheit;
///
/// let celsius = 25.0;
/// let fahrenheit = celsius_to_fahrenheit(celsius);
/// assert_eq!(fahrenheit, 77.0);
/// ```
pub fn celsius_to_fahrenheit(x: f64) -> f64 {
    (x * 9.0 / 5.0) + 32.0
}

/// Converts temperature in Kelvin to Celsius.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::kelvin_to_celsius`
///
/// # Arguments
///
/// * `x` - The temperature in Kelvin.
///
/// # Example
///
/// ```
/// use beacon_std_lib::conversions::kelvin_to_celsius;
///
/// let kelvin = 275.15;
/// let celsius = kelvin_to_celsius(kelvin);
/// assert_eq!(celsius, 2.0);
/// ```
pub fn kelvin_to_celsius(x: f64) -> f64 {
    x - 273.15
}

/// Converts temperature in Kelvin to Fahrenheit.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::kelvin_to_fahrenheit`
///
/// # Arguments
///
/// * `x` - The temperature in Kelvin.
///
/// # Example
///
/// ```
/// use beacon_std_lib::conversions::kelvin_to_fahrenheit;
/// 
/// let fahrenheit = kelvin_to_fahrenheit(275.15);
/// assert_eq!(fahrenheit, 35.6);
/// ```
pub fn kelvin_to_fahrenheit(x: f64) -> f64 {
    (x - 273.15) * 9.0 / 5.0 + 32.0
}

/// Converts a temperature value from Fahrenheit to Kelvin.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::fahrenheit_to_kelvin`
///
/// # Arguments
///
/// * `x` - A temperature value in Fahrenheit.
///
/// # Examples
///
/// ```
/// use beacon_std_lib::conversions::fahrenheit_to_kelvin;
/// 
/// let kelvin = fahrenheit_to_kelvin(32.0);
/// assert_eq!(kelvin, 273.15);
/// ```
pub fn fahrenheit_to_kelvin(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0 + 273.15
}

/// Converts a temperature value from Fahrenheit to Celsius.
/// 
/// # Method name in Beacon Query:
/// 
/// ### `std::conversions::fahrenheit_to_celsius`
///
/// # Arguments
///
/// * `x` - A temperature value in Fahrenheit.
///
/// # Examples
///
/// ```
/// use beacon_std_lib::conversions::fahrenheit_to_celsius;
///
/// let fahrenheit = 32.0;
/// let celsius = fahrenheit_to_celsius(fahrenheit);
///
/// assert_eq!(celsius, 0.0);
/// ```
pub fn fahrenheit_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib_contains() {
        assert!(STD_CONVERSIONS.contains_key("std::conversions::celsius_to_kelvin"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::kelvin_to_celsius"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::celsius_to_fahrenheit"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::fahrenheit_to_celsius"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::kelvin_to_fahrenheit"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::fahrenheit_to_kelvin"));
        assert!(STD_CONVERSIONS.contains_key("std::conversions::empty_conversion"));
    }
}
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

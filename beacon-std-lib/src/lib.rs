pub mod conversions;

use conversions::Conversion;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// A HashMap of all the standard conversions. Usable via the `std::conversions::` prefix.
    /// # Example
    /// ```
    /// use beacon_std::STD_CONVERSIONS;
    /// let fun = STD_CONVERSIONS.get("std::conversions::celsius_to_kelvin").unwrap();
    /// assert_eq!(fun(0.0), 273.15);
    /// ```
    pub static ref STD_CONVERSIONS : &'static HashMap<&'static str, Conversion<f64>> = &crate::conversions::STD_CONVERSIONS;
}
pub mod args;
pub mod conversions;
pub mod time;

use conversions::Conversion;
use lazy_static::lazy_static;
use std::{collections::HashMap, hash::Hash};
use time::{ArgsMethod, ARGS_FUNCTIONS};

pub struct Library {
    modules: HashMap<&'static str, Module>,
}

impl Library {
    pub fn function(&self, name: &str) -> anyhow::Result<ArgsMethod<f64>> {
        let (module_name, function_name) = name.split_once("::").ok_or(anyhow::anyhow!(
            "Function name '{}' does not contain a module name",
            name
        ))?;
        let module = self.modules.get(module_name).ok_or_else(|| {
            anyhow::anyhow!("Module '{}' not found in the standard library", module_name)
        })?;
        module.functions.get(function_name).copied().ok_or_else(|| {
            anyhow::anyhow!(
                "Function '{}' not found in module '{}'",
                function_name,
                module_name
            )
        })
    }
}

pub struct Module {
    functions: &'static ARGS_FUNCTIONS,
}

lazy_static! {
    /// A HashMap of all the standard conversions. Usable via the `std::conversions::` prefix.
    /// # Example
    /// ```
    /// use beacon_std::STD_CONVERSIONS;
    /// let fun = STD_CONVERSIONS.get("std::conversions::celsius_to_kelvin").unwrap();
    /// assert_eq!(fun(0.0), 273.15);
    /// ```
    pub static ref STD_CONVERSIONS : &'static HashMap<&'static str, Conversion<f64>> = &crate::conversions::STD_CONVERSIONS;



    /// The standard library. Contains all the standard modules and their functions.
    /// # Example
    /// ```
    /// use beacon_std_lib::STD;
    /// use beacon_std_lib::args::Args;
    /// let jde = 2460388.929757;
    /// let fun = STD.function("time::jde_to_unix").unwrap();
    /// assert!(fun(jde, Args::none()).is_ok());
    /// ```
    pub static ref STD : Library = Library {
        modules: {
            let mut modules = HashMap::new();
            modules.insert("time", Module { functions: &time::ARGS_FUNCTIONS});
            modules
        },
    };
}

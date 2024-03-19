use std::{collections::HashMap, sync::Arc};

pub struct Args<'f>(&'f Option<HashMap<String, ArgValue>>);

impl<'f> Args<'f> {
    pub fn arg(&self, name: &'f str) -> anyhow::Result<ArgContext<'f>> {
        match self.0 {
            Some(map) => match map.get(name) {
                Some(arg) => Ok(ArgContext {
                    name: name,
                    arg: arg,
                }),
                None => Err(anyhow::anyhow!("Argument {} not found", name.to_string())),
            },
            None => Err(anyhow::anyhow!("Argument {} not found", name.to_string())),
        }
    }

    pub fn new(args: &Option<HashMap<String, ArgValue>>) -> Args {
        Args(args)
    }

    pub fn none() -> Args<'static> {
        Args(&None)
    }
}

pub struct ArgContext<'a> {
    name: &'a str,
    arg: &'a ArgValue,
}

impl ArgContext<'_> {
    pub fn as_str(&self) -> anyhow::Result<&str> {
        match &self.arg {
            ArgValue::String(s) => Ok(s),
            _ => Err(anyhow::anyhow!(
                "Expected {} to be of type: string",
                self.name
            )),
        }
    }

    pub fn as_float(&self) -> anyhow::Result<f64> {
        match &self.arg {
            ArgValue::Float(f) => Ok(*f),
            _ => Err(anyhow::anyhow!(
                "Expected {} to be of type: float",
                self.name
            )),
        }
    }

    pub fn as_bool(&self) -> anyhow::Result<bool> {
        match &self.arg {
            ArgValue::Bool(b) => Ok(*b),
            _ => Err(anyhow::anyhow!(
                "Expected {} to be of type: bool",
                self.name
            )),
        }
    }

    pub fn as_vec(&self) -> anyhow::Result<&Vec<ArgValue>> {
        match &self.arg {
            ArgValue::Vec(v) => Ok(v),
            _ => Err(anyhow::anyhow!(
                "Expected {} to be of type: [...]",
                self.name
            )),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ArgValue {
    Float(f64),
    String(Arc<str>),
    Bool(bool),
    Vec(Vec<ArgValue>),
}

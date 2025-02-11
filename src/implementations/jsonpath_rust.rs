use std::{
    fmt::Display,
    fs,
    io::{self},
    str::FromStr,
};

use jsonpath_rust::{JsonPathValue, parser::JsonPath};
use serde_json::Value;
use thiserror::Error;

use crate::framework::implementation::Implementation;

pub struct JsonpathRust {}

pub struct JsonpathRustResult<'a>(Vec<JsonPathValue<'a, Value>>);

impl Implementation for JsonpathRust {
    type Query = JsonPath;

    type File = String;

    type Error = JsonpathRustError;

    type Result<'a> = usize;

    fn id() -> &'static str {
        "jsonpath_rust"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonpathRust {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        Ok(file)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        JsonPath::from_str(query).map_err(JsonpathRustError::JsonPathInstError)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        let value: Value = serde_json::from_str(&file)?;
        let results = query.find_slice(&value);

        Ok(results.len())
    }
}

impl<'a> Display for JsonpathRustResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for res in &self.0 {
            let val = res.clone().to_data();
            writeln!(f, "{}", val)?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum JsonpathRustError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error("error parsing JSONPath query: '{0}'")]
    JsonPathInstError(<JsonPath as TryFrom<&'static str>>::Error),
}

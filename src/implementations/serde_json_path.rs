use std::{
    fs,
    io::self,
};

use serde_json::Value;
use serde_json_path::{JsonPath, ParseError};
use thiserror::Error;

use crate::framework::implementation::Implementation;

pub struct SerdeJsonPath {}

impl Implementation for SerdeJsonPath {
    type Query = JsonPath;

    type File = String;

    type Error = SerdeJsonPathError;

    type Result<'a> = usize;

    fn id() -> &'static str {
        "serde_json_path"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(SerdeJsonPath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        Ok(file)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        JsonPath::parse(query).map_err(SerdeJsonPathError::JsonPathParseError)
    }

    fn run<'a>(&self, query: &Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        let value: Value = serde_json::from_str(&file)?;
        let result = query.query(&value);
        Ok(result.len())
    }
}

#[derive(Error, Debug)]
pub enum SerdeJsonPathError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonPathParseError(#[from] ParseError),
}

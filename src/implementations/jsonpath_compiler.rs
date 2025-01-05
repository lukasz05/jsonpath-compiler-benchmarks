use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::string::FromUtf8Error;
use thiserror::Error;
use crate::bindings::{ast_deepest, ast_nested_inner, bestbuy_all_nodes, canada_coord_476_1446_1,
                      canada_second_coord_component, citm_seat_category, google_map_routes,
                      inner_array, user_second_mention_index};
use crate::framework::implementation::Implementation;

use memmap2::Mmap;

type QueryFunction = fn(&[u8]) -> String;

struct JsonPathCompilerCore<'a> {
    query_functions: HashMap<&'a str, QueryFunction>
}

pub struct JsonPathCompilerResult(String);

impl JsonPathCompilerCore<'_> {
    fn new() -> Result<Self, JsonPathCompilerError> {
        Ok(JsonPathCompilerCore {
            query_functions: HashMap::from([
                ("$.features[*].geometry.coordinates[*][*][1]", canada_second_coord_component as QueryFunction),
                ("$..coordinates[476][1446][1]", canada_coord_476_1446_1 as QueryFunction),
                ("$..seatCategoryId", citm_seat_category as QueryFunction),
                ("$..inner..inner..type.qualType", ast_nested_inner as QueryFunction),
                ("$..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*..*", ast_deepest as QueryFunction),
                ("$..*", bestbuy_all_nodes as QueryFunction),
                ("$[*].routes[*].legs[*].steps[*].distance.text", google_map_routes as QueryFunction),
                ("$..inner[0]", inner_array as QueryFunction),
                ("$..entities.user_mentions[1]", user_second_mention_index as QueryFunction)
            ])
        })
    }

    fn compile_query(&self, query: &str) -> Result<QueryFunction, JsonPathCompilerError> {
        match self.query_functions.get(query) {
            Some(query_function) => {
                Ok(*query_function)
            }
            None => {
                Err(JsonPathCompilerError::UnrecognizedQuery(query.to_string()))
            }
        }
    }

    fn run<'a>(&self, query_function: &QueryFunction, file: &'a [u8]) -> Result<JsonPathCompilerResult, JsonPathCompilerError> {
        let res = query_function(file);
        Ok(JsonPathCompilerResult(res))
    }
}

impl Display for JsonPathCompilerResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum JsonPathCompilerError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Unrecognized query: '{0}'")]
    UnrecognizedQuery(String)
}

pub struct JsonPathCompiler<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompiler<'_> {
    type Query = QueryFunction;

    type File = Vec<u8>;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath-compiler"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompiler {
            core: JsonPathCompilerCore::new()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = file.into_bytes();
        let padding = vec![0; 64];
        let padded_input = input.into_iter().chain(padding).collect();
        Ok(padded_input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}

pub struct JsonPathCompilerMmap<'a> {
    core: JsonPathCompilerCore<'a>,
}

impl Implementation for JsonPathCompilerMmap<'_> {
    type Query = QueryFunction;

    type File = Mmap;

    type Error = JsonPathCompilerError;

    type Result<'a> = JsonPathCompilerResult;

    fn id() -> &'static str {
        "jsonpath-compiler-mmap"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonPathCompilerMmap {
            core: JsonPathCompilerCore::new()?
        })
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        unsafe {
            let mapped_file = Mmap::map(&file)?;
            Ok(mapped_file)
        }
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(query, file)
    }
}


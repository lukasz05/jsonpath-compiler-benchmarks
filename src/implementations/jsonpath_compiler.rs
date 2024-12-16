use std::fmt::Display;
use std::process::Command;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::framework::implementation::Implementation;

struct JsonPathCompilerCore {}

pub struct JsonPathCompilerResult(String);

impl JsonPathCompilerCore {
    // TODO: make it configurable
    const COMPILER_PATH: &'static str =
        "/Users/lukasz/source/jsonpath-compiler/target/debug/jsonpath-compiler";
    const SIMDJSON_INCLUDE_PATH: &'static str = "-I/opt/homebrew/Cellar/simdjson/3.10.1/include";
    const SIMDJSON_LIB_PATH: &'static str = "-L/opt/homebrew/Cellar/simdjson/3.10.1/lib";
    const GENERATED_QUERY_CODE_FILE_PATH: &'static str = "query.cpp";
    const COMPILED_QUERY_PROG_FILE_PATH: &'static str = "./query";

    fn new() -> Result<Self, JsonPathCompilerError> {
        Ok(JsonPathCompilerCore {})
    }

    fn compile_query(&self, query: &str, mmap: bool) -> Result<(), JsonPathCompilerError> {
        let mut compile = Command::new(Self::COMPILER_PATH);
        compile.args(["-o", Self::GENERATED_QUERY_CODE_FILE_PATH]);
        if mmap {
            compile.arg("--mmap");
        }
        compile.arg(query).status()?;
        Command::new("c++")
            .args([
                Self::GENERATED_QUERY_CODE_FILE_PATH,
                "-std=c++20",
                "-O3",
                Self::SIMDJSON_INCLUDE_PATH,
                Self::SIMDJSON_LIB_PATH,
                "-lsimdjson",
                "-o",
                Self::COMPILED_QUERY_PROG_FILE_PATH
            ])
            .status()?;
        Ok(())
    }

    fn run<'a>(&self, file: &'a String) -> Result<JsonPathCompilerResult, JsonPathCompilerError> {
        let output = Command::new(Self::COMPILED_QUERY_PROG_FILE_PATH)
            .arg(file)
            .output()?;
        Ok(JsonPathCompilerResult(String::from_utf8(output.stdout)?))
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
    #[error("Ahead of time file loading is not supported")]
    AheadOfTimeFileLoadingNotSupportedError
}

pub struct JsonPathCompiler {
    core: JsonPathCompilerCore,
}

impl Implementation for JsonPathCompiler {
    type Query = ();

    type File = String;

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
        Ok(file_path.to_string())
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query, false)
    }

    fn run<'a>(&self, _query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(file)
    }
}

pub struct JsonPathCompilerMmap {
    core: JsonPathCompilerCore,
}

impl Implementation for JsonPathCompilerMmap {
    type Query = ();

    type File = String;

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
        Ok(file_path.to_string())
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        self.core.compile_query(query, true)
    }

    fn run<'a>(&self, _query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        self.core.run(file)
    }
}


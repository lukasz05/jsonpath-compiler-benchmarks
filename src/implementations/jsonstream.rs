use std::{env, io::{self}};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use thiserror::Error;

use crate::framework::implementation::Implementation;
use crate::implementations::jsonstream::JsonStreamError::{CompileQueryError, ExecuteQueryError, FileLoadError};

pub struct JsonStream {}

impl JsonStream {
    fn send_message_to_host(message: &str) -> io::Result<u8> {
        let mut stream = UnixStream::connect("/tmp/jsonstream_host.sock")?;
        stream.write_all(message.as_bytes())?;
        let mut response: [u8; 1] = [0];
        stream.read_exact(&mut response)?;
        Ok(response[0])
    }
}

impl Implementation for JsonStream {
    type Query = ();

    type File = ();

    type Error = JsonStreamError;

    type Result<'a> = u8;

    fn id() -> &'static str {
        "jsonstream"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonStream {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let current_path = env::current_dir()?;
        let file_path = current_path.join(file_path)
            .into_os_string()
            .into_string()
            .unwrap();
        let result = Self::send_message_to_host(&format!("load {file_path}"))?;
        if result == 0 {
            Ok(())
        } else {
            Err(FileLoadError)
        }
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let result = Self::send_message_to_host(&format!("compile {query}"))?;
        if result == 0 {
            Ok(())
        } else {
            Err(CompileQueryError)
        }
    }

    fn run<'a>(&self, _query: &'a Self::Query, _file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        let result = Self::send_message_to_host("execute")?;
        if result == 0 {
            Ok(0)
        } else {
            Err(ExecuteQueryError)
        }
    }
}

#[derive(Error, Debug)]
pub enum JsonStreamError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error loading the JSON file")]
    FileLoadError,
    #[error("error compiling the query")]
    CompileQueryError,
    #[error("error executing the query")]
    ExecuteQueryError
}

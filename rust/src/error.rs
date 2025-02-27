// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use arrow_schema::ArrowError;

#[derive(Debug)]
pub enum Error {
    Arrow(String),
    Schema(String),
    IO(String),
    Index(String),
    /// Stream early stop
    Stop(),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (catalog, message) = match self {
            Self::Arrow(s) => ("Arrow", s.as_str()),
            Self::Schema(s) => ("Schema", s.as_str()),
            Self::IO(s) => ("I/O", s.as_str()),
            Self::Index(s) => ("Index", s.as_str()),
            Self::Stop() => ("Early stop", ""),
        };
        write!(f, "LanceError({catalog}): {message}")
    }
}

impl From<ArrowError> for Error {
    fn from(e: ArrowError) -> Self {
        Self::Arrow(e.to_string())
    }
}

impl From<&ArrowError> for Error {
    fn from(e: &ArrowError) -> Self {
        Self::Arrow(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<object_store::Error> for Error {
    fn from(e: object_store::Error) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<object_store::path::Error> for Error {
    fn from(e: object_store::path::Error) -> Self {
        Self::IO(e.to_string())
    }
}

impl std::error::Error for Error {}

impl From<Error> for ArrowError {
    fn from(value: Error) -> Self {
        match value {
            Error::Arrow(err) => Self::IoError(err), // we lose the error type converting to LanceError
            Error::IO(err) => Self::IoError(err),
            Error::Schema(err) => Self::SchemaError(err),
            Error::Index(err) => Self::IoError(err),
            Error::Stop() => Self::IoError("early stop".to_string()),
        }
    }
}

impl From<sqlparser::parser::ParserError> for Error {
    fn from(e: sqlparser::parser::ParserError) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<sqlparser::tokenizer::TokenizerError> for Error {
    fn from(e: sqlparser::tokenizer::TokenizerError) -> Self {
        Self::IO(e.to_string())
    }
}

impl From<Error> for datafusion::error::DataFusionError {
    fn from(e: Error) -> Self {
        Self::Execution(e.to_string())
    }
}

impl From<datafusion::error::DataFusionError> for Error {
    fn from(e: datafusion::error::DataFusionError) -> Self {
        Self::IO(e.to_string())
    }
}

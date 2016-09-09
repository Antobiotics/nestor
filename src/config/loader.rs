use config::Config;
use std::{io, convert};

extern crate toml;

pub trait ConfigLoader {
    fn load(&self) -> Result<Config, ConfigLoadError>;
}

#[derive(Debug)]
pub enum ConfigLoadError {
    LoadIoError(io::Error),
    ParseError(toml::ParserError),
    DecodeError(toml::DecodeError),
}

impl convert::From<io::Error> for ConfigLoadError {
    fn from(err: io::Error) -> ConfigLoadError {
        ConfigLoadError::LoadIoError(err)
    }
}

impl convert::From<toml::DecodeError> for ConfigLoadError {
    fn from(err: toml::DecodeError) -> ConfigLoadError {
        ConfigLoadError::DecodeError(err)
    }
}

impl convert::From<toml::ParserError> for ConfigLoadError {
    fn from(err: toml::ParserError) -> ConfigLoadError {
        ConfigLoadError::ParseError(err)
    }
}

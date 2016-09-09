use std::fs::File;
use std::io::Read;
use config::{Config};
use config::loader::{ConfigLoader, ConfigLoadError};
use rustc_serialize::Decodable;


extern crate toml;

#[derive(Debug)]
pub struct FileLoader {
    pub file_path: &'static str,
}

impl FileLoader {
    fn load_toml(&self) -> Result<toml::Value, ConfigLoadError> {
        let mut config_file = try!(File::open(self.file_path));
        let mut config_content = String::new();
        try!(config_file.read_to_string(&mut config_content));

        let mut parser = toml::Parser::new(&config_content);

        match parser.parse() {
            Some(toml) => Ok(toml::Value::Table(toml)),
            None => Err(ConfigLoadError::ParseError(parser.errors.pop().unwrap())),
        }

    }
}

impl ConfigLoader for FileLoader {
    fn load(&self) -> Result<Config, ConfigLoadError> {

        match self.load_toml() {
            Ok(toml) => {
                let mut decoder = toml::Decoder::new(toml);
                let config = try!(Config::decode(&mut decoder));

                Ok(config)
            },
            Err(err) => Err(err)
        }
    }
}

extern crate rustc_serialize;

pub mod loader;
pub mod file_loader;


#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct Config {
   pub server: ServerConfig,
   pub redis: RedisConfig,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct ServerConfig {
    pub host: String,
    pub name: String,
    pub port: i16,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
pub struct RedisConfig {
    pub host: String,
    pub port: i16,
    pub password: String,
}

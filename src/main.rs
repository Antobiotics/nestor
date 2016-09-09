#[macro_use] extern crate nickel;
extern crate rustc_serialize;

mod config;

use nickel::{
    Nickel, MiddlewareResult, Request, Response, MediaType,
    JsonBody
};

use config::loader::ConfigLoader;
use config::file_loader::FileLoader;

fn logger<'a, D>(request: &mut Request<D>, response: Response<'a, D>) -> MiddlewareResult<'a, D> {
    println!("logging resquest: {:?}", request.origin.uri);
    response.next_middleware()
}

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname: String,
}

fn main() {

    let config_file_loader = FileLoader{file_path: "./conf/conf.toml"};
    let config = config_file_loader.load();
    match config {
        Ok(c) => {
            let server_details = format!("{host}:{port}",
                                         host=c.server.host,
                                         port=c.server.port);

            let mut server = Nickel::new();

            server.utilize(logger);

            server.utilize(router! {
                get "/usr/:userid" => |request| {
                    format!("This is user: {}", request.param("userid").unwrap())
                }

                get "/content-type" => |_, mut response| {
                    response.set(MediaType::Json);
                    "{'foo': 'bar'}"
                }

                post "/a/post/request" => |request| {
                    let person = request.json_as::<Person>().unwrap();
                    format!("Hello {} {}", person.firstname, person.lastname)
                }
            });

            println!("{:?}", server_details);
            server.listen(&server_details[..]);
        },
        Err(e) => {
            panic!("Error parsing {:?}", e);
        }
    }
}


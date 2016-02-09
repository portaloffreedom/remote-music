#[macro_use]
extern crate rustful;

extern crate serde;
extern crate serde_json;

mod auth;
mod config;

use std::error::Error;
use rustful::{Server, Handler, Context, Response, TreeRouter};
use auth::Auth;
use config::Config;

struct Greeting<'a> {
    text: String,
    auth: &'a Auth,
}

impl Handler for Greeting<'static> {
    fn handle_request(& self, context: Context, response: Response) {
        //Check if the client accessed /hello/:name or /good_bye/:name
        if let Some(name) = context.variables.get("name") {
            //Use the value of :name
            if !self.auth.authenticate(name.as_ref()) {
                return response.send(format!("Porco dio 404\n"));
            }
            response.send(format!("{}, {}\n", self.text, name));
        } else {
            response.send(self.text.clone())
        }
    }
}

fn main() {
    let c = match Config::load_from_file("conf.json") {
        Ok(conf) => conf,
        Err(e) => {
            println!("could not read config file: {}", e.description());
            return;
        },
    };
    println!("{:?}", c);

    static mut auth: *mut Auth = 0 as *mut Auth;
    unsafe {
        auth = &mut Auth::new(c.auth);
    }

    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            //Receive GET requests to /hello and /name
            "hello" => {
                Get: Greeting { text: "hello".to_string(), auth: unsafe {&*auth}},
                ":name" => Get: Greeting { text: "hello".to_string(), auth: unsafe {&*auth}},
            },
            //Receive GET requests to /good_bye and /good_bye/:name
            "good_bye" => {
                Get: Greeting { text: "good bye".to_string(), auth: unsafe {&*auth}},
                ":name" => Get: Greeting { text: "good bye".to_string(), auth: unsafe {&*auth}},
            }
        }
    };

    let s = Server {
        //Use a closure to handle requests.
        handlers: my_router,
        //Set the listening port to `8080`.
        host: 8080.into(),
        //Fill out everything else with default values.
        ..Server::default()
    };

    match s.run() {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}

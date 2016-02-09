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

struct Greeting {
    text: String,
    auth: Auth,
}

impl Handler for Greeting {
    fn handle_request(&self, context: Context, response: Response) {
        //Check if the client accessed /hello/:name or /good_bye/:name
        if let Some(name) = context.variables.get("name") {
            //Use the value of :name
            if !self.auth.authenticate(name.as_ref()) {
                return response.send(format!("Porco dio 404"));
            }
            response.send(format!("{}, {}", self.text, name));
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

    let auth: Auth = Auth::new(c.auth);


    let my_router = insert_routes!{
        //Create a new TreeRouter
        TreeRouter::new() => {
            //Receive GET requests to /hello and /name
            "hello" => {
                Get: Greeting { text: "hello".to_string(), auth: auth.clone()},
                ":name" => Get: Greeting { text: "hello".to_string(), auth: auth.clone()},
            },
            //Receive GET requests to /good_bye and /good_bye/:name
            "good_bye" => {
                Get: Greeting { text: "good bye".to_string(), auth: auth.clone()},
                ":name" => Get: Greeting { text: "good bye".to_string(), auth: auth.clone()},
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

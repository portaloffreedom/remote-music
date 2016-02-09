extern crate serde;
extern crate serde_json;

use std::io;
use std::io::Read;
use std::fs::File;
use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/config.rs"));

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub auth: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Config, io::Error> {
        // TEST SECTION
        let point = ConfigFile { file_path: "data".to_string(), auth_file: "auth_file".to_string() };
        let serialized = serde_json::to_string(&point).unwrap();
        println!("{}", serialized);

        let deserialized: ConfigFile = serde_json::from_str(&serialized).unwrap();
        println!("{:?}", deserialized);
        // END TEST SECTION

        let mut file = try!(File::open(path));

        let mut data = String::new();
        try!(file.read_to_string(& mut data));

        let deserialized: ConfigFile = try!(
            serde_json::from_str(&data).map_err(|err| {
                io::Error::new(io::ErrorKind::Other, err.description())
            })
        );


        let mut auth_file = try!(File::open(deserialized.auth_file));
        let mut auth_code = String::new();
        try!(auth_file.read_to_string(& mut auth_code));


        Ok(Config {
            file_path: deserialized.file_path,
            auth: auth_code,
        })
    }
}

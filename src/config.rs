extern crate toml;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use super::Result;
use super::Failable;

const CONFIG_FILE: &'static str = "config.toml";

#[derive(Debug, RustcDecodable)]
pub struct GitHubConfig {
    pub id: String,
    pub access_token: String,
}

#[derive(Debug, RustcDecodable)]
pub struct Config {
    pub github: GitHubConfig,
}

impl Config {
    /// generates a new config file interactively.
    pub fn generate() -> Failable {
        println!("Please prepare a GitHub access token which has gist scope.");

        // TODO An access token can be acuired programatically.
        // https://developer.github.com/v3/oauth_authorizations/#create-a-new-authorization
        print!("Enter your access token: ");
        io::stdout().flush()?;
        let mut access_token = String::new();
        io::stdin().read_line(&mut access_token)?;

        let mut f = File::create(CONFIG_FILE)?;
        let toml = format!("[github]\naccess_token = \"{}\"\n",
                           access_token.trim());
        f.write_all(toml.as_bytes())?;

        Ok(())
    }

    pub fn load() -> Result<Config> {
        let mut file = File::open(CONFIG_FILE)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        toml::decode_str(&file_content)
            .ok_or(From::from("Invalid config file."))
    }
}


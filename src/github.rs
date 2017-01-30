extern crate curl;

use rustc_serialize::json;
use self::curl::easy::Easy;
use super::Result;
use super::Failable;
use std::io::prelude::*;

#[derive(RustcDecodable)]
struct ResponseGistCreated {
    git_push_url: String,
}

const USER_AGENT: &'static str = "Nyarticles";

fn json_to_create_gist(filename: &str, content: &String) -> String {
    let content = content
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("/", "\\/")
        .replace("\r", "\\r")
        .replace("\n", "\\n")
        .replace("\t", "\\t");
    format!(
"{{
   \"description\": \"An article of Nyarticles.\",
   \"public\": true,
   \"files\": {{
     \"{}\": {{
       \"content\": \"{}\"
     }}
   }}
}}", filename, content)
}

fn check_response_code(response_code: u32, expected_code: u32, error_message: &str) -> Failable {
    if response_code != expected_code {
        Err(From::from(
            format!("{} (Response code = {})", error_message, response_code)
            ))
    } else {
        Ok(())
    }
}

trait EasyExt {
    /// posts data and returns response code and response data.
    fn post_data(&mut self, url: &str, useragent: &str, data: &[u8])
            -> Result<(u32, Vec<u8>)>;
}

impl EasyExt for Easy {
    fn post_data(&mut self, url: &str, useragent: &str, mut data: &[u8])
            -> Result<(u32, Vec<u8>)> {
        self.url(&url)?;
        self.useragent(useragent)?;
        self.post(true)?;
        self.post_field_size(data.len() as u64)?;
        let mut res_data = Vec::new();
        {
            let mut transfer = self.transfer();
            transfer.write_function(|buf| Ok(res_data.write(buf).unwrap()))?;
            transfer.read_function(|buf| Ok(data.read(buf).unwrap()))?;
            transfer.perform()?;
        }
        let res_code = self.response_code()?;
        Ok((res_code, res_data))
    }
}

pub struct GitHub<'a> {
    access_token: &'a str,
}

impl<'a> GitHub<'a> {
    pub fn new(access_token: &'a str) -> Self {
        GitHub { access_token: access_token }
    }

    pub fn get_access_token(&self) -> &'a str {
        self.access_token
    }

    /// creates a gist and returns the push url.
    pub fn create_gist(&self, filename: &str, content: &String)
            -> Result<String> {
        let json = json_to_create_gist(filename, content);

        let mut curl = Easy::new();
        let url = format!("https://api.github.com/gists?access_token={}",
                          self.access_token);
        let (response_code, response_data) =
            curl.post_data(&url, USER_AGENT, json.as_bytes())?;
        let response_json = String::from_utf8(response_data)?;
        //println!("response: {}", response_json);
        check_response_code(response_code, 201, "Failed to create a gist.")?;

        let decoded: ResponseGistCreated = json::decode(&response_json)?;
        Ok(decoded.git_push_url)
    }
}

use std::io::Read;
use reqwest::blocking::{Client};
use serde_json::json;
use crate::structures::default_structures::{FetchDataResponse, PkgExistence};


pub fn fetch_pkg_data(packages_list: Vec<String>) -> Result<FetchDataResponse, String> {
    let client = Client::new();

    let mut new_request_string = "http://127.0.0.1:8080/api/get-pkg-data".to_string();

    let payload = json!({ "package_list": packages_list });
    let response = client.post(&new_request_string).json(&payload).send();

    if response.is_ok() {
        let response_unwrapped = response.unwrap();
        let mut new_response = response_unwrapped.json::<FetchDataResponse>();
        if(new_response.is_ok()) {
            // println!("{:#?}", new_response);
            Ok(new_response.unwrap())
        } else {
            Err("Couldn't recognize response. Maybe, the server is under maintenance.".to_string())
        }
    } else {
        println!("An error occured. Check internet connection or try again later.");
        // println!("{:#?}", response);
        Err("Server returned an error. Check internet connection or try again later.".to_string())
    }
    // println!("came here: {}", package_name);
}

pub fn check_existence(package_name: &str) -> bool {
    let client = Client::new();

    let mut new_request_string = "http://127.0.0.1:8080/api/check-pkg-existence?name=".to_string();
    new_request_string.push_str(package_name);
    let response = client.get(&new_request_string).send();
    if(response.is_ok()) {
        let response_unwrapped = response.unwrap();
        let response_json = response_unwrapped.json::<PkgExistence>();
        if(response_json.is_ok()) {
            if(response_json.unwrap().existence) {
                return true;
            }
        } else {
            println!("Something went wrong. Server returned an error.");
        }
    } else
    {println!("Something went wrong. Check internet connection.");}
    false
}
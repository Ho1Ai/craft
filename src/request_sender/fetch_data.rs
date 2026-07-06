use reqwest::blocking::{Client};
use crate::structures::default_structures::FetchDataResponse;


pub fn fetch_pkg_data(package_name: String) -> Result<FetchDataResponse, String> {
    let client = Client::new();

    let mut new_request_string = "http://127.0.0.1:8000/api/get-pkg-data?name=".to_string();

    new_request_string.push_str(package_name.as_str());

    let response = client.get(&new_request_string).send();

    if response.is_ok() {
        println!("{:#?}", response);
        let mut new_response = response.ok().unwrap().json::<FetchDataResponse>();
        println!("{:#?}, {:#?}", new_response, new_request_string);
        println!("{:?}", new_response);
        Ok(FetchDataResponse{packages: vec![]})
    } else {
        println!("error");
        println!("{:#?}", response);
        Err("Server returned an error. Check internet connection or try again later.".to_string())
    }
    // println!("came here: {}", package_name);
}
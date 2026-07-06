use crate::structures::default_structures as structures;
use reqwest::blocking::Client;

pub fn send_download_request(requested_packages: structures::RequestStructure) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut request_link: String = "http://localhost:8080/pkg-get?name=".to_owned();
    let pkg_list = requested_packages.packages.join(",");
    println!("Downloading: {}", pkg_list);
    request_link.push_str(&pkg_list);
    Ok(())
}
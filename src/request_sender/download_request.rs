use std::fs;
use std::io::Write;
use crate::structures::default_structures as structures;
use reqwest::blocking::Client;
use tar::Archive;

pub fn send_download_request(requested_package: String) -> Result<String, String> {
    let client = Client::new();
    let mut request_link: String = "http://localhost:8080/pkg-get".to_owned();

    let response = client.get(requested_package.as_str()).header("pkg_name", requested_package.clone()).send();

    if response.is_ok() {
        let response_unwrapped = response.unwrap();
        let response_headers = response_unwrapped.headers().clone();
        let mut response_bytes = response_unwrapped.bytes().unwrap();

        let mut tmp_dir_path = "/etc/craft-downloaded/".to_owned();
        tmp_dir_path.push_str(requested_package.as_str());
        tmp_dir_path.push_str(".tar");

        let mut tmp_dir_extracted = "/etc/craft-tmp/".to_owned();
        tmp_dir_extracted.push_str(requested_package.as_str());
        tmp_dir_extracted.push_str("/");

        // let mut final_dir_path = "/usr/local/bin/".to_owned();
        // final_dir_path.push_str(requested_package.as_str());

        let mut curr_file_archive = fs::OpenOptions::new().read(true).append(true).write(true).open(tmp_dir_path.as_str()).unwrap();
        curr_file_archive.write_all(&mut response_bytes).unwrap();
        let archive_path = fs::File::open(tmp_dir_path.as_str()).unwrap();

        let mut archive = Archive::new(archive_path);

        let extr_test = archive.unpack(tmp_dir_extracted.as_str());

        if extr_test.is_err() {
            println!("{} did not extract.", requested_package.as_str());
        }
    }

    Ok("".to_string())
}
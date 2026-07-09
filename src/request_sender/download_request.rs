use std::fs;
use std::io::Write;
use std::process::Command;
use crate::structures::default_structures as structures;
use reqwest::blocking::Client;
use tar::Archive;

pub fn send_download_request(requested_package: &str) -> Result<String, String> {
    let client = Client::new();
    let request_link: String = "http://localhost:8080/api/pkg-get".to_owned();

    let response = client.get(&request_link).header("pkg_name", requested_package).send();

    if response.is_ok() {
        let response_unwrapped = response.unwrap();
        let mut response_headers = response_unwrapped.headers().clone();
        let mut response_bytes = response_unwrapped.bytes().unwrap();

        let mut tmp_dir_path = "/etc/craft-downloaded/".to_owned();
        tmp_dir_path.push_str(requested_package);
        tmp_dir_path.push_str(".tar");

        let mut tmp_dir_extracted = "/etc/craft-tmp/".to_owned();
        tmp_dir_extracted.push_str(requested_package);
        tmp_dir_extracted.push_str("/");

        // let mut final_dir_path = "/usr/local/bin/".to_owned();
        // final_dir_path.push_str(requested_package.as_str());

        let create_dir_result = fs::create_dir_all("/etc/craft-downloaded/");
        if(!create_dir_result.is_ok()) {
            println!("Error creating directory: {:?}", create_dir_result.unwrap_err());
        } else{

            // println!("Path: {:?}", tmp_dir_path);
            let mut curr_file_archive = fs::OpenOptions::new().write(true).append(true).create(true).open(tmp_dir_path.as_str());
            if(curr_file_archive.is_ok()) {
                let mut curr_file_unwrapped = curr_file_archive.unwrap();

                curr_file_unwrapped.write_all(&mut response_bytes).unwrap();
                println!("Successfully wrote downloaded archive");
                let archive_path = fs::File::open(tmp_dir_path.as_str()).unwrap();

                let mut archive = Archive::new(archive_path);

                let extr_test = archive.unpack(tmp_dir_extracted.as_str());

                if extr_test.is_err() {
                    println!("{} was not extracted. Shutting down with error.", requested_package);
                    return Err("Could not extract archive.".to_owned());
                } else {
                    let command_exec = Command::new("make").current_dir(tmp_dir_extracted.as_str()).output();
                    if (command_exec.is_ok()) {
                        let command_exec = Command::new("make").arg("install").current_dir(tmp_dir_extracted.as_str()).output();
                        if command_exec.is_ok() {
                            println!("Installation ended.");
                            let mut str_ret = "/usr/local/bin/".to_owned();
                            str_ret.push_str(&requested_package);
                            return Ok(str_ret);
                        } else {
                            println!("{} was not installed. Shutting down with error.", requested_package);
                            return Err("Could not install.".to_owned());
                        }
                    } else {
                        println!("{} was not installed. Shutting down with error.", requested_package);
                        return Err("Could not install.".to_owned());
                    }
                }
            } else {
                println!("{:#?}", curr_file_archive);
            }
        }
    } else {
        println!("An error occurred. Message: {:#?}", response);

    }

    Ok("".to_string())
}
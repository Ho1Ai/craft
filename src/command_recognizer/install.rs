use std::fs;
use std::io::Write;
use crate::request_sender::download_request::send_download_request;
use crate::request_sender::fetch_data;
use crate::structures::default_structures::{FetchDataResponse, SinglePkg};

pub fn install_start(install_list: Vec<String>) {
    // for elem in install_list {
    //     fetch_data::fetch_pkg_data(elem);
    // }

    let mut pkg_info_result = fetch_data::fetch_pkg_data(install_list);
    if pkg_info_result.is_ok() {
        let pkg_info = pkg_info_result.unwrap();
        let mut packages: Vec<SinglePkg> = Vec::<SinglePkg>::new();
        pkg_info.packages.clone_into(&mut packages);
        for pkg in pkg_info.packages.clone() {
            println!("{} - {} - {}b", pkg.name, pkg.version, pkg.pkg_size);
        }

        print!("Proceed with installation [Y/n]: ");
        std::io::stdout().flush();
        let mut can_proceed = String::new();
        std::io::stdin().read_line(&mut can_proceed).expect("Failed to read line. Falling with an error");

        if can_proceed.as_str() == "\n" || can_proceed.as_str() == "y\n" || can_proceed.as_str() == "Y\n" {
            let i: usize = 0;
            for pkg in pkg_info.packages {
                install_single(pkg.name, pkg.version);
            }
        } else {
            println!("User did not give permission to proceed.");
        }
    } else {
        println!("Installation process is being terminated.");
    }
}

pub fn install_single(package_name: String, pkg_version: String) {
    if fetch_data::check_existence(package_name.as_str()) { // a "good" way to leave this problem
        let download_result = send_download_request(package_name.as_str());
        if download_result.is_ok() {
            let write_res = write_pkg_info(package_name, pkg_version, download_result.unwrap().as_str());
            if write_res.is_ok() {
                println!("Successfully wrote package info.");
            } else {
                println!("Could not write package info.");
            }
        }
    } else {
        println!("{} does not exist.", package_name);
    }
}

pub fn write_pkg_info(name: String, version: String, path: &str) -> Result<&'static str, &'static str> { // why don't I use &str instead of String... well, to be replaced
    let check_cfg_existence = fs::File::open("/etc/craft-installed-pkgs.totmb");

    if !check_cfg_existence.is_ok() {
        let creation = fs::File::create("/etc/craft-installed-pkgs.totmb");
        if !creation.is_ok() {
            return Err("Failed to create config file. Write sudo touch /etc/craft-installed-pkgs.totmb and try again.");
        }
    }

    let mut res_str = name.to_string();
    res_str.push_str("=");
    res_str.push_str(version.as_str());
    res_str.push_str("=");
    res_str.push_str(path);

    let file = fs::read_to_string("/etc/craft-installed-pkgs.totmb");
    if file.is_ok() {
        res_str.push_str("\n");
        res_str.push_str(file.unwrap().as_str());
        let write_test = fs::write("/etc/craft-installed-pkgs.totmb", res_str);
        if write_test.is_ok() {
            return Ok("success");
        } else {
            return Err("Error writing to /etc/craft-installed-pkgs.totmb");
        }
    } else {
        return Err("Error reading package info");
    }
}
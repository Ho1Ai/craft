use crate::request_sender::download_request::send_download_request;
use crate::request_sender::fetch_data;

pub fn install_start(install_list: Vec<String>) {
    // for elem in install_list {
    //     fetch_data::fetch_pkg_data(elem);
    // }

    let mut pkg_info_result = fetch_data::fetch_pkg_data(install_list);
    if pkg_info_result.is_ok() {
        let pkg_info = pkg_info_result.unwrap();
        for pkg in pkg_info.packages.clone() {
            println!("{} - {} - {}b", pkg.name, pkg.version, pkg.pkg_size);
        }
        let mut can_proceed = String::new();
        std::io::stdin().read_line(&mut can_proceed).expect("Failed to read line. Falling with an error");

        if can_proceed.as_str() == "\n" || can_proceed.as_str() == "y\n" || can_proceed.as_str() == "Y\n" {
            for pkg in pkg_info.packages {
                install_single(pkg.name);
            }
        } else {
            println!("User did not give permission to proceed.");
        }
    } else {
        println!("Installation process is being terminated.");
    }
}

pub fn install_single(package_name: String) {
    if fetch_data::check_existence(package_name.as_str()) { // a "good" way to leave this problem
        send_download_request(package_name);
    } else {
        println!("{} does not exist.", package_name);
    }
}
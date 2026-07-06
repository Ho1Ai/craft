use crate::request_sender::fetch_data;

pub fn install_start(install_list: String) {
    // for elem in install_list {
    //     fetch_data::fetch_pkg_data(elem);
    // }

    fetch_data::fetch_pkg_data(install_list);
}
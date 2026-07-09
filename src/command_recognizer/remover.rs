use std::fs;

pub fn rm_package(pkg_name: &str) {
    let path = check_writing_for_path(pkg_name);
}

pub fn check_writing_for_path(pkg_name: &str) -> Result<String, &'static str> {
    let file = fs::read_to_string("/etc/craft-installed-pkgs.totmb");
    if(file.is_ok()) {
        let new_file_strings = file.unwrap();
        let file_vec = new_file_strings.as_str().split("\n").collect::<Vec<&str>>();
        let mut line_id = 0;
        for elem in file_vec {
            if elem.starts_with(pkg_name) {
                return Ok(elem.to_owned());
            }
        }
        return Err("");
    } else {
        println!("No config file found.");
        return Err("No config file found");
    }
}
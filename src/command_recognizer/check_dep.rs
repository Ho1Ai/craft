use reqwest::blocking::Client;
use crate::structures::default_structures::DependenciesList;

pub fn check_deps(pkg_name: &str) -> Result<&'static str, &'static str> {
    let client = Client::new();
    let mut request_link: String = "http://localhost:8080/api/get-pkg-dependencies".to_owned();

    let response = client.get(&request_link).header("pkg_name", pkg_name).send();

    if response.is_ok() {
        let response = response.unwrap().json::<DependenciesList>();

        if response.is_ok() {
            println!("Dependencies for {}: {:?}", pkg_name, response.unwrap());
        }else { println!("Error: {:?}", response) }
    } else {
        println!("{:#?}", response);
    }

    Err("An error occured while checking dependencies.")
}
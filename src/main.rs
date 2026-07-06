mod structures;
mod request_sender;
mod command_recognizer;

const CURRENT_VERSION: &str = "00001a";


fn main() {
    println!("Craft package manager. Alpha version: {}", CURRENT_VERSION);

    let mut arguments: Vec<String> = std::env::args().collect();

    let mut input_result = command_recognizer::command_recognizer::recognize_command(arguments);

    // let mut test_start_struct: structures::default_structures::PackagesList = structures::default_structures::PackagesList {raw_packages_list: vec![]};

    if (input_result.is_ok == false) {
        return
    }

    command_recognizer::command_recognizer::inner_runtime(input_result);

    // request_sender::download_request::send_download_request(test_request).unwrap();
}

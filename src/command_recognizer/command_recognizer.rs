use crate::structures::default_structures;
use crate::structures::default_structures::RecognizedCommand;
use crate::command_recognizer::install;
// use std::iter::;

const HELP_LIST: &str = "Craft package manager. Help list:\n  $ craft -i package1 package2 ... - install package with current name\n";

pub fn recognize_command(input_vec: Vec<String>) -> default_structures::RecognizedCommand {
    let mut result = RecognizedCommand {is_ok: true, unsplit_fields: vec![]};


    //checks:
    if input_vec.len() < 2 {
        println!("Not enough arguments. Enter craft --help to get more information.");
        result.is_ok = false;
        return result;
    }

    if input_vec[1].clone() == "--help".to_string() {
        println!("{}", HELP_LIST);
    }


    let mut new_response_vector: Vec<String> = Vec::new();

    let mut pass_index = false;
    for elem in input_vec {
        if !pass_index {
            pass_index = true;
            continue;
        }

        new_response_vector.push(elem);
    }

    //result insertion:
    result.unsplit_fields = new_response_vector;

    result
}

pub fn inner_runtime (input_struct: default_structures::RecognizedCommand) {
    let mut request_list = String::new();

    let mut start_skip = true;
    let mut first_elem_skip = true;
    let str_arr = input_struct.unsplit_fields.clone();
    for elem in str_arr.clone() {
        if start_skip { // skips -i flag
            start_skip = false;
            continue;
        } else if first_elem_skip {
            first_elem_skip = false;
        } else {
        request_list.push_str(",");
        }

        request_list.push_str(&elem);
    }

    if input_struct.unsplit_fields[0].clone() == "-i".to_string() {
        install::install_start(request_list);
    }
}


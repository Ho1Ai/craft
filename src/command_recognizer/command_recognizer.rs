use is_root::is_root;
use crate::command_recognizer::check_dep::check_deps;
use crate::structures::default_structures;
use crate::structures::default_structures::RecognizedCommand;
use crate::command_recognizer::install;
use crate::request_sender::fetch_data;
// use std::iter::;

const HELP_LIST: &str = "Craft package manager. Help list:\n  $ craft -i package1 package2 ... - install package with current name\n  $ craft -d package1 package2 ... - resolve dependencies list for package1, package2...\n  $ craft -r package1 package2 ... - remove package1, package2...";

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
    if input_struct.unsplit_fields[0].clone() == "-i".to_string() {
        if (!is_root()) {
            println!("Craft package manager with flag -i should be started under root. Shutting down.");
            return;
        }
        let mut packages_list = Vec::<String>::new();
        let mut start_skip = true;

        for elem in input_struct.unsplit_fields {
            if start_skip {
                start_skip = false;
                continue;
            } else {
                packages_list.push(elem);
            }
        }

        install::install_start(packages_list);
        return;
    }

    if input_struct.unsplit_fields[0].clone() == "-d".to_string() {
        let mut packages_list = Vec::<String>::new();
        let mut start_skip = true;

        for elem in input_struct.unsplit_fields {
            if start_skip {
                start_skip = false;
                continue;
            } else {
                packages_list.push(elem); // and where is the logic of this move...
            }
        }

        for pkg in packages_list {
            check_deps_single(pkg.as_str());
        }
    }
}

pub fn check_deps_single (package: &str) {
    if fetch_data::check_existence(package) {
        // println!("Craft package manager already exists.");
        check_deps(package);
    }
}
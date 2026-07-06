use serde::{ Serialize, Deserialize };
use crate::structures;

pub struct RequestStructure {
    pub packages: Vec<String>,
}
// // legacy
// pub struct PackagesList {
//     pub raw_packages_list: Vec<SinglePackage>
// }
//
// pub struct SinglePackage {
//     pub name: String,
// }

pub struct RecognizedCommand {
    pub is_ok: bool,
    pub unsplit_fields: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct FetchDataResponse {
    pub packages: Vec<String>,
}
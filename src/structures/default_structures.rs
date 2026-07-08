use serde::{ Serialize, Deserialize };

// tbh, unnecessary struct
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

#[derive(Deserialize, Debug, Clone)]
pub struct SinglePkg {
    pub id: i64,
    pub name: String,
    pub version: String,
    pub pkg_size: usize
}

#[derive(Deserialize, Debug, Clone)]
pub struct FetchDataResponse {
    pub packages: Vec<SinglePkg>,
}

#[derive(Deserialize, Debug)]
pub struct PkgExistence{
    pub existence: bool
}
#![allow(non_snake_case)]
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageMetadata {
    pub Name: String,
    pub Version: String,
    pub ID: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub isAdmin: bool,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageRating {
    pub NetScore: f64,
    pub BusFactor: f64,
    pub Correctness: f64,
    pub RampUp: f64,
    pub ResponsiveMaintainer: f64,
    pub LicenseScore: f64,
    pub GoodPinningPractice: f64,
    pub GoodEngineeringProcess: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageHistoryEntry {
    pub User: User,
    pub Date: String,
    pub PackageMetadata: PackageMetadata,
    pub Action: String,
}


#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct PackageQuery {
    pub Version: Option<String>,
    pub Name: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageRegEx {
    pub RegEx: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Package {
    pub metadata: PackageMetadata,
    pub data: PackageData,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageData {
    pub Content: Option<String>,
    pub URL: String,
    pub JSProgram: Option<String>,
}

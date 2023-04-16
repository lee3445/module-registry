#![allow(non_snake_case)]
use rocket::data::Outcome;
use rocket::request;
use rocket::request::FromRequest;
use rocket::serde::{Deserialize, Serialize};
use rocket::Request;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageMetadata {
    pub Name: String,
    pub Version: String,
    pub ID: String,
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

#[derive(Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct PackageQuery {
    pub Version: Option<String>,
    pub Name: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Package {
    pub metadata: PackageMetadata,
    pub data: PackageData,
}

impl<'a, 'r> FromRequest<'a, 'r> for Package {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        Outcome::Success(Self);
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageData {
    pub Content: Option<String>,
    pub URL: String,
    pub JSProgram: Option<String>,
}

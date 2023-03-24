#![allow(non_snake_case)]
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PackageRating {
    pub BusFactor: f64,
    pub Correctness: f64,
    pub RampUp: f64,
    pub ResponsiveMaintainer: f64,
    pub LicenseScore: f64,
    pub GoodPinningPractice: f64,
}

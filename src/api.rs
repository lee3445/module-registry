mod schema;

use crate::database::ModuleDB;
use schema::*;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Either;
use rocket::State;
use rocket::fs::{NamedFile, relative};

use std::path::Path;
//#[cfg(test)]
//mod tests;

#[get("/")]
pub async fn world() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("index.html"))).await.ok()
}

#[get("/package/<id>")]
pub async fn package_retrieve(
    id: String,
    mod_db: &State<ModuleDB>,
) -> (Status, Either<Json<Package>, &'static str>) {
    // get package
    let mod_r = mod_db.read().await;
    let res = mod_r.get(&id);
    if res.is_none() {
        return (Status::NotFound, Either::Right("Package does not exist."));
    }

    let db = res.unwrap();

    let metadata = PackageMetadata {
        Name: db.name.to_string(),
        Version: db.version.to_string(),
        ID: db.id.to_string(),
    };
    let data = PackageData {
        Content: "Replace with base64 encoding".to_string(),
        URL: db.url.to_string(),
        JSProgram: "JS".to_string(),
    };
    let response = Package { metadata, data };
    (Status::Ok, Either::Left(Json(response)))
}

#[get("/package/<id>/rate")]
pub async fn package_rate(
    id: String,
    mod_db: &State<ModuleDB>,
) -> (Status, Either<Json<PackageRating>, &'static str>) {
    // get package metadata
    let mod_r = mod_db.read().await;
    let res = mod_r.get(&id);
    if res.is_none() {
        return (Status::NotFound, Either::Right("Package does not exist."));
    }

    // get scores from metadata
    let scores = res.unwrap();
    let ret = PackageRating {
        NetScore: scores.overall,
        BusFactor: scores.bus,
        Correctness: scores.correct,
        RampUp: scores.rampup,
        ResponsiveMaintainer: scores.responsive,
        LicenseScore: scores.license,
        GoodPinningPractice: scores.version,
        GoodEngineeringProcess: scores.review,
    };
    (Status::Ok, Either::Left(Json(ret)))
}

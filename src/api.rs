mod schema;

use crate::conversion::*;
use crate::database::ModuleDB;
use schema::*;

use rocket::fs::{relative, NamedFile};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Either;
use rocket::State;

use std::path::Path;
//#[cfg(test)]
//mod tests;

#[get("/")]
pub async fn world() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("index.html")))
        .await
        .ok()
}

//http://127.0.0.1:8000/package/postcss
#[get("/package/<id>")]
pub async fn package_retrieve(
    id: String,
    mod_db: &State<ModuleDB>,
) -> (Status, Either<Json<Package>, &'static str>) {
    // get package id from database
    let mod_r = mod_db.read().await;
    let res = mod_r.get(&id);
    if res.is_none() {
        return (Status::NotFound, Either::Right("Package does not exist."));
    }
    let db = res.unwrap();

    // initialize metadata and data
    let metadata = PackageMetadata {
        Name: db.name.to_string(),
        Version: db.version.to_string(),
        ID: db.id.to_string(),
    };
    let data = PackageData {
        Content: zip_to_base64(db.path.as_str()).await.unwrap(),
        URL: db.url.to_string(),
        JSProgram: None,
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

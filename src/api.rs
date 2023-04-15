mod schema;

use crate::database::ModuleDB;
use schema::*;

use rocket::fs::{relative, NamedFile};
use rocket::http::{Header, Status};
use rocket::response::{status, Responder};
use rocket::serde::json::Json;
use rocket::Either;
use rocket::State;

use rocket::tokio::fs;

use std::fmt::write;
use std::path::Path;
//#[cfg(test)]
//mod tests;

#[get("/")]
pub async fn world() -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("index.html")))
        .await
        .ok()
}

#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test!"
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

#[delete("/reset")]
pub async fn package_reset(mod_db: &State<ModuleDB>) -> (Status, &'static str) {
    let mut write_lock = mod_db.write().await;
    let del = write_lock.drain();
    for (k, v) in del {
        if fs::remove_file(v.path).await.is_err() {
            println!("cannot remove file for module: {}", k);
        }
    }
    (Status::Ok, "Registry is reset.")
}

#[delete("/package/<id>")]
pub async fn package_delete(id: String, mod_db: &State<ModuleDB>) -> (Status, &'static str) {
    // get package
    let mut mod_r = mod_db.write().await;
    let res = mod_r.remove(&id);
    if res.is_none() {
        return (Status::NotFound, "No such package.");
    }
    if fs::remove_file(res.unwrap().path).await.is_err() {
        println!("cannot remove file for module");
    }
    (Status::Ok, "Package is deleted.")
}

#[put("/authenticate")]
pub async fn authenticate() -> (Status, &'static str) {
    (Status::NotImplemented, "Not implemented")
}

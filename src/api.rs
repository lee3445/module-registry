mod schema;

use crate::database::ModuleDB;
use schema::*;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Either;
use rocket::State;
//#[cfg(test)]
//mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
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
        BusFactor: scores.bus,
        Correctness: scores.correct,
        RampUp: scores.rampup,
        ResponsiveMaintainer: scores.responsive,
        LicenseScore: scores.license,
        GoodPinningPractice: scores.version,
    };
    (Status::Ok, Either::Left(Json(ret)))
}

#[get("/package/<_>/rate")]
pub async fn package_rate_bad() -> status::BadRequest<()> {
    status::BadRequest::<()>(None)
}

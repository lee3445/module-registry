mod schema;

use crate::database::{get_by_name, ModuleDB};
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

// requests with offset that are not u32 will be accepted, offset = None
#[post("/packages?<offset>", data = "<query>")]
pub async fn packages_list(
    offset: Option<u32>,
    query: Json<Vec<PackageQuery>>,
    mod_db: &State<ModuleDB>,
) -> Either<Json<Vec<PackageMetadata>>, (Status, &'static str)> {
    println!("{:?}", offset);
    let mut ret = Vec::new();
    // sort ret
    // idx to offset
    let query_vec = query.to_vec();

    let mod_r = mod_db.read().await;
    for q in query_vec {
        if q.Name == "*" {
            // get all packages
            ret.clear();
            for v in mod_r.values() {
                ret.push(PackageMetadata {
                    Name: v.name.clone(),
                    Version: v.ver.clone(),
                    ID: v.id.clone(),
                });
            }
            break;
        }

        if let Some(m) = get_by_name(&mod_r, &q.Name) {
            ret.push(PackageMetadata {
                Name: m.name.clone(),
                Version: m.ver.clone(),
                ID: m.id.clone(),
            });
        }
    }

    // sort result and get page offset
    ret.sort_by(|a, b| a.ID.cmp(&b.ID));
    let offset: usize = offset.unwrap_or(0).try_into().unwrap_or(0);
    if ret.len() < offset + 20 {
        ret = ret.drain(offset..).collect();
    } else {
        ret = ret.drain(offset..offset + 20).collect();
    }

    Either::Left(Json(ret))
}
#[post("/packages")]
pub async fn packages_list_400() -> status::BadRequest<&'static str> {
    status::BadRequest(Some(
        "There is missing field(s) in the PackageQuery/offset or it is formed improperly.",
    ))
}
// reroute 422 to 400
#[catch(422)]
pub fn packages_list_422() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Error processing data"))
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

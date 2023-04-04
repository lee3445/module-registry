mod schema;

use crate::database::{get_by_name, ModuleDB};
use schema::*;

use rocket::http::{Header, Status};
use rocket::response::{status, Responder};
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

#[get("/test")]
pub fn test() -> &'static str {
    "Hello, test!"
}

// path forwarding:
// 1. request with usize offset and valid query -> accept
// 2. request with non-usize offset and valid query -> 400
// 3. request with no offset and valid query -> accept, offset = 0
// 4. otherwise -> 400
// offset in header is "" when the last page is returned
#[post("/packages?<offset>", data = "<query>", rank = 1)]
pub async fn packages_list(
    offset: usize,
    query: Json<Vec<PackageQuery>>,
    mod_db: &State<ModuleDB>,
) -> Either<PackageListResponse, status::BadRequest<&'static str>> {
    let mut ret = Vec::new();
    let query_vec = query.to_vec();

    let mod_r = mod_db.read().await;
    // find maching package for each query
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

    // check if offset is out of range
    if offset >= ret.len() {
        return Either::Right(status::BadRequest(Some("Offset out of range")));
    }

    // sort result and get next offset
    let page_size = 20;
    ret.sort_by(|a, b| a.ID.cmp(&b.ID));
    let next_offset = if offset + page_size >= ret.len() {
        String::new()
    } else {
        (offset + page_size).to_string()
    };

    // keep entries offset..(offset+page_size)
    if ret.len() < offset + page_size {
        ret = ret.drain(offset..).collect();
    } else {
        ret = ret.drain(offset..offset + page_size).collect();
    }

    Either::Left(PackageListResponse {
        offset: Header::new("offset", next_offset),
        body: Json(ret),
    })
}
#[post("/packages?<offset>", data = "<query>", rank = 2)]
pub async fn packages_list_bad_offset(
    offset: Option<String>,
    query: Json<Vec<PackageQuery>>,
    mod_db: &State<ModuleDB>,
) -> Either<PackageListResponse, status::BadRequest<&'static str>> {
    match offset {
        Some(_) => Either::Right(status::BadRequest(Some(
            "Offset must be a non-negative integer, or no packages are matched by query",
        ))),
        None => packages_list(0, query, mod_db).await,
    }
}
#[post("/packages", rank = 3)]
pub async fn packages_list_400() -> status::BadRequest<&'static str> {
    status::BadRequest(Some(
        "There is missing field(s) in the PackageQuery/AuthenticationToken/offset or it is formed improperly.",
    ))
}
// reroute 422 to 400
// 422 is possible when passed in invalid query
#[catch(422)]
pub fn packages_list_422() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Error processing data"))
}
// no other way to set custom headers other than this
#[derive(Responder)]
pub struct PackageListResponse {
    body: Json<Vec<PackageMetadata>>,
    offset: Header<'static>,
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

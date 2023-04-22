mod schema;

use crate::conversion::*;
use crate::database::{get_by_name, ModuleDB};
use schema::*;

use rocket::fs::{relative, NamedFile};
use rocket::http::{Header, Status};
use rocket::response::{status, Responder};
use rocket::serde::json::Json;
use rocket::Either;
use rocket::State;

use rocket::tokio::fs;

use std::io::Read;
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

#[put("/package/<id>", data = "<package>")]
pub async fn package_update(
    id: String,
    package: Json<Package>,
    mod_db: &State<ModuleDB>,
) -> (Status, &'static str) {
    let mod_r = mod_db.read().await;
    let res = mod_r.get(&id);
    if res.is_none() {
        return (Status::NotFound, "Package does not exist.");
    }

    let db = res.unwrap();

    //update the package if metadata matches
    if (package.metadata.Name == db.name)
        && (package.metadata.Version == db.ver)
        && (package.metadata.ID == db.id)
    {
        if let Ok(_) = base64_to_zip(
            package.data.Content.as_ref().unwrap().as_str(),
            db.path.as_str(),
        )
        .await
        {
            (Status::Ok, "Version is updated.")
        } else {
            (Status::NotFound, "Package does not exist.")
        }
    } else {
        (Status::NotFound, "Package does not exist.")
    }
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
// no other way to set custom headers other than this
#[derive(Responder)]
pub struct PackageListResponse {
    body: Json<Vec<PackageMetadata>>,
    offset: Header<'static>,
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
        Name: db.name.clone(),
        Version: db.ver.clone(),
        ID: db.id.clone(),
    };
    let data = PackageData {
        Content: Some(zip_to_base64(db.path.as_str()).await.unwrap()),
        URL: None, //db.url.clone(),
        JSProgram: "if (process.argv.length === 7) {
            console.log('Success')
            process.exit(0)
            } else {
            console.log('Failed')
            process.exit(1)
            }"
        .to_string(),
    };
    let response = Package { metadata, data };
    (Status::Ok, Either::Left(Json(response)))
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

#[put("/authenticate")]
pub async fn authenticate() -> (Status, &'static str) {
    (Status::NotImplemented, "Not implemented")
}

#[get("/package/byName/<name>", rank = 1)]
pub async fn package_by_name_get(
    name: String,
    mod_db: &State<ModuleDB>,
) -> Either<Json<Vec<PackageHistoryEntry>>, (Status, &'static str)> {
    let mut ret = Vec::new();
    let mod_r = mod_db.read().await;
    for v in mod_r.values() {
        if v.name == name {
            // history is not implemented, so only PackageMetadata is filled in
            ret.push(PackageHistoryEntry {
                User: User {
                    name: String::new(),
                    isAdmin: false,
                },
                Date: String::new(),
                PackageMetadata: PackageMetadata {
                    Name: v.name.clone(),
                    ID: v.id.clone(),
                    Version: v.ver.clone(),
                },
                Action: "CREATE".to_string(),
            });
        }
    }

    // 404 if no entry matches the name
    if ret.is_empty() {
        Either::Right((Status::NotFound, "Package does not exist"))
    } else {
        Either::Left(Json(ret))
    }
}

#[delete("/package/byName/<name>")]
pub async fn package_by_name_delete(name: String, mod_db: &State<ModuleDB>) -> (Status, String) {
    let mut mod_w = mod_db.write().await;

    let (del, keep) = mod_w.drain().partition(|(_, v)| v.name == name);
    *mod_w = keep;

    // release write lock early because deleting files takes time
    let _ = mod_w.downgrade();

    // remove files associated with deleted modules
    let num_deleted = del.len();
    if num_deleted == 0 {
        (Status::NotFound, "Package does not exist".to_string())
    } else {
        for (k, v) in del {
            if fs::remove_file(v.path).await.is_err() {
                println!("cannot remove file for module: {}", k);
            }
        }
        (Status::Ok, format!("{} package(s) deleted", num_deleted))
    }
}

#[post("/package/byRegex", data = "<query>")]
pub async fn package_by_regex_get(
    query: Json<PackageRegEx>,
    mod_db: &State<ModuleDB>,
) -> Either<Json<Vec<PackageMetadata>>, (Status, &'static str)> {
    let re = regex::Regex::new(&query.RegEx);
    if let Ok(re) = re {
        let mut ret = Vec::new();
        let mod_r = mod_db.read().await;
        for v in mod_r.values() {
            if re.is_match(&v.name) || match_readme(&re, &v.path).is_some() {
                ret.push(PackageMetadata {
                    Name: v.name.clone(),
                    ID: v.id.clone(),
                    Version: v.ver.clone(),
                });
            }
        }
        if ret.is_empty() {
            Either::Right((Status::NotFound, "No package found under this regex."))
        } else {
            Either::Left(Json(ret))
        }
    } else {
        Either::Right((Status::BadRequest, "malformed regex"))
    }
}
// return option so I can use ? in code
pub fn match_readme(re: &regex::Regex, path: &str) -> Option<()> {
    // zip doesn't work well with async fs
    let mut fp = zip::ZipArchive::new(std::fs::File::open(path).ok()?).ok()?;
    let mut readme = fp.by_name("README.md").ok()?;
    let mut contents = String::new();
    readme.read_to_string(&mut contents).ok()?;
    re.is_match(&contents).then_some(())
}

// reroute 422 to 400
// 422 is possible when passed in invalid query
#[catch(422)]
pub fn redirect_422_to_400() -> status::BadRequest<&'static str> {
    status::BadRequest(Some("Error processing data"))
}

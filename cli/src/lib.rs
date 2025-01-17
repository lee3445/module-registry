#![allow(unused)]

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info, warn, LevelFilter};
use std::io::{self, Write};
extern crate octocrab;

use octocrab::{
    models::{self, repos::RepoCommit},
    params, Octocrab, Page,
};

use anyhow::anyhow;

use serde_json::{json, Map, Value};

use dotenv::dotenv;
use std::env;

use regex::Regex;

use std::fmt;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod calc_bus_factor;
mod calc_responsive_maintainer;
mod correctness;
mod octo;
mod ramp_up;
mod reqw;
mod review;
mod version;

extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
mod calc_license;

#[derive(Parser)]
struct Cli {
    path: String,
}

#[derive(Debug)]
struct CustomError(String);

#[derive(Clone)]
pub struct GithubRepo {
    url: String,
    scores: Vec<f32>,
}

impl fmt::Debug for GithubRepo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Metrics {{")?;
        writeln!(f, "    URL: {}", self.url)?;
        writeln!(f, "    ramp_up: {}", self.rampup())?;
        writeln!(f, "    correctness: {}", self.correct())?;
        writeln!(f, "    bus_factor: {}", self.bus())?;
        writeln!(f, "    responsiveness: {}", self.responsive())?;
        writeln!(f, "    license: {}", self.license())?;
        writeln!(f, "    version: {}", self.version())?;
        writeln!(f, "    review: {}", self.review())?;
        writeln!(f, "}}")
    }
}

impl GithubRepo {
    fn new(url: String, scores: Vec<f32>) -> Self {
        GithubRepo { url, scores }
    }

    pub fn overall(&self) -> f32 {
        self.scores[0]
    }

    fn overall_set(&mut self, overall_score: f32) {
        self.scores[0] = overall_score;
    }

    pub fn bus(&self) -> f32 {
        self.scores[1]
    }

    fn bus_set(&mut self, bus_score: f32) {
        self.scores[1] = bus_score;
    }

    pub fn correct(&self) -> f32 {
        self.scores[2]
    }

    fn correct_set(&mut self, correct_score: f32) {
        self.scores[2] = correct_score;
    }

    pub fn license(&self) -> f32 {
        self.scores[3]
    }

    fn license_set(&mut self, license_score: f32) {
        self.scores[3] = license_score;
    }

    pub fn responsive(&self) -> f32 {
        self.scores[4]
    }

    fn responsive_set(&mut self, responsive_score: f32) {
        self.scores[4] = responsive_score;
    }

    pub fn rampup(&self) -> f32 {
        self.scores[5]
    }

    fn rampup_set(&mut self, rampup_score: f32) {
        self.scores[5] = rampup_score;
    }

    pub fn version(&self) -> f32 {
        self.scores[6]
    }

    fn version_set(&mut self, score: f32) {
        self.scores[6] = score;
    }

    pub fn review(&self) -> f32 {
        self.scores[7]
    }

    fn review_set(&mut self, score: f32) {
        self.scores[7] = score;
    }
}

pub async fn working() -> bool {
    println!(
        "{:?} {:?}",
        "bsd-3-clause".to_string(),
        calc_license::calc_licenses("bsd-3-clause".to_string()).await
    );
    true
}

pub async fn rate(url: &str, token: &str) -> Option<GithubRepo> {
    let (owner, repo) = extract_owner_and_repo(url).await?;
    println!("repo in rate:{}/{}", owner, repo);

    // calculate metrics
    let scores = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
    let mut github = GithubRepo::new(url.to_string(), scores);
    calc_metrics(&mut github, token.to_string(), owner, repo).await;
    Some(github)
}

// fn read_github_repos_from_file(filename: &str) -> Vec<GithubRepo> {
//     let file = match File::open(filename) {
//         Ok(file) => file,
//         Err(err) => {
//             println!("Error opening file: {}", err);
//             return vec![];
//         }
//     };
//
//     let reader = BufReader::new(file);
//
//     let mut repos = vec![];
//     for line in reader.lines() {
//         let line = line.unwrap();
//         let scores = vec![-1.0, -1.0, -1.0, -1.0, -1.0, -1.0];
//         let repo = GithubRepo::new(line, scores);
//         repos.push(repo);
//     }
//
//     repos
// }
pub async fn extract_owner_and_repo(url: &str) -> Option<(String, String)> {
    let u = reqwest::Url::parse(url).ok()?;
    let sch = u.scheme();
    if sch != "https" {
        return None;
    }

    // get github url
    let ghurl: String;
    if let Some(domain) = u.domain() {
        if domain == "www.npmjs.com" {
            // handle npm URLs
            let npm_url = url.to_string().replace(
                "https://www.npmjs.com/package/",
                "https://registry.npmjs.org/",
            );

            let input = reqwest::get(npm_url).await.ok()?.text().await.ok()?;

            // parse url into generic JSON value
            let root: Value = serde_json::from_str(&input).ok()?;

            // access element using .get()
            let giturl: &str = root
                .get("repository")
                .and_then(|value| value.get("url"))
                .and_then(|value| value.as_str())?;

            // // force scheme to https
            // let spl = giturl.split_once("://")?;
            // let giturl = "https://".to_owned() + spl.1;

            // // Do not need to check if url contains git+, just do replace. That would take care of it
            // let derefurl = giturl.replace(".git", "");
            // ghurl = derefurl;
            ghurl = giturl.to_string();
        } else if domain != "github.com" {
            return None;
        } else {
            ghurl = url.to_string();
        }
    } else {
        return None;
    }
    let re = Regex::new(r".*github.com/([^/]+)/([^/^\.]+)(\.git)?").unwrap();
    let captures = re.captures(&ghurl)?;

    Some((captures[1].to_string(), captures[2].to_string()))
}

pub async fn wget(url: &str, path: &str) -> Option<()> {
    let ret = reqwest::get(url).await.ok()?;
    if !ret.status().is_success() {
        return None;
    }
    let mut wrt = tokio::fs::File::create(path).await.ok()?;
    tokio::io::copy(&mut ret.bytes().await.ok()?.as_ref(), &mut wrt)
        .await
        .ok()
        .and_then(|_| Some(()))
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     env_logger::init();
//     dotenv().ok();
//     let args = Cli::parse();
//     let stdout = io::stdout();
//     let mut handle_lock = stdout.lock();
//     let token: String = env::var("GITHUB_TOKEN")
//         .expect("GITHUB_TOKEN env variable is required")
//         .into();
//     let mut repos_list = read_github_repos_from_file(&args.path);
//     //println!("{}", repos_list);
//     let repo_info = extract_owner_and_repo(repos_list.first().unwrap().url.as_str());
//     let owner = repo_info.clone().unwrap().0;
//     let repo_name = repo_info.clone().unwrap().1;
//
//     let repo = octo::get_repo(token.clone(), owner.clone(), repo_name.clone()).await;
//
//     for repository in &mut repos_list {
//         calc_metrics(repository, token.clone(), owner.clone(), repo_name.clone()).await;
//         //sort_repositories(repos_list.as_mut());
//         create_ndjson(
//             repository.url.as_str(),
//             repository.overall(),
//             repository.rampup(),
//             repository.correct(),
//             repository.bus(),
//             repository.responsive(),
//             repository.license(),
//         );
//     }
//
//     Ok(())
// }

async fn calc_metrics(repository: &mut GithubRepo, token: String, owner: String, repo: String) {
    // calculate responsiveness
    let mut issue_response_times =
        octo::get_issue_response_times(token.clone(), owner.clone(), repo.clone())
            .await
            .unwrap_or(vec![1.0, 1.0]);
    let mut responsive_score = calc_responsive_maintainer::calc_responsive_maintainer(
        issue_response_times[0],
        issue_response_times[1],
    ) as f32;
    repository.responsive_set(responsive_score);

    // calculate license compatibility
    let mut resp =
        octo::get_license(token.clone(), owner.clone().as_str(), repo.clone().as_str()).await;
    let data_layer = resp.get_mut("data").expect("Data key not found");
    let repository_layer = data_layer
        .get_mut("repository")
        .expect("Repository key not found");
    let license_layer = repository_layer
        .get_mut("licenseInfo")
        .expect("License key not found");
    let mut license_score = 0.0;
    if license_layer.get("key").is_some() {
        license_score = calc_license::calc_licenses(
            license_layer
                .get("key")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
        )
        .await as f32;
    }
    repository.license_set(license_score);

    // calculate ramp up time
    let octo = Octocrab::builder()
        .personal_token(token.clone())
        .build()
        .unwrap();

    let mut ramp_up_score = ramp_up::get_weighted_score(octo.clone(), owner.clone(), repo.clone())
        .await
        .unwrap();
    repository.rampup_set(ramp_up_score as f32);

    // calculate correctness
    let mut correctness_score =
        correctness::get_weighted_score(token.clone(), owner.clone(), repo.clone())
            .await
            .unwrap();
    repository.correct_set(correctness_score as f32);

    let client = reqw::client(&token).unwrap();

    // calculate bus factor
    let bus = reqw::graph_json(&client,
            format!("{{\"query\" : \"query {{ repository(owner:\\\"{}\\\", name:\\\"{}\\\") {{ mentionableUsers {{ totalCount }} }} }}\" }}", owner, repo)
            ).await.unwrap();
    let collaborators = bus["data"]["repository"]["mentionableUsers"]["totalCount"]
        .as_i64()
        .unwrap();
    let bus_factor_score: f32 = ((2.0 * collaborators as f32) / (collaborators as f32 + 1.0)) - 1.0;
    repository.bus_set(bus_factor_score);

    // calculate version pinning
    let ver = version::calc_version(&client, &owner, &repo).await;
    repository.version_set(ver);

    // calculate code review percentage
    let rev = review::calc_review(&client, &owner, &repo).await;
    repository.review_set(rev);

    repository.overall_set(
        license_score as f32
            * (correctness_score as f32 * 0.5
                + ramp_up_score as f32 * 0.2
                + responsive_score * 0.2
                + bus_factor_score * 0.1),
    );
}

fn create_ndjson(
    url: &str,
    net_score: f32,
    ramp_up_score: f32,
    correctness_score: f32,
    bus_factor_score: f32,
    responsive_maintainer_score: f32,
    license_score: f32,
) {
    let json = json!({
        "URL": url,
        "NET_SCORE": format!("{:.1}", net_score),
        "RAMP_UP_SCORE": format!("{:.1}", ramp_up_score),
        "CORRECTNESS_SCORE": format!("{:.1}", correctness_score),
        "BUS_FACTOR_SCORE": format!("{:.1}", bus_factor_score),
        "RESPONSIVE_MAINTAINER_SCORE": format!("{:.1}", responsive_maintainer_score),
        "LICENSE_SCORE": format!("{:.1}", license_score),
    });
    let ndjson = json.to_string();
    println!("{}", ndjson);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::calc_responsive_maintainer::calc_responsive_maintainer;
//     use std::fs::File;
//     use std::io::prelude::*;
//     use tempfile::tempdir;
//
//     #[test]
//     fn test_calc_responsive_maintainer() {
//         let owner = "cloudinary";
//         let repo_name = "cloudinary_npm";
//         let expected_output = 0.0;
//         let token: String = std::env::var("GITHUB_TOKEN")
//             .expect("GITHUB_TOKEN env variable is required")
//             .into();
//
//         let result = calc_responsive_maintainer::calc_responsive_maintainer(0.0, 0.0);
//         assert_eq!(result, expected_output);
//     }
//
//     #[test]
//     fn test_read_github_repos_from_file() {
//         let temp_dir = tempdir().unwrap();
//         let test_file_path = temp_dir.path().join("test_file.txt");
//         let mut file = File::create(&test_file_path).unwrap();
//         file.write_all(b"https://github.com/lodash/lodash\nhttps://github.com/nullivex/nodist\nhttps://www.npmjs.com/package/browserify").unwrap();
//
//         let repos = read_github_repos_from_file(test_file_path.to_str().unwrap());
//         assert_eq!(repos.len(), 3);
//         assert_eq!(
//             repos.get(0).unwrap().url,
//             String::from("https://github.com/lodash/lodash")
//         );
//         assert_eq!(
//             repos.get(1).unwrap().url,
//             String::from("https://github.com/nullivex/nodist")
//         );
//         assert_eq!(
//             repos.get(2).unwrap().url,
//             String::from("https://www.npmjs.com/package/browserify")
//         );
//     }
// }

// fn sort_repositories(repositories: &mut Vec<GithubRepo>) {
//     repositories.sort_by(|a, b| {
//         let overall_cmp = b.overall().cmp(&a.overall());
//         if overall_cmp == Ordering::Equal {
//             let bus_cmp = b.bus().cmp(&a.bus());
//             if bus_cmp == Ordering::Equal {
//                 a.license().cmp(&b.license())
//             } else {
//                 bus_cmp
//             }
//         } else {
//             overall_cmp
//         }
//     });
// }

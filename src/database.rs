use rocket::tokio::sync::RwLock;
use std::{collections::HashMap, path::PathBuf};

pub type ModuleDB = RwLock<HashMap<String, Module>>;

// create empty ModuleDB
// pub fn module_db() -> ModuleDB {
//     RwLock::new(HashMap::new())
// }
//
// this one has a default entry
pub async fn module_db() -> ModuleDB {
    let mut hm = HashMap::new();
    hm.insert(
        "postcss".to_string(),
        Module::new(
            "postcss".to_string(),
            "https://www.npmjs.com/package/postcss".to_string(),
        )
        .await
        .unwrap(),
    );
    RwLock::new(hm)
}

#[derive(Default, Debug)]
pub struct Module {
    pub name: String,
    // id of module
    pub id: String,
    pub ver: String,
    // url to webpage for module
    pub url: String,
    // file storing contents of module
    pub path: PathBuf,

    // module scores
    pub overall: f64,
    pub bus: f64,
    pub correct: f64,
    pub license: f64,
    pub responsive: f64,
    pub rampup: f64,
    pub version: f64,
    pub review: f64,
}

impl Module {
    // initialize struct
    // TODO: add path
    async fn new(id: String, url: String) -> Option<Self> {
        let scores = cli::rate(&url, env!("GITHUB_TOKEN")).await?;
        Some(Self {
            id,
            url,

            overall: scores.overall() as f64,
            bus: scores.bus() as f64,
            correct: scores.correct() as f64,
            license: scores.license() as f64,
            responsive: scores.responsive() as f64,
            rampup: scores.rampup() as f64,
            version: scores.version() as f64,
            review: scores.review() as f64,

            ..Default::default()
        })
    }
}

pub fn get_by_name<'a>(map: &'a HashMap<String, Module>, name: &str) -> Option<&'a Module> {
    for (k, v) in map {
        if v.name == name {
            return Some(v);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // test cli crate
    #[rocket::async_test]
    async fn rate_not_url() {
        assert!(cli::rate("not/a/url", env!("GITHUB_TOKEN")).await.is_none());
    }

    #[rocket::async_test]
    async fn rate_bad_url() {
        assert!(cli::rate("https://osu.ppy.sh", env!("GITHUB_TOKEN"))
            .await
            .is_none());
    }

    #[rocket::async_test]
    async fn rate_github() {
        assert!(
            cli::rate("https://github.com/postcss/postcss", env!("GITHUB_TOKEN"))
                .await
                .is_some()
        );
    }

    #[rocket::async_test]
    async fn rate_npm_1() {
        assert!(cli::rate(
            "https://www.npmjs.com/package/postcss",
            env!("GITHUB_TOKEN")
        )
        .await
        .is_some());
    }

    #[rocket::async_test]
    async fn rate_npm_2() {
        assert!(cli::rate(
            "https://www.npmjs.com/package/minimist",
            env!("GITHUB_TOKEN")
        )
        .await
        .is_some());
    }

    // test Module
    #[rocket::async_test]
    async fn module_new() {
        let res = Module::new(
            "postcss".to_string(),
            "https://www.npmjs.com/package/postcss".to_string(),
        )
        .await
        .unwrap();

        assert_eq!(res.id, "postcss");
        assert!(res.responsive >= 0.0 && res.responsive <= 1.0);
    }

    #[rocket::async_test]
    async fn module_new_bad() {
        let res = Module::new("no".to_string(), "not a url".to_string()).await;

        assert!(res.is_none());
    }

    // test ModuleDB
    #[rocket::async_test]
    async fn module_db() {
        let mdb: ModuleDB = RwLock::new(HashMap::new());
        mdb.write()
            .await
            .insert("1".to_string(), Default::default());

        // can have multiple readers
        let r1 = mdb.read().await;
        let r2 = mdb.read().await;
        assert!(r1.get("1").is_some());
        assert!(r2.get("2").is_none());
        assert!(r1.get("3").is_none());
    }
}
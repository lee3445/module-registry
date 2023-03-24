use rocket::tokio::sync::RwLock;
use std::{collections::HashMap, path::PathBuf};

pub type ModuleDB = RwLock<HashMap<String, Module>>;

// create empty ModuleDB
// pub fn module_db() -> ModuleDB {
//     RwLock::new(HashMap::new())
// }
pub fn module_db() -> ModuleDB {
    let mut hm = HashMap::new();
    hm.insert("minimist".to_string(), Module::new("minimist".to_string(), "https://www.npmjs.com/package/minimist".to_string()));
    RwLock::new(hm)
}

#[derive(Default, Debug)]
pub struct Module {
    pub id: String,
    pub url: String,
    pub path: PathBuf,

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
    // initialize struct, should calculate score with url
    fn new(id: String, url: String) -> Self {
        Self {
            id,
            url,
            ..Default::default()
        }
    }
}

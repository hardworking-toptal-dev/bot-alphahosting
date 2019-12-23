extern crate serde;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub alfahosting: AlfahostingConfig,
    pub acme: AcmeConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlfahostingConfig {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcmeConfig {
    #[serde(default)]
    pub directory_url: DirectoryUrl,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryUrl(pub String);

impl Default for DirectoryUrl {
    #[cfg(not(production))]
    fn default() -> Self {
        DirectoryUrl("https://acme-staging-v02.api.letsencrypt.org/directory".to_string())
    }

    #[cfg(production)]
    fn default() -> Self {
        DirectoryUrl("https://acme-v02.api.letsencrypt.org/directory".to_string())
    }
}




extern crate serde;
extern crate toml;

use serde::{Deserialize, Serialize};
use toml::value::Table;

pub fn load_config(path: &str) -> Result<Configuration, Box<dyn std::error::Error>> {
    let config_string: String = std::fs::read_to_string(path)?;
    let config: Configuration = toml::from_str(config_string.as_ref()).unwrap();
    Ok(config)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(default)]
    pub certpath: Certpath,
    pub alfahosting: AlfahostingConfig,
    pub imap: ImapConfig,
    pub acme: AcmeConfig,
    pub domains: Option<Table>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Certpath(pub String);

impl Default for Certpath {
    #[cfg(not(build = "release"))]
    fn default() -> Self {
        Certpath("./tryout/letsencrypt".to_string())
    }

    #[cfg(build = "release")]
    fn default() -> Self {
        Certpath("/etc/letsencrypt".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlfahostingConfig {
    pub username: String,
    pub password: String,
    pub ipid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImapConfig {
    pub domain: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AcmeConfig {
    #[serde(default)]
    pub directory_url: DirectoryUrl,
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryUrl(pub String);

impl Default for DirectoryUrl {
    #[cfg(not(build = "release"))]
    fn default() -> Self {
        DirectoryUrl("https://acme-staging-v02.api.letsencrypt.org/directory".to_string())
    }

    #[cfg(build = "release")]
    fn default() -> Self {
        DirectoryUrl("https://acme-v02.api.letsencrypt.org/directory".to_string())
    }
}

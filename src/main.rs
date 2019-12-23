extern crate toml;

mod config;
mod certbot;

use crate::config::{Configuration, AlfahostingConfig, AcmeConfig, DirectoryUrl};

fn main() {
    let config = Configuration {
        alfahosting: AlfahostingConfig {
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        },
        acme: AcmeConfig {
            directory_url: DirectoryUrl(
                "https://acme-staging-v02.api.letsencrypt.org/directory".to_string()
            ),
        },
    };

    // Convert the Configuration to a TOML string.
    let serialized = toml::to_string(&config).unwrap();

    println!("{}", serialized);
}

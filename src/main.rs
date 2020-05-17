extern crate headless_chrome;
extern crate rand;
extern crate toml;

mod browse;
mod certbot;
mod config;
mod imap;

use crate::browse::{
    browse_alfahosting_check_login_protection, browse_alfahosting_dns, browse_alfahosting_signin,
    browse_alfahosting_solve_login_protection,
};
use crate::certbot::request_cert;
use crate::config::{
    load_config, AcmeConfig, AlfahostingConfig, Certpath, Configuration, DirectoryUrl, ImapConfig,
};
use crate::imap::get_code_from_inbox;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use toml::value::{Table, Value};

#[cfg(not(build = "release"))]
const CONFIG_FILE_PATH: &str = "./tryout/config.toml";

#[cfg(build = "release")]
const CONFIG_FILE_PATH: &str = "/etc/certbot-alfahosting/config.toml";

fn main() {
    // test_config();
    println!("Starting certbot-alfahosting...");
    let config: Configuration = load_config(CONFIG_FILE_PATH).unwrap();
    if let Some(domains) = config.domains {
        let browser = configure_browser();
        println!("Browser has been configured.");
        let tab = browser.wait_for_initial_tab().unwrap();
        println!("Successfully opened initial tab.");
        browse_alfahosting_signin(&tab, &config.alfahosting);
        if browse_alfahosting_check_login_protection(&tab) {
            std::thread::sleep(std::time::Duration::from_millis(
                15_000 + (rand::random::<u64>() % 15_000),
            ));
            let code: String = match get_code_from_inbox(&config.imap) {
                Ok(code) => code,
                Err(err) => panic!(
                    "An error occured while trying to authenticate at IMAP server: {:?}",
                    err
                ),
            };
            browse_alfahosting_solve_login_protection(&tab, code);
        }
        println!("Successfully logged into alfahosting account.");
        browse_alfahosting_dns(&tab, &config.alfahosting);
        println!("Alfahosting DNS view successfully initialized.");
        for (names, value_alfahosting_id) in domains {
            match value_alfahosting_id {
                toml::Value::Array(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::Boolean(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::Datetime(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::Float(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::Integer(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::Table(_v) => {
                    println!("Invalid type for domains \"{}\". Must be a string.", names)
                }
                toml::Value::String(alfahosting_id) => {
                    if let Err(e) = request_cert(
                        &config.certpath,
                        &config.acme,
                        names.as_str(),
                        alfahosting_id.as_str(),
                        &tab,
                    ) {
                        println!(
                            "An error occured while getting certificate for \"{}\": {}",
                            names, e
                        );
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(30_000));
        }
    }
}

fn configure_browser() -> Browser {
    // let browser: Browser = Browser::default().unwrap();
    let mut lob = LaunchOptionsBuilder::default();
    lob.idle_browser_timeout(std::time::Duration::from_millis(300_000));
    lob.headless(true);
    lob.sandbox(false);
    lob.port(Some(8467));
    match Browser::new(lob.build().unwrap()) {
        Ok(browser) => browser,
        Err(err) => panic!("{}", err.as_fail()),
    }
}

#[allow(dead_code)]
fn test_config() {
    let mut domains = Table::new();
    domains.insert(
        String::from("example.com"),
        Value::String(String::from("123456")),
    );
    let config = Configuration {
        certpath: Certpath(String::from(".")),
        alfahosting: AlfahostingConfig {
            username: String::from("testuser"),
            password: String::from("testpass"),
            ipid: String::from("123456"),
        },
        imap: ImapConfig {
            domain: String::from("imap.example.com"),
            port: 993,
            username: String::from("username"),
            password: String::from("password"),
        },
        acme: AcmeConfig {
            directory_url: DirectoryUrl(String::from(
                "https://acme-staging-v02.api.letsencrypt.org/directory",
            )),
            account: String::from("user@example.com"),
        },
        domains: Some(domains),
    };

    println!("{:?}", config);

    // Convert the Configuration to a TOML string.
    let serialized = toml::to_string(&config).unwrap();

    println!("{}", serialized);
}

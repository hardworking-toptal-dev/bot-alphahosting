extern crate headless_chrome;
extern crate rand;
extern crate toml;

mod browse;
mod certbot;
mod config;
mod imap;

use crate::browse::{
    browse_alfahosting_check_login_protection, browse_alfahosting_dns,
    browse_alfahosting_domain_create_acme, browse_alfahosting_domain_delete_last_dns_entry,
    browse_alfahosting_signin, browse_alfahosting_solve_login_protection,
};
use crate::config::{
    load_config, AcmeConfig, AlfahostingConfig, Certpath, Configuration, DirectoryUrl, ImapConfig,
};
use crate::imap::get_code_from_inbox;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use toml::value::{Table, Value};

#[cfg(not(production))]
const CONFIG_FILE_PATH: &str = "./tryout/certbot-alfahosting.conf";

#[cfg(production)]
const CONFIG_FILE_PATH: &str = "/etc/certbot-alfahosting.conf";

fn main() {
    // test_config();
    let config: Configuration = load_config(CONFIG_FILE_PATH).unwrap();
    let browser = configure_browser();
    let tab = browser.wait_for_initial_tab().unwrap();
    browse_alfahosting_signin(&tab, &config.alfahosting);
    if browse_alfahosting_check_login_protection(&tab) {
        std::thread::sleep(std::time::Duration::from_millis(
            15_000 + (rand::random::<u64>() % 15_000),
        ));
        let code: String = get_code_from_inbox(&config.imap).unwrap();
        browse_alfahosting_solve_login_protection(&tab, code);
    }
    browse_alfahosting_dns(&tab, &config.alfahosting);
    browse_alfahosting_domain_create_acme(
        &tab,
        String::from("315259"),
        String::from("novitaslaboratories.com"),
        String::from("this is just another test"),
    );
    browse_alfahosting_domain_delete_last_dns_entry(&tab, String::from("315259"));
    std::thread::sleep(std::time::Duration::from_millis(10_000));
}

fn configure_browser() -> Browser {
    // let browser: Browser = Browser::default().unwrap();
    let mut lob = LaunchOptionsBuilder::default();
    lob.idle_browser_timeout(std::time::Duration::from_millis(120_000));
    lob.headless(false);
    let browser: Browser = Browser::new(lob.build().unwrap()).unwrap();
    browser
}

#[allow(dead_code)]
fn test_config() {
    let mut domains = Table::new();
    domains.insert(
        String::from("novitaslaboratories.com"),
        Value::String(String::from("315259")),
    );
    domains.insert(
        String::from("novitas-labs.com"),
        Value::String(String::from("315260")),
    );
    domains.insert(
        String::from("novitaslabs.com"),
        Value::String(String::from("315261")),
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
            account: String::from("appdev.lukaswagner@gmail.com"),
        },
        domains: Some(domains),
    };

    println!("{:?}", config);

    // Convert the Configuration to a TOML string.
    let serialized = toml::to_string(&config).unwrap();

    println!("{}", serialized);
}

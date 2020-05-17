extern crate rand;

use crate::config::AlfahostingConfig;

use headless_chrome::browser::tab::Tab;
use std::sync::Arc;

pub fn browse_alfahosting_signin(tab: &Arc<Tab>, config: &AlfahostingConfig) {
    tab.navigate_to("https://alfahosting.de/kunden-login/")
        .unwrap();
    tab.wait_until_navigated().unwrap();
    let accept_cookie_button = tab
        .find_element("[data-cookiefirst-button=\"primary\"]")
        .unwrap();
    accept_cookie_button.click().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        5_000 + (rand::random::<u64>() % 3_000),
    ));
    let form_login = tab.find_element("#loginForm").unwrap();
    let input_username = tab.find_element("#username").unwrap();
    let input_password = tab.find_element("#password").unwrap();
    let submit_button = tab
        .find_element("#loginForm input[type=\"submit\"]")
        .unwrap();
    tab.press_key("PageDown").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(500));
    form_login.scroll_into_view().unwrap();
    input_username.type_into(config.username.as_ref()).unwrap();
    input_password.type_into(config.password.as_ref()).unwrap();
    submit_button.click().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        5_000 + (rand::random::<u64>() % 5_000),
    ));
    if browse_alfahosting_is_login_error(&tab) {
        panic!("The provided username or password was not correct.");
    }
}

pub fn browse_alfahosting_check_login_protection(tab: &Arc<Tab>) -> bool {
    match tab.find_element("#loginProtectionForm") {
        Ok(_f) => true,
        Err(_e) => false,
    }
}

pub fn browse_alfahosting_solve_login_protection(tab: &Arc<Tab>, protection_code: String) {
    let form_protection = tab.find_element("#loginProtectionForm").unwrap();
    let input_code = tab.find_element("#loginProtectionForm #code").unwrap();
    let submit_button = tab
        .find_element("#loginProtectionForm input[type=\"submit\"]")
        .unwrap();
    tab.press_key("PageDown").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
    form_protection.scroll_into_view().unwrap();
    input_code.type_into(protection_code.as_ref()).unwrap();
    submit_button.click().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        5_000 + (rand::random::<u64>() % 5_000),
    ));
    if browse_alfahosting_is_login_error(&tab) {
        panic!("The provided code was incorrect.");
    }
}

fn browse_alfahosting_is_login_error(tab: &Arc<Tab>) -> bool {
    match tab.find_element("#login.errorSection") {
        Ok(_f) => true,
        Err(_e) => false,
    }
}

pub fn browse_alfahosting_dns(tab: &Arc<Tab>, config: &AlfahostingConfig) {
    tab.navigate_to(
        (String::from("https://secure.alfahosting.de/kunden/index.php/Kundencenter:Tarife?ipid=")
            + config.ipid.as_ref())
        .as_ref(),
    )
    .unwrap();
    tab.wait_until_navigated().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        2_000 + (rand::random::<u64>() % 3_000),
    ));
    tab.navigate_to(
        "https://secure.alfahosting.de/kunden/index.php/Kundencenter:Tarife/Details#dns",
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        2_000 + (rand::random::<u64>() % 3_000),
    ));
    let button_continue = tab.find_element("#dns_layer a").unwrap();
    button_continue.click().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        2_000 + (rand::random::<u64>() % 3_000),
    ));
}

pub fn browse_alfahosting_domain_create_acme(
    tab: &Arc<Tab>,
    id: String,
    domain: String,
    text: String,
) {
    let body = tab.find_element("body").unwrap();
    dns_open_domain_and_scroll_into_view(tab, &id);
    body.call_js_fn(
        format!(
            "function () {{document.fire('dns:add_zonelist', {});return false;}}",
            id
        )
        .as_str(),
        false,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
    let input_zonedata = tab.find_element("#zonelist_pattern").unwrap();
    // _acme-challenge.novitaslaboratories.com. IN TXT "this is just a manual test"
    input_zonedata
        .type_into(format!("_acme-challenge.{}. IN TXT \"{}\"", domain, text).as_str())
        .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
    body.call_js_fn(
        "function () {document.fire('dns:exec_zonelist_add');return false;}",
        false,
    )
    .unwrap();
    overwrite_window_popups(tab);
    dns_save(tab, &id);
    dns_close_domain(tab, &id);
}

pub fn browse_alfahosting_domain_delete_last_dns_entry(tab: &Arc<Tab>, id: String) {
    dns_open_domain_and_scroll_into_view(tab, &id);
    overwrite_window_popups(tab);
    let last_entry_id = tab
        .find_elements(format!("#dns_entries_{} .dns_entry", id).as_str())
        .unwrap()
        .iter()
        .fold(String::new(), |current, elem| {
            if let Some(attributes) = elem.get_attributes().unwrap() {
                if let Some(new_id) = attributes.get("id") {
                    return new_id.clone();
                }
            }
            current
        });
    let button_delete = tab
        .find_element(format!("#{} .dns_entry_action", last_entry_id).as_str())
        .unwrap();
    button_delete.click().unwrap();
    dns_save(tab, &id);
    dns_close_domain(tab, &id);
}

fn dns_open_domain_and_scroll_into_view(tab: &Arc<Tab>, id: &str) {
    let header = tab.find_element(format!("#hdr{}", id).as_str()).unwrap();
    tab.press_key("PageDown").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
    header.scroll_into_view().unwrap();
    header.click().unwrap();
    tab.press_key("PageDown").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
    header.scroll_into_view().unwrap();
}

fn dns_close_domain(tab: &Arc<Tab>, id: &str) {
    let header = tab.find_element(format!("#hdr{}", id).as_str()).unwrap();
    header.click().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 500),
    ));
}

fn overwrite_window_popups(tab: &Arc<Tab>) {
    let body = tab.find_element("body").unwrap();
    body.call_js_fn(
        "function () {window.confirm = function myConfirm() {return true;}}",
        false,
    )
    .unwrap();
    body.call_js_fn(
        "function () {window.alert = function myAlert() {return true;}}",
        false,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        500 + (rand::random::<u64>() % 176),
    ));
}

fn dns_save(tab: &Arc<Tab>, id: &str) {
    let body = tab.find_element("body").unwrap();
    body.call_js_fn(
        format!(
            "function () {{document.fire('dns:save_records',{});return false;}}",
            id
        )
        .as_str(),
        false,
    )
    .unwrap();
    std::thread::sleep(std::time::Duration::from_millis(
        2_000 + (rand::random::<u64>() % 3_000),
    ));
}

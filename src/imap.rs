extern crate imap;
extern crate native_tls;
extern crate regex;

use crate::config::ImapConfig;
use regex::Regex;
use std::error::Error;

pub fn get_code_from_inbox(config: &ImapConfig) -> Result<String, Box<dyn Error>> {
    let domain: &str = config.domain.as_ref();
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, config.port), domain, &tls).unwrap();
    let mut imap_session = client
        .login(config.username.to_string(), config.password.to_string())
        .map_err(|e| e.0)?;
    // imap_session.select("INBOX").unwrap();
    let inbox = imap_session.examine("INBOX").unwrap();
    let highest_uid = inbox.exists;
    for i in 0..20 {
        let uid = highest_uid - i;
        let messages = imap_session.fetch(uid.to_string(), "(ENVELOPE RFC822.TEXT)")?;
        for message in messages.iter() {
            if let Some(s) = handle_message(message) {
                return Ok(s);
            }
        }
    }
    imap_session.logout()?;
    Ok(String::from("test"))
}

fn handle_message(message: &imap::types::Fetch) -> Option<String> {
    let envelope = message.envelope().unwrap();
    if envelope.subject.unwrap() == "=?UTF-8?Q?Alfahosting_-_Anmeldung_von_einem_neuen_Ger=C3=A4t?="
    {
        let body = std::str::from_utf8(message.text().unwrap())
            .unwrap()
            .to_string();
        // println!(
        //     "Subject is: {}\nSender is: {:?}\nBody is: {:?}",
        //     envelope.subject.unwrap(),
        //     envelope.sender,
        //     body,
        // );
        let re = Regex::new(r"\s{2}(?P<code>\d{6})\s{2}").unwrap();
        let caps = re.captures(body.as_ref()).unwrap();
        return Some(String::from(&caps["code"]));
    }
    None
}

#[derive(Debug)]
struct ImapError(String);

impl std::fmt::Display for ImapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ImapError {
    fn description(&self) -> &str {
        self.0.as_ref()
    }
}

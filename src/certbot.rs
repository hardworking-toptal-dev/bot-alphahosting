extern crate acme_lib;

use crate::browse::{
    browse_alfahosting_domain_create_acme, browse_alfahosting_domain_delete_last_dns_entry,
};
use crate::config::{AcmeConfig, Certpath};
use acme_lib::create_p384_key;
use acme_lib::order::{CsrOrder, NewOrder};
use acme_lib::persist::FilePersist;
use acme_lib::{Certificate, Directory, DirectoryUrl, Error};
use headless_chrome::browser::tab::Tab;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::sync::Arc;

pub fn request_cert(
    certpath: &Certpath,
    config: &AcmeConfig,
    names: &str,
    alfahosting_id: &str,
    tab: &Arc<Tab>,
) -> Result<(), Error> {
    let names_arr: Vec<&str> = names.split(' ').collect();
    let (primary, alt) = names_arr.split_first().unwrap();
    let url = DirectoryUrl::Other(config.directory_url.0.as_str());

    #[cfg(not(build = "release"))]
    let persist_path = String::from(certpath.0.as_str())
        + "/accounts/acme-staging-v02.api.letsencrypt.org/directory";
    #[cfg(build = "release")]
    let persist_path =
        String::from(certpath.0.as_str()) + "/accounts/acme-v01.api.letsencrypt.org/directory";
    create_dir_all(persist_path.as_str()).unwrap();
    // Save/load keys and certificates to current dir.
    let persist = FilePersist::new(persist_path.as_str());

    // Create a directory entrypoint.
    let dir = Directory::from_url(persist, url)?;

    let acc = dir.account(config.account.as_ref())?;

    if let Some(cert) = acc.certificate(primary)? {
        if cert.valid_days_left() > 30 {
            println!("Certificate \"{}\" does not need to be renewed.", primary);
            return Ok(());
        }
    }

    let ord_new = acc.new_order(primary, &alt)?;

    let ord_csr = do_challenges(names, tab, alfahosting_id, ord_new)?;
    let (pkey_pri, pkey_pub) = create_p384_key();
    let ord_cert = ord_csr.finalize_pkey(pkey_pri, pkey_pub, 5000)?;
    let cert = ord_cert.download_and_save_cert()?;

    // TODO archive old certs

    save_cert_to_live(certpath.0.as_str(), primary, &cert);

    println!("Certificate \"{}\" has been successfully renewed.", primary);
    Ok(())
}

fn do_challenges(
    names: &str,
    tab: &Arc<Tab>,
    alfahosting_id: &str,
    mut ord_new: NewOrder<FilePersist>,
) -> Result<CsrOrder<FilePersist>, Error> {
    loop {
        // are we done?
        if let Some(ord_csr) = ord_new.confirm_validations() {
            break Ok(ord_csr);
        }

        // Get the possible authorizations
        for auth in ord_new.authorizations()? {
            if auth.need_challenge() {
                let mut retries = 5;
                while retries != 0 {
                    let chall = auth.dns_challenge();
                    browse_alfahosting_domain_create_acme(
                        tab,
                        String::from(alfahosting_id),
                        String::from(auth.domain_name()),
                        chall.dns_proof(),
                    );
                    std::thread::sleep(std::time::Duration::from_millis(5_000));
                    let result_chall = chall.validate(0);
                    match result_chall.as_ref() {
                        Ok(_v) => {
                            browse_alfahosting_domain_delete_last_dns_entry(
                                tab,
                                String::from(alfahosting_id),
                            );
                            break;
                        }
                        Err(error) => {
                            browse_alfahosting_domain_delete_last_dns_entry(
                                tab,
                                String::from(alfahosting_id),
                            );
                            retries -= 1;
                            match error {
                                Error::ApiProblem(e) => println!("An api error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                                Error::Base64Decode(e) => println!("A base64 error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                                Error::Call(e) => println!("A call error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                                Error::Io(e) => println!("An I/O error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                                Error::Json(e) => println!("A json error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                                Error::Other(e) => println!("An undefined error occured while trying to validate dns challenge for \"{}\" (retrying {} times): {}", names, retries, e),
                            }
                            if retries == 0 {
                                if let Err(e) = result_chall {
                                    return Err(e);
                                }
                            }
                        }
                    }
                }
            }
        }
        ord_new.refresh()?;
    }
}

fn save_cert_to_live(certpath: &str, primary: &str, cert: &Certificate) {
    let save_path = String::from(certpath) + format!("/live/{}", primary).as_str();
    create_dir_all(save_path.as_str()).unwrap();
    let mut file_fullchain = File::create(String::from(&save_path) + "/fullchain.pem").unwrap();
    file_fullchain
        .write_all(cert.certificate().as_bytes())
        .unwrap();
    let mut file_privkey = File::create(String::from(&save_path) + "/privkey.pem").unwrap();
    file_privkey
        .write_all(cert.private_key().as_bytes())
        .unwrap();
    let fullchain_arr: Vec<&str> = cert.certificate().split("\n\n").collect();
    let (singlecert, chain) = fullchain_arr.split_first().unwrap();
    let mut file_cert = File::create(String::from(&save_path) + "/cert.pem").unwrap();
    file_cert.write_all(singlecert.as_bytes()).unwrap();
    let mut file_chain = File::create(String::from(&save_path) + "/chain.pem").unwrap();
    file_chain.write_all(chain.join("\n\n").as_bytes()).unwrap();
}

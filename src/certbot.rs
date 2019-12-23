extern crate acme_lib;

use acme_lib::{Error, Directory, DirectoryUrl};
use acme_lib::persist::FilePersist;
use acme_lib::create_p384_key;

use crate::config::AcmeConfig;

#[cfg(production)]
const DEFAULT_ACME_DIRECTORY_URL: &str = "https://acme-v02.api.letsencrypt.org/directory";
#[cfg(not(production))]
const DEFAULT_ACME_DIRECTORY_URL: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";

pub fn request_cert(config: AcmeConfig) -> Result<(), Error>{
    Result::Ok(())
}

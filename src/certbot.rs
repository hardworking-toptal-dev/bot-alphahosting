extern crate acme_lib;

// use acme_lib::create_p384_key;
// use acme_lib::persist::FilePersist;
// use acme_lib::{Directory, DirectoryUrl, Error};

// use crate::config::{AcmeConfig, Certpath};

// pub fn request_cert(certpath: Certpath, config: AcmeConfig) -> Result<(), Error> {
//     let url = DirectoryUrl::Other(config.directory_url.0.as_ref());

//     // Save/load keys and certificates to current dir.
//     let persist = FilePersist::new(certpath.0);

//     // Create a directory entrypoint.
//     let dir = Directory::from_url(persist, url)?;

//     let acc = dir.account(config.account.as_ref())?;

//     Result::Ok(())
// }

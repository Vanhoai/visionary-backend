use openssl::{pkey::PKey, rsa::Rsa};
use std::{fs::File, io::Write};

pub struct RsaAlgorithms {}

impl RsaAlgorithms {
    pub fn new() -> Self {
        RsaAlgorithms {}
    }

    pub fn generate_keypair(
        &self,
        key_size: u32,
        private_key_path: &str,
        public_key_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rsa = Rsa::generate(key_size)?;

        let keypair = PKey::from_rsa(rsa)?;

        let private_key_pem = keypair.private_key_to_pem_pkcs8()?;
        let mut private_key_file = File::create(private_key_path)?;
        private_key_file.write_all(&private_key_pem)?;

        let public_key_pem = keypair.public_key_to_pem()?;
        let mut public_key_file = File::create(public_key_path)?;
        public_key_file.write_all(&public_key_pem)?;

        Ok(())
    }
}

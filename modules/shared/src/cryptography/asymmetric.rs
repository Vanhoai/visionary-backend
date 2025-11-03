use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use std::fs::File;
use std::io::Write;

#[derive(Clone)]
pub enum CurveAlgorithms {
    SECP256K1,
    SECP384R1,
    SECP521R1,
}

type KeypairResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct AsymmetricKeyGenerator;

impl AsymmetricKeyGenerator {
    pub fn generate_ec_keypair(
        curve: CurveAlgorithms,
        private_key_path: &str,
        public_key_path: &str,
    ) -> KeypairResult<(Vec<u8>, Vec<u8>)> {
        // Convert curve name to NID
        let nid = match curve {
            CurveAlgorithms::SECP256K1 => Nid::SECP256K1,
            CurveAlgorithms::SECP384R1 => Nid::SECP384R1,
            CurveAlgorithms::SECP521R1 => Nid::SECP521R1,
        };

        // Create EC group for the specified curve
        let group = EcGroup::from_curve_name(nid)?;

        // Generate EC key pair
        let ec_key = EcKey::generate(&group)?;

        // Create PKey from EC key
        let keypair = PKey::from_ec_key(ec_key)?;

        // Serialize private key to PEM format
        let private_key_pem = keypair.private_key_to_pem_pkcs8()?;

        let mut private_key_file = File::create(private_key_path)?;
        private_key_file.write_all(&private_key_pem)?;

        // Serialize public key to PEM format
        let public_key_pem = keypair.public_key_to_pem()?;

        let mut public_key_file = File::create(public_key_path)?;
        public_key_file.write_all(&public_key_pem)?;
        Ok((private_key_pem, public_key_pem))
    }

    pub fn generate_rsa_keypair(
        key_size: u32,
        private_key_path: &str,
        public_key_path: &str,
    ) -> KeypairResult<(Vec<u8>, Vec<u8>)> {
        let rsa = Rsa::generate(key_size)?;

        let keypair = PKey::from_rsa(rsa)?;

        let private_key_pem = keypair.private_key_to_pem_pkcs8()?;
        let mut private_key_file = File::create(private_key_path)?;
        private_key_file.write_all(&private_key_pem)?;

        let public_key_pem = keypair.public_key_to_pem()?;
        let mut public_key_file = File::create(public_key_path)?;
        public_key_file.write_all(&public_key_pem)?;
        Ok((private_key_pem, public_key_pem))
    }
}

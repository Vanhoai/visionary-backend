use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use rand::Rng;
use std::fs;
use std::path::Path;
use std::sync::Arc;

// internal modules
use crate::configs::APP_CONFIG;
use crate::cryptography::asymmetric::{AsymmetricKeyGenerator, CurveAlgorithms};
use crate::cryptography::keypair::Keypair;
use crate::functions::path_functions::PathFunctions;

pub enum TokenType {
    Access,
    Refresh,
}

pub struct KeyManager {
    pub access_keys: Keypair,
    pub refresh_keys: Keypair,
}

impl KeyManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let algorithm_type = APP_CONFIG.crypto.algorithm_type.clone();
        match algorithm_type.as_str() {
            "SYMMETRIC" => Self::init_symmetric(),
            "ASYMMETRIC" => Self::init_asymmetric(),
            _ => Err("Invalid algorithm_type. Use 'SYMMETRIC' or 'ASYMMETRIC'".into()),
        }
    }

    fn init_symmetric() -> Result<Self, Box<dyn std::error::Error>> {
        let algorithm = Keypair::parse_algorithm(&APP_CONFIG.crypto.algorithm)?;
        let root = PathFunctions::root_path();

        let access_secret = Self::get_or_generate_secret(format!("{}/keys/access_secret.key", root).as_str())?;
        let refresh_secret = Self::get_or_generate_secret(format!("{}/keys/refresh_secret.key", root).as_str())?;

        Ok(KeyManager {
            access_keys: Keypair {
                encoding_key: EncodingKey::from_secret(access_secret.as_bytes()),
                decoding_key: DecodingKey::from_secret(access_secret.as_bytes()),
                algorithm,
            },
            refresh_keys: Keypair {
                encoding_key: EncodingKey::from_secret(refresh_secret.as_bytes()),
                decoding_key: DecodingKey::from_secret(refresh_secret.as_bytes()),
                algorithm,
            },
        })
    }

    fn init_asymmetric() -> Result<Self, Box<dyn std::error::Error>> {
        let algorithm = Keypair::parse_algorithm(&APP_CONFIG.crypto.algorithm)?;
        let root = PathFunctions::root_path();

        match APP_CONFIG.crypto.algorithm.as_str() {
            "RS256" | "RS384" | "RS512" => {
                let key_size = APP_CONFIG.crypto.key_size.unwrap_or(2048);

                Self::ensure_rsa_keys(
                    key_size,
                    format!("{}/keys/access_private.pem", root).as_str(),
                    format!("{}/keys/access_public.pem", root).as_str(),
                )?;

                Self::ensure_rsa_keys(
                    key_size,
                    format!("{}/keys/refresh_private.pem", root).as_str(),
                    format!("{}/keys/refresh_public.pem", root).as_str(),
                )?;

                let access_keys = Self::load_rsa_keys(
                    format!("{}/keys/access_private.pem", root).as_str(),
                    format!("{}/keys/access_public.pem", root).as_str(),
                    algorithm,
                )?;

                let refresh_keys = Self::load_rsa_keys(
                    format!("{}/keys/refresh_private.pem", root).as_str(),
                    format!("{}/keys/refresh_public.pem", root).as_str(),
                    algorithm,
                )?;

                Ok(KeyManager { access_keys, refresh_keys })
            },
            "ES256" | "ES384" => {
                let curve = match APP_CONFIG.crypto.curve.as_deref() {
                    Some("secp256k1") => CurveAlgorithms::SECP256K1,
                    Some("secp384r1") => CurveAlgorithms::SECP384R1,
                    Some("secp521r1") => CurveAlgorithms::SECP521R1,
                    _ => CurveAlgorithms::SECP256K1,
                };

                Self::ensure_ec_keys(
                    curve.clone(),
                    format!("{}/keys/access_ec_private.pem", root.as_str()).as_str(),
                    format!("{}/keys/access_ec_public.pem", root.as_str()).as_str(),
                )?;

                Self::ensure_ec_keys(
                    curve,
                    format!("{}/keys/refresh_ec_private.pem", root.as_str()).as_str(),
                    format!("{}/keys/refresh_ec_public.pem", root.as_str()).as_str(),
                )?;

                let access_keys = Self::load_ec_keys(
                    format!("{}/keys/access_ec_private.pem", root).as_str(),
                    format!("{}/keys/access_ec_public.pem", root).as_str(),
                    algorithm,
                )?;

                let refresh_keys = Self::load_ec_keys(
                    format!("{}/keys/refresh_ec_private.pem", root).as_str(),
                    format!("{}/keys/refresh_ec_public.pem", root).as_str(),
                    algorithm,
                )?;

                Ok(KeyManager { access_keys, refresh_keys })
            },
            _ => Err("Unsupported asymmetric algorithm".into()),
        }
    }

    fn ensure_rsa_keys(key_size: u32, private_path: &str, public_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(private_path).exists() || !Path::new(public_path).exists() {
            fs::create_dir_all("keys")?;
            AsymmetricKeyGenerator::generate_rsa_keypair(key_size, private_path, public_path)?;
            println!("Generated RSA keypair: {} and {}", private_path, public_path);
        }
        Ok(())
    }

    fn ensure_ec_keys(
        curve: CurveAlgorithms,
        private_path: &str,
        public_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !Path::new(private_path).exists() || !Path::new(public_path).exists() {
            fs::create_dir_all("keys")?;
            AsymmetricKeyGenerator::generate_ec_keypair(curve, private_path, public_path)?;
        }
        Ok(())
    }

    fn load_rsa_keys(
        private_path: &str,
        public_path: &str,
        algorithm: Algorithm,
    ) -> Result<Keypair, Box<dyn std::error::Error>> {
        let private_key = fs::read(private_path)?;
        let public_key = fs::read(public_path)?;

        Ok(Keypair {
            encoding_key: EncodingKey::from_rsa_pem(&private_key)?,
            decoding_key: DecodingKey::from_rsa_pem(&public_key)?,
            algorithm,
        })
    }

    fn load_ec_keys(
        private_path: &str,
        public_path: &str,
        algorithm: Algorithm,
    ) -> Result<Keypair, Box<dyn std::error::Error>> {
        let private_key = fs::read(private_path)?;
        let public_key = fs::read(public_path)?;

        Ok(Keypair {
            encoding_key: EncodingKey::from_ec_pem(&private_key)?,
            decoding_key: DecodingKey::from_ec_pem(&public_key)?,
            algorithm,
        })
    }

    fn get_or_generate_secret(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        if Path::new(path).exists() {
            Ok(fs::read_to_string(path)?)
        } else {
            fs::create_dir_all("keys")?;
            let secret = Self::generate_random_secret(64);
            fs::write(path, &secret)?;
            println!("Generated new secret at: {}", path);
            Ok(secret)
        }
    }

    fn generate_random_secret(length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789!@#$%^&*";
        let mut rng = rand::rng();
        (0..length)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

pub static KEY_MANAGER: Lazy<Arc<KeyManager>> = Lazy::new(|| {
    let manager = KeyManager::new().expect("Failed to initialize KeyManager");
    Arc::new(manager)
});

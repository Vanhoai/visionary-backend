use crate::configs::APP_CONFIG;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use std::io::Error;

type BoxGeneric<T> = Result<T, Box<dyn std::error::Error>>;

pub enum AlgorithmType {
    Symmetric,
    Asymmetric,
}

impl AlgorithmType {
    pub fn from_string(algorithm_type: String) -> BoxGeneric<Self> {
        match algorithm_type.as_str() {
            "SYMMETRIC" => Ok(AlgorithmType::Symmetric),
            "ASYMMETRIC" => Ok(AlgorithmType::Asymmetric),
            _ => Err(format!("Unknown algorithm: {}", algorithm_type).into()),
        }
    }
}

pub struct Keypair {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub algorithm: Algorithm,
}

impl Keypair {
    pub fn parse_algorithm(alg: &str) -> BoxGeneric<Algorithm> {
        match alg {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            _ => Err(format!("Unknown algorithm: {}", alg).into()),
        }
    }
}

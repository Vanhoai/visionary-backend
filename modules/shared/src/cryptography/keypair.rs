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
    fn parse_algorithm(alg: &str) -> BoxGeneric<Algorithm> {
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

// impl JwtManager {
//     pub fn new() -> BoxGeneric<Self> {
//         let algorithm_type = APP_CONFIG.jwt.algorithm_type.clone();
//
//         match algorithm_type.as_str() {
//             "SYMMETRIC" => Self::init_symmetric(),
//             "ASYMMETRIC" => Self::init_asymmetric(),
//             _ => Err("Invalid algorithm_type. Use 'SYMMETRIC' or 'ASYMMETRIC'"
//                 .into()),
//         }
//     }
//
//     fn init_symmetric() -> BoxGeneric<Self> {
//         let algorithm = Self::parse_algorithm(&APP_CONFIG.jwt.algorithm)?;
//
//         let access_secret =
//             Self::get_or_generate_secret("keys/access_secret.key")?;
//         let refresh_secret =
//             Self::get_or_generate_secret("keys/refresh_secret.key")?;
//
//         Ok(JwtManager {
//             access_keys: Keypair {
//                 encoding_key: EncodingKey::from_secret(
//                     access_secret.as_bytes(),
//                 ),
//                 decoding_key: DecodingKey::from_secret(
//                     access_secret.as_bytes(),
//                 ),
//                 algorithm,
//             },
//             refresh_keys: Keypair {
//                 encoding_key: EncodingKey::from_secret(
//                     refresh_secret.as_bytes(),
//                 ),
//                 decoding_key: DecodingKey::from_secret(
//                     refresh_secret.as_bytes(),
//                 ),
//                 algorithm,
//             },
//         })
//     }
//
//     fn init_asymmetric() -> BoxGeneric<Self> {
//         let algorithm = Self::parse_algorithm(&APP_CONFIG.jwt.algorithm)?;
//
//         match APP_CONFIG.jwt.algorithm.as_str() {
//             "RS256" | "RS384" | "RS512" => {
//                 let key_size = APP_CONFIG.jwt.key_size.unwrap_or(2048);
//
//                 // Generate or load RSA keys for access token
//                 Self::ensure_rsa_keys(
//                     key_size,
//                     "keys/access_private.pem",
//                     "keys/access_public.pem",
//                 )?;
//
//                 // Generate or load RSA keys for refresh token
//                 Self::ensure_rsa_keys(
//                     key_size,
//                     "keys/refresh_private.pem",
//                     "keys/refresh_public.pem",
//                 )?;
//
//                 let access_keys = Self::load_rsa_keys(
//                     "keys/access_private.pem",
//                     "keys/access_public.pem",
//                     algorithm,
//                 )?;
//
//                 let refresh_keys = Self::load_rsa_keys(
//                     "keys/refresh_private.pem",
//                     "keys/refresh_public.pem",
//                     algorithm,
//                 )?;
//
//                 Ok(JwtManager { access_keys, refresh_keys })
//             },
//             "ES256" | "ES384" => {
//                 let curve = match APP_CONFIG.jwt.curve.as_deref() {
//                     Some("secp256k1") => CurveAlgorithms::SECP256K1,
//                     Some("secp384r1") => CurveAlgorithms::SECP384R1,
//                     Some("secp521r1") => CurveAlgorithms::SECP521R1,
//                     _ => CurveAlgorithms::SECP256K1,
//                 };
//
//                 // Generate or load EC keys for access token
//                 Self::ensure_ec_keys(
//                     curve.clone(),
//                     "keys/access_ec_private.pem",
//                     "keys/access_ec_public.pem",
//                 )?;
//
//                 // Generate or load EC keys for refresh token
//                 Self::ensure_ec_keys(
//                     curve,
//                     "keys/refresh_ec_private.pem",
//                     "keys/refresh_ec_public.pem",
//                 )?;
//
//                 let access_keys = Self::load_ec_keys(
//                     "keys/access_ec_private.pem",
//                     "keys/access_ec_public.pem",
//                     algorithm,
//                 )?;
//
//                 let refresh_keys = Self::load_ec_keys(
//                     "keys/refresh_ec_private.pem",
//                     "keys/refresh_ec_public.pem",
//                     algorithm,
//                 )?;
//
//                 Ok(JwtManager { access_keys, refresh_keys })
//             },
//             _ => Err("Unsupported asymmetric algorithm".into()),
//         }
//     }
//
//     fn ensure_rsa_keys(
//         key_size: u32,
//         private_path: &str,
//         public_path: &str,
//     ) -> BoxGeneric<()> {
//         if !Path::new(private_path).exists() || !Path::new(public_path).exists()
//         {
//             fs::create_dir_all("keys")?;
//             let rsa = RsaAlgorithms::new();
//             rsa.generate_keypair(key_size, private_path, public_path)?;
//         }
//
//         Ok(())
//     }
//
//     fn ensure_ec_keys(
//         curve: CurveAlgorithms,
//         private_path: &str,
//         public_path: &str,
//     ) -> BoxGeneric<()> {
//         if !Path::new(private_path).exists() || !Path::new(public_path).exists()
//         {
//             fs::create_dir_all("keys")?;
//             let ec = EcAlgorithms::new();
//             ec.generate_keypair(curve, private_path, public_path)?;
//         }
//
//         Ok(())
//     }
//
//     fn load_rsa_keys(
//         private_path: &str,
//         public_path: &str,
//         algorithm: Algorithm,
//     ) -> BoxGeneric<Keypair> {
//         let private_key = fs::read(private_path)?;
//         let public_key = fs::read(public_path)?;
//
//         Ok(Keypair {
//             encoding_key: EncodingKey::from_rsa_pem(&private_key)?,
//             decoding_key: DecodingKey::from_rsa_pem(&public_key)?,
//             algorithm,
//         })
//     }
//
//     fn load_ec_keys(
//         private_path: &str,
//         public_path: &str,
//         algorithm: Algorithm,
//     ) -> BoxGeneric<Keypair> {
//         let private_key = fs::read(private_path)?;
//         let public_key = fs::read(public_path)?;
//
//         Ok(Keypair {
//             encoding_key: EncodingKey::from_ec_pem(&private_key)?,
//             decoding_key: DecodingKey::from_ec_pem(&public_key)?,
//             algorithm,
//         })
//     }
//
//     fn get_or_generate_secret(path: &str) -> BoxGeneric<String> {
//         if Path::new(path).exists() {
//             Ok(fs::read_to_string(path)?)
//         } else {
//             fs::create_dir_all("keys")?;
//             let secret = Self::generate_random_secret(64);
//             fs::write(path, &secret)?;
//             Ok(secret)
//         }
//     }
//
//     fn generate_random_secret(length: usize) -> String {
//         use rand::Rng;
//         use std::iter;
//         const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//                                 abcdefghijklmnopqrstuvwxyz\
//                                 0123456789)(*&^%$#@!~";
//
//         let mut rng = rand::rng();
//         iter::repeat(())
//             .map(|()| CHARSET[rng.random_range(0..CHARSET.len())] as char)
//             .take(length)
//             .collect()
//     }
//

// }

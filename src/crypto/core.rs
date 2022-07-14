use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};

const SALT_LEN: usize = 16;
const KEY_SIZE: usize = 2048;

/// Server secret information
#[derive(Debug)]
struct ServerKey {
    /// The core private key used to decrypt all secrets
    privkey: RsaPrivateKey,

    /// The associated public key
    pubkey: RsaPublicKey,

    /// The pseudo random number salt used for all secret hashing
    salt_base: [u8; SALT_LEN],
}

impl ServerKey {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let privkey = RsaPrivateKey::new(&mut rng, KEY_SIZE).expect("failed to generate a key");
        let pubkey = RsaPublicKey::from(&privkey);

        let raw_salt = SaltString::generate(&mut OsRng);
        let raw_salt = raw_salt.as_bytes();
        println!("length: {}", raw_salt.len());
        let mut salt = [0u8; SALT_LEN];
        for i in 0..SALT_LEN {
            salt[i] = raw_salt[i];
        }

        println!("[{}] salt is: {:?}", salt.len(), salt);

        Self {
            privkey,
            pubkey,
            salt_base: salt,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serverkey() {
        let sk = ServerKey::new();
        println!("generated server key: {:?}", sk);
    }
}

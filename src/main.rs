use spark::core::server;
use spark::crypto::types::*;
use spark::crypto::Encryptor;
use spark::primitives::payloads::{GenericPayload, Payload};
use spark::primitives::secret::*;
use std::path::Path;

fn test_keygen() {
    // Encrypt & write a key
    let sk = ServerKey::new();
    println!("key: {:?}", sk);
    println!("keygen done");
    sk.write_key().unwrap();
}

fn test_key_unlock() {
    // Read & decrypt key
    let dec =
        ServerKey::read_key(Path::new("./data/4b577a584a64.esk")).unwrap();
    println!("decrypted: {:?}", dec);
}

fn test_secretstore() {
    let key: ServerKey =
        ServerKey::read_key(Path::new("./data/4f717474396a.esk")).unwrap();

    let secret = Secret {
        secret: Payload::Generic(GenericPayload::new(
            "secret message"
                .chars()
                .map(|c| c as u8)
                .collect::<Vec<u8>>(),
        )),
        header: Header::new("first secret", None, None, 0, Scope::Public)
            .unwrap(),
    };
    let enc_secret = key.encrypt(secret).unwrap();

    let mut server = server::Server::load(&Path::new("./data/db1")).unwrap();
    //server.put_secret(enc_secret).unwrap();

    server.dump();
}

fn main() {
    //test_keygen();
    //test_key_unlock();
    test_secretstore();
}

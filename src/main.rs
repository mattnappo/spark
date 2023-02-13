#[macro_use]
extern crate lazy_static;

use spark::core::server;
use spark::crypto::types::*;
use spark::crypto::Encryptor;
use spark::primitives::payloads::{GenericPayload, Payload};
use spark::primitives::secret::*;
use std::collections::HashMap;
use std::path::Path;

lazy_static! {
    static ref TEST_ID: Vec<u8> = vec![
        75, 121, 173, 217, 106, 74, 193, 179, 201, 191, 116, 157, 69, 163, 184,
        136, 132, 142, 92, 60, 225, 65, 114, 165, 45, 20, 144, 65, 57, 92, 129,
        103
    ];
}

// TODO: move all these tests to real tests with dependency injection
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

fn test_encrypter() {
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
    println!("    secret: {secret:?}");
    //let copy = secret.clone();
    let enc_secret = key.encrypt(secret).unwrap();
    let dec_secret = key.decrypt(enc_secret).unwrap();
    println!("dec_secret: {dec_secret:?}");
    //assert_eq!(secret, dec_secret);
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

    // Test get
    let query_header = Header {
        id: SecretID::from_vec(&TEST_ID).unwrap(),
        label: "first secret".to_string(),
        desc: None,
        tag: None,
        creation: 1676267126443,
        expiration: 0,
        scope: Scope::Public,
    };

    let query_copy = query_header.clone();

    let got_secret: EncSecret =
        server.get_secret(query_header).unwrap().unwrap();

    println!("got secret: {got_secret:?}");

    let decrypted_secret: Secret = key.decrypt(got_secret).unwrap();
    println!("decrypted secret: {decrypted_secret:?}");

    let got_from_id = server.get_secret_from_id(query_copy.id).unwrap();
    println!("secret got from id: {got_from_id:?}");
}

fn main() {
    //test_keygen();
    //test_key_unlock();
    test_secretstore();
    //test_encrypter();
}

#[macro_use]
extern crate lazy_static;

use spark::core::server;
use spark::core::server::Protocol;
use spark::crypto::types::*;
use spark::crypto::Encryptor;
use spark::primitives::payloads::{GenericPayload, Payload};
use spark::primitives::secret::*;
use std::path::Path;

pub static TESTKEY_PATH: &str = "./data/336d78316b4c.esk";

lazy_static! {
    static ref TEST_ID: Vec<u8> = vec![
        158, 199, 205, 249, 103, 8, 115, 109, 112, 251, 108, 133, 109, 73, 139,
        71, 32, 65, 130, 169, 149, 110, 61, 77, 92, 90, 191, 220, 76, 136, 10,
        130
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
    let dec = ServerKey::read_key(Path::new(TESTKEY_PATH)).unwrap();
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

fn test_server() {
    let key: ServerKey = ServerKey::read_key(Path::new(TESTKEY_PATH)).unwrap();

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

    let mut server =
        server::Server::init("./data/db1", TESTKEY_PATH, 3030).unwrap();
    //store.put_secret(enc_secret).unwrap();

    server.print_db();

    // Test get
    let query_header = Header {
        id: SecretID::from_vec(&TEST_ID).unwrap(),
        label: "first secret".to_string(),
        desc: None,
        tag: None,
        creation: 1676584357264,
        expiration: 0,
        scope: Scope::Public,
    };

    let query_copy = query_header.clone();
    let mut query_copy2 = query_header.clone();

    let got_secret: EncSecret = server.get(query_header).unwrap().unwrap();

    println!("got secret: {got_secret:?}");

    let decrypted_secret: Secret = key.decrypt(got_secret).unwrap();
    println!("decrypted secret: {decrypted_secret:?}");

    query_copy2.creation = 0;
    eprintln!("{:?}", server.get(query_copy2));

    server
        .list()
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, x)| println!("secret[{i}]: {x:?}"));
}

fn main() {
    //test_keygen();
    //test_key_unlock();
    test_server();
    //test_encrypter();
}

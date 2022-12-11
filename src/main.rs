use spark::crypto::types;
use std::path::Path;

fn test_keygen() {
    // Encrypt & write a key
    let sk = types::ServerKey::new();
    println!("key: {:?}", sk);
    println!("keygen done");
    sk.write_key().unwrap();
}

fn test_key_unlock() {
    // Read & decrypt key
    let dec = types::ServerKey::read_key(Path::new("./data/4b577a584a64.esk"))
        .unwrap();
    println!("decrypted: {:?}", dec);
}

fn main() {
    //test_keygen();
    test_key_unlock();
}

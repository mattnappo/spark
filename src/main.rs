use spark::crypto::types;
use std::path::Path;

fn main() {
    // Encrypt & write a key
    /*
    let sk = types::ServerKey::new();
    println!("key: {:?}", sk);
    println!("keygen done");
    sk.write_key().unwrap();
    */

    // Read & decrypt key
    let dec = types::ServerKey::read_key(Path::new("./data/453344396443.esk"))
        .unwrap();

    println!("decrypted: {:?}", dec);
}

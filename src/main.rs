use spark::crypto::types;

fn main() {
    let sk = types::ServerKey::new();
    println!("sk: {:?}", bincode::serialize(&sk));
    println!("keygen done");
    let locked = sk.lock().unwrap();
    println!("locked 1: {:?}", locked);
}

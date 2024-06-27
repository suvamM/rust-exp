use rand::RngCore;
use base_62;
use base64::{engine::general_purpose, Engine as _};
use base64::prelude::*;
use substring::Substring;

fn main() {
    // test_character_decoding();
    test_base_62_encoding();
}

fn test_base_62_encoding() {
    let mut key_bytes = vec![0; 66];
    let mut rng = rand::thread_rng();
    rng.try_fill_bytes(&mut key_bytes).expect("filling bytes failed");
    let mut encoded = base_62::encode(&key_bytes);
    println!("{}", encoded);

    encoded = encoded.to_string().substring(0, 85).to_string();
    encoded = format!("{}Q==", encoded);
    println!("{}", encoded);

    // let key_bytes = general_purpose::STANDARD.decode(&encoded).unwrap();
    let key_bytes = BASE64_STANDARD.decode(&encoded).unwrap();

}

fn test_character_decoding() {
    let test_char = 'a';
    let key = format!("{:85}Q==", test_char.to_string().repeat(85));

    println!("{}", key);

    let key_bytes =base64::decode(&key).unwrap();
    println!("{}", key);
}


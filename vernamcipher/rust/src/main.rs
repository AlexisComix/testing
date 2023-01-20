/*
    This program is a basic Vernam (One-time pad)
    cipher implemented in Rust.
 */

 use rand::prelude::*;

fn main() {
    // Let text be the input
    let text: String = "asigfoegofbioiba82u10u".to_owned();
    let key: String = gen_ascii_chars(
        text.len().try_into().unwrap()
    );
    
    // Convert to a u8 representation
    let text_bin: Vec<u8> = to_binary(text);
    let key_bin: Vec<u8> = to_binary(key);

    // Xor the key and text
    let text_xor_key: Vec<u8> = binary_xor(text_bin, key_bin);

    // Convert back to string
    let ciphertext: String = vecu8_to_string(text_xor_key);

    // Print the text such that it shows unicode sequences
    // That it cannot represent
    println!("{:?}", ciphertext);

    // Print what it can represent
    println!("{}", ciphertext);
}

fn gen_ascii_chars(length: u8) -> String {
    /*
        Generate a random string of ascii characters to
        a specified length
     */
    let arr: Vec<u8> = (0..length)
        .collect::<Vec<u8>>()
        .into_iter()
        .map(|_| thread_rng().gen())
        .collect::<Vec<u8>>();
    vecu8_to_string(arr)
}

fn to_binary(plaintext: String) -> Vec<u8> {
    /*
        Returns a Vec of u8s that represent the binary 
        representation of every char in plaintext.
     */
    plaintext.into_bytes()
}

fn binary_xor(bin_text: Vec<u8>, bin_key: Vec<u8>) -> Vec<u8> {
    /*
        Binary XOR each item in a Vec<u8> with another Vec<u8>
     */
    bin_text
        .iter()
        .zip(bin_key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>()
}

fn vecu8_to_string(bin_text: Vec<u8>) -> String {
    /*
        Convert a vec of u8 to a string
     */
    bin_text
        .iter()
        .map(|x| match std::char::from_u32((*x).into()) {
            Some(c) => c,
            None => '�',
        })
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
}

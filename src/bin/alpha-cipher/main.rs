// https://www.reddit.com/r/dailyprogrammer/comments/879u8b/20180326_challenge_355_easy_alphabet_cipher/

use std::io;

fn main() {
    println!("Keyword:");
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).expect("Failed to read line");
    keyword = keyword.trim_end().to_owned();
    
    println!("Message:");
    let mut msg = String::new();
    io::stdin().read_line(&mut msg).expect("Failed to read line");
    msg = msg.trim_end().to_owned();

    println!("Encoded message -> {}", encode(&msg, &keyword));
}

fn get_table() -> Vec<Vec<char>> {
    let alpha: String = String::from("abcdefghijklmnopqrstuvwxyz");

    let mut cipher = vec![];
    for i in 0..alpha.len() {
        let seq = format!("{}{}", &alpha[..i], &alpha[i+1..]);
        cipher.push(seq.chars().collect::<Vec<char>>())
    }

    cipher
}

fn encode(msg: &str, keyword: &str) -> String {
    // TODO
    String::new()
}

fn decode(encoded: &str, keyword: &str) -> String {
    // TODO
    String::new()
}
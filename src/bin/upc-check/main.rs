// https://www.reddit.com/r/dailyprogrammer/comments/a72sdj/20181217_challenge_370_easy_upc_check_digits/

use std::io;

fn main() {
    let reader = io::stdin();
    println!("Please enter a UPC");

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).expect("Failed to read line");
        input = input.trim_end().to_owned();
        match input.len() <= 11 {
            true => {
                println!("The check number is {}", upc(input));
                break;
            }
            false => {
                println!("Please enter a UPC of length 11 or less.");
                continue;
            }
        };
    }
}

pub fn upc(input: String) -> u32 {
    let vec = format!("{:0>11}", input)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let odd = vec.iter().step_by(2).fold(0, |sum, n| sum + n);
    let even = vec.iter().skip(1).step_by(2).fold(0, |sum, n| sum + n);
    let combined = (odd * 3) + even;
    let modulo = combined % 10;

    if modulo == 0 {
        modulo
    } else {
        10 - modulo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upc_test() {
        assert_eq!(2, upc(String::from("03600029145")));
        assert_eq!(4, upc(String::from("04210000526")));
        assert_eq!(4, upc(String::from("12345678910")));
        assert_eq!(0, upc(String::from("1234567")));
    }
}

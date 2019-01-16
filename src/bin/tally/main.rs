// https://www.reddit.com/r/dailyprogrammer/comments/8jcffg/20180514_challenge_361_easy_tally_program/

use std::collections::HashMap;

fn main() {
    let entries = vec!["abcde", "dbbaCEDbdAacCEAadcB", "EbAAdbBEaBaaBBdAccbeebaec"];
    for entry in entries {
        let score = tally(String::from(entry));
        println!("{:?}", score);
    }
}

fn tally(input: String) -> Vec<(String, i32)> {
    let mut scores: HashMap<String, i32> = HashMap::new();

    for c in input.chars() {
        match scores.get_mut(&c.to_string().to_lowercase()) {
            Some(x) => {
                *x = match c.is_lowercase() {
                    true => *x + 1,
                    false => *x - 1,
                }
            },
            None => {
                scores.insert(c.to_string().to_lowercase(), match c.is_lowercase() {
                    true => 1,
                    false => -1,
                });
            },
        }
    }

    let mut scores_vec: Vec<(String, i32)> = Vec::with_capacity(scores.len());
    for score in scores {
        scores_vec.push(score);
    }

    scores_vec.sort_by(|x, y| y.1.cmp(&x.1));
    scores_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tally_test() {
        unimplemented!();
    }
}
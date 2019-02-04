// https://www.reddit.com/r/dailyprogrammer/comments/akv6z4/20190128_challenge_374_easy_additive_persistence/

fn main() {
    let entries = vec![13, 1234, 9876, 199];
    for entry in entries {
        println!("{} -> {}", entry, additive(entry));
    }
}

fn additive(mut input: u32) -> u32 {
    let mut count = 0;
    loop {
        if input < 10 {
            return count;
        } else {
            input = input
                .to_string()
                .chars()
                .fold(0, |sum, n| sum + n.to_string().parse::<u32>().unwrap());

            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn additive_test() {
        assert_eq!(additive(13), 1);
        assert_eq!(additive(1234), 2);
        assert_eq!(additive(9876), 2);
        assert_eq!(additive(199), 3);
    }
}

// https://www.reddit.com/r/dailyprogrammer/comments/98ufvz/20180820_challenge_366_easy_word_funnel_1/

fn main() {
    let entries = vec![("leave", "eave"), ("reset", "rest"), ("dragoon", "dragon"), ("eave", "leave"), ("sleet", "lets"), ("skiff", "ski")];
    for entry in entries {
        println!("{} > {} -> {}", entry.0, entry.1, funnel(entry.0, entry.1));
    }
}

fn funnel(str1: &str, str2: &str) -> bool {
    for i in 0..str1.len() {
        let mut new = str1.to_owned();
        new.remove(i);

        if new == str2.to_owned() {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funnel_test() {
        assert_eq!(funnel("leave", "eave"), true);
        assert_eq!(funnel("reset", "rest"), true);
        assert_eq!(funnel("dragoon", "dragon"), true);
        assert_eq!(funnel("eave", "leave"), false);
        assert_eq!(funnel("sleet", "lets"), false);
        assert_eq!(funnel("skiff", "ski"), false);
    }
}
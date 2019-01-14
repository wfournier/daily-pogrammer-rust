use std::collections::HashMap;

fn main() {
    println!("{}", balanced("xxxyyyzzz"));
}

fn balanced(input: &str) -> bool {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    for c in input.chars() {
        match char_map.get_mut(&c) {
            Some(x) => {
                *x += 1;
            },
            None => {
                char_map.insert(c, 1);
            },
        }
    }

    for (key, value) in &char_map {
        println!("{} => {}", key, value);
    }

    !char_map.values().any(|&n| n != input.len() / char_map.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_test() {
        assert_eq!(balanced("xxxyyy"), true);
        assert_eq!(balanced("yyyxxx"), true);
        assert_eq!(balanced("xxxyyyy"), false);
        assert_eq!(balanced("yyxyxxyxxyyyyxxxyxyx"), true);
        assert_eq!(balanced("xyxxxxyyyxyxxyxxyy"), false);
        assert_eq!(balanced(""), true);
        assert_eq!(balanced("x"), true);
    }

    #[test]
    fn balanced_bonus_test() {
        assert_eq!(balanced("xxxyyyzzz"), true);
        assert_eq!(balanced("abccbaabccba"), true);
        assert_eq!(balanced("xxxyyyzzzz"), false);
        assert_eq!(balanced("abcdefghijklmnopqrstuvwxyz"), true);
        assert_eq!(balanced("pqq"), false);
        assert_eq!(balanced("fdedfdeffeddefeeeefddf"), false);
        assert_eq!(balanced("www"), true);
    }
}
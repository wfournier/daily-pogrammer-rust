// https://www.reddit.com/r/dailyprogrammer/comments/8s0cy1/20180618_challenge_364_easy_create_a_dice_roller/

use rand::Rng;

fn main() {
    for d in vec![4, 6, 8, 10, 12, 20] {
        for n in 0..3 {
            roll(format!("{}d{}", n + 1, d));
        }
    }
}

fn roll(dice: String) -> u32 {
    let mut rng = rand::thread_rng();

    let r: Vec<u32> = dice.to_lowercase().split("d").map(|n| n.parse::<u32>().unwrap()).collect();
    let (n, d) = (r[0], r[1]);

    let mut rolls: Vec<u32> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        rolls.push(rng.gen_range(1, d + 1));
    }

    let sum = rolls.iter().fold(0, |sum, x| sum + x);
    println!("{} => {} {:?}", dice, sum, rolls);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_test() {
        for d in vec![4, 6, 8, 10, 12, 20] {
            for n in 0..3 {
                let r = roll(format!("{}d{}", n + 1, d));
                assert!(r >= n + 1 && r <= (n + 1) * d);
            }
        }
    }
}
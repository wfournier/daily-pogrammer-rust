// https://www.reddit.com/r/dailyprogrammer/comments/a0lhxx/20181126_challenge_369_easy_hex_colors/

extern crate hex;
use rand::Rng;

pub struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl HexColor {
    pub fn new(r: u8, g: u8, b: u8) -> HexColor {
        HexColor {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn from(r: &str, g: &str, b: &str) -> HexColor {
        let decode = match hex::decode(format!("{}{}{}", r, g, b)) {
            Ok(v) => v,
            Err(_) => panic!("Invalid hex data."),
        };

        HexColor {
            red: decode[0] as u8,
            green: decode[1] as u8,
            blue: decode[2] as u8,
        }
    }

    fn to_hex(&self) -> String {
        format!(
            "#{}",
            hex::encode_upper(vec![self.red, self.green, self.blue])
        )
    }

    fn to_string(&self) -> String {
        format!(
            "(r: {}, g: {}, b: {}) => {}",
            self.red,
            self.green,
            self.blue,
            self.to_hex()
        )
    }

    fn blend(colors: &Vec<HexColor>) -> HexColor {
        HexColor {
            red: (colors.iter().fold(0f32, |sum, hex| sum + hex.red as f32) / (colors.len() as f32))
                .round() as u8,
            green: (colors.iter().fold(0f32, |sum, hex| sum + hex.green as f32)
                / (colors.len() as f32))
                .round() as u8,
            blue: (colors.iter().fold(0f32, |sum, hex| sum + hex.blue as f32)
                / (colors.len() as f32))
                .round() as u8,
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut hexes: Vec<HexColor> = Vec::new();
    for _ in 0..3 {
        // Add 3 randomly generated HexColor to the vector
        let (r, g, b): (u8, u8, u8) = (
            rng.gen_range(0, std::u8::MAX),
            rng.gen_range(0, std::u8::MAX),
            rng.gen_range(0, std::u8::MAX),
        );
        let hex = HexColor::new(r, g, b);
        println!("{}", hex.to_string());
        hexes.push(hex);
    }

    let blend = HexColor::blend(&hexes);
    println!(
        "{} + {} + {} = {}",
        hexes[0].to_hex(),
        hexes[1].to_hex(),
        hexes[2].to_hex(),
        blend.to_string()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex_test() {
        assert_eq!(HexColor::new(255, 99, 71).to_hex(), String::from("#FF6347"));
        assert_eq!(
            HexColor::new(184, 134, 11).to_hex(),
            String::from("#B8860B")
        );
        assert_eq!(
            HexColor::new(189, 183, 107).to_hex(),
            String::from("#BDB76B")
        );
        assert_eq!(HexColor::new(0, 0, 205).to_hex(), String::from("#0000CD"));
    }

    #[test]
    fn blend_test() {
        assert_eq!(
            HexColor::blend(&vec![
                HexColor::from("00", "00", "00"),
                HexColor::from("77", "88", "99")
            ])
            .to_hex(),
            String::from("#3C444D")
        );
        assert_eq!(
            HexColor::blend(&vec![
                HexColor::from("E6", "E6", "FA"),
                HexColor::from("FF", "69", "B4"),
                HexColor::from("B0", "C4", "DE")
            ])
            .to_hex(),
            String::from("#DCB1D9")
        );
    }
}

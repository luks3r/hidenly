use bimap::BiMap;


lazy_static::lazy_static! {
    static ref HEX_CHAR_MAP: BiMap<char, &'static str> = {
        let mut map = BiMap::new();
        map.insert(' ', "\u{200B}\u{2060}");
        map.insert('0', "\u{200B}\u{200B}");
        map.insert('1', "\u{200B}\u{200C}");
        map.insert('2', "\u{200B}\u{200D}");
        map.insert('3', "\u{200B}\u{200E}");
        map.insert('4', "\u{200B}\u{200F}");
        map.insert('5', "\u{200C}\u{200B}");
        map.insert('6', "\u{200C}\u{200C}");
        map.insert('7', "\u{200C}\u{200D}");
        map.insert('8', "\u{200C}\u{200E}");
        map.insert('9', "\u{200C}\u{200F}");
        map.insert('A', "\u{200D}\u{200B}");
        map.insert('B', "\u{200D}\u{200C}");
        map.insert('C', "\u{200D}\u{200D}");
        map.insert('D', "\u{200D}\u{200E}");
        map.insert('E', "\u{200D}\u{200F}");
        map.insert('F', "\u{200E}\u{200B}");
        map
    };
}



fn string_to_hex_str(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|b| format!("{:X}", b).to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn hex_str_to_string(input: &str) -> String {
    let bytes = (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();

    String::from_utf8_lossy(&bytes).to_string()
}

fn hex_str_to_encoded(hex: &str) -> String {
    let mut string_result = String::new();
    for ch in hex.chars() {
        if let Some(hex_char) = HEX_CHAR_MAP.get_by_left(&ch) {
            string_result.push_str(hex_char);
        }
    }
    string_result
}

fn encoded_to_hex_str(input: &str) -> String {
    let mut string_result = String::new();
    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        let pair_str = chunk.iter().collect::<String>();
        if let Some(mapped_str) = HEX_CHAR_MAP.get_by_right(pair_str.as_str()) {
            string_result.push(*mapped_str);
        }
    }
    string_result
}

fn wrap(input: &str, secret: &str) -> String {
    let middle_index = input.len() / 2;
    let (first_half, second_half) = input.split_at(middle_index);
    format!("{}\u{FEFF}{}\u{FEFF}{}", first_half, secret, second_half)
}

fn unwrap(input: &str) -> String {
    let parts: Vec<&str> = input.split("\u{FEFF}").collect();
    if parts.len() < 2 {
        input.to_string()
    } else {
        parts[1].to_string()
    }
}

pub fn encode(input: &str, secret: &str) -> String {
    let hex_str = string_to_hex_str(secret);
    let encoded = hex_str_to_encoded(hex_str.as_str());
    wrap(input, &encoded)
}

pub fn decode(input: &str) -> String {
    let unwrapped = unwrap(input);
    let hex_str = encoded_to_hex_str(&unwrapped);
    hex_str_to_string(hex_str.as_str())
}

pub fn encode_binary(input: &str, secret: Vec<u8>) -> String {
    String::new()
}

pub fn decode_binary(input: &str) -> Vec<u8> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_hex_str() {
        let input_string = "Hello, World!";
        let expexted_output = "48656C6C6F2C20576F726C6421";
        let result = string_to_hex_str(input_string);
        assert_eq!(result, expexted_output);
    }

    #[test]
    fn test_hex_str_to_string() {
        let input_string = "48656C6C6F2C20576F726C6421";
        let expexted_output = "Hello, World!";
        let result = hex_str_to_string(input_string);
        assert_eq!(result, expexted_output);
    }

    #[test]
    fn test_hex_str_to_encoded() {
        let input_string = "48656C6C6F2C20576F726C6421"; // "Hello, World!"
        let expected_output = "\u{200b}\u{200f}\u{200c}\u{200e}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200b}\u{200d}\u{200d}\u{200d}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200c}\u{200d}\u{200b}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200b}\u{200f}\u{200b}\u{200d}\u{200b}\u{200c}";
        let result = hex_str_to_encoded(input_string);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_encoded_to_hex_str() {
        let input_hex = "\u{200b}\u{200f}\u{200c}\u{200e}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200b}\u{200d}\u{200d}\u{200d}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200c}\u{200d}\u{200b}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200b}\u{200f}\u{200b}\u{200d}\u{200b}\u{200c}";
        let expected_output = "48656C6C6F2C20576F726C6421";
        let result = encoded_to_hex_str(input_hex);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_encoded() {
        let input_string = "hello";
        let secret_string = "Hello, World!";

        let expected_output = "he\u{FEFF}\u{200b}\u{200f}\u{200c}\u{200e}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200b}\u{200d}\u{200d}\u{200d}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200c}\u{200d}\u{200b}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200b}\u{200f}\u{200b}\u{200d}\u{200b}\u{200c}\u{FEFF}llo";

        let result = encode(input_string, secret_string);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode() {
        let input_string = "he\u{FEFF}\u{200b}\u{200f}\u{200c}\u{200e}\u{200c}\u{200c}\u{200c}\u{200b}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200b}\u{200d}\u{200d}\u{200d}\u{200b}\u{200d}\u{200b}\u{200b}\u{200c}\u{200b}\u{200c}\u{200d}\u{200c}\u{200c}\u{200e}\u{200b}\u{200c}\u{200d}\u{200b}\u{200d}\u{200c}\u{200c}\u{200d}\u{200d}\u{200c}\u{200c}\u{200b}\u{200f}\u{200b}\u{200d}\u{200b}\u{200c}\u{FEFF}llo";
        let expected_output = "Hello, World!";
        let result = decode(input_string);
        assert_eq!(result, expected_output);
    }
}

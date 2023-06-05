use base64::{engine::general_purpose, Engine as _};
use bimap::BiMap;

lazy_static::lazy_static! {
    static ref BASE64_CHAR_MAP: BiMap<char, &'static str> = {
        let mut map = BiMap::new();
        map.insert('A', "\u{2060}\u{2060}");
        map.insert('B', "\u{2060}\u{200B}");
        map.insert('C', "\u{2060}\u{200C}");
        map.insert('D', "\u{2060}\u{200D}");
        map.insert('E', "\u{2060}\u{200E}");
        map.insert('F', "\u{2060}\u{200F}");
        map.insert('G', "\u{2060}\u{202D}");
        map.insert('H', "\u{2060}\u{202C}");
        map.insert('I', "\u{200B}\u{2060}");
        map.insert('J', "\u{200B}\u{200B}");
        map.insert('K', "\u{200B}\u{200C}");
        map.insert('L', "\u{200B}\u{200D}");
        map.insert('M', "\u{200B}\u{200E}");
        map.insert('N', "\u{200B}\u{200F}");
        map.insert('O', "\u{200B}\u{202D}");
        map.insert('P', "\u{200B}\u{202C}");
        map.insert('Q', "\u{200C}\u{2060}");
        map.insert('R', "\u{200C}\u{200B}");
        map.insert('S', "\u{200C}\u{200C}");
        map.insert('T', "\u{200C}\u{200D}");
        map.insert('U', "\u{200C}\u{200E}");
        map.insert('V', "\u{200C}\u{200F}");
        map.insert('W', "\u{200C}\u{202D}");
        map.insert('X', "\u{200C}\u{202C}");
        map.insert('Y', "\u{200D}\u{2060}");
        map.insert('Z', "\u{200D}\u{200B}");
        map.insert('a', "\u{200D}\u{200C}");
        map.insert('b', "\u{200D}\u{200D}");
        map.insert('c', "\u{200D}\u{200E}");
        map.insert('d', "\u{200D}\u{200F}");
        map.insert('e', "\u{200D}\u{202D}");
        map.insert('f', "\u{200D}\u{202C}");
        map.insert('g', "\u{200E}\u{2060}");
        map.insert('h', "\u{200E}\u{200B}");
        map.insert('i', "\u{200E}\u{200C}");
        map.insert('j', "\u{200E}\u{200D}");
        map.insert('k', "\u{200E}\u{200E}");
        map.insert('l', "\u{200E}\u{200F}");
        map.insert('m', "\u{200E}\u{202D}");
        map.insert('n', "\u{200E}\u{202C}");
        map.insert('o', "\u{200F}\u{2060}");
        map.insert('p', "\u{200F}\u{200B}");
        map.insert('q', "\u{200F}\u{200C}");
        map.insert('r', "\u{200F}\u{200D}");
        map.insert('s', "\u{200F}\u{200E}");
        map.insert('t', "\u{200F}\u{200F}");
        map.insert('u', "\u{200F}\u{202D}");
        map.insert('v', "\u{200F}\u{202C}");
        map.insert('w', "\u{202D}\u{2060}");
        map.insert('x', "\u{202D}\u{200B}");
        map.insert('y', "\u{202D}\u{200C}");
        map.insert('z', "\u{202D}\u{200D}");
        map.insert('0', "\u{202D}\u{200E}");
        map.insert('1', "\u{202D}\u{200F}");
        map.insert('2', "\u{202D}\u{202D}");
        map.insert('3', "\u{202D}\u{202C}");
        map.insert('4', "\u{202C}\u{2060}");
        map.insert('5', "\u{202C}\u{200B}");
        map.insert('6', "\u{202C}\u{200C}");
        map.insert('7', "\u{202C}\u{200D}");
        map.insert('8', "\u{202C}\u{200E}");
        map.insert('9', "\u{202C}\u{200F}");
        map.insert('+', "\u{202C}\u{202D}");
        map.insert('/', "\u{202C}\u{202C}");
        map
    };
}

fn encode_base64(input: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(input)
}

fn decode_base64(input: &str) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD.decode(input).unwrap()
}

fn base64_to_encoded(input: &str) -> String {
    let mut string_result = String::new();
    for ch in input.chars() {
        if let Some(hex_char) = BASE64_CHAR_MAP.get_by_left(&ch) {
            string_result.push_str(hex_char);
        }
    }
    string_result
}

fn encoded_to_base64(input: &str) -> String {
    let mut string_result = String::new();
    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        let pair_str = chunk.iter().collect::<String>();
        if let Some(mapped_str) = BASE64_CHAR_MAP.get_by_right(pair_str.as_str()) {
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
    let preprocessed = encode_base64(secret.as_bytes());
    let encoded = base64_to_encoded(preprocessed.as_str());
    wrap(input, &encoded)
}

pub fn decode(input: &str) -> String {
    let unwrapped = unwrap(input);
    let processed = encoded_to_base64(&unwrapped);
    String::from_utf8(decode_base64(processed.as_str())).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() {
        let input_string = "Hello, World!";
        let expected_output = "SGVsbG8sIFdvcmxkIQ";
        let result = encode_base64(input_string.as_bytes());
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode_base64() {
        let input_string = "SGVsbG8sIFdvcmxkIQ";
        let expected_output = "Hello, World!";
        let result = decode_base64(input_string);
        assert_eq!(String::from_utf8(result).unwrap(), expected_output);
    }

    #[test]
    fn test_base64_to_encoded() {
        let input_string = "SGVsbG8sIFdvcmxkIQ"; // "Hello, World!"
        let expected_output = "\u{202d}\u{200b}\u{200f}\u{202d}\u{202d}\u{200e}\u{200e}\u{200c}\u{200c}\u{200d}\u{200f}\u{202d}\u{200b}\u{2060}\u{200e}\u{200c}\u{200f}\u{2060}\u{200f}\u{202c}\u{200c}\u{200f}\u{200e}\u{200f}\u{200c}\u{200e}\u{200d}\u{200f}\u{200e}\u{202d}\u{200d}\u{200d}\u{200f}\u{2060}\u{202c}\u{202e}";
        let result = base64_to_encoded(input_string);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_encoded_to_base64() {
        let input_hex = "\u{202d}\u{200b}\u{200f}\u{202d}\u{202d}\u{200e}\u{200e}\u{200c}\u{200c}\u{200d}\u{200f}\u{202d}\u{200b}\u{2060}\u{200e}\u{200c}\u{200f}\u{2060}\u{200f}\u{202c}\u{200c}\u{200f}\u{200e}\u{200f}\u{200c}\u{200e}\u{200d}\u{200f}\u{200e}\u{202d}\u{200d}\u{200d}\u{200f}\u{2060}\u{202c}\u{202e}";
        let expected_output = "SGVsbG8sIFdvcmxkIQ";
        let result = encoded_to_base64(input_hex);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_encoded() {
        let input_string = "hello";
        let secret_string = "Hello, World!";

        let expected_output = "he\u{FEFF}\u{202d}\u{200b}\u{200f}\u{202d}\u{202d}\u{200e}\u{200e}\u{200c}\u{200c}\u{200d}\u{200f}\u{202d}\u{200b}\u{2060}\u{200e}\u{200c}\u{200f}\u{2060}\u{200f}\u{202c}\u{200c}\u{200f}\u{200e}\u{200f}\u{200c}\u{200e}\u{200d}\u{200f}\u{200e}\u{202d}\u{200d}\u{200d}\u{200f}\u{2060}\u{202c}\u{202e}\u{FEFF}llo";

        let result = encode(input_string, secret_string);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_decode() {
        let input_string = "he\u{FEFF}\u{202d}\u{200b}\u{200f}\u{202d}\u{202d}\u{200e}\u{200e}\u{200c}\u{200c}\u{200d}\u{200f}\u{202d}\u{200b}\u{2060}\u{200e}\u{200c}\u{200f}\u{2060}\u{200f}\u{202c}\u{200c}\u{200f}\u{200e}\u{200f}\u{200c}\u{200e}\u{200d}\u{200f}\u{200e}\u{202d}\u{200d}\u{200d}\u{200f}\u{2060}\u{202c}\u{202e}\u{FEFF}llo";
        let expected_output = "Hello, World!";
        let result = decode(input_string);
        assert_eq!(result, expected_output);
    }
}

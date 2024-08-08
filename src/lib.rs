use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::Write;
use encoding_rs::UTF_8;

pub fn encodep(text: &str) -> String {
    let (cow, _, _) = UTF_8.encode(text);
    let data = cow.into_owned();

    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&data).unwrap();
    let compressed = encoder.finish().unwrap();

    encode64(&compressed)
}

fn encode6bit(b: u8) -> char {
    match b {
        0..=9 => (b + 48) as char,
        10..=35 => (b + 55) as char,
        36..=61 => (b + 61) as char,
        62 => '-',
        63 => '_',
        _ => '?',
    }
}

fn append3bytes(b1: u8, b2: u8, b3: u8) -> String {
    let c1 = b1 >> 2;
    let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
    let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
    let c4 = b3 & 0x3F;

    let mut r = String::new();
    r.push(encode6bit(c1 & 0x3F));
    r.push(encode6bit(c2 & 0x3F));
    r.push(encode6bit(c3 & 0x3F));
    r.push(encode6bit(c4 & 0x3F));
    r
}

fn encode64(c: &[u8]) -> String {
    let mut str = String::new();
    let len = c.len();

    for i in (0..len).step_by(3) {
        if i + 2 == len {
            str.push_str(&append3bytes(c[i], c[i + 1], 0));
        } else if i + 1 == len {
            str.push_str(&append3bytes(c[i], 0, 0));
        } else {
            str.push_str(&append3bytes(c[i], c[i + 1], c[i + 2]));
        }
    }
    str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encodep() {
        let input = "@startuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response
@enduml";
        let encoded = encodep(input);
        println!("Encoded result: {}", encoded);

        assert_eq!(encoded, "RSp13O0m243HUwTWWIxWmRIEu0QraaXIGGNsbtZs_5z-KMkF-JbI7TGHyWoBR1DKjmFPg3SZOLZnTbHBqI27-V2FqajOCHNaFPul");        
    }
}
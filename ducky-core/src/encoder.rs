use crate::errors::CompilerResult;

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

pub fn hex_encode(value: u16) -> String {
    let bytes = value.to_le_bytes();
    let mut s = String::with_capacity(4);
    for &byte in &bytes {
        s.push(HEX_DIGITS[(byte >> 4) as usize] as char);
        s.push(HEX_DIGITS[(byte & 0x0f) as usize] as char);
    }
    s
}

pub fn dec_to_hex(value: usize) -> String {
    let mut tmp = [0u8; 16];
    let mut i = tmp.len();
    let mut v = value;
    loop {
        i -= 1;
        tmp[i] = HEX_DIGITS[v & 0x0f];
        v >>= 4;
        if v == 0 {
            break;
        }
    }
    let digits = &tmp[i..];
    let mut s = String::with_capacity(digits.len().max(4));
    for _ in digits.len()..4 {
        s.push('0');
    }
    for &d in digits {
        s.push(d as char);
    }
    s
}

pub fn swap_hex(hex_str: &str) -> String {
    if hex_str.len() >= 4 {
        format!("{}{}", &hex_str[2..4], &hex_str[0..2])
    } else {
        hex_str.to_string()
    }
}

pub fn format_hex(hex_str: &str) -> String {
    hex_str
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .to_lowercase()
}

pub fn build_delay_bytes(mut delay: u32) -> Vec<String> {
    let mut result = Vec::new();

    while delay > 0 {
        result.push("00".to_string());
        if delay > 255 {
            result.push("FF".to_string());
            delay -= 255;
        } else {
            result.push(format!("{:02x}", delay));
            delay = 0;
        }
    }

    result
}

pub fn append_hex_string_array(output: &mut Vec<u8>, hex_array: Vec<String>) -> CompilerResult<()> {
    for hex in hex_array {
        if hex.len() > 2 {
            output.push(parse_hex_byte(&hex[0..2]));
            output.push(parse_hex_byte(&hex[2..4]));
        } else {
            output.push(parse_hex_byte(&hex));
        }
    }
    Ok(())
}

fn parse_hex_byte(s: &str) -> u8 {
    u8::from_str_radix(s, 16).unwrap_or(0)
}

pub fn normalize_var_value(val: &str) -> (u8, u8) {
    let s = match val.len() {
        4 => val.to_string(),
        2 => format!("00{}", val),
        1 => format!("000{}", val),
        n if n > 4 => val[0..4].to_string(),
        _ => format!("{:0<4}", val),
    };
    (parse_hex_byte(&s[0..2]), parse_hex_byte(&s[2..4]))
}

pub fn hex_to_byte_array(hex_string: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let clean = hex_string.replace(" ", "");

    for i in (0..clean.len()).step_by(2) {
        if i + 2 <= clean.len()
            && let Ok(byte) = u8::from_str_radix(&clean[i..i + 2], 16)
        {
            result.push(byte);
        }
    }

    result
}

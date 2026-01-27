use crate::errors::CompilerResult;

pub fn hex_encode(value: u16) -> String {
    let bytes = value.to_le_bytes();
    format!("{:02x}{:02x}", bytes[0], bytes[1])
}

pub fn dec_to_hex(value: usize) -> String {
    format!("{:04x}", value)
}

pub fn swap_hex(hex_str: &str) -> String {
    if hex_str.len() >= 4 {
        format!("{}{}", &hex_str[2..4], &hex_str[0..2])
    } else {
        hex_str.to_string()
    }
}

pub fn format_hex(hex_str: &str) -> String {
    hex_str.trim_start_matches("0x").trim_start_matches("0X").to_lowercase()
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

pub fn append_hex_string_array(output: &mut Vec<String>, hex_array: Vec<String>) -> CompilerResult<()> {
    for hex in hex_array {
        if hex.len() > 2 {
            output.push(hex[0..2].to_string());
            output.push(hex[2..4].to_string());
        } else {
            output.push(hex);
        }
    }
    Ok(())
}

pub fn hex_to_byte_array(hex_string: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let clean = hex_string.replace(" ", "");
    
    for i in (0..clean.len()).step_by(2) {
        if i + 2 <= clean.len() {
            if let Ok(byte) = u8::from_str_radix(&clean[i..i + 2], 16) {
                result.push(byte);
            }
        }
    }
    
    result
}

use old_crate::Error;

// v1.x

pub fn encode(input: &str) -> String {
    input.chars().map(|c| format!("{:02x}", c as u32)).collect()
}

pub fn decode(hex: &str) -> Result<String, old_crate::Error> {
    let bytes: Vec<u8> = hex
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).map_err(|_| old_crate::Error::parse())?;
            u8::from_str_radix(s, 16).map_err(|_| old_crate::Error::parse())
        })
        .collect::<Result<_, _>>()?;
    String::from_utf8(bytes).map_err(|_| old_crate::Error::parse())
}

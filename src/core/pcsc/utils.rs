#[derive(Debug, Clone, Copy)]
pub enum ByteFormat {
    Hex,
    Decimal,
}

pub fn format_bytes(bytes: &[u8], format: ByteFormat) -> String {
    match format {
        ByteFormat::Hex => bytes
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" "),

        ByteFormat::Decimal => bytes
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(""),
    }
}

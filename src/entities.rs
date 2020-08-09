pub fn buffer_to_string(buffer: &[u8]) -> String {
    String::from_utf8_lossy(&buffer[..])
    .trim_matches(char::from(0))
    .to_string()
}

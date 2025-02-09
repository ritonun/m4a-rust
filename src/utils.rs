pub fn by_to_u32(bytes: &[u8]) -> u32 {
    //! Convert an array of 4 bytes into a u32
    u32::from_be_bytes(bytes.try_into().expect("Slice should be 4 bytes"))
}

pub fn by_to_string(bytes: &[u8]) -> String {
    u32_to_string(by_to_u32(bytes))
}

pub fn u32_to_string(value: u32) -> String {
    let bytes = value.to_be_bytes();
    String::from_utf8_lossy(&bytes).to_string()
}

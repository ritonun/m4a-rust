pub fn get_chunk_size(array: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*array)
}

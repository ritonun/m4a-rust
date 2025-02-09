use crate::chunks::*;
use crate::utils::*;

pub fn get_chunk_size(array: &[u8; 4]) -> u32 {
    u32::from_be_bytes(*array)
}

pub fn get_chunk_info(data: &[u8]) -> Option<Chunk> {
    if data.len() != 8 {
        panic!("Slice should be 8 byte");
    }

    Some(Chunk::new(
        by_to_u32(&data[0..4]),
        &data[4..8].try_into().expect("Slice should be 4 byte"),
    ))
}

pub fn match_chunk_type(chunk: &Chunk, data: &[u8]) -> Option<ChunkBox> {
    match chunk.chunk_type.as_str() {
        "ftyp" => Some(ChunkBox::Ftyp(Ftyp::new(chunk.clone(), &data))),
        _ => None,
    }
}

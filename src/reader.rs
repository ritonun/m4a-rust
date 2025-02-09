use crate::chunks::*;
use crate::utils::*;

pub fn get_chunk_info(data: &Vec<u8>, counter: usize) -> Option<Chunk> {
    let size = by_to_u32(&data[0..4]);
    Some(Chunk::new(size, &data[counter..counter + size as usize]))
}

pub fn match_chunk_type(chunk: &Chunk) -> Option<ChunkBox> {
    match chunk.chunk_type.as_str() {
        "ftyp" => Some(ChunkBox::Ftyp(Ftyp::new(chunk.clone()))),
        _ => None,
    }
}

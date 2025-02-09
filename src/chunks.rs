use std::fmt;

use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub size: u32,
    pub chunk_type: String,
    pub data: Vec<u8>,
}

pub trait M4aChunk {
    fn new(chunk: Chunk) -> Self;
}

impl Chunk {
    pub fn new(size: u32, data: &[u8]) -> Chunk {
        Chunk {
            size,
            data: data.to_vec(),
            chunk_type: Chunk::match_chunk_type(
                &data[4..8].try_into().expect("Slice must be 4 byte"),
            ),
        }
    }

    fn match_chunk_type(type_byte: &[u8; 4]) -> String {
        match type_byte {
            b"ftyp" => String::from("ftyp"),
            _ => {
                eprintln!("Could not find chunk type {:?}", type_byte);
                String::new()
            }
        }
    }
}

#[derive(Debug)]
pub struct Ftyp {
    chunk: Chunk,
    major_brand: u32,
    minor_version: u32,
    compatible_brands: Vec<u32>,
}

impl M4aChunk for Ftyp {
    fn new(chunk: Chunk) -> Ftyp {
        Ftyp {
            chunk: chunk.clone(),
            major_brand: by_to_u32(&chunk.data[4..8]),
            minor_version: by_to_u32(&chunk.data[8..12]),
            compatible_brands: {
                let mut all: Vec<u32> = Vec::new();
                let compatible_brands_number = chunk.size - 16;
                if compatible_brands_number > 0 {
                    for i in (16..chunk.size).step_by(4) {
                        all.push(by_to_u32(&chunk.data[i as usize..i as usize + 4]));
                    }
                }
                all
            },
        }
    }
}

#[derive(Debug)]
pub enum ChunkBox {
    Ftyp(Ftyp),
}

impl fmt::Display for Ftyp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ftyp {{  Chunk: {:?} , major_brand: {}, minor_version: {}, compatible_brands: {:?} }}",
            self.chunk,
            u32_to_string(self.major_brand),
            self.minor_version,
            self.compatible_brands
        )
    }
}

use std::fmt;

use crate::utils::*;

#[derive(Debug, Clone)]
pub struct Chunk {
    size: u32,
    chunk_type: String,
}

impl Chunk {
    pub fn new(size: u32, type_byte: &[u8; 4]) -> Chunk {
        Chunk {
            size,
            chunk_type: Chunk::match_chunk_type(type_byte),
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

pub struct Ftyp {
    chunk: Chunk,
    major_brand: u32,
    minor_version: u32,
    compatible_brands: Vec<u32>,
}

impl Ftyp {
    pub fn new(chunk: Chunk, bytes: &[u8]) -> Ftyp {
        Ftyp {
            chunk: chunk.clone(),
            major_brand: by_to_u32(&bytes[4..8]),
            minor_version: by_to_u32(&bytes[8..12]),
            compatible_brands: {
                let mut all: Vec<u32> = Vec::new();
                let compatible_brands_number = chunk.size - 16;
                if compatible_brands_number > 0 {
                    for i in (16..chunk.size).step_by(4) {
                        all.push(by_to_u32(&bytes[i as usize..i as usize + 4]));
                    }
                }
                all
            },
        }
    }
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

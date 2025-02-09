// lib.rs
use std::path::PathBuf;

mod chunks;
mod reader;
mod utils;
use chunks::ChunkBox;
use utils::*;

pub fn read(filepath: PathBuf) -> std::io::Result<()> {
    let file = std::fs::read(filepath)?;
    println!("file len: {}", file.len());

    // read chunk
    let mut counter = 0;
    let mut all_chunks: Vec<ChunkBox> = Vec::new();
    while counter < file.len() {
        println!("Reading chunk counter {}", counter);
        // get chunk_size
        let chunk = match reader::get_chunk_info(&file[counter..counter + 8]) {
            Some(c) => c,
            None => panic!("Error reading file at counter {}", counter),
        };

        match reader::match_chunk_type(&chunk, &file[counter..counter + chunk.size as usize]) {
            Some(c) => all_chunks.push(c),
            None => println!(
                "Chunk at  {}..{} is either invalid or skippable",
                counter,
                counter + chunk.size as usize
            ),
        }

        counter += chunk.size as usize;
        if counter >= file.len() {
            break;
        }
    }

    Ok(())
}

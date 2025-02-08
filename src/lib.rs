// lib.rs
use std::path::PathBuf;

mod chunks;
mod reader;
mod utils;

pub fn read(filepath: PathBuf) -> std::io::Result<()> {
    let file = std::fs::read(filepath)?;
    println!("file len: {}", file.len());

    // read chunk
    let mut counter = 0;
    while counter < file.len() {
        // get chunk_size
        let chunk_size = reader::get_chunk_size(
            &file[counter..counter + 4]
                .try_into()
                .expect("Slice must be 4 byte"),
        );

        let chunk = chunks::Chunk::new(
            chunk_size,
            &file[counter + 4..counter + 8].try_into().expect("4b"),
        );
        println!("{:?}", chunk);

        let ftyp = chunks::Ftyp::new(chunk, &file[counter..counter + chunk_size as usize]);
        println!("{}", ftyp);
        todo!();

        if counter >= file.len() {
            break;
        }
    }

    Ok(())
}

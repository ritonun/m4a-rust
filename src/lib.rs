// lib.rs
use std::path::PathBuf;

pub fn read(filepath: PathBuf) -> std::io::Result<()> {
    let file = std::fs::read(filepath)?;

    Ok(())
}

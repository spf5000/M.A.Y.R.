use std::io;
use std::fs::{self};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn read_models(dir: &Path) -> anyhow::Result<Vec<String>> {
    let mut output = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                output.extend_from_slice(&read_models(&path)?);
            }

            if path.extension().is_some() && path.extension().unwrap().to_str().unwrap_or("") == "smithy" {
                output.push(String::from_utf8(fs::read(path)?)?);
            }
        }
    }
    Ok(output)
}
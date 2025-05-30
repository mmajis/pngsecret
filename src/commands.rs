use std::str::FromStr;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::Result;

pub(crate) fn encode(p0: EncodeArgs) -> Result<()> {
    use crate::png::Png;

    let file_path = p0.file_path;
    let chunk_type_str = p0.chunk_type;
    let message = p0.message;

    // Load the PNG file
    let mut png = Png::from_file(&file_path)?;

    // Create a new chunk with the provided type and message
    let chunk_type = ChunkType::from_str(&chunk_type_str)?;
    png.append_chunk(Chunk::new(chunk_type, message.into()));

    // Save the modified PNG back to the file
    png.save_to_file(&file_path)?;

    Ok(())
}

pub(crate) fn decode(p0: DecodeArgs) -> Result<()> {
    use crate::png::Png;
    let file_path = p0.file_path;
    let chunk_type_str = p0.chunk_type;

    let png = Png::from_file(&file_path)?;
    let chunk_type = ChunkType::from_str(&chunk_type_str)?;

    if let Some(chunk) = png.chunk_by_type(&chunk_type.to_string()) {
        let data = chunk.data_as_string()?;
        println!("{}", data);
    } else {
        println!("Chunk type '{}' not found.", chunk_type_str);
    }

    Ok(())    
}

pub(crate) fn remove(p0: RemoveArgs) -> Result<()> {
    use crate::png::Png;
    let file_path = p0.file_path;
    let chunk_type_str = p0.chunk_type;

    let mut png = Png::from_file(&file_path)?;

    if png.remove_first_chunk(&chunk_type_str).is_ok() {
        png.save_to_file(&file_path)?;
        println!("Chunk type '{}' removed successfully.", chunk_type_str);
    } else {
        println!("Chunk type '{}' not found.", chunk_type_str);
    }

    Ok(())
}

pub(crate) fn print(p0: PrintArgs) -> Result<()> {
    use crate::png::Png;
    let file_path = p0.file_path;

    let png = Png::from_file(&file_path)?;

    for chunk in png.chunks() {
        println!("Chunk Type: {}, Length: {}", chunk.chunk_type(), chunk.data().len());
        if let Ok(data_str) = chunk.data_as_string() {
            println!("Data: {}", data_str);
        } else {
            println!("Data: [binary data]");
        }
    }
    Ok(())
}

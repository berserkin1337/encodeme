use crate::chunk::Chunk;
use crate::chunk_type;
use crate::png;
use anyhow::Context;
use anyhow::Error;
use clap::ArgMatches;
use std::str::FromStr;
use std::result::Result::Ok;
pub fn encode(matches: &ArgMatches) -> Result<(), Error> {
    let mut img = png::Png::from_path(matches.value_of("path").unwrap())?;
    let chunk_type = chunk_type::ChunkType::from_str(matches.value_of("chunk_type").unwrap());
    let chunk_type = match chunk_type {
        Ok(tp) => tp,
        Err(err) => panic!("Invalid chunk type: {:?}", err),
    };

    let message = matches.value_of("message").unwrap().as_bytes();
    let chunk = Chunk::new(chunk_type, message.to_vec());
    img.append_chunk(chunk);
    let output_path = matches
        .value_of("output")
        .unwrap_or_else(|| matches.value_of("path").unwrap());
    std::fs::write(output_path, img.as_bytes())
        .with_context(|| format!("Could not write to {}", output_path))?;
    Ok(())
}

pub fn decode(matches: &ArgMatches) -> Result<(), Error> {
    let mut img = png::Png::from_path(matches.value_of("path").unwrap())?;
    // find chunk with given type
    let chunk = img.chunk_by_type(matches.value_of("chunk_type").unwrap());
    // check if the chunk is null
    let chunk = match chunk {
        Some(chunk) => chunk,
        None => {
            println!(
                "No chunk found with type {}",
                matches.value_of("chunk_type").unwrap()
            );
            return Ok(());
        }
    };
    // print the message
    println!("{}", String::from_utf8(chunk.data().to_vec())?);
    Ok(())
}

pub fn remove(matches: &ArgMatches) -> Result<(), Error> {
    let mut img = png::Png::from_path(matches.value_of("path").unwrap())?;
    let deleted_chunk = img.delete_chunk(matches.value_of("chunk_type").unwrap());

    let chunk = match deleted_chunk {
        Ok(chunk) => chunk,
        Err(err) => panic!("Could not find chunk: {:?}", err),
    };

    let message = chunk.data_as_string()?;
    println!("{}", message);
    let output_path = matches.value_of("path").unwrap();
    std::fs::write(output_path, img.as_bytes())
        .with_context(|| format!("Could not write to {}", output_path))?;
    Ok(())
}

// pub fn print(matches: &ArgMatches) -> Result<(), Error> {
//     let img = png::Png::from_path(matches.value_of("path").unwrap())?;
//     let indent = " ".repeat(4);
//     for (i, chunk) in img.chunks().iter().enumerate() {
//         println!("[{}] {}", i + 1, chunk);
//         if matches.verbose {
//             let chunk_type = chunk.chunk_type();
//             let is_reserved_bit_valid = chunk_type.is_reserved_bit_valid();
//             let is_safe_to_copy = chunk_type.is_safe_to_copy();
//             println!("{}is critical: {}", indent, chunk_type.is_critical());
//             println!("{}is public: {}", indent, chunk_type.is_public());
//             println!("{}has valid reserve bit: {}", indent, is_reserved_bit_valid);
//             println!("{}is safe to copy: {}", indent, is_safe_to_copy);
//             println!("{}crc as dec: {}", indent, chunk.crc());
//             println!("{}crc as hex: {:x}", indent, chunk.crc());
//         }
//     }

//     Ok(())
// }

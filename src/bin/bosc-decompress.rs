//! Decompression utility for BOSC files
//!
//! This utility reads a compressed BOSC file and decompresses it into the BOS format.
//! It can be used to convert BOSC files back to their original BOS format for further processing.
//!
//! # Usage
//! bosc-decompress <input_file> [output_file]

use boytacean::state::{SaveStateFormat, Serialize, StateManager};
use std::{env::args, error::Error, fs::File, io::Read, path::Path};

fn print_usage() {
    println!("Usage: bosc-decompress <input_file> [output_file]");
    println!("If output_file is not specified, it will use input_file with .bos extension");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let input_path = Path::new(&args[1]);
    let output_path = if args.len() > 2 {
        Path::new(&args[2]).to_path_buf()
    } else {
        let mut output = input_path.to_path_buf();
        output.set_extension("bos");
        output
    };

    println!(
        "Decompressing BOSC file to BOS format {}",
        output_path.display()
    );

    // read input file with the compressed values
    let mut input_file = File::open(input_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    // validates that input is BOSC format
    let format = StateManager::format(&input_data)?;
    if format != SaveStateFormat::Bosc {
        return Err("Input file is not in BOSC format".into());
    }

    // reads as BOSC and convert to BOS
    let mut bosc_state = StateManager::read_bosc(&input_data)?;
    let mut output_file = File::create(output_path)?;
    bosc_state.bos().write(&mut output_file)?;

    println!("Successfully decompressed BOSC file to BOS format");

    Ok(())
}

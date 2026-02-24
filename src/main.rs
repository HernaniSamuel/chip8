use chip8::chip8::{Chip8, Chip8Error};
use std::env;

fn main() {
    // Now, it'll run in the model "chip8 file.ch8"
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || !args[1].ends_with(".ch8") {
        let message = if args.len() != 2 {
            "ERROR: Must have only 2 args. \n    example: 'chip8 file.ch8'".to_string()
        } else {
            "ERROR: chip8 only accepts .ch8 files.".to_string()
        };
        eprintln!("{}", message);
        std::process::exit(1)
    }

    // With the .ch8 file, it's time to read and run it
    let file_name: String = args[1].clone();
    let rom = std::fs::read(&file_name).expect("Failed to read ROM");
    let chip = create_and_run_chip(&rom as &[u8]);
    match chip {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {:?}", e)
        }
    }
}

fn create_and_run_chip(rom: &[u8]) -> Result<(), Chip8Error> {
    let mut chip = Chip8::new(rom)?;
    chip.step()?;
    Ok(())
}

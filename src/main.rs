use chip8::chip8::{Chip8, Chip8Error};
use std::{
    env,
    time::{Duration, Instant},
};

fn main() -> Result<(), Chip8Error> {
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
    let mut chip = Chip8::new(&rom)?;

    // Chip-8 main loop
    let sixty_hz = Duration::from_micros(16_666); // 1/60s ≈ 16.666 ms
    let mut last_tick = Instant::now();
    while chip.display.is_open() {
        chip.update_keyboard();
        for _ in 0..10 {
            // more steps
            chip.step()?;
        }

        if chip.draw_flag {
            chip.display.render();
            chip.draw_flag = false;
        } else {
            chip.display.window_update();
        }

        if last_tick.elapsed() >= sixty_hz {
            chip.decrease_timers();
            last_tick = Instant::now();
        }
    }

    Ok(())
}

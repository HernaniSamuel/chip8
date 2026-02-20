use chip8::chip8::Chip8;
use std::thread;
use std::time::Duration;

fn main() {
    let mut chip8 = Chip8::new();

    // Adding a checkered pattern to see the screen and colors
    for y in 0..32 {
        for x in 0..64 {
            if (x + y) % 2 == 0 {
                chip8.display.set_pixel(y * 64 + x, 1).unwrap();
            }
        }
    }

    // Let's add some time in dt to see audio working
    chip8.set_st(150);

    // Main loop
    while chip8.display.is_open() {
        chip8.keyboard.update(chip8.display.window());

        for key in 0..16 {
            if chip8.keyboard.is_pressed(key).unwrap() {
                println!("Tecla 0x{:X} pressionada", key);
            }
        }
        
        if chip8.get_st() > &0  {
            chip8.audio.start_beep();
        } else {
            chip8.audio.stop_beep();
        }

        chip8.decrease_timers();
        chip8.display.render();

        thread::sleep(Duration::from_millis(16));
    }
}

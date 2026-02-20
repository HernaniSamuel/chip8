use chip8::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();

    for y in 0..32 {
        for x in 0..64 {
            if (x + y) % 2 == 0 {
                chip8.display.set_pixel(y * 64 + x, 1).unwrap();
            }
        }
    }

    while chip8.display.is_open() {
        chip8.display.render();
    }
}

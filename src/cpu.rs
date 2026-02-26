use crate::chip8::{Chip8, Chip8Error};

// Already implemented instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ClearDisplay,
    LoadVxByte(usize, u8),
    AddVxByte(usize, u8),
    SetI(u16),
    Draw(usize, usize, u8),
    Jump(u16),
}

// I decided to implement fetch, decode, execute and step here to avoid chip8.rs with 1000+ LOC
impl Chip8 {
    pub fn step(&mut self) -> Result<(), Chip8Error> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        self.execute(instruction)?;

        Ok(())
    }

    pub fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let high = self.get_ram(*self.get_pc())? as u16;
        let low = self.get_ram(*self.get_pc() + 1)? as u16;
        let opcode = (high << 8) | low;
        Ok(opcode)
    }

    pub fn decode(&self, opcode: u16) -> Result<Instruction, Chip8Error> {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => Ok(Instruction::ClearDisplay),
                _ => Err(Chip8Error::UnknownInstruction),
            },
            0x1000 => Ok(Instruction::Jump(nnn)),
            0x6000 => Ok(Instruction::LoadVxByte(x, nn)),
            0x7000 => Ok(Instruction::AddVxByte(x, nn)),
            0xA000 => Ok(Instruction::SetI(nnn)),
            0xD000 => Ok(Instruction::Draw(x, y, n)),
            _ => Err(Chip8Error::UnknownInstruction),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::ClearDisplay => {
                for i in 0..2048 {
                    self.display.set_pixel(i, 0)?;
                }
                self.increment_pc()?;
                self.draw_flag = true;
            }

            Instruction::LoadVxByte(x, nn) => {
                self.set_v(x, nn)?;
                self.increment_pc()?;
            }

            Instruction::SetI(nnn) => {
                self.set_i(nnn)?;
                self.increment_pc()?;
            }

            Instruction::AddVxByte(x, nn) => {
                self.set_v(x, self.get_v(x)? + nn)?;
                self.increment_pc()?;
            }

            // Draw instruction made by ChatGPT because IO isn't my focus
            Instruction::Draw(x_reg, y_reg, n) => {
                let vx = self.get_v(x_reg)? as usize;
                let vy = self.get_v(y_reg)? as usize;

                self.set_v(0xF, 0)?;

                for row in 0..n as usize {
                    let i = *self.get_i();
                    let sprite_byte = self.get_ram(i + row as u16)?;
                    for col in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 1;
                        if sprite_pixel == 0 {
                            continue;
                        }

                        let x = (vx + col) % 64;
                        let y = (vy + row) % 32;
                        let index = y * 64 + x;

                        let pixel = self.display.get_pixel(index)?;
                        if pixel == 1 {
                            self.set_v(0xF, 1)?;
                        }
                        self.display.set_pixel(index, pixel ^ sprite_pixel)?;
                    }
                }

                self.draw_flag = true;
                self.increment_pc()?;
            }

            Instruction::Jump(nnn) => {
                self.set_pc(nnn)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_instruction() {
        let rom: &[u8] = &[0x00, 0xE0]; // 0x00E0 clear instruction code
        let mut chip = Chip8::new(rom).unwrap();
        assert_eq!(chip.fetch().unwrap(), 0x00E0);
        assert_eq!(chip.decode(0x00E0).unwrap(), Instruction::ClearDisplay);

        // Adding a checkered pattern to populate pixels and test execute clear
        for y in 0..32 {
            for x in 0..64 {
                if (x + y) % 2 == 0 {
                    chip.display.set_pixel(y * 64 + x, 1).unwrap();
                }
            }
        }

        let old_pc = *chip.get_pc();
        chip.execute(Instruction::ClearDisplay).unwrap();
        for i in 0..2048 {
            assert_eq!(chip.display.get_pixel(i).unwrap(), 0);
        }

        // Testing PC increment
        assert_eq!(*chip.get_pc(), old_pc + 2);

        // Testing draw_flag
        assert_eq!(chip.draw_flag, true);
    }
}

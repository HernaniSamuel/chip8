use crate::chip8::{Chip8, Chip8Error};

// I decided to implement fetch, decode, execute and step here to avoid chip8.rs with 1000+ LOC
impl Chip8 {
    pub fn fetch(&mut self) -> Result<u16, Chip8Error> {
        let mut opcode: u16 = 0;
        let mut byte: u8 = self.get_ram(*self.get_pc())?;
        opcode += byte as u16;
        byte = self.get_ram(*self.get_pc() + 1)?;
        opcode += byte as u16;
        Ok(opcode)
    }

    pub fn decode(&self, opcode: u16) -> Result<Instruction, Chip8Error> {
        /*
           nibble = 4 bits, an opcode can be divided in 4 nibbles
           x = secound nibble
           y = third nibble
           n = last nibble
           nn = last 2 nibbles
           nnn = last 3 nibbles
        */
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode {
            0x00E0 => Ok(Instruction::Clear),
            /* instructions used in ibm-logo.ch8 (https://github.com/Timendus/chip8-test-suite/blob/main/README.md). I'm using this rom to have a first view of my emulator working
            00E0 - Clear the screen
            6xnn - Load normal register with immediate value
            Annn - Load index register with immediate value
            7xnn - Add immediate value to normal register
            Dxyn - Draw sprite to screen (un-aligned)
            */
            _ => Err(Chip8Error::UnknownInstruction),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> Result<(), Chip8Error> {
        match instruction {
            Instruction::Clear => {
                for i in 0..2048 {
                    self.display.set_pixel(i, 0)?;
                }
                self.set_pc(*self.get_pc() + 2)?;
                self.draw_flag = true;
            }
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Chip8Error> {
        let opcode = self.fetch()?;
        let instruction = self.decode(opcode)?;
        self.execute(instruction)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Clear,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clear_instruction() {
        let rom: &[u8] = &[0x00, 0xE0]; // 0x00E0 clear instruction code
        let mut chip = Chip8::new(rom).unwrap();
        assert_eq!(chip.fetch().unwrap(), 0x00E0);
        assert_eq!(chip.decode(0x00E0).unwrap(), Instruction::Clear);

        // Adding a checkered pattern to populate pixels and test execute clear
        for y in 0..32 {
            for x in 0..64 {
                if (x + y) % 2 == 0 {
                    chip.display.set_pixel(y * 64 + x, 1).unwrap();
                }
            }
        }

        let old_pc = *chip.get_pc();
        chip.execute(Instruction::Clear).unwrap();
        for i in 0..2048 {
            assert_eq!(chip.display.get_pixel(i).unwrap(), 0);
        }

        // Testing PC increment
        assert_eq!(*chip.get_pc(), old_pc + 2);

        // Testing draw_flag
        assert_eq!(chip.draw_flag, true);
    }
}

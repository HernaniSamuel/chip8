use crate::chip8::{Chip8, Chip8Error};

// Already implemented instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ClearDisplay,              // 00E0
    LoadVxByte(usize, u8),     // 6xnn
    AddVxByte(usize, u8),      // 7xnn
    SetI(u16),                 // Annn
    Draw(usize, usize, u8),    // Dxyn
    Jump(u16),                 // 1nnn
    JumpIfEq(usize, u8),       // 3xnn
    JumpIfDiff(usize, u8),     // 4xnn
    JumpIfVEq(usize, usize),   // 5xy0
    JumpIfVDiff(usize, usize), // 9xy0
    Call(u16),                 // 2nnn
    Return,                    // 00EE
    SetVxToVy(usize, usize),   // 8xy0
    VxEqVxORvy(usize, usize),  // 8xy1
    VxEqVxANDvy(usize, usize), // 8xy2
    VxEqVxXORvy(usize, usize), // 8xy3
    JimCarrey(usize, usize),   // 8xy4
    BorrowSub(usize, usize),   // 8xy5
    VyBorrowSub(usize, usize), // 8xy7
    VxRShift(usize),           // 8xy6
    VxLShift(usize),           // 8xyE
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
                0x00EE => Ok(Instruction::Return),
                _ => Err(Chip8Error::UnknownInstruction),
            },
            0x1000 => Ok(Instruction::Jump(nnn)),
            0x6000 => Ok(Instruction::LoadVxByte(x, nn)),
            0x7000 => Ok(Instruction::AddVxByte(x, nn)),
            0xA000 => Ok(Instruction::SetI(nnn)),
            0xD000 => Ok(Instruction::Draw(x, y, n)),
            0x3000 => Ok(Instruction::JumpIfEq(x, nn)),
            0x4000 => Ok(Instruction::JumpIfDiff(x, nn)),
            0x5000 => Ok(Instruction::JumpIfVEq(x, y)),
            0x9000 => Ok(Instruction::JumpIfVDiff(x, y)),
            0x2000 => Ok(Instruction::Call(nnn)),
            0x8000 => match n {
                0x0 => Ok(Instruction::SetVxToVy(x, y)),
                0x1 => Ok(Instruction::VxEqVxORvy(x, y)),
                0x2 => Ok(Instruction::VxEqVxANDvy(x, y)),
                0x3 => Ok(Instruction::VxEqVxXORvy(x, y)),
                0x4 => Ok(Instruction::JimCarrey(x, y)),
                0x5 => Ok(Instruction::BorrowSub(x, y)),
                0x7 => Ok(Instruction::VyBorrowSub(x, y)),
                0x6 => Ok(Instruction::VxRShift(x)),
                0xE => Ok(Instruction::VxLShift(x)),
                _ => Err(Chip8Error::UnknownInstruction),
            },

            _ => Err(Chip8Error::UnknownInstruction),
            /* Next instructions
               |3xnn    |2nnn    |8xy4    Fx55
               |4xnn    |00EE    |8xy5    Fx33
               |5xy0    |8xy0    |8xy7    Fx1E
               |7xnn    |8xy1    |8xy6    Registers
               |9xy0    |8xy2    |8xyE
               |1nnn    |8xy3    Fx65
            */
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

            Instruction::JumpIfEq(x, nn) => {
                if self.get_v(x)? == nn {
                    self.increment_pc()?;
                    self.increment_pc()?;
                    // If vx == nn, skip next instruction
                } else {
                    self.increment_pc()?;
                    // else, just increment normally
                }
            }

            Instruction::JumpIfDiff(x, nn) => {
                if self.get_v(x)? != nn {
                    self.increment_pc()?;
                    self.increment_pc()?;
                    // If vx != nn, skip next instruction
                } else {
                    self.increment_pc()?;
                    // else, just increment normally
                }
            }

            Instruction::JumpIfVEq(x, y) => {
                if self.get_v(x)? == self.get_v(y)? {
                    self.increment_pc()?;
                    self.increment_pc()?;
                    // If vx == vy, skip next instruction
                } else {
                    self.increment_pc()?;
                    // else, just increment normally
                }
            }

            Instruction::JumpIfVDiff(x, y) => {
                if self.get_v(x)? != self.get_v(y)? {
                    self.increment_pc()?;
                    self.increment_pc()?;
                    // If vx != vy, skip next instruction
                } else {
                    self.increment_pc()?;
                    // else, just increment normally
                }
            }

            Instruction::Call(nnn) => {
                self.increment_pc()?;
                let pc = *self.get_pc();
                self.push_stack(pc)?;
                self.set_pc(nnn)?;
            }

            Instruction::Return => {
                let addr = self.pop_stack()?;
                self.set_pc(addr)?;
            }

            Instruction::SetVxToVy(x, y) => {
                self.set_v(x, self.get_v(y)?)?;
                self.increment_pc()?;
            }

            Instruction::VxEqVxORvy(x, y) => {
                let value = self.get_v(x)? | self.get_v(y)?;
                self.set_v(x, value)?;
                self.increment_pc()?;
            }

            Instruction::VxEqVxANDvy(x, y) => {
                let value = self.get_v(x)? & self.get_v(y)?;
                self.set_v(x, value)?;
                self.increment_pc()?;
            }

            Instruction::VxEqVxXORvy(x, y) => {
                let value = self.get_v(x)? ^ self.get_v(y)?;
                self.set_v(x, value)?;
                self.increment_pc()?;
            }

            Instruction::JimCarrey(x, y) => {
                let result: u16 = self.get_v(x)? as u16 + self.get_v(y)? as u16;
                self.set_v(x, (result & 0xFF) as u8)?;
                self.set_v(0xF, if result > 255 { 1 } else { 0 })?;
                self.increment_pc()?;
            }

            Instruction::BorrowSub(x, y) => {
                let vx = self.get_v(x)?;
                let vy = self.get_v(y)?;
                self.set_v(x, vx.wrapping_sub(vy))?;
                self.set_v(0xF, if vx >= vy { 1 } else { 0 })?;
                self.increment_pc()?;
            }

            Instruction::VyBorrowSub(x, y) => {
                let vx = self.get_v(x)?;
                let vy = self.get_v(y)?;
                self.set_v(x, vy.wrapping_sub(vx))?;
                self.set_v(0xF, if vy >= vx { 1 } else { 0 })?;
                self.increment_pc()?;
            }

            Instruction::VxRShift(x) => {
                let vx = self.get_v(x)?;
                self.set_v(x, vx >> 1)?;
                self.set_v(0xF, vx & 0x1)?;
                self.increment_pc()?;
            }

            Instruction::VxLShift(x) => {
                let vx = self.get_v(x)?;
                self.set_v(x, vx << 1)?;
                self.set_v(0xF, (vx & 0x80) >> 7)?;
                self.increment_pc()?;
            }
        }

        Ok(())
    }
}

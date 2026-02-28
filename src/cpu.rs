use crate::chip8::{Chip8, Chip8Error};

/*  HEX     DESCRIPTION                                   ENUM NAME

    00E0    Clear the display                              ClearDisplay
    00EE    Return from subroutine                         Return

    1nnn    Jump to address nnn                            Jump
    2nnn    Call subroutine at nnn                         Call

    3xnn    Skip next instruction if Vx == nn              JumpIfEq
    4xnn    Skip next instruction if Vx != nn              JumpIfDiff
    5xy0    Skip next instruction if Vx == Vy              JumpIfVEq

    6xnn    Set Vx = nn                                    LoadVxByte
    7xnn    Set Vx = Vx + nn                               AddVxByte

    8xy0    Set Vx = Vy                                    SetVxToVy
    8xy1    Set Vx = Vx OR Vy                              VxEqVxORvy
    8xy2    Set Vx = Vx AND Vy                             VxEqVxANDvy
    8xy3    Set Vx = Vx XOR Vy                             VxEqVxXORvy
    8xy4    Set Vx = Vx + Vy, set VF = carry               JimCarrey
    8xy5    Set Vx = Vx - Vy, set VF = NOT borrow          BorrowSub
    8xy6    Set Vx = Vx >> 1, VF = least significant bit   VxRShift
    8xy7    Set Vx = Vy - Vx, set VF = NOT borrow          VyBorrowSub
    8xyE    Set Vx = Vx << 1, VF = most significant bit    VxLShift

    9xy0    Skip next instruction if Vx != Vy              JumpIfVDiff

    Annn    Set I = nnn                                    SetI
    Bnnn    Jump to address nnn + V0                       JumpV0

    Cxnn    Set Vx = random byte AND nn                    Random

    Dxyn    Draw sprite at (Vx, Vy) with height n          Draw

    Ex9E    Skip next instruction if key[Vx] is pressed    SkipIfKeyPressed
    ExA1    Skip next instruction if key[Vx] not pressed   SkipIfKeyNotPressed

    Fx07    Set Vx = delay timer value                     LoadDelayTimer
    Fx0A    Wait for key press, store key in Vx            WaitKey
    Fx15    Set delay timer = Vx                           SetDelayTimer
    Fx18    Set sound timer = Vx                           SetSoundTimer
    Fx1E    Set I = I + Vx                                 AddVxI
    Fx29    Set I to sprite location for digit Vx          LoadFont
    Fx33    Store BCD of Vx in memory at I                 BCD
    Fx55    Store V0..Vx in memory starting at I           StoreMemV
    Fx65    Load V0..Vx from memory starting at I          LoadMemV
*/

// Already implemented instructions
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    ClearDisplay,               // 00E0
    LoadVxByte(usize, u8),      // 6xnn
    AddVxByte(usize, u8),       // 7xnn
    SetI(u16),                  // Annn
    Draw(usize, usize, u8),     // Dxyn
    Jump(u16),                  // 1nnn
    JumpIfEq(usize, u8),        // 3xnn
    JumpIfDiff(usize, u8),      // 4xnn
    JumpIfVEq(usize, usize),    // 5xy0
    JumpIfVDiff(usize, usize),  // 9xy0
    Call(u16),                  // 2nnn
    Return,                     // 00EE
    SetVxToVy(usize, usize),    // 8xy0
    VxEqVxORvy(usize, usize),   // 8xy1
    VxEqVxANDvy(usize, usize),  // 8xy2
    VxEqVxXORvy(usize, usize),  // 8xy3
    JimCarrey(usize, usize),    // 8xy4
    BorrowSub(usize, usize),    // 8xy5
    VyBorrowSub(usize, usize),  // 8xy7
    VxRShift(usize),            // 8xy6
    VxLShift(usize),            // 8xyE
    AddVxI(usize),              // Fx1E
    LoadMemV(usize),            // Fx65
    StoreMemV(usize),           // Fx55
    BCD(usize),                 // Fx33
    SkipIfKeyPressed(usize),    // Ex9E
    SkipIfKeyNotPressed(usize), // ExA1
    JumpV0(u16),                // Bnnn
    Random(usize, u8),          // Cxnn
    LoadDelayTimer(usize),      // Fx07
    WaitKey(usize),             // Fx0A
    SetDelayTimer(usize),       // Fx15
    SetSoundTimer(usize),       // Fx18
    LoadFont(usize),            // Fx29
}

// I decided to implement fetch, decode, execute and step here to avoid chip8.rs with 1000+ LOC
impl Chip8 {
    pub fn step(&mut self) -> Result<(), Chip8Error> {
        let opcode = self.fetch()?;
        // println!("PC: {:#X} | Opcode: {:#X}", self.get_pc(), opcode); // uncomment to see pc position and actual opcode
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

            0xF000 => match nn {
                0x1E => Ok(Instruction::AddVxI(x)),
                0x65 => Ok(Instruction::LoadMemV(x)),
                0x55 => Ok(Instruction::StoreMemV(x)),
                0x33 => Ok(Instruction::BCD(x)),
                0x07 => Ok(Instruction::LoadDelayTimer(x)),
                0x0A => Ok(Instruction::WaitKey(x)),
                0x15 => Ok(Instruction::SetDelayTimer(x)),
                0x18 => Ok(Instruction::SetSoundTimer(x)),
                0x29 => Ok(Instruction::LoadFont(x)),
                _ => Err(Chip8Error::UnknownInstruction),
            },

            0xE000 => match nn {
                0x9E => Ok(Instruction::SkipIfKeyPressed(x)),
                0xA1 => Ok(Instruction::SkipIfKeyNotPressed(x)),
                _ => Err(Chip8Error::UnknownInstruction),
            },

            0xB000 => Ok(Instruction::JumpV0(nnn)),
            0xC000 => Ok(Instruction::Random(x, nn)),

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
                self.set_v(x, self.get_v(x)?.wrapping_add(nn))?;
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

            Instruction::AddVxI(x) => {
                let vx = self.get_v(x)? as u16;
                self.set_i(*self.get_i() + vx)?;
                self.increment_pc()?;
            }

            Instruction::LoadMemV(x) => {
                let i = *self.get_i();
                for index in 0..=x {
                    self.set_v(index, self.get_ram(i + index as u16)?)?;
                }
                self.increment_pc()?;
            }

            Instruction::StoreMemV(x) => {
                let i = *self.get_i();
                for index in 0..=x {
                    self.set_ram(index as u16 + i, self.get_v(index)?)?;
                }
                self.increment_pc()?;
            }

            Instruction::BCD(x) => {
                let i = *self.get_i();
                let v = self.get_v(x)?;
                self.set_ram(i, v / 100)?;
                self.set_ram(i + 1, (v / 10) % 10)?;
                self.set_ram(i + 2, v % 10)?;
                self.increment_pc()?;
            }

            Instruction::SkipIfKeyPressed(x) => {
                self.set_pc(if self.keyboard.is_pressed(self.get_v(x)? as usize)? {
                    *self.get_pc() + 4
                } else {
                    *self.get_pc() + 2
                })?;
            }

            Instruction::SkipIfKeyNotPressed(x) => {
                self.set_pc(if !self.keyboard.is_pressed(self.get_v(x)? as usize)? {
                    *self.get_pc() + 4
                } else {
                    *self.get_pc() + 2
                })?;
            }

            Instruction::JumpV0(nnn) => {
                self.set_pc(nnn + self.get_v(0)? as u16)?;
            }

            Instruction::Random(x, nn) => {
                let random: u8 = rand::random();
                self.set_v(x, random & nn)?;
                self.increment_pc()?;
            }

            Instruction::LoadDelayTimer(x) => {
                let delay_timer = *self.get_dt();
                self.set_v(x, delay_timer)?;
                self.increment_pc()?;
            }

            Instruction::WaitKey(x) => {
                if let Some(key) = self.keyboard.get_pressed_key() {
                    self.set_v(x, key as u8)?;
                    self.increment_pc()?;
                }
            }

            Instruction::SetDelayTimer(x) => {
                self.set_dt(self.get_v(x)?);
                self.increment_pc()?;
            }

            Instruction::SetSoundTimer(x) => {
                self.set_st(self.get_v(x)?);
                self.increment_pc()?;
            }

            Instruction::LoadFont(x) => {
                let digit = self.get_v(x)? as u16;
                self.set_i(0x50 + digit * 5)?;
                self.increment_pc()?;
            }
        }

        Ok(())
    }
}

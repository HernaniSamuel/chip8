#[derive(Debug, Clone)]
pub enum Chip8Error {
    PCOutOfBounds,
    IOutOfBounds,
    InvalidMemoryAccess,
    StackOverflow,
    StackUnderflow,
    InvalidRegisterAccess,
    InvalidPixelAccess,
    InvalidKeyAccess,
}

pub struct Chip8 {
    // Program Counter, points to the next instruction in ram
    pc: u16,

    // Registers
    v: [u8; 16],

    // stack memory and pointer
    sp: u8,
    stack: [u16; 16],

    // ram memory and pointer (I doesn't point to instructions, only for normal memory)
    i: u16,
    ram: [u8; 4096],

    // display buffer
    display: [u8; 64 * 32],

    // keyboard buffer
    keyboard: [bool; 16],

    // delay timer
    dt: u8,

    // sound timer
    st: u8,
}

// the chip8 impl only worry about safe state transition of its attributes, the logic beyond the changes isn't resposability of this impl
impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            pc: 0x200,
            v: [0; 16],
            sp: 0,
            stack: [0; 16],
            i: 0,
            ram: [0; 4096],
            display: [0; 64 * 32],
            keyboard: [false; 16],
            dt: 0,
            st: 0,
        }
    }

    // Safe stack operations
    pub fn push_stack(&mut self, value: u16) -> Result<bool, Chip8Error> {
        if self.sp >= 16 {
            Err(Chip8Error::StackOverflow)
        } else {
            self.stack[self.sp as usize] = value;
            self.sp += 1;
            Ok(true)
        }
    }

    pub fn pop_stack(&mut self) -> Result<bool, Chip8Error> {
        if self.sp == 0 {
            Err(Chip8Error::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(true)
        }
    }

    // Safe PC operations
    pub fn set_pc(&mut self, value: u16) -> Result<bool, Chip8Error> {
        if value > 4094 {
            Err(Chip8Error::PCOutOfBounds)
        } else {
            self.pc = value;
            Ok(true)
        }
    }

    // Safe I operations
    pub fn set_i(&mut self, value: u16) -> Result<bool, Chip8Error> {
        if value >= 4096 {
            Err(Chip8Error::IOutOfBounds)
        } else {
            self.i = value;
            Ok(true)
        }
    }

    // Safe ram usage
    pub fn get_ram(&self, index: u16) -> Result<u8, Chip8Error> {
        if index >= 4096 {
            Err(Chip8Error::InvalidMemoryAccess)
        } else {
            Ok(self.ram[index as usize])
        }
    }

    pub fn set_ram(&mut self, index: u16, value: u8) -> Result<bool, Chip8Error> {
        if index >= 4096 {
            Err(Chip8Error::InvalidMemoryAccess)
        } else {
            self.ram[index as usize] = value;
            Ok(true)
        }
    }

    // Safe V usage
    pub fn get_v(&self, index: usize) -> Result<u8, Chip8Error> {
        if index >= 16 {
            Err(Chip8Error::InvalidRegisterAccess)
        } else {
            Ok(self.v[index])
        }
    }

    pub fn set_v(&mut self, index: usize, value: u8) -> Result<bool, Chip8Error> {
        if index >= 16 {
            Err(Chip8Error::InvalidRegisterAccess)
        } else {
            self.v[index] = value;
            Ok(true)
        }
    }

    // Safe screen usage
    pub fn get_pixel(&self, index: usize) -> Result<u8, Chip8Error> {
        if index >= 64 * 32 {
            Err(Chip8Error::InvalidPixelAccess)
        } else {
            Ok(self.display[index])
        }
    }

    pub fn set_pixel(&mut self, index: usize, value: u8) -> Result<bool, Chip8Error> {
        if index >= 64 * 32 {
            Err(Chip8Error::InvalidPixelAccess)
        } else {
            self.display[index] = value;
            Ok(true)
        }
    }

    // Safe keyboard usage
    pub fn get_key_state(&self, index: usize) -> Result<bool, Chip8Error> {
        if index >= 16 {
            Err(Chip8Error::InvalidKeyAccess)
        } else {
            Ok(self.keyboard[index])
        }
    }

    pub fn set_key_state(&mut self, index: usize, value: bool) -> Result<bool, Chip8Error> {
        if index >= 16 {
            Err(Chip8Error::InvalidKeyAccess)
        } else {
            self.keyboard[index] = value;
            Ok(true)
        }
    }

    // Set and decrease timers
    pub fn set_dt(&mut self, value: u8) {
        self.dt = value;
    }

    pub fn set_st(&mut self, value: u8) {
        self.st = value;
    }

    pub fn decrease_timers(&mut self) {
        self.st = if self.st > 0 { self.st - 1 } else { self.st };
        self.dt = if self.dt > 0 { self.dt - 1 } else { self.dt };
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // testing stack safety
    #[test]
    fn test_stack_underflow() {
        let mut chip = Chip8::new();
        assert!(chip.pop_stack().is_err());
    }

    #[test]
    fn test_stack_overflow() {
        let mut chip = Chip8::new();
        for i in 0..16 {
            chip.push_stack(i).unwrap();
        }
        assert!(chip.push_stack(0).is_err());
    }

    #[test]
    fn test_stack_push() {
        let mut chip = Chip8::new();
        assert!(chip.push_stack(11).is_ok());
        assert_eq!(chip.stack[chip.sp as usize - 1], 11);
    }

    #[test]
    fn test_stack_pop() {
        let mut chip = Chip8::new();
        chip.push_stack(101).unwrap();
        chip.pop_stack().unwrap();
        assert_eq!(chip.sp, 0);
    }

    #[test]
    fn test_stack_push_pop_sequence() {
        let mut chip = Chip8::new();
        chip.push_stack(11).unwrap();
        chip.push_stack(22).unwrap();
        chip.pop_stack().unwrap();
        assert_eq!(chip.sp, 1);
        chip.pop_stack().unwrap();
        assert_eq!(chip.sp, 0);
    }

    #[test]
    fn test_stack_full_cycle() {
        let mut chip =Chip8::new();
        for i in 0..16 {
            chip.push_stack(i).unwrap();
        }
        for _ in 0..16 {
            chip.pop_stack().unwrap();
        }
        assert_eq!(chip.sp, 0);
        assert!(chip.push_stack(99).is_ok());
    }
}

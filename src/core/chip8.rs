#[derive(Debug, Clone)]
pub enum Chip8Error {
    PCOutOfBounds,
    IOutOfBounds,
    InvalidMemoryAccess,
    StackOverflow,
    StackUnderflow,
    InvalidRegisterAccess,
    InvalidPixelAccess,
    InvalidPixelValue,
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
        if self.pc > 4094 {
            Err(Chip8Error::PCOutOfBounds)
        } else {
            self.pc = value;
            Ok(true)
        }
    }

    // Safe I operations
    pub fn set_i(&mut self, value: u16) -> Result<bool, Chip8Error> {
        self.i = value;
        if self.i >= 4096 {
            Err(Chip8Error::IOutOfBounds)
        } else {
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
            if value == 1 || value == 0 {
                self.display[index] = value;
                Ok(true)
            } else {
                Err(Chip8Error::InvalidPixelValue)
            }
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
        let mut chip = Chip8::new();
        for i in 0..16 {
            chip.push_stack(i).unwrap();
        }
        for _ in 0..16 {
            chip.pop_stack().unwrap();
        }
        assert_eq!(chip.sp, 0);
        assert!(chip.push_stack(99).is_ok());
    }

    // testing PC safety
    #[test]
    fn test_pc_out_of_boundaries() {
        let mut chip = Chip8::new();
        assert!(chip.set_pc(4095).is_ok());
        assert!(chip.set_pc(4096).is_err());
    }

    #[test]
    fn test_pc_value_alteration() {
        let mut chip = Chip8::new();
        chip.set_pc(1).unwrap();
        assert_eq!(chip.pc, 1);
        chip.set_pc(0x200).unwrap();
        assert_eq!(chip.pc, 0x200);
        chip.set_pc(0x10).unwrap();
        assert_eq!(chip.pc, 0x10);
        chip.set_pc(0).unwrap();
        assert_eq!(chip.pc, 0);
        assert!(chip.set_pc(4096).is_ok());
        assert_eq!(chip.pc, 4096);
    }

    // testing I safety
    #[test]
    fn test_i_out_of_boundaries() {
        let mut chip = Chip8::new();
        assert!(chip.set_i(4095).is_ok());
        assert!(chip.set_i(4096).is_err());
    }

    #[test]
    fn test_i_value_alteration() {
        let mut chip = Chip8::new();
        chip.set_i(1).unwrap();
        assert_eq!(chip.i, 1);
        chip.set_i(0x200).unwrap();
        assert_eq!(chip.i, 0x200);
        chip.set_i(0x10).unwrap();
        assert_eq!(chip.i, 0x10);
        chip.set_i(0).unwrap();
        assert_eq!(chip.i, 0);
    }

    // Testing safe ram handling
    #[test]
    fn test_invalid_memory_access() {
        let mut chip = Chip8::new();
        assert!(chip.get_ram(4096).is_err());
        assert!(chip.get_ram(4095).is_ok());
        assert!(chip.set_ram(4096, 255).is_err());
        assert!(chip.set_ram(4095, 255).is_ok());
    }

    #[test]
    fn test_ram_full_use() {
        let mut chip = Chip8::new();
        let mut j: u8 = 0;
        for i in 0..4096 {
            assert!(chip.set_ram(i, j).is_ok());
            j = j.wrapping_add(1);
        }
        j = 0;
        for i in 0..4096 {
            assert_eq!(chip.get_ram(i).unwrap(), j);
            j = j.wrapping_add(1);
        }
    }

    // Testing keyboard safety
    #[test]
    fn test_keypad_full_use() {
        let mut chip = Chip8::new();
        for i in 0..16 {
            assert!(chip.set_key_state(i, true).is_ok());
        }
        for i in 0..16 {
            assert_eq!(chip.get_key_state(i).unwrap(), true);
        }
        for i in 0..16 {
            assert!(chip.set_key_state(i, false).is_ok());
        }
        for i in 0..16 {
            assert_eq!(chip.get_key_state(i).unwrap(), false);
        }
        assert!(chip.get_key_state(16).is_err());
    }

    // Testing screen safety
    #[test]
    fn test_screen_full_use() {
        let mut chip = Chip8::new();
        // Changes every pixel to 1
        for i in 0..2048 { // 64*32 = 2048
            assert!(chip.set_pixel(i, 1).is_ok());
        }
        // Check if limits are being checked
        assert!(chip.set_pixel(2048, 1).is_err());
        assert!(chip.set_pixel(0, 2).is_err());
        // Check if all pixels have the value of 1
        for i in 0..2048 {
            assert_eq!(chip.get_pixel(i).unwrap(), 1);
        }
        // These 2 for loops set every pixel to 0 and check if it's really 0
        for i in 0..2048 {
            assert!(chip.set_pixel(i, 0).is_ok());
        }
        for i in 0..2048 {
            assert_eq!(chip.get_pixel(i).unwrap(), 0);
        }
    }

    // Testing V safety
    #[test]
    fn test_v_full_use() {
        let mut chip = Chip8::new();
        // test error case
        assert!(chip.set_v(16, 255).is_err());
        assert!(chip.get_v(16).is_err());
        // test full use
        for i in 0..16 {
            assert!(chip.set_v(i, i as u8).is_ok());
        }
        for i in 0..16 {
            assert_eq!(chip.get_v(i).unwrap(), i as u8);
        }
    }

    // testing dt and st 
    #[test]
    fn test_timers() {
        let mut chip = Chip8::new();
        chip.set_dt(250);
        chip.set_st(150);
        for _ in 0..256 {
            chip.decrease_timers();
        }
        assert_eq!(chip.dt, 0);
        assert_eq!(chip.st, 0);
    }
}

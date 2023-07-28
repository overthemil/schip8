mod opcodes;

use crate::errors::ChipError;
use crate::Screen;
use opcodes::Opcode;
use opcodes::execute;

const NUM_REGISTERS: usize = 0x10;
const STACK_SIZE: usize = 16;

pub struct Cpu {
    // Registers
    pub v: [u8; NUM_REGISTERS],
    pub i: u16,
    pub pc: usize,
    pub sp: usize,
    pub timer_delay: u8,
    pub timer_sound: u8,
    pub stack: [u16; STACK_SIZE],
}

impl Cpu {
    pub fn push(&mut self, value: u16) -> Result<(), ChipError> {
        if self.sp == (STACK_SIZE - 1) {
            return Err(ChipError::StackOverflow(self.stack.len()));
        }

        self.stack[self.sp] = value;
        self.sp += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, ChipError> {
        if self.sp == 0 {
            return Err(ChipError::StackUnderflow());
        }

        self.sp -= 1;
        let value = self.stack[self.sp];

        Ok(value)
    }

    pub fn step(&mut self, memory: &mut [u8], screen: &mut Screen) -> Result<(), ChipError> {
        // Fetch
        let opcode_hex = self.fetch(memory)?;

        // Decode
        let opcode = Opcode::from(opcode_hex);

        // Execute
        execute(opcode, self, memory, screen)?;

        Ok(())
    }

    fn fetch(&mut self, memory: &mut [u8]) -> Result<u16, ChipError> {
        if (self.pc + 1) >= memory.len() {
            return Err(ChipError::AddressOutOfBounds{address: self.pc + 1, limit: memory.len()});
        }

        let hi = memory[self.pc] as u16;
        let lo = memory[self.pc + 1] as u16; 

        // The CHIP-8 is big endian
        let opcode: u16 = (hi << 8) | lo;
        self.pc += 2;
        
        Ok(opcode)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            v: [0; NUM_REGISTERS],
            i: 0,
            pc: 0,
            sp: 0,
            timer_delay: 0,
            timer_sound: 0,
            stack: [0; STACK_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut cpu = Cpu::default();
        assert_eq!(cpu.stack[cpu.sp], 0);

        cpu.push(1).unwrap();
        assert_eq!(cpu.stack[cpu.sp - 1], 1);
        assert_eq!(cpu.sp, 1);

        cpu.push(5).unwrap();
        assert_eq!(cpu.stack[cpu.sp - 1], 5);
        assert_eq!(cpu.sp, 2);

        cpu.sp = 15;
        let e = cpu.push(1); 
        assert!(matches!(e, Err(ChipError::StackOverflow(_))));
    } 

    #[test]
    fn pop() {
        let mut cpu = Cpu::default();

        let e = cpu.pop(); 
        assert!(matches!(e, Err(ChipError::StackUnderflow())));

        cpu.push(1).unwrap();
        cpu.push(2).unwrap();
        cpu.push(3).unwrap();

        let val = cpu.pop().unwrap();
        assert_eq!(val, 3);
        assert_eq!(cpu.sp, 2);
        let val = cpu.pop().unwrap();
        assert_eq!(val, 2);
        assert_eq!(cpu.sp, 1);
        let val = cpu.pop().unwrap();
        assert_eq!(val, 1);
        assert_eq!(cpu.sp, 0);

        let e = cpu.pop(); 
        assert!(matches!(e, Err(ChipError::StackUnderflow())));
    }
}

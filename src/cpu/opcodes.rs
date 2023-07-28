use rand::Rng;

use crate::errors::ChipError;
use crate::Screen;
use super::Cpu;
use crate::memory::FONT_BASE_ADDRESS;

pub struct Opcode {
    hex: u16,

    // First nibble
    prefix: u8,
    // Second nibble
    pub x: u8,
    // Third nibble
    pub y: u8,
    // Fourth nibble
    pub n: u8,
    // Second + Third nibble
    pub nn: u8,
    // Second + Third + Fourth nibble
    nnn: u16,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Opcode {
           hex: value,
           prefix: (value >> 12) as u8,
           x: ((value & 0x0F00) >> 8) as u8,
           y: ((value & 0x00F0) >> 4) as u8,
           n: (value & 0xF) as u8,
           nn: (value & 0xFF) as u8,
           nnn: value & 0xFFF,
       }
    }
}

pub fn execute(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8], screen: &mut Screen) -> Result<(), ChipError> {
    let mut rng = rand::thread_rng();

    match opcode.prefix {
        0x0 => execute_prefix_0(opcode, cpu, screen)?,
        0x1 => cpu.pc = opcode.nnn as usize, 
        0x2 => call_subroutine(opcode, cpu)?,
        0x3 => skip_if(cpu.v[opcode.x as usize] == opcode.nn, cpu),
        0x4 => skip_if(cpu.v[opcode.x as usize] != opcode.nn, cpu),
        0x5 => skip_if(cpu.v[opcode.x as usize] == cpu.v[opcode.y as usize], cpu),
        0x6 => cpu.v[opcode.x as usize] = opcode.nn,
        0x7 => cpu.v[opcode.x as usize] = cpu.v[opcode.x as usize].wrapping_add(opcode.nn),
        0x8 => execute_prefix_8(opcode, cpu)?,
        0x9 => skip_if(cpu.v[opcode.x as usize] != cpu.v[opcode.y as usize], cpu),
        0xA => cpu.i = opcode.nnn,
        0xB => cpu.pc = opcode.nnn as usize + cpu.v[0] as usize,
        0xC => cpu.v[opcode.x as usize] = rng.gen_range(0x00..0xFF) & opcode.nn,
        0xD => draw_sprite(opcode, cpu, memory, screen)?,
        0xE => execute_prefix_e(opcode, cpu)?,
        0xF => execute_prefix_f(opcode, cpu, memory)?,
        _ => { return Err(ChipError::OpcodeNotImplemented{opcode: opcode.hex}) }
    }

    Ok(())
}

fn execute_prefix_0(opcode: Opcode, cpu: &mut Cpu, screen: &mut Screen) -> Result<(), ChipError> {
    match opcode.hex {
        0x00E0 => screen.clear_screen(),
        0x00EE => cpu.pc = cpu.pop()? as usize,
        _ => () 
    }

    Ok(())
}

fn execute_prefix_8(opcode: Opcode, cpu: &mut Cpu) -> Result<(), ChipError> {
    match opcode.n {
        0x0 => cpu.v[opcode.x as usize] = cpu.v[opcode.y as usize],
        0x1 => cpu.v[opcode.x as usize] |= cpu.v[opcode.y as usize],
        0x2 => cpu.v[opcode.x as usize] &= cpu.v[opcode.y as usize],
        0x3 => cpu.v[opcode.x as usize] ^= cpu.v[opcode.y as usize],
        0x4 => add_registers(opcode.x, opcode.y, cpu), 
        0x5 => sub_registers(opcode.x, opcode.x, opcode.y, cpu), 
        0x6 => shift_right(opcode.x, cpu),
        0x7 => sub_registers(opcode.x, opcode.y, opcode.x, cpu),
        0xE => shift_left(opcode.x, cpu),
        _ => { return Err(ChipError::OpcodeNotImplemented{ opcode: opcode.hex }); }
    }

    Ok(())
}

fn execute_prefix_e(opcode: Opcode, cpu: &mut Cpu) -> Result<(), ChipError> {
    match opcode.hex & 0x00FF {
        0x9E => skip_if(cpu.keypad[opcode.x as usize], cpu), 
        0xA1 => skip_if(!cpu.keypad[opcode.x as usize], cpu), 
        _ => { return Err(ChipError::OpcodeNotImplemented{ opcode: opcode.hex }); }
    }
Ok(())
}

fn execute_prefix_f(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8]) -> Result<(), ChipError> {
    match opcode.hex & 0x00FF {
        0x07 => cpu.v[opcode.x as usize] = cpu.timer_delay,
        0x0A => get_input(opcode, cpu), 
        0x15 => cpu.timer_delay = cpu.v[opcode.x as usize],
        0x18 => cpu.timer_sound = cpu.v[opcode.x as usize],
        0x1E => cpu.i += cpu.v[opcode.x as usize] as u16,
        0x29 => cpu.i = (FONT_BASE_ADDRESS + (cpu.v[opcode.x as usize] as usize * 5)) as u16, 
        0x33 => store_bcd(opcode, cpu, memory),
        0x55 => store_registers(opcode, cpu, memory),
        0x65 => retrieve_registers(opcode, cpu, memory),
        _ => { return Err(ChipError::OpcodeNotImplemented{ opcode: opcode.hex }); }
    }

    Ok(())
}

fn call_subroutine(opcode: Opcode, cpu: &mut Cpu) -> Result<(), ChipError> {
   cpu.push(cpu.pc as u16)?;
   cpu.pc = opcode.nnn as usize;

   Ok(())
}

fn skip_if(skip: bool, cpu: &mut Cpu) {
    if skip {
        cpu.pc += 2;
    }
}

fn draw_sprite(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8], screen: &mut Screen) -> Result<(), ChipError> {
    let sprite_height = opcode.n as usize;  
    let sprite_x = cpu.v[opcode.x as usize] as usize;
    let sprite_y = cpu.v[opcode.y as usize] as usize;
    let sprite_base_addr = cpu.i as usize;
    let mut collided = false;

    for y in 0..sprite_height {
        let sprite_hslice: u8 = memory[sprite_base_addr + y]; 

        for x in 0..8 {
            if (sprite_hslice & (0x80 >> x)) != 0 {
                let pos_x = (sprite_x + x) % screen.width;
                let pos_y = (sprite_y + y) % screen.height;
                collided |= screen.get_pixel(pos_x, pos_y);
                screen.toggle_pixel(pos_x, pos_y);
            }
        }
    }

    if collided {
        cpu.v[0xF] = 1;
    } else {
        cpu.v[0xF] = 0;
    }

    Ok(())
}

fn add_registers(reg_lhs: u8, reg_rhs: u8, cpu: &mut Cpu) {
    let (sum, overflowed) = cpu.v[reg_lhs as usize].overflowing_add(cpu.v[reg_rhs as usize]);
    cpu.v[reg_lhs as usize] = sum;
    cpu.v[0xF] = match overflowed {
        true => 1,
        false => 0,
    };
}

fn sub_registers(reg_store: u8, reg_lhs: u8, reg_rhs: u8, cpu: &mut Cpu) {
    let (value, overflowed) = cpu.v[reg_lhs as usize].overflowing_sub(cpu.v[reg_rhs as usize]);
    cpu.v[reg_store as usize] = value;
    cpu.v[0xF] = match overflowed {
        true => 0,
        false => 1,
    };
}

fn shift_right(reg: u8, cpu: &mut Cpu) {
    cpu.v[0xF] = cpu.v[reg as usize] & 0x01;
    cpu.v[reg as usize] >>= 1; 
}

fn shift_left(reg: u8, cpu: &mut Cpu) {
    cpu.v[0xF] = (cpu.v[reg as usize] >> 7) & 0x01;
    cpu.v[reg as usize] <<= 1;
}

fn get_input(opcode: Opcode, cpu: &mut Cpu) {
    let input = cpu.keypad.iter().position(|&x| x);

    match input {
        Some(key) => cpu.v[opcode.x as usize] = key as u8,
        None => cpu.pc -= 2,
    };
}

fn store_bcd(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8]) {
    let (bcd2, bcd1, bcd0) = bcd(cpu.v[opcode.x as usize]);
    memory[cpu.i as usize] = bcd2;
    memory[cpu.i as usize + 1] = bcd1;
    memory[cpu.i as usize + 2] = bcd0;
}

fn bcd(input: u8) -> (u8, u8, u8) { 
    // Double Dabble Algorithm
    let mut bcd2: u8 = 0;
    let mut bcd1: u8 = 0;
    let mut bcd0: u8 = 0;
    let mut hex: u8 = input;

    for i in 0..8 {
        let input_bit = (hex >> 7) & 1;
        let bcd0_msb = (bcd0 >> 3) & 1;
        let bcd1_msb = (bcd1 >> 3) & 1;
        hex <<= 1;

        bcd0 = (bcd0 << 1) & 0x0F;
        bcd0 |= input_bit;
        if bcd0 >= 5 && i < 7 {
            bcd0 += 3;
        }

        bcd1 = (bcd1 << 1) & 0x0F;
        bcd1 |= bcd0_msb;
        if bcd1 >= 5 && i < 7 {
            bcd1 += 3;
        }

        bcd2 = (bcd2 << 1) & 0x0F;
        bcd2 |= bcd1_msb;
        if bcd2 >= 5 && i < 7 {
            bcd2 += 3;
        }
    }

    (bcd2, bcd1, bcd0)
}  

fn store_registers(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8]) {
    for i in 0..(opcode.x + 1) as usize {
        memory[cpu.i as usize + i] = cpu.v[i];
    }
}

fn retrieve_registers(opcode: Opcode, cpu: &mut Cpu, memory: &mut [u8]) {
    for i in 0..(opcode.x + 1) as usize {
        cpu.v[i] = memory[cpu.i as usize + i];
    }
}

#[cfg(test)]
mod tests {
    use super::bcd;
    use super::Cpu;
    use super::Screen;
    use crate::errors::ChipError;

    #[test]
    fn opcode_00e0() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x00, 0xe0, 0x00, 0x00];
        screen.set_pixel(5, 10);
        screen.set_pixel(50, 30);
       
        assert!(screen.get_pixel(5, 10));
        assert!(screen.get_pixel(50, 30));
        cpu.step(&mut memory, &mut screen).unwrap();
        assert!(!screen.get_pixel(5, 10));
        assert!(!screen.get_pixel(50, 30));
    }

    #[test]
    fn opcode_00ee() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x00, 0xee, 0x00, 0x00];

        assert_eq!(cpu.pc, 0);
        cpu.push(0x01).unwrap();
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.pc, 0x001);

        cpu.pc = 0x000;
        let e = cpu.step(&mut memory, &mut screen);
        assert!(matches!(e, Err(ChipError::StackUnderflow())));
    }

    #[test]
    fn opcode_1nnn() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x12, 0x34, 0x00, 0x00];

        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.pc, 0x234);
    }

    #[test]
    fn opcode_2nnn() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x00, 0x21, 0x23, 0x00];
        cpu.pc = 0x01;
        
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.pc, 0x123);
        assert_eq!(cpu.stack[cpu.sp], 0x03);
    }

    #[test]
    fn opcode_6xnn() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x62, 0xF1, 0x00, 0x00];

        assert_eq!(cpu.v[0x2], 0);
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.v[0x2], 0xF1);
    }

    #[test]
    fn opcode_7xnn() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0x75, 0xA1, 0x00, 0x00];

        cpu.v[0x5] = 0x32;
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.v[0x5], 0xD3);
    }

    #[test]
    fn opcode_annn() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 4] = [0xA1, 0x23, 0x00, 0x00];

        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.i, 0x123);
    }

    #[test]
    fn opcode_fx33() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 5] = [0xF0, 0x33, 0x00, 0x00, 0x00];

        cpu.v[0] = 123;
        cpu.i = 0x002;
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(memory[0x002], 0x01);
        assert_eq!(memory[0x003], 0x02);
        assert_eq!(memory[0x004], 0x03);

        cpu.v[0] = 0x97;
        cpu.i = 0x002;
        cpu.pc = 0;
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(memory[0x002], 0x01);
        assert_eq!(memory[0x003], 0x05);
        assert_eq!(memory[0x004], 0x01);
    }

    #[test]
    fn opcode_fx65() {
        let mut cpu = Cpu::default();
        let mut screen = Screen::default();
        let mut memory: [u8; 5] = [0xF2, 0x65, 0x01, 0x02, 0x03];

        cpu.i = 0x002;
        cpu.step(&mut memory, &mut screen).unwrap();
        assert_eq!(cpu.v[0], 0x01);
        assert_eq!(cpu.v[1], 0x02);
        assert_eq!(cpu.v[2], 0x03);
    }

    #[test]
    fn test_bcd() {
        let (bcd2, bcd1, bcd0) = bcd(123);
        assert_eq!(bcd2, 1);
        assert_eq!(bcd1, 2);
        assert_eq!(bcd0, 3);

        let (bcd2, bcd1, bcd0) = bcd(0x97);
        assert_eq!(bcd2, 1);
        assert_eq!(bcd1, 5);
        assert_eq!(bcd0, 1);
    }
}

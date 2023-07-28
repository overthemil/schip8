use crate::errors::ChipError;
use crate::Screen;
use super::Cpu;

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
    match opcode.prefix {
        0x0 => execute_prefix_0(opcode, cpu, screen)?,
        0x1 => { cpu.pc = opcode.nnn as usize }, 
        0x6 => { cpu.v[opcode.x as usize] = opcode.nn },
        0x7 => { cpu.v[opcode.x as usize] = cpu.v[opcode.x as usize].wrapping_add(opcode.nn) },
        0xA => { cpu.i = opcode.nnn },
        0xD => draw_sprite(opcode, cpu, memory, screen)?,
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

#[cfg(test)]
mod tests {
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
}

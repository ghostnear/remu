use crate::Configs as Configs;
use crate::Components as Components;

pub struct CPU
{
    pc: u16,

    reg: [u8; 16],
    index : u16,

    stack_ptr: u8,
    stack: [u16; 16],

    halt_flag: bool,

    timer: Components::Timer
}

impl CPU
{
    pub fn new(config: &Configs::CPUConfig) -> Self
    {
        Self {
            pc: 0,
            reg: [0; 16],
            index: 0,
            stack_ptr: 0,
            stack: [0; 16],
            timer: Components::Timer::new(&config.timer),
            halt_flag: false
        }
    }

    #[inline]
    pub fn set_pc(&mut self, pc: u16)
    {
        self.pc = pc;
    }

    #[inline]
    pub fn halted(&self) -> bool
    {
        return self.halt_flag;
    }

    #[inline]
    pub fn push_stack(&mut self, value: u16)
    {
        if self.stack_ptr >= 16
        {
            error!("Stack overflow!");
            panic!("Stack overflow!");
        }

        self.stack[self.stack_ptr as usize] = value;
        self.stack_ptr += 1;
    }

    #[inline]
    pub fn pop_stack(&mut self) -> u16
    {
        if self.stack_ptr == 0
        {
            error!("Stack underflow!");
            panic!("Stack underflow!");
        }

        self.stack_ptr -= 1;
        return self.stack[self.stack_ptr as usize];
    }

    #[inline]
    pub fn step(&mut self, ram: &mut Components::RAM, display: &mut Components::Display)
    {
        let opcode = ram.read_word(self.pc);
        self.pc += 2;
        
        // Split the opcode into nibbles and do the execution.
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        match nibbles
        {
            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.pop_stack();
            }

            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                display.clear();
            }

            // JP NNN
            (0x1, _, _, _) => {
                if self.pc - 2 == opcode & 0xFFF
                {
                    warn!("CPU infinite loop detected, halting...");
                    self.halt_flag = true;
                }

                self.pc = opcode & 0xFFF;
            }

            // CALL NNN
            (0x2, _, _, _) => {
                self.push_stack(self.pc);
                self.pc = opcode & 0xFFF;
            }

            // SKP, Vx, KK
            (0x3, _, _, _) => {
                if self.reg[nibbles.1 as usize] == (opcode & 0xFF) as u8
                {
                    self.pc += 2;
                }
            }

            // SKNE, VX, KK
            (0x4, _, _, _) => {
                if self.reg[nibbles.1 as usize] != (opcode & 0xFF) as u8
                {
                    self.pc += 2;
                }
            }

            // SKP, Vx, Vy
            (0x5, _, _, 0x0) => {
                if self.reg[nibbles.1 as usize] == self.reg[nibbles.2 as usize]
                {
                    self.pc += 2;
                }
            }

            // Vx = KK
            (0x6, _, _, _) => {
                self.reg[nibbles.1 as usize] = (opcode & 0xFF) as u8;
            }

            // ADD Vx, KK
            (0x7, _, _, _) => {
                self.reg[nibbles.1 as usize] = self.reg[nibbles.1 as usize].wrapping_add((opcode & 0xFF) as u8);
            }

            // LD Vx, Vy
            (0x8, _, _, 0x0) => {
                self.reg[nibbles.1 as usize] = self.reg[nibbles.2 as usize];
            }

            // OR Vx, Vy
            (0x8, _, _, 0x1) => {
                self.reg[nibbles.1 as usize] |= self.reg[nibbles.2 as usize];
            }

            // AND Vx, Vy
            (0x8, _, _, 0x2) => {
                self.reg[nibbles.1 as usize] &= self.reg[nibbles.2 as usize];
            }

            // XOR Vx, Vy
            (0x8, _, _, 0x3) => {
                self.reg[nibbles.1 as usize] ^= self.reg[nibbles.2 as usize];
            }

            // ADC, Vx, Vy
            (0x8, _, _, 0x4) => {
                let (result, overflow) = self.reg[nibbles.1 as usize].overflowing_add(self.reg[nibbles.2 as usize]);
                self.reg[nibbles.1 as usize] = result;
                self.reg[0xF] = overflow as u8;
            }

            // SUBB, Vx, Vy
            (0x8, _, _, 0x5) => {
                let (result, overflow) = self.reg[nibbles.1 as usize].overflowing_sub(self.reg[nibbles.2 as usize]);
                self.reg[nibbles.1 as usize] = result;
                self.reg[0xF] = !overflow as u8;
            }

            // SHR, Vx
            (0x8, _, _, 0x6) => {
                let overflow = self.reg[nibbles.1 as usize] & 0x1;
                self.reg[nibbles.1 as usize] >>= 1;
                self.reg[0xF] = overflow;
            }

            // SUBN, Vx, Vy
            (0x8, _, _, 0x7) => {
                let (result, overflow) = self.reg[nibbles.2 as usize].overflowing_sub(self.reg[nibbles.1 as usize]);
                self.reg[nibbles.1 as usize] = result;
                self.reg[0xF] = !overflow as u8;
            }

            // SHL, Vx
            (0x8, _, _, 0xE) => {
                let overflow = (self.reg[nibbles.1 as usize] & 0x80) >> 7;
                self.reg[nibbles.1 as usize] <<= 1;
                self.reg[0xF] = overflow;
            }

            // SKNE, Vx, Vy
            (0x9, _, _, 0x0) => {
                if self.reg[nibbles.1 as usize] != self.reg[nibbles.2 as usize]
                {
                    self.pc += 2;
                }
            }

            // I = NNN
            (0xA, _, _, _) => {
                self.index = opcode & 0xFFF;
            }

            // DRW X, Y, n
            (0xD, _, _, _) => {
                display.set_flag();

                let x = self.reg[nibbles.1 as usize];
                let y = self.reg[nibbles.2 as usize];
                let n = nibbles.3;

                let mut collision = false;

                for i in 0..n as u8
                {
                    let byte = ram.read_byte((self.index + i as u16) as usize);
                    for j in 0..8 as u8
                    {
                        if display.set_pixel(x + j, y + i, (byte >> (7 - j)) & 1 == 1)
                        {
                            collision = true;
                        }
                    }
                }

                self.reg[0xF] = collision as u8;
            }

            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.index += self.reg[nibbles.1 as usize] as u16;
            }

            // LDB, Vx
            (0xF, _, 0x3, 0x3) => {
                let mut value = self.reg[nibbles.1 as usize];
                for i in 0..3
                {
                    ram.write_byte((self.index + 2 - i) as usize, value % 10);
                    value /= 10;
                }
            }

            // LD, [I], Vx
            (0xF, _, 0x5, 0x5) => {
                for i in 0..nibbles.1 + 1
                {
                    ram.write_byte((self.index + i) as usize, self.reg[i as usize]);
                }
            }

            // LD, Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                for i in 0..nibbles.1 + 1
                {
                    self.reg[i as usize] = ram.read_byte((self.index + i) as usize);
                }
            }

            _ => {
                error!("Unknown opcode: {:04X}", opcode);
                panic!("Unknown opcode: {:04X}", opcode);
            }
        }
    }

    pub fn update(&mut self, ram: &mut Components::RAM, display: &mut Components::Display, delta: f64)
    {
        self.timer.update(delta);
        
        // We are ready to execute the opcode.
        if self.timer.value() == 0
        {
            self.timer.set(1);
            self.step(ram, display);
        }
    }

}
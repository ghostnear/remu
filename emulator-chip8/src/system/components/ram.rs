use crate::Configs;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct RAM
{
    start: usize,
    size: usize,
    memory: Vec<u8>
}

impl RAM
{
    pub fn new(config: &Configs::RAMConfig) -> Self
    {
        let mut result = Self {
            start: config.start,
            size: config.size,
            memory: vec![0; config.size]
        };

        result.memory[0..FONTSET_SIZE].copy_from_slice(&FONTSET[..]);

        return result;
    }

    #[inline]
    pub fn read_byte(&self, address: usize) -> u8
    {
        if address >= self.size
        {
            error!("Attempted to read byte from invalid address: {:#04X}", address);
            panic!("Attempted to read byte from invalid address: {:#04X}", address);
        }

        return self.memory[address];
    }

    #[inline]
    pub fn write_byte(&mut self, address: usize, value: u8)
    {
        if address >= self.size
        {
            error!("Attempted to write byte to invalid address: {:#04X}", address);
            panic!("Attempted to write byte to invalid address: {:#04X}", address);
        }

        self.memory[address] = value;
    }

    #[inline]
    pub fn read_word(&self, address: u16) -> u16
    {
        if address >= (self.size - 1) as u16
        {
            error!("Attempted to read word from invalid address: {:#04X}", address);
            panic!("Attempted to read word from invalid address: {:#04X}", address);
        }

        return (self.memory[address as usize] as u16) << 8 | (self.memory[(address + 1) as usize] as u16);
    }

    pub fn load_rom_data(&mut self, data: &[u8])
    {
        if data.len() > self.size - self.start
        {
            error!("ROM file is too big!");
            panic!("ROM file is too big!");
        }

        let start = self.start;
        let end = self.start + data.len();
        self.memory[start..end].copy_from_slice(data);

        info!("Loaded ROM data consisting of {} bytes into RAM.", data.len());
    }

}
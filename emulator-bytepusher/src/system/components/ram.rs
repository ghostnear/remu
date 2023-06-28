use crate::Configs;

pub struct RAM
{
    size: usize,
    memory: Vec<u8>
}

impl RAM
{
    pub fn new(config: &Configs::RAMConfig) -> Self
    {
        Self {
            size: config.size,
            memory: vec![0; config.size]
        }
    }

    #[inline]
    pub fn read_byte(&self, address: u32) -> u8
    {
        if address >= self.size as u32
        {
            error!("Attempted to read byte from invalid address: {:#04X}", address);
            panic!("Attempted to read byte from invalid address: {:#04X}", address);
        }

        return self.memory[address as usize];
    }

    #[inline]
    pub fn write_byte(&mut self, address: u32, value: u8)
    {
        if address >= self.size as u32
        {
            error!("Attempted to write byte to invalid address: {:#04X}", address);
            panic!("Attempted to write byte to invalid address: {:#04X}", address);
        }

        self.memory[address as usize] = value;
    }

    #[inline]
    pub fn read_triple_byte(&self, address: u32) -> u32
    {
        if address >= (self.size - 2) as u32
        {
            error!("Attempted to read triple byte from invalid address: {:#04X}", address);
            panic!("Attempted to read triple byte from invalid address: {:#04X}", address);
        }

        return (self.memory[address as usize] as u32) << 16 | (self.memory[(address + 1) as usize] as u32) << 8 | (self.memory[(address + 2) as usize] as u32);
    }

    pub fn load_rom_data(&mut self, data: &[u8])
    {
        if data.len() > self.size
        {
            error!("ROM file is too big!");
            panic!("ROM file is too big!");
        }

        self.memory[0..data.len()].copy_from_slice(data);

        info!("Loaded ROM data consisting of {} bytes into RAM.", data.len());
    }

}
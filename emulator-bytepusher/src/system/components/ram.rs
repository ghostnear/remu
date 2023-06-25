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
        if data.len() > self.size
        {
            error!("ROM file is too big!");
            panic!("ROM file is too big!");
        }

        self.memory[0..data.len()].copy_from_slice(data);

        info!("Loaded ROM data consisting of {} bytes into RAM.", data.len());
    }

}
pub struct Keyboard
{
    keys: [u8; 2]
}

impl Keyboard
{
    pub fn new() -> Self
    {
        Self {
            keys: [0; 2]
        }
    }

    #[inline]
    pub fn is_pressed(&self, key: u8) -> bool
    {
        let index = key / 8;
        let bit = key % 8;

        return (self.keys[index as usize] & (1 << bit)) != 0;
    }

    #[inline]
    pub fn press(&mut self, key: u8)
    {
        let index = key / 8;
        let bit = key % 8;

        self.keys[index as usize] |= 1 << bit;
    }
    
    #[inline]
    pub fn release(&mut self, key: u8)
    {
        let index = key / 8;
        let bit = key % 8;

        self.keys[index as usize] &= !(1 << bit);
    }
}
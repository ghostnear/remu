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
}
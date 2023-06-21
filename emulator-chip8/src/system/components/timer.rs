use crate::Configs as Configs;

pub struct Timer
{
    timer: f64,
    rate: f64,
    value: u8
}

impl Timer
{
    pub fn new(config: &Configs::TimerConfig) -> Self
    {
        Self {
            timer: 0.0,
            rate: config.rate,
            value: 0
        }
    }

    pub fn update(&mut self, delta : f64)
    {
        self.timer += delta;

        let period = 1.0f64 / (self.rate * 1.0);

        if self.timer >= period
        {
            self.timer -= period;
            if self.value > 0
            {
                self.value -= 1;
            }
        }
    }

    pub fn set(&mut self, value: u8)
    {
        self.value = value;
    }

    pub fn value(&self) -> u8
    {
        return self.value;
    }
    
}
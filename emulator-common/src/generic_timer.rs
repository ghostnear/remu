pub struct GenericTimerConfig
{
    pub rate: f64
}

impl GenericTimerConfig
{
    pub fn default() -> Self
    {
        Self {
            rate: 1.0
        }
    }
}

pub struct GenericTimer
{
    timer: f64,
    rate: f64,
    value: u8
}

impl GenericTimer
{
    pub fn new(config: &GenericTimerConfig) -> Self
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

        let rate = self.rate();

        if self.timer >= rate
        {
            self.timer -= rate;
            if self.value > 0
            {
                self.value -= 1;
            }
        }
    }

    #[inline]
    pub fn rate(&self) -> f64
    {
        return 1.0f64 / (self.rate * 1.0);
    }

    #[inline]
    pub fn passed(&self) -> f64
    {
        return self.timer;
    }

    #[inline]
    pub fn set(&mut self, value: u8)
    {
        self.value = value;
    }

    #[inline]
    pub fn get(&self) -> u8
    {
        return self.value;
    }
    
}
pub struct DeltaTimer
{
    start: std::time::Instant,
    end: std::time::Instant,
    delta: f64
}

impl DeltaTimer
{
    pub fn new() -> Self
    {
        Self
        {
            start: std::time::Instant::now(),
            end: std::time::Instant::now(),
            delta: 0.0
        }
    }

    pub fn update(&mut self)
    {
        self.start = self.end;
        self.end = std::time::Instant::now();
        self.delta = self.end.duration_since(self.start).as_secs_f64();
    }

    #[inline]
    pub fn get(&mut self) -> f64
    {
        return self.delta;
    }
}
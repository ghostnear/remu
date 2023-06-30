use serde_json::Value;

pub struct EmulatorConfig
{
    pub ram_config: RAMConfig,
    pub cpu_config: CPUConfig
}

impl EmulatorConfig
{
    pub fn default() -> Self
    {
        Self {
            ram_config: RAMConfig::default(),
            cpu_config: CPUConfig::default()
        }
    }

    pub fn from_json(_data: &Value) -> Self
    {
        let result = Self::default();

        return result;
    }
}

pub struct CPUConfig
{
    pub timer: TimerConfig
}

impl CPUConfig
{
    pub fn default() -> Self
    {
        Self {
            timer: TimerConfig {
                rate: 30.0
            }
        }
    }
}

pub struct RAMConfig
{
    pub size: usize
}

impl RAMConfig
{
    pub fn default() -> Self
    {
        Self {
            size: 0x1000000
        }
    }
}

pub struct TimerConfig
{
    pub rate: f64
}

impl TimerConfig
{
    pub fn default() -> Self
    {
        Self {
            rate: 1.0
        }
    }
}
use serde_json::Value;

pub struct EmulatorConfig
{
    pub ram_config: RAMConfig,
    pub cpu_config: CPUConfig,
    pub display_config: DisplayConfig,
    pub sound_timer_config: TimerConfig,
    pub delta_timer_config: TimerConfig
}

impl EmulatorConfig
{
    pub fn default() -> Self
    {
        Self {
            ram_config: RAMConfig::default(),
            cpu_config: CPUConfig::default(),
            display_config: DisplayConfig::default(),
            sound_timer_config: TimerConfig {
                rate: 60.0
            },
            delta_timer_config: TimerConfig {
                rate: 60.0
            }
        }
    }

    pub fn from_json(_data: &Value) -> Self
    {
        let mut _result = Self::default();

        return _result;
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
                rate: 1000.0
            }
        }
    }
}

pub struct RAMConfig
{
    pub start: usize,
    pub size: usize
}

impl RAMConfig
{
    pub fn default() -> Self
    {
        Self {
            start: 0x200,
            size: 0x1000
        }
    }
}

pub struct DisplayConfig
{
    pub width: u8,
    pub height: u8
}

impl DisplayConfig
{
    pub fn default() -> Self
    {
        Self {
            width: 64,
            height: 32
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
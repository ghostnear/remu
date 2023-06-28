use serde_json::Value;

pub struct EmulatorConfig
{
    pub ram_config: RAMConfig,
    pub cpu_config: CPUConfig,
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
            sound_timer_config: TimerConfig {
                rate: 60.0
            },
            delta_timer_config: TimerConfig {
                rate: 60.0
            }
        }
    }

    pub fn from_json(data: &Value) -> Self
    {
        let mut result = Self::default();

        // Change the defaults if they are changed in the config.
        result.cpu_config.timer.rate = data["instruction_rate"].as_f64().unwrap_or(result.cpu_config.timer.rate);

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
                rate: 60.0
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
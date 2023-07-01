use serde_json::Value;

use emulator_common::GenericTimerConfig;

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
    pub timer: GenericTimerConfig
}

impl CPUConfig
{
    pub fn default() -> Self
    {
        Self {
            timer: GenericTimerConfig {
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
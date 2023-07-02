use emulator_common::GenericTimerConfig;
use serde_json::Value;

pub struct EmulatorConfig
{
	pub ram_config: RAMConfig,
	pub cpu_config: CPUConfig,
	pub display_config: DisplayConfig,
	pub sound_timer_config: GenericTimerConfig,
	pub delta_timer_config: GenericTimerConfig
}

impl EmulatorConfig
{
	pub fn default() -> Self
	{
		Self {
			ram_config: RAMConfig::default(),
			cpu_config: CPUConfig::default(),
			display_config: DisplayConfig::default(),
			sound_timer_config: GenericTimerConfig { rate: 60.0 },
			delta_timer_config: GenericTimerConfig { rate: 60.0 }
		}
	}

	pub fn from_json(data: &Value) -> Self
	{
		let mut result = Self::default();

		// Change the defaults if they are changed in the config.
		result.cpu_config.timer.rate = data["instruction_rate"]
			.as_f64()
			.unwrap_or(result.cpu_config.timer.rate);
		result.ram_config.start = data["loading_address"]
			.as_u64()
			.unwrap_or(result.ram_config.start as u64) as usize;

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
			timer: GenericTimerConfig { rate: 1000.0 }
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

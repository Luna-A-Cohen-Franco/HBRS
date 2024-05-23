pub enum HacommStatusConsts{
	Light,
	Curtain,
	HVAC,
	IAS,
	Doorlock,
}

impl HacommStatusConsts{
	pub fn get_value(&self) -> u8{
		match self{
			HacommStatusConsts::Light => 1,
			HacommStatusConsts::Curtain => 4,
			HacommStatusConsts::HVAC => 6,
			HacommStatusConsts::IAS => 8,
			HacommStatusConsts::Doorlock => 7,
		}
	}
}
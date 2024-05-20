pub struct FloatHelper{

}

impl FloatHelper{
    pub fn string_to_double(value: &str) -> Result<f64, std::num::ParseFloatError>{
		return value.parse::<f64>();
	}

	pub fn double_to_string(value: f64, number_of_decimals: u8) -> String{
		if value == 0.0{
			return "0".to_string();
		}

		let text = format!("{:.*}", number_of_decimals as usize, value);
		let text = text.replace(",", ".");

		return text;
	}

	pub fn format_temperature_to_string(value: f64, number_of_decimals: u8) -> String{
		if value <= -300.0{
			return "0".to_string();
		}

		let text = FloatHelper::double_to_string(value, number_of_decimals);

		return format!("{} °C", text);
	}
}
pub struct FloatHelper{

}

impl FloatHelper{
    
}
/*

public static class FloatHelper
{
	public static double StringToDouble(string value)
	{
		return double.Parse(value, CultureInfo.InvariantCulture);
	}

	public static string DoubleToString(double value)
	{
		if (value == 0.0)
		{
			return "0";
		}
		string text = value.ToString("F");
		text = text.Replace(",", ".");
		string[] array = text.Split(new char[1] { '.' });
		if (array != null && array.Length > 1)
		{
			text = array[0] + "." + array[1].Substring(0, 1);
		}
		return text.Replace(",", ".");
	}

	public static string FormatTemperatureToString(double value, int numberOfDecimals)
	{
		if (value <= -300.0)
		{
			return "0";// StringResources.Instance.Devices.Thermostat_UnknownTemperature;
		}
		string text = value.ToString("F");
		text = text.Replace(",", ".");
		string[] array = text.Split(new char[1] { '.' });
		if (array != null && array.Length > 1)
		{
			text = ((numberOfDecimals <= 0) ? array[0] : (array[0] + "." + array[1].Substring(0, numberOfDecimals)));
		}
		return text + " °C";
	}
}

*/
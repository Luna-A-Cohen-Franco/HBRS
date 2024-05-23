class FloatHelper:
    @staticmethod
    def string_to_double(value: str) -> float:
        return float(value)

    @staticmethod
    def double_to_string(value: float, number_of_decimals: int) -> str:
        if value == 0.0:
            return "0"

        text = "{:." + str(number_of_decimals) + "f}"
        text = text.format(value)
        text = text.replace(",", ".")

        return text

    @staticmethod
    def format_temperature_to_string(value: float, number_of_decimals: int) -> str:
        if value <= -300.0:
            return "0"

        text = FloatHelper.double_to_string(value, number_of_decimals)

        return text + " °C"
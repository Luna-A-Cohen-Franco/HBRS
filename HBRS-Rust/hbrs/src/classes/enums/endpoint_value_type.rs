use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Clone, PartialEq, Debug)]
pub enum EndpointValueType
{
	Unknown,
	IsOn,
	Dim,
	DimRate,
	AutoOff,
	OffTimeout,
	Position,
	LightLevel,
	AppliancePower,
	MeteringType,
	MeteringUnit,
	MeteringUnitsPerPulse,
	MeterEndpoint,
	MeasuredTemperatureC,
	ThermostatMode,
	ThermostatFanMode,
	HVACTimerOnMinutes,
	HVACTimerOffMinutes,
	HVACFlags,
	Reserved4,
	ThermostatDesiredTempC,
	DigitalPortValue,
	DigitalPortInputCount,
	AnalogRawValue,
	AnalogValue,
	AnalogRawMin,
	AnalogRawMax,
	AnalogRealMin,
	AnalogRealMax,
	DataSourceEndpoint,
	VTMarginC,
	VTOffMacro,
	VTHeatMacroAbove,
	VTHeatMacroBelow,
	VTCoolMacroAbove,
	VTCoolMacroBelow,
	VTFanMacroLow,
	VTFanMacroMid,
	VTFanMacroHi,
	VTFanMacroAuto,
	TemperatureOffset,
	DimmerDisabled,
	DoorLockState,
	HumidityPercentage,
	MeteringMultiplier,
	MeteringSumation,
	IASSensorState,
	AlarmPartitionNumber,
	AlarmPartitionState,
	Reserved10,
	Reserved0,
	Reserved1,
	Reserved2,
	Reserved3,
	Reserved5,
	Reserved6,
	Reserved7,
	Reserved8,
	Reserved9,
	ValuePair1,
	ValuePair2,
	ValuePair3,
	ValuePair4,
	ValuePair5,
	ValuePair6,
	ValuePair7,
	ValuePair8,
	ValuePair9,
	ValuePair10,
	ValuePair11,
	ValuePair12,
	ValuePair13,
	ValuePair14,
	ValuePair15,
	ValuePair16,
	ValuePair17,
	ValuePair18,
	ValuePair19,
	ValuePair20
}
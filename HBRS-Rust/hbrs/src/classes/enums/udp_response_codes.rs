use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum UDPResponseCodes
{
	None,
	JoinEnumerateResponse,
	JoinScanResponseStarted,
	JoinSwitchCloudResponse,
	JoinJoinResponse
}
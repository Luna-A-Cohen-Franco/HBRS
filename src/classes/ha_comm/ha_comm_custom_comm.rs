pub struct HACommCustomComm{
    pub command: u8,
    pub data: Vec<u8>,
}

impl HACommCustomComm{
    pub fn new() -> Self{
        Self{
            command: 0,
            data: Vec::new(),
        }
    }

    pub fn set_bytes(&mut self, data: Vec<u8>, header_offset: usize){
        self.command = data[header_offset];
        self.data = Vec::new();
    }

    pub fn get_byte(&self) -> Vec<u8>{
        if self.data.len() == 0{
            self.command = 0;
        }

        return self.command;
    }
}
/*
public class HACommand_CustomCommand
{
	public byte Command { get; set; }

	public byte[] Data { get; set; }

	public void SetBytes(byte[] data, int headerOffset)
	{
		Command = data[headerOffset];
		Data = null;
	}

	public byte[] GetBytes()
	{
		if (Data == null || Data.Length == 0)
		{
			return new byte[1] { Command };
		}
		return ByteArraysHelper.Combine(Command, Data);
	}
}
*/
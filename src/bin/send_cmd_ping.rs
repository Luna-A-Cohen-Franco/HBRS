use hbrs::classes::ha_comm::ha_command::HACommand;

fn main(){
    let comm = HACommand::new();
    
}
/*public static HACommand Send_Cmd_Ping()
   {
      HACommand command = new HACommand();
      command.Header.ProtocolVersion = 0;
      command.Header.SourceMAC = new byte[6];
      command.Header.DestinationMAC = new byte[6] { 255, 255, 255, 255, 255, 255 };
      Random RandNumber = new Random();
      byte NextSequenceNumberSent = Convert.ToByte(RandNumber.Next(256));
      command.Header.SequenceNumber = NextSequenceNumberSent;
      command.Header.SourceEndpoint = 0;
      command.Header.DestinationEndpoint = 0;
      command.Header.CommandID = 224;
      command.Ping = new HACommand_Ping();
      return command;
}*/
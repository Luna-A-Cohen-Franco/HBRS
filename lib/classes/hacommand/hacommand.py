import os
import sys

from lib.consts.other import Other
from lib.utils.byte_arrays_helper import ByteArraysHelper
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from lib.classes.hacommand.header import Header
from lib.classes.join.join import Join
from lib.classes.hacommand.ping import Ping
from lib.classes.status.status import Status
from lib.classes.hacommand.set_mode import SetMode
from lib.classes.hacommand.hvac_comm import HVACComm
from lib.classes.hacommand.custom_comm import CustomComm

class HACommand:
    def __init__(self):
        self.header = Header()
        self.join = None
        self.ping = None
        self.status = None
        self.hvac_set_mode = None
        self.hvac_command = None
        self.custom_command = None
        self.dim_cmd = None
        self.set_auto_off = None

    def clear_bytes(self):
        self.header.clear_bytes()
        self.join = None
        self.ping = None
        self.custom_command = None
        self.set_auto_off = None
    
    def get_bytes(self):
        if self.header.command_id in [1, 2, 34, 40, 41, 164, 175, 228]:
            return self.header.get_bytes()
        elif self.header.command_id == 3:
            return ByteArraysHelper.combine_1v_1b(self.header.get_bytes(), self.dim_cmd.get_byte())
        elif self.header.command_id == 97:
            return ByteArraysHelper.combine_1v_1b(self.header.get_bytes(), self.hvac_set_mode.get_byte())
        elif self.header.command_id == 98:
            return ByteArraysHelper.combine_1v_1b(self.header.get_bytes(), self.hvac_command.get_byte())
        elif self.header.command_id == 161:
            return ByteArraysHelper.combine_2v(self.header.get_bytes(), self.join.get_bytes())
        elif self.header.command_id == 162:
            return ByteArraysHelper.combine_2v(self.header.get_bytes(), self.custom_command.get_bytes())
        elif self.header.command_id == 178:
            return ByteArraysHelper.combine_2v(self.header.get_bytes(), self.set_auto_off.get_bytes())
        elif self.header.command_id == 224:
            return ByteArraysHelper.combine_2v(self.header.get_bytes(), self.ping.get_bytes())
        else:
            return []

    def set_bytes(self, data, subcomm_wait_res):
        self.header.set_bytes(data)

        if self.header.command_id == 161:
            if self.join is None:
                self.join = Join()

            self.join.set_bytes(data, Other.HeaderOffset, subcomm_wait_res)
        elif self.header.command_id == 253:
            if self.status is None:
                self.status = Status()

            self.status.set_bytes(data, 0)
        elif self.header.command_id == 162:
            if self.custom_command is None:
                self.custom_command = CustomComm()

            self.custom_command.set_bytes(data, 0)

    def get_header(self):
        return self.header

    def set_header(self, header):
        self.header = header

    def get_join(self):
        return self.join

    def set_join(self, join):
        self.join = join

    def get_ping(self):
        return self.ping

    def set_ping(self, ping):
        self.ping = ping

    def get_status(self):
        return self.status
    
    def set_status(self, status):
        self.status = status

    def get_hvac_set_mode(self):
        return self.hvac_set_mode
    
    def set_hvac_set_mode(self, hvac_set_mode):
        self.hvac_set_mode = hvac_set_mode

    def get_hvac_command(self):
        return self.hvac_command

    def set_hvac_command(self, hvac_command):
        self.hvac_command = hvac_command
    
    def get_custom_command(self):
        return self.custom_command

    def set_custom_command(self, custom_command):
        self.custom_command = custom_command

    def get_dim_cmd(self):
        return self.dim_cmd
    
    def set_dim_cmd(self, dim_cmd):
        self.dim_cmd = dim_cmd

    def get_set_auto_off(self):
        return self.set_auto_off   
    
import random
import os
import sys

from lib.classes.hacommand.ping import Ping
from lib.classes.join.join_req import JoinReq
sys.path.append(os.path.dirname(__file__) + "/..")

from lib.classes.addresses.mac import MacAddress
from lib.classes.hacommand.hacommand import HACommand
from lib.classes.join.join import Join

class CommandGenerator:
    @staticmethod
    def send_ping():
        comm = HACommand()
        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress([255, 255, 255, 255, 255, 255]))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(224)

        comm.ping = Ping();  

        return comm

    @staticmethod     
    def send_join_scan(mac: MacAddress):
        comm = HACommand()
        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(mac)

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(161)

        comm.set_join(Join())
        comm.join.sub_command = 5
        return comm

    @staticmethod
    def send_join_request(mac: MacAddress, ssid: str, security_type: int, encryption_type: int, key: str):
        comm = HACommand()

        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(mac)

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(0)
        comm.get_header().set_id(161)

        join = Join()

        join.set_join_req(JoinReq.new(ssid, security_type, encryption_type, key))

        join.set_sub_command(3)

        comm.set_join(join)

        return comm

    @staticmethod
    def send_custom_comm(mac: MacAddress, dest_endpoint: int):
        comm = HACommand()

        comm.get_header().set_protocol_version(0)
        comm.get_header().set_source_mac(MacAddress([0, 0, 0, 0, 0, 0]))
        comm.get_header().set_destination_mac(MacAddress(mac))

        rng = random.SystemRandom()
        next_sqnc_num = rng.randint(0, 255)
        comm.get_header().set_sequence_number(next_sqnc_num)

        comm.get_header().set_source_endpoint(0)
        comm.get_header().set_destination_endpoint(dest_endpoint)
        comm.get_header().set_id(2)
              
        return comm

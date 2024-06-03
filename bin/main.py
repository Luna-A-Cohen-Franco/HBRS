from threading import Thread
from concurrent.futures import ThreadPoolExecutor
import time
import socket
import os
import sys
sys.path.append(os.path.dirname(__file__) + "/..")

from bin.runner import Runner
from lib.classes.addresses.mac import MacAddress

def data_received(runner, client, rep):
    command = runner.send_ping()
    ping = command.get_bytes()

    def thread_func():
        print("Receiving")
        (mac, _) = runner.data_received(client)
        runner.mac = mac

    Thread(target=thread_func).start()

    client.sendto(ping, rep)

    time.sleep(5)

def data_scan(runner, client, rep):
    command_scan = runner.send_join_scan()
    data_scan = command_scan.get_bytes()

    def thread_func():
        print("Scanning")
        runner.data_received_scan(client)

    Thread(target=thread_func).start()

    client.sendto(data_scan, rep)

    time.sleep(15)

def data_join(runner, client, rep, key):
    command_join = runner.send_join_request(runner.ssid, runner.security_type, runner.encryption_type, key)

    data_join = command_join.get_bytes()

    print(list(data_join))
    def thread_func():
        print("Joining")
        time.sleep(5)
        runner.data_received_join(client)

    Thread(target=thread_func).start()

    client.sendto(data_join, rep)

    time.sleep(2)

def data_custom(runner, client, rep):
    custom_comm = runner.send_custom_comm(1)

    data_custom = custom_comm.get_bytes()

    print("Sending custom signal")

    client.sendto(data_custom, rep)

    time.sleep(2)

def find_new_ip(runner, client):
    runner.find_new_ip(client)

def main():
    runner = Runner(
        mac=MacAddress([0, 0, 0, 0, 0, 0]),
        ssid=[],
        security_type=0,
        encryption_type=0
    )

    lep = ('0.0.0.0', 20910)
    rep = ('10.10.100.254', 20910)

    client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    client.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)  
    client.bind(lep)

    data_received(runner, client, rep)

    ssid = "IMPLabo"
    data_scan(runner, client, rep, ssid)

    key = "j2LK98!we".encode()
    data_join(runner, client, rep, key)

    nipep = ('0.0.0.0', 20911)
    nipclient = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    nipclient.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)  
    nipclient.bind(nipep)

    nip = find_new_ip(runner, nipclient)

    niprep = (nip, 20911) # Might be 20910

    #TODO: Send custom blink command
    data_custom(runner, client, niprep)

main()
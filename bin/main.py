from threading import Thread
from concurrent.futures import ThreadPoolExecutor
import time
import socket

from bin.runner import Runner
from lib.classes.addresses.mac import MacAddress

def data_received(runner, client, rep):
    command = runner.send_ping()
    data = command.get_bytes()

    def thread_func():
        print("Receiving")
        runner.data_received(client)

    Thread(target=thread_func).start()

    client.sendto(data, rep)

    time.sleep(2)

def data_scan(runner, client, rep):
    command_scan = runner.send_join_scan()
    data_scan = command_scan.get_bytes()

    def thread_func():
        print("Scanning")
        runner.data_received_scan(client)

    Thread(target=thread_func).start()

    client.sendto(data_scan, rep)
    time.sleep(15)

def data_join(runner, client, rep):
    bytes2 = "j2LK98!we".encode()

    command_join = runner.send_join_request(runner.ssid, runner.security_type, runner.encryption_type, bytes2)

    data_join = command_join.get_bytes()

    def thread_func():
        print("Joining")
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
    client.bind(lep)

    with ThreadPoolExecutor() as executor:
        executor.submit(data_received, runner, client, rep)
        executor.submit(data_scan, runner, client, rep)
        executor.submit(data_join, runner, client, rep)
        executor.submit(data_custom, runner, client, rep)

    print(runner.ssid)
    print(runner.security_type)
    print(runner.encryption_type)

main()
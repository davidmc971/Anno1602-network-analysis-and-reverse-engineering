#! python

import socket
from threading import Thread
import sys
import struct
from importlib import reload
import parser
import config

# IP_CLIENT_ORIGIN = '192.168.178.38'
# IP_HOST_PROXIED = '192.168.178.36'
IP_CLIENT_ORIGIN = '10.20.0.2'
IP_HOST_PROXIED = '10.20.0.1'
# IP_HOST_ORIGIN = '192.168.2.20'
# IP_CLIENT_PROXIED = '192.168.2.190'
# IP_HOST_ORIGIN = '192.168.2.36'
# IP_CLIENT_PROXIED = '192.168.2.110'
# IP_HOST_ORIGIN = '192.168.0.66'
# IP_CLIENT_PROXIED = '192.168.0.204'
IP_HOST_ORIGIN = '10.30.0.2'
IP_CLIENT_PROXIED = '10.30.0.1'
# DirectPlay port
PORT_DPLAY = 47624
# TCP and UDP game session ports
PORT_RANGE_GAME = range(2300, 2401)


class DPlayProxy(Thread):

    def __init__(self, src_ip, dst_ip, port, dplay_proxied_ip):
        super(DPlayProxy, self).__init__()
        self.dst_ip = dst_ip
        self.src_ip = src_ip
        self.port = port
        self.dplay_proxied_ip = dplay_proxied_ip
        self.running = False
        self.active_conn = False
        self.identifier = "[DPLAY, {}, {}->{} as {}]".format(port, src_ip, dst_ip, dplay_proxied_ip)
        print("{} init done.".format(self.identifier, self.port))

    def run(self):
        self.running = True
        while self.running:
            serversocket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            serversocket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            serversocket.bind((self.src_ip, self.port))
            serversocket.listen(1)
            client_socket, client_addr = serversocket.accept()
            host_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            host_socket.connect((self.dst_ip, self.port))
            print("{} accepted connection from {}".format(
                self.identifier, self.port, client_addr))
            print("{} client peer: {} | host peer: {}".format(
                self.identifier, self.port, client_socket.getpeername(), host_socket.getpeername()))
            self.active_conn = True

            def handle_client():
                while self.active_conn:
                    try:
                        reload(config)
                    except Exception as e:
                        print(e)
                    data_client = bytearray(client_socket.recv(config.BUFFER_SIZE))
                    if data_client:
                        #dplay_package_size = struct.unpack("<H", data_client[0:2])[0]
                        #dplay_s_addr_in_ip_address = struct.unpack("<BBBB", data_client[8:12])
                        #print (dplay_s_addr_in_ip_address)
                        if self.dplay_proxied_ip != None:
                            dplay_proxied_ip = self.dplay_proxied_ip
                            addr = list(map(int, dplay_proxied_ip.split('.')))
                            struct.pack_into("<BBBB", data_client, 8, addr[0], addr[1], addr[2], addr[3])
                            print("[DPLAY, {}] injected proxy ip".format(self.port))
                        if config.VERBOSE_LOGGING:
                            self.log('client', data_client)
                        host_socket.sendall(data_client)
            
            def handle_host():
                while self.active_conn:
                    data_host = bytearray(host_socket.recv(config.BUFFER_SIZE))
                    if data_host:
                        if config.VERBOSE_LOGGING:
                            self.log('host', data_host)
                        client_socket.sendall(data_host)

            thread1 = Thread(target = handle_client)
            thread1.start()
            thread2 = Thread(target = handle_host)
            thread2.start()

    def log(self, side, data):
        print("{} {}: {}".format(self.identifier, side, data[:].hex()))
        # teststr = ""
        # for byte in data[:]:
        #     teststr += str(int(byte))
        #     teststr += " "
        # print("[{}, TCP] {}: {}".format(self.port, side, teststr))

class UDPProxy(Thread):

    def __init__(self, ip_client_origin, ip_host_origin, ip_client_proxied, ip_host_proxied, port):
        super(UDPProxy, self).__init__()
        self.ip_client_origin = ip_client_origin
        self.ip_host_origin = ip_host_origin
        self.ip_client_proxied = ip_client_proxied
        self.ip_host_proxied = ip_host_proxied
        self.port = port
        self.running = False
        self.active_conn_from_client = False
        self.active_conn_from_host = False
        self.identifier = "[UDP, {}]".format(port)
        self.identifier_from_client = "[UDP, {}, {}->{} as {}]".format(port, ip_client_proxied, ip_client_origin, ip_host_proxied)
        self.identifier_from_host = "[UDP, {}, {}->{} as {}]".format(port, ip_host_proxied, ip_host_origin, ip_client_proxied)
        print("{} init done.".format(self.identifier, self.port))
        # testproxy2 = UDPProxy(IP_CLIENT_PROXIED, IP_CLIENT_ORIGIN, port, IP_HOST_PROXIED)
        # testproxy2.start()
        # testproxy3 = UDPProxy(IP_HOST_PROXIED, IP_HOST_ORIGIN, port, IP_CLIENT_PROXIED)
        # testproxy3.start()

    def run(self):
        # self.running = True
        # while self.running:
        serversocket_listen_client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        serversocket_listen_client.bind((self.ip_host_proxied, self.port))
        serversocket_listen_host = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        serversocket_listen_host.bind((self.ip_client_proxied, self.port))

        def handle_incoming_from_client():
            addr_src = None
            addr_dst = (self.ip_host_origin, self.port)
            self.active_conn_from_client = True
            if config.VERBOSE_LOGGING:
                print("[UDP, {}] Ready to receive from client".format(self.port))
            while self.active_conn_from_client:
                try:
                    reload(config)
                except Exception as e:
                    print(e)
                data, address = serversocket_listen_client.recvfrom(config.BUFFER_SIZE)
                if addr_src == None:
                    addr_src = address
                if address == addr_src:
                    if config.VERBOSE_LOGGING:
                        self.log(self.identifier_from_client, address[0], data)
                    serversocket_listen_host.sendto(data, addr_dst)
                if data:
                    try:
                        reload(parser)
                        parser.parse(data, self.port, "client")
                    except Exception as e:
                        print("[{},{}] {}".format("client", self.port, e))


        def handle_incoming_from_host():
            addr_src = None
            addr_dst = (self.ip_client_origin, self.port)
            self.active_conn_from_host = True
            if config.VERBOSE_LOGGING:
                print("[UDP, {}] Ready to receive from host".format(self.port))
            # print("[{}, UDP] UDP ready to receive".format(self.port))
            while self.active_conn_from_host:
                try:
                    reload(config)
                except Exception as e:
                    print(e)
                data, address = serversocket_listen_host.recvfrom(config.BUFFER_SIZE)
                if addr_src == None:
                    addr_src = address
                if address == addr_src:
                    if config.VERBOSE_LOGGING:
                        self.log(self.identifier_from_host, address[0], data)
                    serversocket_listen_client.sendto(data, addr_dst)
                if data:
                    try:
                        reload(parser)
                        parser.parse(data, self.port, "host")
                    except Exception as e:
                        print("[{},{}] {}".format("host", self.port, e))
        
        thread1 = Thread(target = handle_incoming_from_client)
        thread1.start()
        thread2 = Thread(target = handle_incoming_from_host)
        thread2.start()

    def log(self, identifier, side, data):
        print("{} {}: {}".format(identifier, side, data[:].hex()))
        # teststr = ""
        # for byte in data[:]:
        #     teststr += str(int(byte))
        #     teststr += " "
        # print("[{}, TCP] {}: {}".format(self.port, side, teststr))

#! TCP not actually needed for DPLAY
# proxy_dplay0 = DPlayProxy(IP_HOST_PROXIED, IP_HOST_ORIGIN, PORT_DPLAY, IP_CLIENT_PROXIED)
# proxy_dplay0.start()
testproxy3 = UDPProxy(IP_HOST_ORIGIN, IP_CLIENT_ORIGIN, IP_HOST_PROXIED, IP_CLIENT_PROXIED, PORT_DPLAY)
testproxy3.start()

# for port in PORT_RANGE_GAME:
#     testproxy0 = DPlayProxy(IP_CLIENT_PROXIED, IP_CLIENT_ORIGIN, port, IP_HOST_PROXIED)
#     testproxy0.start()
#     testproxy1 = DPlayProxy(IP_HOST_PROXIED, IP_HOST_ORIGIN, port, IP_CLIENT_PROXIED)
#     testproxy1.start()
#     testproxy2 = UDPProxy(IP_CLIENT_ORIGIN, IP_HOST_ORIGIN, IP_CLIENT_PROXIED, IP_HOST_PROXIED, port)
#     testproxy2.start()

testproxy0 = DPlayProxy(IP_CLIENT_PROXIED, IP_CLIENT_ORIGIN, 2300, IP_HOST_PROXIED)
testproxy0.start()
testproxy1 = DPlayProxy(IP_HOST_PROXIED, IP_HOST_ORIGIN, 2300, IP_CLIENT_PROXIED)
testproxy1.start()
testproxy2 = UDPProxy(IP_CLIENT_ORIGIN, IP_HOST_ORIGIN, IP_CLIENT_PROXIED, IP_HOST_PROXIED, 2350)
testproxy2.start()
import socket
import time

HOST = '127.0.0.1'  # The server's hostname or IP address
PORT = 4242        # The port used by the server

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))
    s.sendall(b'\x00\x1e\x00abcdefghijklmnopqrstuvwxyz!')
    for i in range(10):
        s.sendall(b'\xFF\xFF\xFF' + i.to_bytes(1, 'little'))
        print(str(i) + ':' + str(s.recv(1024)))
        time.sleep(1)

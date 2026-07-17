

// runPipeProxy({
//   PORT_DPLAY,
//   PORTS_GAME,
//   IP_ANNO_CLIENT,
//   IP_PROXY_HOST,
//   IP_ANNO_HOST,
//   IP_PROXY_CLIENT,
// });

// const TCPProxyDPLAY1 = TCPProxy({ hostIP: IP_ANNO_HOST, port: PORT_DPLAY});
// TCPProxyDPLAY1.on("connection", (socket) => {
//   const TCPProxyDPLAY2 = TCPProxy({ hostIP: socket.remoteAddress!, port: PORT_GAME_RANGE_START});
// })

// const TCPProxyDPLAY2 = TCPProxy({ hostIP: IP_ANNO_HOST, port: PORTS_GAME[0]});
// TCPProxyDPLAY1.on("connection", (socket) => {
//   const TCPProxyDPLAY2 = TCPProxy({ hostIP: socket.remoteAddress!, port: PORT_GAME_RANGE_START});
// })

// DPlay TCP 47624 #1: Client -> Proxy -> Host
// DPlay TCP 47624 #2: Host -> Proxy -> Client
// Establish connection to Host as soon as Client connects

// const TCPRelay1 = net.createServer();
// TCPRelay1.on("connection", (clientSocket) => {
//   console.log(`[TCPRelay1] received connection.`);
//   const hostConnection = new net.Socket();
//   hostConnection.connect(PORT_DPLAY, IP_ANNO_HOST, () => {
//     console.log("[TCPRelay1] connected to 1602 host.")
//     clientSocket.on("data", (data) => {
//       console.log("[TCPRelay1] client -> host: ", data.toString("hex"));
//       hostConnection.write(data);
//     });
//     hostConnection.on("data", (data) => {
//       console.log("[TCPRelay1] host -> client: ", data.toString("hex"));
//       clientSocket.write(data);
//     });
//   });
// });
// TCPRelay1.listen(PORT_DPLAY);

// const controller = new AbortController();
// const signal = controller.signal;
// const UDPRelay1 = dgram.createSocket({
//   type: "udp4", signal
// });

// UDPRelay1.on('message', (msg, rinfo) => {
//   console.log(`server got: ${msg.toString("hex")} from ${rinfo.address}:${rinfo.port}`);
//   UDPRelay1.send(msg, PORT_DPLAY, IP_ANNO_HOST);
// });

// UDPRelay1.bind(PORT_DPLAY);

// const TCPRelay2 = net.createServer();
// TCPRelay2.on("connection", (clientSocket) => {
//   console.log(`[TCPRelay2] received connection.`);
//   const hostConnection = new net.Socket();
//   hostConnection.connect(PORT_GAME_RANGE_START, IP_ANNO_HOST, () => {
//     console.log("[TCPRelay2] connected to 1602 host.")
//     clientSocket.on("data", (data) => {
//       console.log("[TCPRelay2] client -> host: ", data.toString("hex"));
//       hostConnection.write(data);
//     });
//     hostConnection.on("data", (data) => {
//       console.log("[TCPRelay2] host -> client: ", data.toString("hex"));
//       clientSocket.write(data);
//     });
//   });
// });
// TCPRelay2.listen(PORT_GAME_RANGE_START);

// const controller2 = new AbortController();
// const signal2 = controller2.signal;
// const UDPRelay2 = dgram.createSocket({
//   type: "udp4", signal: signal2
// });

// UDPRelay2.on('message', (msg, rinfo) => {
//   console.log(`server got: ${msg.toString("hex")} from ${rinfo.address}:${rinfo.port}`);
//   UDPRelay2.send(msg, PORT_GAME_RANGE_START, IP_ANNO_HOST);
// });

// UDPRelay2.bind(PORT_GAME_RANGE_START);

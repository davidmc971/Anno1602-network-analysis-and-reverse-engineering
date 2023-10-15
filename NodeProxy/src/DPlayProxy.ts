import net from "net";
import dgram from "dgram";

export interface TCPProxyProps {
  hostIP: string;
  port: number;
}

export const TCPProxy = ({ hostIP, port }: TCPProxyProps): net.Server => {
  // listen on connection and forward each to host
  const TCPListener1 = net.createServer((clientToProxyConnection) => {
    const localAddress = clientToProxyConnection.localAddress
      ?.split(":")
      .slice(-1)[0];
    const localPort = clientToProxyConnection.localPort;
    const remoteAddress = clientToProxyConnection.remoteAddress
      ?.split(":")
      .slice(-1)[0];
    const remotePort = clientToProxyConnection.remotePort;
    console.log(
      `Connection received on ${localAddress}:${localPort} from ${remoteAddress}:${remotePort}`
    );
    const proxyToHostConnection = net.createConnection(
      {
        host: hostIP,
        port: port,
      },
      () => {
        const hostAddress = proxyToHostConnection.remoteAddress;
        const hostPort = proxyToHostConnection.remotePort;
        const proxyAddress = proxyToHostConnection.localAddress;
        const proxyPort = proxyToHostConnection.localPort;
        console.log(
          `Connection established to ${hostAddress}:${hostPort} from ${proxyAddress}:${proxyPort}`
        );
        const logClientToHost = (msg: string) =>
          console.log(
            `${remoteAddress}:${remotePort} -> ${localAddress}:${localPort} => ${proxyAddress}:${proxyPort} -> ${hostAddress}:${hostPort}:\n${msg}`
          );
        const logHostToClient = (msg: string) =>
          console.log(
            `${hostAddress}:${hostPort} -> ${proxyAddress}:${proxyPort} => ${localAddress}:${localPort} -> ${remoteAddress}:${remotePort}:\n${msg}`
          );
        clientToProxyConnection.on("data", (clientToProxyData) => {
          logClientToHost(clientToProxyData.toString("hex"));
          proxyToHostConnection.write(clientToProxyData);
        });
        proxyToHostConnection.on("data", (hostToProxyData) => {
          logHostToClient(hostToProxyData.toString("hex"));
          clientToProxyConnection.write(hostToProxyData);
        });
      }
    );
  });
  TCPListener1.listen(port);
  return TCPListener1;
};

// const log = (msg: string) => console.log(`DPLAY: ${msg}`);

// const DPlayListener = net.createServer();
// DPlayListener.on("connection", (stream) => {
//   log("client connection received");

//   const proxyConnection = new net.Socket();
//   proxyConnection.connect(2300, "10.30.0.2", () => {
//     log("host connection established");
//     stream.on("data", (data) => {
//       log(`clnConn ${data.toString("hex")}`);
//       proxyConnection.write(data);
//     });
//     proxyConnection.on("data", (data) => {
//       log(`hstConn ${data.toString("hex")}`);
//       stream.write(data);
//     });
//   });
// });

// const controller = new AbortController();
// const signal = controller.signal;
// const DPlayUDP = dgram.createSocket({
//   type: "udp4", signal
// });

// DPlayUDP.on('message', (msg, rinfo) => {
//   console.log(`server got: ${msg} from ${rinfo.address}:${rinfo.port}`);
//   DPlayUDP.send(msg, 47624, "10.30.0.2");
// });

// DPlayUDP.bind(47624);

// export default { DPlayListener };

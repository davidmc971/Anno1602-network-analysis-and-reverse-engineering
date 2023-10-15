import net from "net";
import dgram from "dgram";
import { TCPProxy } from "./DPlayProxy";
import { runPipeProxy } from "./PipeProxy";

const PORT_DPLAY = 47624;
const PORT_GAME_RANGE_START = 2300;
const PORT_GAME_RANGE_END = 2400;
const PORTS_GAME = Array.from(
  Array(PORT_GAME_RANGE_END - PORT_GAME_RANGE_START + 1).keys()
).map((x) => x + PORT_GAME_RANGE_START);

const IP_ANNO_CLIENT = "10.20.0.2";
const IP_PROXY_HOST = "10.20.0.1";

// // const IP_ANNO_HOST = "10.30.0.2";
// // const IP_PROXY_CLIENT = "10.30.0.1";
// // const IP_ANNO_HOST = "192.168.2.36";
// // const IP_PROXY_CLIENT = "192.168.2.110";
// const IP_ANNO_HOST = "192.168.0.66";
// const IP_PROXY_CLIENT = "192.168.0.204";

// const IP_ANNO_CLIENT = "192.168.178.38";
// const IP_PROXY_HOST = "192.168.178.36";

const IP_ANNO_HOST = "10.30.0.2";
const IP_PROXY_CLIENT = "10.30.0.1";

export interface ProxyRunConfig {
  PORT_DPLAY: number;
  PORTS_GAME: number[];
  IP_ANNO_CLIENT: string;
  IP_PROXY_HOST: string;
  IP_ANNO_HOST: string;
  IP_PROXY_CLIENT: string;
}

export const Anno1602ProxyRunConfig: ProxyRunConfig = {
  PORT_DPLAY,
  PORTS_GAME,
  IP_ANNO_CLIENT,
  IP_PROXY_HOST,
  IP_ANNO_HOST,
  IP_PROXY_CLIENT,
};

export interface Parser {
  parse: (data: Buffer) => Buffer;
}

const dplayParser: Parser = {
  parse: (data: Buffer) => {
    // if(data.length >= 24) {
    //   // size = struct.unpack("<H", data[0:2])[0]
    //   const size = data.readUInt16LE(0);
    //   // dplay_id = struct.unpack("<2c", data[2:4])
    //   // dplay_id = "".join([dplay_id[1].hex(), dplay_id[0].hex()])
    //   const dplayID = data.toString("hex", 2, 4);
    //   // action = struct.unpack("<cccc", data[20:24])
    //   // action = "".join([action[0].decode("ASCII"), action[1].decode("ASCII"), action[2].decode("ASCII"), action[3].decode("ASCII")])
    //   const action = data.toString("ascii", 20, 24);
    //   // if dplay_id == "fab0" and action == "play":
    //   //     return True
    //   // if config.VERBOSE_LOGGING:
    //   //     print("size: {}, dplay_id: 0x{}, action: {}".format(size, dplay_id, action))
    // }
    return data;
  },
};

const config = Anno1602ProxyRunConfig;

const UDPProxy = (
  port: number,
  config: ProxyRunConfig,
  parser?: Parser,
  injectProxyIP?: Parser
) => {
  const controller = new AbortController();
  const signal = controller.signal;
  const UDPSocketToHost = dgram.createSocket({
    type: "udp4",
    signal,
  });
  const UDPSocketToClient = dgram.createSocket({
    type: "udp4",
    signal,
  });

  // DirectPlay / UDP
  // Client:47624 -> Proxy:47624 -> Host:47624
  // Host:47624 -> Proxy:47624 -> Client:47624

  UDPSocketToHost.on("message", (msg, rinfo) => {
    let data = msg;
    console.log(
      `[DPLAY/UDP, ${rinfo.address}:${rinfo.port} to host]: ${msg.toString(
        "hex"
      )}`
    );
    if (injectProxyIP != null) {
      data = injectProxyIP.parse(data);
    }
    const toSend = parser?.parse(data);
    console.log("sending", (toSend ?? data).toString("hex"));
    UDPSocketToClient.send(toSend ?? data, port, config.IP_ANNO_CLIENT);
  });

  UDPSocketToClient.on("message", (msg, rinfo) => {
    console.log(`[DPLAY/UDP, host to client]: ${msg.toString("hex")}`);
    UDPSocketToHost.send(
      parser ? parser.parse(msg) : msg,
      port,
      config.IP_ANNO_HOST
    );
  });

  UDPSocketToHost.bind(port, config.IP_PROXY_HOST);
  UDPSocketToClient.bind(port, config.IP_PROXY_CLIENT);
};

const injectIPparser: Parser = {
  parse: (data) => {
    //addr = list(map(int, dplay_proxied_ip.split('.')))
    const addr = config.IP_PROXY_CLIENT.split(".");
    //struct.pack_into("<BBBB", data_client, 8, addr[0], addr[1], addr[2], addr[3])
    console.log(
      "Replacing Client IP with Proxy IP:",
      data.toString("hex", 8, 12)
    );
    addr.forEach((value, index) =>
      data.writeUIntLE(parseInt(value), 8 + index, 1)
    );
    console.log(
      "Replaced Client IP with Proxy IP:",
      data.toString("hex", 8, 12)
    );
    return data;
  },
};



/* //!
[DPLAY/UDP, 10.20.0.2:46125 to host]: 3400b0fa020008fc000000000000000000000000706c617902000e0021d354dee6a1d0118e7600608c96c1480000000001000000
Replacing Client IP with Proxy IP: 00000000
Replaced Client IP with Proxy IP: 0a1e0001
sending 3400b0fa020008fc0a1e00010000000000000000706c617902000e0021d354dee6a1d0118e7600608c96c1480000000001000000
*/

// config.PORTS_GAME.slice(0, 1).forEach((port) => {
  
// });

UDPProxy(47624, config, dplayParser);

UDPProxy(2350, config);

const TCPRelay1 = net.createServer();
TCPRelay1.on("connection", (clientToProxySocket) => {
  console.log(`[TCPRelay] received client connection.`);
  const proxyToHostSocket = new net.Socket();
  proxyToHostSocket.connect(2300, config.IP_ANNO_HOST, () => {
    console.log("[TCPRelay] connected to 1602 host.");

    clientToProxySocket.on("data", (data) => {
      console.log("[TCPRelay1] client -> host: ", data.toString("hex"));
      proxyToHostSocket.write(data);
    });
    proxyToHostSocket.on("data", (data) => {
      const toSend = injectIPparser.parse(data);
      console.log("[TCPRelay1] host -> client: ", toSend.toString("hex"));
      clientToProxySocket.write(toSend);
    });

  });
});

TCPRelay1.listen(2300);
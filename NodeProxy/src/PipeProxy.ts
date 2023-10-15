import net from "net";
import { AnnoProxyRunConfig } from "./app";
import { PacketParser } from "./PacketParser";

export function runPipeProxy(config: AnnoProxyRunConfig) {
  console.log("Starting pipeProxy...")
  const clientConnection = net.createServer((connection) => {
    const hostConnection = net.createConnection({
      host: config.IP_ANNO_HOST,
      port: config.PORT_DPLAY,
    });
    connection.pipe(PacketParser, { end: false }).pipe(hostConnection, { end: false });
    hostConnection.pipe(PacketParser, { end: false }).pipe(connection, { end: false });
  });
  clientConnection.listen(config.PORT_DPLAY);
}

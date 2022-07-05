import dgram from "dgram";
import DPlayProxy from "./DPlayProxy";

const PORT_DPLAY = 47624;
const PORT_GAME_RANGE_START = 2300;
const PORT_GAME_RANGE_END = 2400;
const PORTS_GAME = Array.from(
  Array(PORT_GAME_RANGE_END - PORT_GAME_RANGE_START + 1).keys()
).map((x) => x + PORT_GAME_RANGE_START);

const server = dgram.createSocket("udp4");

server.on("message", (msg, rinfo) => {
  console.log(msg);
  console.log(rinfo);
});

server.bind();

DPlayProxy.DPlayListener.listen(PORT_DPLAY);
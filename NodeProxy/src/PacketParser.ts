import { Transform } from "stream";

export const PacketParser = new Transform({
  transform(chunk, encoding, callback) {
    console.log(`Parsing packet: ${chunk.toString("hex")}`)
    this.push(chunk);
    callback();
  },
});

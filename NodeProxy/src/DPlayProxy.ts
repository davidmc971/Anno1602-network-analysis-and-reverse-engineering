import net from "net";

const DPlayListener = net.createServer();
DPlayListener.on("connection", (stream) => {
  console.log("connection received");
  stream.on("data", (data) => console.log(data));
});

export default { DPlayListener };

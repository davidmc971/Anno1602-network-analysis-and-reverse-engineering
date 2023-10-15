import express from "express";
import logger from "./logger";
import { Socket, createConnection } from "net";
import { WebSocketServer } from "ws";
import { createServer } from "http";

const app = express();
const httpServer = createServer(app);

const ws = new WebSocketServer({
  server: httpServer,
});

ws.on("connection", (client, req) => {
  logger.log("Got ws connection from " + req.socket.remoteAddress + ":" + req.socket.remotePort);
  const guacdSocket = createConnection({
    host: "127.0.0.1",
    port: 4822,
  });
  client.on("message", (data) => {
    logger.log("Message from client to guacd:", data.toString("utf-8"));
    guacdSocket.push(data);
  });
  guacdSocket.on("data", (data) => {
    logger.log("Message from guacd to client:", data.toString("utf-8"));
    client.send(data);
  });
  client.on("close", () => {
    guacdSocket.destroy();
  });
  guacdSocket.on("close", () => {
    client.close();
  });
});

httpServer.listen(8080, () => {
  logger.log("Server listening on port 8080.");
});

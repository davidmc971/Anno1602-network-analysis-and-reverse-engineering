import express from "express";
import { Socket, createConnection } from "net";
import { WebSocketServer } from "ws";
import { createServer } from "http";
import { spawn } from "child_process";
import chalk from "chalk";
import { buffer } from "stream/consumers";
import { createWriteStream, writeFileSync } from "fs";

const webmSignature = "1A 45 DF A3";

writeFileSync("video.webm", "");
const outputFile = createWriteStream("./video.webm");
// outputFile.write(
//   Buffer.of(...webmSignature.split(" ").map((hexStr) => parseInt(hexStr, 16)))
// );

writeFileSync("video_late_start.webm", "");
const secondaryTestFile = createWriteStream("./video_late_start.webm");

const app = express();
const httpServer = createServer(app);

const ws = new WebSocketServer({
  server: httpServer,
});

const ffmpegArgs = `-f x11grab -video_size 800x600 -framerate 30 -thread_queue_size 256 -i :0.0 -f pulse -thread_queue_size 256 -i default -ac 1 -c:v libvpx-vp9 -deadline realtime -cpu-used 8 -g 6 -b:v 500K -crf 48 -b:a 96K -c:a libopus -f webm pipe:3`;

const testProcess = spawn("ffmpeg", ffmpegArgs.split(" "), {
  stdio: ["pipe", "ignore", "pipe", "overlapped"],
});

testProcess.stderr?.on("data", (data) => console.log(chalk.bgRed(data)));

const videoPipe = testProcess.stdio[3];

videoPipe?.on("data", (chunk) => {
  // console.log(`[${chalk.greenBright("VIDEO CHUNK")}]`, chunk.length!);
  outputFile.write(chunk);
});

videoPipe?.on("end", () => console.log(`[${chalk.greenBright("VIDEO END")}]`));

app.get("/", (req, res) => {
  res.sendFile("./static/index.html", {
    root: process.cwd(),
  });
});

app.get("/video", (req, res) => {
  secondaryTestFile.write(
    Buffer.of(...webmSignature.split(" ").map((hexStr) => parseInt(hexStr, 16)))
  );
  const headers = {
    "Content-Type": "video/webm",
    "Transfer-Encoding": "chunked",
  };
  res.writeHead(200, headers);
  res.write("4");
  res.write("\r\n");
  res.write(
    Buffer.of(...webmSignature.split(" ").map((hexStr) => parseInt(hexStr, 16)))
  );
  res.write("\r\n");
  videoPipe?.on("data", (chunk: Buffer) => {
    secondaryTestFile.write(chunk);
    res.write(chunk.length.toString(16));
    res.write("\r\n");
    res.write(chunk);
    res.write("\r\n");
  });
});

const cleanShutdown = () => {
  console.log("Clean shutdown requested.");
  const shutdown = () => {
    httpServer.close();
    process.exit();
  };
  if (!testProcess.killed) {
    testProcess.stdin?.write("q\n");
    testProcess.on("exit", shutdown);
    setTimeout(shutdown, 2000);
  } else {
    shutdown();
  }
};
process.on("SIGINT", cleanShutdown);
process.on("SIGTERM", cleanShutdown);
process.on("exit", cleanShutdown);

httpServer.listen(8070, () => {
  console.log("Server listening on port 8070.");
});

import express from "express";
import { createServer } from "http";
import { spawn } from "child_process";
import chalk from "chalk";
import SocketIO from "socket.io";
import eiows from "eiows";
import msgpackParser from "socket.io-msgpack-parser";
import cors from "cors";
import { Transform } from "stream";

const pngBase64Prefix = "data:image/png;base64,";

const pngSignature = {
  magic: [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a],
  ihdr: [0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52],
  iend: [0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82],
};

const app = express();
const httpServer = createServer(app);

app.use(cors());

const ws = new SocketIO.Server(httpServer, {
  wsEngine: eiows.Server,
  // WARN: Needs to be used on client as well:
  parser: msgpackParser,
  cors: {
    origin: "*",
  },
});

// const ffmpegArgs = `-f x11grab -video_size 800x600 -framerate 30 -thread_queue_size 256 -i :0.0 -f pulse -thread_queue_size 256 -i default -ac 1 -c:v libvpx-vp9 -deadline realtime -cpu-used 8 -g 6 -b:v 500K -crf 48 -b:a 96K -c:a libopus -f webm pipe:3`.split(" ");

// prettier-ignore
const ffmpegArgs = [
  // video in
  "-f", "x11grab", "-video_size", "800x600", "-framerate", "30", "-thread_queue_size", "256", "-i", ":0.0",
  // audio in
  // "-f", "pulse", "-thread_queue_size", "256", "-i", "default", "-ac", "1",
  // video out
  // "-c:v", "libx264", "-bsf:v", "h264_mp4toannexb", "-max_delay", "0", "-g", "6", "-b:v", "500K", "-crf", "48", "-bf", "0",
  // "-c:v", "mjpeg", "-q", "12", "-an",
  "-c:v", "png", "-q:v", "24",
  // audio out
  // "-b:a", "96K", "-c:a", "libopus",
  // output format
  "-f", "image2", "-c", "png", "-update", "1",
  // output destination
  "pipe:3",
];

const testProcess = spawn("ffmpeg", ffmpegArgs, {
  stdio: ["pipe", "ignore", "pipe", "overlapped"],
});

// testProcess.stderr?.on("data", (data) => console.log(chalk.bgRed(data)));

const videoPipe = testProcess.stdio[3];

videoPipe?.on("drain", () => console.log("DRAIN VIDEO pls"));

let pngBuffer: number[] = [];
let pngIndex = 0;
let match = 0;
const pngChunkStream = new Transform({
  transform(chunk: Buffer, _, callback) {
    // console.log(`[${chalk.greenBright("VIDEO CHUNK")}]`, chunk.length);
    pngBuffer.push(...chunk);

    for (; pngIndex < pngBuffer.length; pngIndex++) {
      const currentByte = pngBuffer[pngIndex];

      if (currentByte === pngSignature.iend[match++]) {
        // console.log("match", match);
        if (match === pngSignature.iend.length) {
          const pngToSend = pngBuffer.splice(0, pngIndex + 1);
          pngIndex = 0;
          match = 0;
          // console.log("PNG found, size:", pngToSend.length);
          this.push(Buffer.from(pngToSend));
        }
      } else {
        match = 0;
      }
    }

    // console.log(pngBuffer.length);

    callback();
  },
});

const pngBase64EncoderStream = new Transform({
  encoding: "utf-8",
  transform(chunk: Buffer, _, callback) {
    this.push(pngBase64Prefix.concat(chunk.toString("base64")));
    callback();
  },
});

videoPipe?.pipe(pngChunkStream).pipe(pngBase64EncoderStream);
videoPipe?.on("end", () => console.log(`[${chalk.greenBright("VIDEO END")}]`));

// import { writeFileSync } from "fs";
// let pngCounter = 0;
// pngChunkStream.on("data", (data) => {
//   writeFileSync(`./out/frame_${pngCounter++}.png`, data);
// });

app.get("/", (req, res) => {
  res.sendFile("./static/index.html", {
    root: process.cwd(),
  });
});

ws.on("connection", (socket) => {
  console.log(chalk.bgGreenBright("WS CONNECT"));
  const listener = (chunk: Buffer) => {
    socket.send(chunk);
  };
  pngBase64EncoderStream.on("data", listener);
  socket.on("close", () => {
    pngBase64EncoderStream.off("data", listener);
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

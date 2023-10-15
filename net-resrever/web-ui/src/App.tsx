import { useEffect, useState } from "react";
import "./App.css";
import socketIO from "socket.io-client";
import msgpackParser from "socket.io-msgpack-parser";

const wsClient = socketIO("ws://localhost:8070", {
  parser: msgpackParser,
  autoConnect: false,
});

// let once = false;
// function logOnce(str: string) {
//   if (!once) {
//     console.log(str);
//     once = true;
//   }
// }

export default function App() {
  // const containerRef = useRef<HTMLDivElement>(null);
  const [imgData, setImgData] = useState<string>();

  useEffect(() => {
    wsClient.connect();

    wsClient.on("message", (data) => {
      setImgData(data);
      // logOnce(data);
    });

    return () => {
      wsClient.offAny();
      wsClient.disconnect();
    };
  }, []);

  return (
    <main
      style={{
        display: "flex",
        width: "100%",
        height: "100%",
        backgroundColor: "pink",
        overflow: "hidden",
      }}
    >
      <img
        style={{
          border: "1px solid pink",
          aspectRatio: "4 / 3",
          display: "block",
          flex: 1,
          maxWidth: "50%",
          objectFit: "contain",
        }}
        src={imgData}
      ></img>
      <img
        style={{
          border: "1px solid pink",
          aspectRatio: "4 / 3",
          display: "block",
          flex: 1,
          objectFit: "contain",
          maxWidth: "50%",
        }}
        src={imgData}
      ></img>
    </main>
  );
}

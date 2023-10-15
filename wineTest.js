const { spawn } = require("child_process");

const WINEPREFIX = "/home/david/.PlayOnLinux/wineprefix/Anno1602KE_2/";
const gameFolder =
  WINEPREFIX + "drive_c/Program Files/Anno 1602 Königs-Edition/";
const executable = "1602.exe";
// const pathToExecutable = gameFolder + executable;

const wineProcess = spawn("wine", [executable], {
  env: {
    ...process.env,
    WINEPREFIX,
    WINEDEBUG: "+winsock,+dplay"
  },
  cwd: gameFolder,
});

wineProcess.stdout.on("data", (data) => {
  console.log(`stdout: ${data}`.trimEnd());
});

wineProcess.stderr.on("data", (data) => {
  console.log(`stderr: ${data}`.trimEnd());
});

wineProcess.on("close", (code) => {
  console.log(`child process exited with code ${code}`);
});

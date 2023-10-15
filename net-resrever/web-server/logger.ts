import chalk from "chalk";

namespace logger {
  export function log(...data: any[]) {
    console.log(
      ...(data.length ? [`[${chalk.blueBright("INFO")}]`, new Date().toJSON(), ...data] : data)
    );
  }
}
export default logger;

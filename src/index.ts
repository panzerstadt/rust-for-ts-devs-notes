import * as fs from "fs";

const run = async () => {
  const file = await fs.readFileSync("src/lines", "utf-8");
  file.split("\n").forEach((r) => console.log(r));
};

run();

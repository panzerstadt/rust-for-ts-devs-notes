import * as fs from "fs";

export const all_lines = () => {
  const run = async () => {
    const file = await fs.readFileSync("lines", "utf-8");
    file.split("\n").forEach((r) => console.log(r));
  };

  run();
};

export const every_other_line = () => {
  const run = async () => {
    const file = await fs.readFileSync("lines", "utf-8");
    // file.split("\n").forEach((r, i) => i % 2 && console.log(r));
    file
      .split("\n")
      .filter((_, i) => i % 2 === 0)
      .forEach((r) => console.log(r));
  };

  run();
};

/*
in order
- every other line
- skip first 2
- print 2
*/
export const custom = () => {
  const run = async () => {
    const file = await fs.readFileSync("lines", "utf-8");
    // file.split("\n").forEach((r, i) => i % 2 && console.log(r));
    file
      .split("\n")
      .filter((_, i) => i % 2 === 0)
      .map((d) => {
        console.log(`debug 1: ${d}`);
        return d;
      })
      .filter((_, i) => i > 1 && i < 4)
      .map((d) => {
        console.log(`debug 2: ${d}`);
        return d;
      })
      .forEach((r) => console.log(r));
  };

  run();
};

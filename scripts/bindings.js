// generate-index.js
import fs from "fs";
import path from "path";

const bindingsDir = "./bindings";   // change if needed
const indexFile = path.join(bindingsDir, "index.ts");

// read all .ts files in bindings folder
const files = fs
  .readdirSync(bindingsDir)
  .filter(
    (f) =>
      f.endsWith(".ts") &&
      f !== "index.ts" &&
      !f.endsWith(".d.ts")
  );

// create export lines
const exports = files
  .map((file) => {
    const name = file.replace(".ts", "");
    return `export * from "./${name}";`;
  })
  .join("\n")
  .trim() + "\n";

// write to index.ts
fs.writeFileSync(indexFile, exports);

console.log("✔ index.ts updated:");
console.log(exports);

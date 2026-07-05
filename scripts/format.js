#!/usr/bin/env node
import { execSync } from "node:child_process";

function run(cmd) {
  console.log(`$ ${cmd}`);
  execSync(cmd, { stdio: "inherit", cwd: "../backoffice" });
}

function checkInstalled(component) {
  try {
    const out = execSync(`rustup component list --installed`, { encoding: "utf8" });
    return out.includes(component);
  } catch {
    return false;
  }
}

function commandExists(cmd) {
  try {
    execSync(`where ${cmd}`, { stdio: "ignore" });
    return true;
  } catch {
    return false;
  }
}

if (!checkInstalled("rustfmt")) {
  console.log("Installing rustfmt...");
  run("rustup component add rustfmt");
}

if (!commandExists("cargo-sort")) {
  console.log("Installing cargo-sort...");
  run("cargo install cargo-sort");
}

if (!checkInstalled("clippy")) {
  console.log("Installing clippy...");
  run("rustup component add clippy");
}

console.log("Running formatting and checks...");
run("cargo fmt");
run("cargo sort");
run("cargo group-imports --fix");
// run("cargo clippy -- -D warnings");

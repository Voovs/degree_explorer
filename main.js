import init, { run_app } from "./pkg/degree_explorer.js";

async function main() {
  await init("/pkg/degree_explorer_bg.wasm");
  run_app();
}

main();

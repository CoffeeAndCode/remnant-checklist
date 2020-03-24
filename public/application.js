import init, { start } from "./pkg/remnant.js";

init("./pkg/remnant_bg.wasm").then(_ => {
  start();
}, console.error);

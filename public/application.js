import init, { greet } from "./pkg/remnant.js";

init("./pkg/remnant_bg.wasm").then(_ => {
  greet();
}, console.error);

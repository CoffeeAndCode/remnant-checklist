import init, { start } from "./pkg/remnant.js";

document.body.addEventListener("touchmove", function (event) {
  event.preventDefault();
});

init("./pkg/remnant_bg.wasm").then((_) => {
  start();
}, console.error);

if (navigator.serviceWorker) {
  navigator.serviceWorker
    .register("/serviceworker.js", { scope: "/" })
    .then(function () {
      console.log("[Companion]", "Service worker registered!");
    }, console.error);
}

var CACHE_VERSION = "14";
var CACHE_NAME = CACHE_VERSION + ":sw-cache-";

var cachedURLs = [
  "/application.css",
  "/application.js",
  "/css/todo.css",
  "/pkg/remnant.js",
  "/pkg/remnant_bg.wasm",
  "/index.html",
  "/images/icon-32.png",
  "/images/icon-180.png",
  "/images/icon-192.png",
  "/images/icon-512.png",
  "/images/remnant-logo.png",
  "/manifest.webmanifest",
  "/pkg/snippets/remnant-68c4c8796a519e8d/src/js/share.js",
  "/pkg/snippets/remnant-68c4c8796a519e8d/src/js/stats.js",
];

function onInstall(event) {
  console.log("[Serviceworker]", "Installing!", event);
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then(function prefill(cache) {
      return cache.addAll(cachedURLs);
    }, console.error)
  );
}

function onActivate(event) {
  console.log("[Serviceworker]", "Activating!", CACHE_NAME, event);
  event.waitUntil(
    caches.keys().then(function (cacheNames) {
      return Promise.all(
        cacheNames
          .filter(function (cacheName) {
            // Return true if you want to remove this cache,
            // but remember that caches are shared across
            // the whole origin
            return cacheName.indexOf(CACHE_VERSION) !== 0;
          })
          .map(function (cacheName) {
            return caches.delete(cacheName);
          })
      );
    })
  );
}

// Borrowed from https://github.com/TalAter/UpUp
function onFetch(event) {
  event.respondWith(
    // try to return untouched request from network first
    fetch(event.request).catch(function () {
      // if it fails, try to return request from the cache
      return caches.match(event.request).then(function (response) {
        if (response) {
          return response;
        }

        // if not found in cache, return default offline content
        // (only if this is a navigation request. In older browsers we check if this is a text/html request. Thanks @jeffposnick)
        if (
          event.request.mode === "navigate" ||
          (event.request.method === "GET" &&
            event.request.headers.get("accept").includes("text/html"))
        ) {
          console.log("[Serviceworker]", "Fetching offline content", event);
          return caches.match("/index.html");
        }
      });
    })
  );
}

self.addEventListener("install", onInstall);
self.addEventListener("activate", onActivate);
self.addEventListener("fetch", onFetch);

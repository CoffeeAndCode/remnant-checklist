var CACHE_VERSION = "10";
var CACHE_NAME = CACHE_VERSION + ":sw-cache-";

var cachedURLs = [
  "/application.js",
  "/pkg/remnant.js",
  "/pkg/remnant_bg.wasm",
  "/index.html",
  "/images/icon-32.png",
  "/images/icon-192.png",
  "/images/icon-512.png",
  "/manifest.webmanifest",
  "https://cdn.jsdelivr.net/npm/todomvc-app-css@2.1.2/index.css",
  "https://cdn.jsdelivr.net/npm/todomvc-common@1.0.5/base.css",
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

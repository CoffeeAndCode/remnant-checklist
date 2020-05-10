var CACHE_VERSION = "4";
var CACHE_NAME = CACHE_VERSION + ":sw-cache-";

var cachedURLs = [
  "https://remnant.coffee.dev/application.js",
  "https://remnant.coffee.dev/pkg/remnant.js",
  "https://remnant.coffee.dev/pkg/remnant_bg.wasm",
  "https://remnant.coffee.dev/index.html",
  "https://remnant.coffee.dev/images/icon-32.png",
  "https://remnant.coffee.dev/images/icon-192.png",
  "https://remnant.coffee.dev/images/icon-512.png",
  "https://remnant.coffee.dev/manifest.webmanifest",
  "https://cdn.jsdelivr.net/npm/todomvc-app-css@2.1.2/index.css",
  "https://cdn.jsdelivr.net/npm/todomvc-common@1.0.5/base.css",
];

function onInstall(event) {
  console.log("[Serviceworker]", "Installing!", event);
  self.skipWaiting();
  event.waitUntil(
    caches.open(CACHE_NAME).then(function prefill(cache) {
      return cache.addAll(cachedURLs);
    })
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

function isHTMLRequest(request) {
  return (
    request.mode === "navigate" ||
    (request.method === "GET" &&
      request.headers.get("accept").includes("text/html"))
  );
}

// Borrowed from https://github.com/TalAter/UpUp
function onFetch(event) {
  if (cachedURLs.includes(event.request.url) || isHTMLRequest(event.request)) {
    event.respondWith(
      // try to return untouched request from network first
      fetch(event.request).catch(function () {
        // if it fails, try to return request from the cache
        return caches.match(event.request).then(function (response) {
          if (response) {
            return response;
          }

          // if not found in cache, return default offline content for navigate requests
          if (isHTMLRequest(event.request)) {
            console.log("[Serviceworker]", "Fetching offline content", event);
            return caches.match("/index.html");
          }
        });
      })
    );
  }
}

self.addEventListener("install", onInstall);
self.addEventListener("activate", onActivate);
self.addEventListener("fetch", onFetch);

const CACHE = 'v1';
const FILES = [
  './',
  './index.html',
  './style.css',
  './manifest.json',
  './icon.svg',
  './pkg/one_two_three_four.js',
  './pkg/one_two_three_four_bg.wasm',
];

self.addEventListener('install', e => {
  e.waitUntil(
    caches.open(CACHE).then(cache => cache.addAll(FILES)).then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', e => {
  e.waitUntil(
    caches.keys().then(keys => Promise.all(
      keys.filter(k => k !== CACHE).map(k => caches.delete(k))
    )).then(() => clients.claim())
  );
});

self.addEventListener('fetch', e => {
  e.respondWith(
    caches.match(e.request).then(cached => {
      const fetched = fetch(e.request).then(res => {
        if (res.ok) caches.open(CACHE).then(cache => cache.put(e.request, res.clone()));
        return res;
      });
      return cached || fetched;
    })
  );
});

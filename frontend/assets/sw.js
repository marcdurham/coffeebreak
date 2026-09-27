// Coffee Breaks service worker: network-first for the app shell so new
// deploys show up, cache fallback for offline. API and map tiles are never
// cached here.
const CACHE = 'sb-shell-v2';

self.addEventListener('install', (e) => {
  self.skipWaiting();
});

self.addEventListener('activate', (e) => {
  e.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(keys.filter((k) => k !== CACHE).map((k) => caches.delete(k)))
    ).then(() => self.clients.claim())
  );
});

// A non-navigation request answered with HTML is the server's SPA fallback
// for a file that no longer exists (e.g. an old hashed .wasm after a
// deploy) -- never cache that under the asset's URL.
function cacheable(req, res) {
  if (!res.ok || res.type === 'opaque') return res.type === 'opaque';
  const type = res.headers.get('content-type') || '';
  return req.mode === 'navigate' || !type.includes('text/html');
}

self.addEventListener('fetch', (e) => {
  const req = e.request;
  const url = new URL(req.url);
  if (req.method !== 'GET') return;
  if (url.pathname.startsWith('/api/')) return; // always live
  if (url.origin !== location.origin && !url.hostname.includes('unpkg.com')
      && !url.hostname.includes('fonts.g')) return; // skip map tiles etc.

  // The shell (index.html) must bypass the HTTP cache too, or a stale copy
  // pointing at old hashed files can outlive a deploy.
  const live = req.mode === 'navigate' ? fetch(req, { cache: 'no-store' }) : fetch(req);

  e.respondWith(
    live
      .then((res) => {
        if (cacheable(req, res)) {
          const copy = res.clone();
          caches.open(CACHE).then((c) => c.put(req, copy));
        }
        return res;
      })
      .catch(() =>
        caches.match(req, { ignoreSearch: req.mode === 'navigate' })
          .then((hit) => hit || (req.mode === 'navigate' ? caches.match('/') : undefined))
          .then((hit) => hit || Response.error())
      )
  );
});

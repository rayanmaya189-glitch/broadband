import '@testing-library/jest-dom/vitest';

// ─── Provide a working localStorage/sessionStorage polyfill ─────────────────
// jsdom in some Node/Vitest versions doesn't provide a usable localStorage.
// We install a Map-based polyfill here so every test file gets it automatically.
const storageMap = new Map<string, string>();
const polyfill: Storage = {
  getItem: (key: string) => storageMap.get(key) ?? null,
  setItem: (key: string, value: string) => { storageMap.set(key, String(value)); },
  removeItem: (key: string) => { storageMap.delete(key); },
  clear: () => { storageMap.clear(); },
  get length() { return storageMap.size; },
  key: (i: number) => [...storageMap.keys()][i] ?? null,
};

Object.defineProperty(globalThis, 'localStorage', { value: polyfill, writable: true, configurable: true });
Object.defineProperty(globalThis, 'sessionStorage', { value: polyfill, writable: true, configurable: true });

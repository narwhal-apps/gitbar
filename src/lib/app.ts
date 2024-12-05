import { fetch } from '@tauri-apps/plugin-http';

export const SERVER_PORTS = [44212, 23212, 44331];

export async function getServerPort() {
  for (const port of SERVER_PORTS) {
    const res = await fetch(`http://localhost:${port}/ping`, {
      method: 'GET',
      headers: { Accept: 'application/json' },
    });

    if (res.ok) {
      const data = await res.json();
      if (data.pong) {
        return port;
      }
    }
  }

  return SERVER_PORTS[0];
}

import { ResponseType, fetch } from '@tauri-apps/plugin-http';

export const SERVER_PORTS = [44212, 23212, 44331];

export async function getServerPort() {
  for (const port of SERVER_PORTS) {
    const res = await fetch<{ pong: true }>(`http://localhost:${port}/ping`, {
      method: 'GET',
      headers: { Accept: 'application/json' },
      responseType: ResponseType.JSON,
    });

    if (res.ok && res.data.pong) {
      return port;
    }
  }

  return SERVER_PORTS[0];
}

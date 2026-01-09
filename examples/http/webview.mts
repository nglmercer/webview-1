import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { Application, getWebviewVersion } from '../../index.js';
import { Worker, WorkerOptions } from 'node:worker_threads';

const __dirname = dirname(fileURLToPath(import.meta.url));

console.log('Initializing http server worker...');

const worker = new Worker(join(__dirname, 'server.mjs'), {
  stdout: true,
  stderr: true,
} as WorkerOptions);

worker.on('message', (message: string) => {
  if (message === 'ready') createWindow();
});

function createWindow() {
  console.log(`Initializing webview (version: ${getWebviewVersion()})`);

  const app = new Application();
  const window = app.createBrowserWindow();
  const webview = window.createWebview();

  if (!webview.isDevtoolsOpen()) webview.openDevtools();
  webview.loadUrl('http://localhost:3000');

  app.run();
}

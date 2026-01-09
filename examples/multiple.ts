import { Application } from '../index.js'

const webview1 = new Application();

webview1.createBrowserWindow().createWebview({ url: 'https://nodejs.org' });

const webview2 = new Application();
webview2.createBrowserWindow().createWebview({ url: 'https://wikipedia.org' });

async function run() {
    await Promise.all([webview1.run(), webview2.run()]);
}

run();
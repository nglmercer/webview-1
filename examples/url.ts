import { Application, Theme } from '../index.js';

const app = new Application();
const window = app.createBrowserWindow({
  title: 'Node.js',
});

window.createWebview({
  url: 'https://nodejs.org',
});

window.setTheme(Theme.Dark);

app.run();

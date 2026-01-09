import { Application } from "../index.js";

const app = Application.newNonBlocking()
const window = app.createBrowserWindow({
  transparent: true,
  decorations: false,
});

const webview = window.createWebview({
    html: /* html */ `
      <html>
        <body style="background-color:rgba(87,87,87,0.5);">
          <h1>Hello, transparent!</h1>
        </body>
      </html>`,
    transparent: true,
    enableDevtools: true,
});

app.run();
setTimeout(() => {
  console.log("Closing app");
    app.exit()
}, 10000)
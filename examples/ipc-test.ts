import { WindowBuilder, WebViewBuilder, EventLoop, TaoTheme } from '../index'


async function main() {
  console.log('--- IPC Test ---')
  
  try {
    const eventLoop = new EventLoop()
    const window = new WindowBuilder()
      .withTitle('Antigravity Premium Dashboard')
      .withInnerSize(1200, 800)
      .withTheme(TaoTheme.Dark)
      .withDecorated(true)
      .withMenubar(true)
      .build(eventLoop)
    const html = `
      <!DOCTYPE html>
      <html>
        <head>
          <style>
            body { font-family: sans-serif; background: #1a1a1a; color: white; padding: 20px; }
            button { padding: 10px; cursor: pointer; }
            #output { margin-top: 20px; padding: 10px; background: #333; border-radius: 4px; }
            .log { margin: 5px 0; padding: 5px; background: #222; border-left: 3px solid #4CAF50; }
          </style>
        </head>
        <body>
          <h1>IPC Test</h1>
          <button id="send-btn">Send to Rust</button>
          <div id="output">Waiting for messages...</div>
          
          <script>
            const btn = document.getElementById('send-btn');
            const output = document.getElementById('output');
            
            // Log function for debugging
            function log(msg) {
              const logEntry = document.createElement('div');
              logEntry.className = 'log';
              logEntry.innerText = msg;
              output.appendChild(logEntry);
            }
            
            log('IPC interface initialized');
            
            btn.addEventListener('click', () => {
              const message = 'Hello from JS at ' + new Date().toLocaleTimeString();
              log('Sending: ' + message);
              if (window.ipc && window.ipc.postMessage) {
                window.ipc.postMessage(message);
              } else {
                log('ERROR: window.ipc.postMessage not available!');
              }
            });
            
            window.__webview_on_message__ = (msg) => {
              log('Received from Rust: ' + msg);
            };
          </script>
        </body>
      </html>
    `
    
    const builder = new WebViewBuilder()
      .withHtml(html)
      .withTitle('IPC Test')
      .withWidth(600)
      .withHeight(400)
      .withIpcHandler((msg) => {
        console.log('Rust received:', msg);
        // Reply back
        setTimeout(() => {
          webview.send('ACK: ' + msg);
        }, 500);
      });
    
    const webview = builder.buildOnWindow(window, 'ipc-webview')
    webview.openDevtools()
    // Add another listener at runtime
    webview.on((msg) => {
      console.log('Second listener received:', msg);
    });

    setInterval(() => {
      // @ts-ignore
      eventLoop.runIteration();
      // Keep references alive
      (window as any)._keepAlive = true;
      (webview as any)._keepAlive = true;
    }, 10)
  } catch (error) {
    console.error('Error:', error)
  }
}

main()

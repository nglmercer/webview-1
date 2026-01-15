import { WebViewBuilder, EventLoop } from '../index'

async function main() {
  console.log('--- WebView by HTML Example ---')
  
  try {
    const eventLoop = new EventLoop()
    
    const html = `
      <!DOCTYPE html>
      <html>
        <head>
          <style>
            body {
              font-family: system-ui, -apple-system, sans-serif;
              background: #0f172a;
              color: white;
              display: flex;
              flex-direction: column;
              align-items: center;
              justify-content: center;
              height: 100vh;
              margin: 0;
            }
            h1 { color: #38bdf8; font-size: 3rem; margin-bottom: 0.5rem; }
            p { color: #94a3b8; font-size: 1.25rem; }
            .card {
              background: #1e293b;
              padding: 2rem;
              border-radius: 1rem;
              box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
              text-align: center;
              border: 1px solid #334155;
            }
          </style>
        </head>
        <body>
          <div class="card">
            <h1>¡Hola!</h1>
            <p>Este es un WebView renderizado desde un String HTML.</p>
            <p style="font-size: 0.8rem; margin-top: 20px; opacity: 0.5;">Powered by @webviewjs/webview</p>
          </div>
        </body>
      </html>
    `
    
    const builder = new WebViewBuilder()
      .withHtml(html)
      .withTitle('HTML Simple - Webview')
      .withWidth(600)
      .withHeight(400)
    
    console.log('Creating webview with simple HTML content')
    const webview = builder.build(eventLoop, 'html-webview')
    
    console.log('✓ Webview created. ID:', webview.id)
    console.log('Presiona Ctrl+C para salir')
    
    eventLoop.run()
  } catch (error) {
    console.error('Error:', error)
  }
}

main()

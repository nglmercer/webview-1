import { WebViewBuilder, EventLoop } from '../index'

async function main() {
  console.log('--- WebView by URL Example ---')
  
  try {
    const eventLoop = new EventLoop()
    
    const builder = new WebViewBuilder()
      .withUrl('https://www.google.com')
      .withTitle('Google - Webview')
      .withWidth(1024)
      .withHeight(768)
    
    console.log('Creating webview with URL: https://www.google.com')
    const webview = builder.build(eventLoop, 'url-webview')
    
    console.log('✓ Webview created. ID:', webview.id)
    console.log('Presiona Ctrl+C para salir')
    
    eventLoop.run()
  } catch (error) {
    console.error('Error:', error)
  }
}

main()

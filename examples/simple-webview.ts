/**
 * Ejemplo simplificado de webview que no se cierra
 * 
 * Este ejemplo demuestra cómo crear un webview básico
 * usando @webviewjs/webview
 */

import { WebViewBuilder, EventLoop } from '../index'

/**
 * Función principal que crea un webview básico
 */
async function main() {
  console.log('╔════════════════════════════════════════════════════════════╗')
  console.log('║  Ejemplo de WebView con @webviewjs/webview                  ║')
  console.log('╚════════════════════════════════════════════════════════════╝')
  
  try {
    // Crear event loop
    const eventLoop = new EventLoop()
    console.log('Event loop creado')
    
    // Crear webview básico con HTML
    console.log('\nCreando webview básico...')
    
    const htmlContent = `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Mi Primer WebView</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
      margin: 0;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      color: white;
    }
    .container {
      text-align: center;
      padding: 40px;
      background: rgba(255, 255, 255, 0.1);
      backdrop-filter: blur(10px);
      border-radius: 20px;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    }
    h1 {
      font-size: 2.5em;
      margin-bottom: 20px;
      text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
    }
    p {
      font-size: 1.2em;
      line-height: 1.6;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>¡Hola desde WebView!</h1>
    <p>Este es tu primer webview con @webviewjs/webview</p>
  </div>
</body>
</html>`
    
    const builder = new WebViewBuilder()
      .withHtml(htmlContent)
      .withTitle('Mi Primer WebView')
      .withWidth(800)
      .withHeight(600)
      .withX(100)
      .withY(100)
      .withResizable(true)
      .withDecorations(true)
      .withVisible(true)
      .withFocused(true)
      .withMenubar(true)
    
    const webview = builder.build('webview-1')
    console.log('✓ WebView creado con ID:', webview.id)
    console.log('✓ Label:', webview.label)
    
    // Ejecutar JavaScript en el webview
    console.log('\nEjecutando JavaScript en el webview...')
    await webview.evaluateScript('console.log("JavaScript ejecutado desde Node.js")')
    console.log('✓ JavaScript ejecutado correctamente')
    
    // Abrir DevTools
    console.log('\nAbriendo DevTools...')
    await webview.openDevtools()
    console.log('✓ DevTools abierto')
    
    // Verificar estado de DevTools
    const devtoolsOpen = await webview.isDevtoolsOpen()
    console.log('✓ DevTools está abierto:', devtoolsOpen)
    
    // Cerrar DevTools
    console.log('\nCerrando DevTools...')
    await webview.closeDevtools()
    console.log('✓ DevTools cerrado')
    
    console.log('\n✓ Ejemplo de webview ejecutado correctamente')
    console.log('\nNota: El webview se crea pero no se muestra visualmente')
    console.log('      en este entorno de prueba. En una aplicación real,')
    console.log('      usarías eventLoop.run() para mantener el webview abierto.')
    
  } catch (error) {
    console.error('Error al ejecutar ejemplo:', error)
    process.exit(1)
  }
}

// Ejecutar si este archivo se ejecuta directamente
if (require.main === module) {
  main()
}

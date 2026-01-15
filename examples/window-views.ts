/**
 * Ejemplo de ventana con múltiples vistas (WebViews)
 * 
 * Este ejemplo demuestra cómo crear una ventana básica y luego
 * añadirle múltiples WebViews como elementos de vista.
 */

import { WindowBuilder, WebViewBuilder, EventLoop, TaoTheme } from '../index'

async function main() {
  
  try {
    const eventLoop = new EventLoop()
    console.log('Event loop creado')
    
    // 1. Crear la ventana principal
    const window = new WindowBuilder()
      .withTitle('Ventana con Múltiples Vistas')
      .withInnerSize(1000, 800)
      .withTheme(TaoTheme.Dark)
      .build(eventLoop)
    
    console.log('✓ Ventana creada con ID:', window.id)

    // 2. Crear la primera vista (WebView arriba - Header)
    console.log('\n2. Añadiendo Header (Vista 1)...')
    const headerHtml = `
      <!DOCTYPE html>
      <html>
        <head>
          <style>
            body {
              margin: 0;
              padding: 0;
              background: #1e293b;
              color: white;
              font-family: system-ui;
              display: flex;
              align-items: center;
              justify-content: center;
              height: 100vh;
              border-bottom: 2px solid #38bdf8;
            }
            h1 { margin: 0; font-size: 1.5rem; color: #38bdf8; }
          </style>
        </head>
        <body>
          <h1>Panel de Control Multi-Vista</h1>
        </body>
      </html>
    `
    const headerView = new WebViewBuilder()
      .withHtml(headerHtml)
      .buildOnWindow(window, 'header-view')
    

    // 3. Crear la segunda vista (WebView Principal - Contenido)
    const contentHtml = `
      <!DOCTYPE html>
      <html>
        <head>
          <style>
            body {
              margin: 0;
              padding: 40px;
              background: #0f172a;
              color: #94a3b8;
              font-family: system-ui;
            }
            .grid {
              display: grid;
              grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
              gap: 20px;
            }
            .card {
              background: #1e293b;
              padding: 20px;
              border-radius: 10px;
              border: 1px solid #334155;
            }
            .card h3 { color: white; margin-top: 0; }
            button {
              background: #38bdf8;
              color: #0f172a;
              border: none;
              padding: 10px 20px;
              border-radius: 5px;
              cursor: pointer;
              font-weight: bold;
            }
          </style>
        </head>
        <body>
          <h2>Dashboard</h2>
          <div class="grid">
            <div class="card">
              <h3>Estadísticas</h3>
              <p>Usuarios activos: 1,234</p>
              <button onclick="alert('Actualizando...')">Actualizar</button>
            </div>
            <div class="card">
              <h3>Estado</h3>
              <p>Sistema operando correctamente</p>
            </div>
          </div>
        </body>
      </html>
    `
    const contentView = new WebViewBuilder()
      .withHtml(contentHtml)
      .buildOnWindow(window, 'content-view')
    
      console.log({
          window,
          headerView,
          contentView
      })
    eventLoop.run()
  } catch (error) {
    console.error('Error al ejecutar ejemplo:', error)
    process.exit(1)
  }
}

main()

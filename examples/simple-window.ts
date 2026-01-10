/**
 * Ejemplo simplificado de ventana que no se cierra
 * 
 * Este ejemplo demuestra cómo crear una ventana básica
 * usando @webviewjs/webview
 */

import { WindowBuilder, EventLoop } from '../index'

/**
 * Función principal que crea una ventana básica
 */
async function main() {
  console.log('╔════════════════════════════════════════════════════════════╗')
  console.log('║  Ejemplo de Ventana con @webviewjs/webview                  ║')
  console.log('╚════════════════════════════════════════════════════════════╝')
  
  try {
    // Crear event loop
    const eventLoop = new EventLoop()
    console.log('Event loop creado')
    
    // Crear ventana básica
    console.log('\nCreando ventana básica...')
    
    const builder = new WindowBuilder()
      .withTitle('Mi Primera Ventana')
      .withInnerSize(800, 600)
      .withPosition(100, 100)
      .withResizable(true)
      .withDecorated(true)
      .withVisible(true)
      .withFocused(true)
      .withMenubar(true)
    
    const window = builder.build()
    console.log('✓ Ventana creada con ID:', window.id)
    console.log('✓ Título:', window.title())
    console.log('✓ Tamaño:', window.innerSize())
    console.log('✓ Posición:', window.outerPosition())
    console.log('✓ Visible:', window.isVisible())
    console.log('✓ Redimensionable:', window.isResizable())
    console.log('✓ Decorada:', window.isDecorated())
    console.log('✓ Maximizada:', window.isMaximized())
    console.log('✓ Minimizada:', window.isMinimized())
    console.log('✓ Always on top:', window.isAlwaysOnTop())
    console.log('✓ Focuseada:', window.isFocused())
    
    console.log('\n✓ Ejemplo de ventana ejecutado correctamente')
    console.log('\nIniciando event loop para mantener la ventana abierta...')
    console.log('Presiona Ctrl+C para cerrar la ventana')
    
    // Ejecutar el event loop para mantener la ventana abierta
    eventLoop.run()
    
  } catch (error) {
    console.error('Error al ejecutar ejemplo:', error)
    process.exit(1)
  }
}

// Ejecutar si este archivo se ejecuta directamente
if (require.main === module) {
  main()
}

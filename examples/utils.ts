/**
 * Utilidades para simplificar la creación de ventanas y webviews
 * con @webviewjs/webview
 */

import {
    WindowOptions,
    //@ts-ignore
    WindowAttributes,
    WindowSizeConstraints,
    TaoTheme,
    WebViewAttributes,
    WryTheme,
    InitializationScript,
    Size,
    //@ts-ignore
    Position
} from '../index'

/**
 * Opciones por defecto para ventanas
 */
const DEFAULT_WINDOW_OPTIONS: Partial<WindowOptions> = {
  width: 800,
  height: 600,
  x: 100,
  y: 100,
  resizable: true,
  decorations: true,
  alwaysOnTop: false,
  visible: true,
  transparent: false,
  maximized: false,
  focused: true,
  menubar: true,
  icon: undefined,
  theme: undefined
}

/**
 * Opciones por defecto para webviews
 */
const DEFAULT_WEBVIEW_OPTIONS: Partial<WebViewAttributes> = {
  width: 800,
  height: 600,
  x: 100,
  y: 100,
  resizable: true,
  menubar: true,
  maximized: false,
  minimized: false,
  visible: true,
  decorations: true,
  alwaysOnTop: false,
  transparent: false,
  focused: true,
  icon: undefined,
  theme: undefined,
  userAgent: undefined,
  initializationScripts: [],
  dragDrop: true,
  backgroundColor: undefined
}

/**
 * Crea opciones de ventana con valores por defecto
 */
export function createWindowOptions(
  title: string,
  overrides: Partial<WindowOptions> = {}
): WindowOptions {
  return {
    ...DEFAULT_WINDOW_OPTIONS,
    title,
    ...overrides
  } as WindowOptions
}

/**
 * Crea opciones de webview con valores por defecto
 */
export function createWebViewOptions(
  overrides: Partial<WebViewAttributes> = {}
): WebViewAttributes {
  return {
    ...DEFAULT_WEBVIEW_OPTIONS,
    ...overrides
  } as WebViewAttributes
}

/**
 * Crea una ventana básica con título
 */
export function createBasicWindow(title: string): WindowOptions {
  return createWindowOptions(title)
}

/**
 * Crea una ventana con tema oscuro
 */
export function createDarkWindow(title: string): WindowOptions {
  return createWindowOptions(title, {
    theme: TaoTheme.Dark
  })
}

/**
 * Crea una ventana sin decoraciones (frameless)
 */
export function createFramelessWindow(title: string): WindowOptions {
  return createWindowOptions(title, {
    decorations: false,
    alwaysOnTop: true,
    transparent: true,
    resizable: false,
    menubar: false
  })
}

/**
 * Crea una ventana maximizada
 */
export function createMaximizedWindow(title: string): WindowOptions {
  return createWindowOptions(title, {
    width: 1920,
    height: 1080,
    x: 0,
    y: 0,
    maximized: true
  })
}

/**
 * Crea una ventana centrada en el monitor
 */
export function createCenteredWindow(title: string, monitorSize: Size): WindowOptions {
  const width = 800
  const height = 600

  return createWindowOptions(title, {
    width,
    height,
    x: Math.floor((monitorSize.width - width) / 2),
    y: Math.floor((monitorSize.height - height) / 2)
  })
}

/**
 * Crea una ventana con restricciones de tamaño
 */
export function createWindowWithConstraints(
  title: string,
  constraints: WindowSizeConstraints
): { window: WindowOptions; constraints: WindowSizeConstraints } {
  return {
    window: createWindowOptions(title),
    constraints
  }
}

/**
 * Crea un webview básico con URL
 */
export function createBasicWebView(url: string): WebViewAttributes {
  return createWebViewOptions({
    url,
    title: 'WebView'
  })
}

/**
 * Crea un webview con contenido HTML
 */
export function createHtmlWebView(html: string, title = 'WebView HTML'): WebViewAttributes {
  return createWebViewOptions({
    html,
    url: undefined,
    title
  })
}

/**
 * Crea un webview con tema oscuro
 */
export function createDarkWebView(url: string): WebViewAttributes {
  return createWebViewOptions({
    url,
    theme: WryTheme.Dark
  })
}

/**
 * Crea un webview transparente (frameless)
 */
export function createTransparentWebView(html: string): WebViewAttributes {
  return createWebViewOptions({
    html,
    url: undefined,
    transparent: true,
    decorations: false,
    alwaysOnTop: true,
    resizable: false,
    menubar: false,
    dragDrop: false
  })
}

/**
 * Crea un webview con scripts de inicialización
 */
export function createWebViewWithScripts(
  url: string,
  scripts: InitializationScript[]
): WebViewAttributes {
  return createWebViewOptions({
    url,
    initializationScripts: scripts
  })
}

/**
 * Crea un script de inicialización simple
 */
export function createInitScript(js: string, once = false): InitializationScript {
  return { js, once }
}

/**
 * Crea un HTML básico para webview
 */
export function createBasicHtml(title: string, content: string): string {
  return `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>${title}</title>
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
    <h1>${title}</h1>
    <p>${content}</p>
  </div>
</body>
</html>`
}

/**
 * Crea un HTML con un contador interactivo
 */
export function createCounterHtml(title = 'Contador Interactivo'): string {
  return `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>${title}</title>
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
      font-size: 2em;
      margin-bottom: 30px;
      text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
    }
    .counter {
      font-size: 4em;
      font-weight: bold;
      margin: 30px 0;
      text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
    }
    button {
      padding: 15px 30px;
      font-size: 1.2em;
      margin: 10px;
      border: none;
      border-radius: 10px;
      cursor: pointer;
      background: white;
      color: #667eea;
      font-weight: bold;
      transition: all 0.3s ease;
    }
    button:hover {
      transform: scale(1.05);
      box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
    }
    button:active {
      transform: scale(0.95);
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Contador</h1>
    <div class="counter" id="counter">0</div>
    <button onclick="decrement()">-</button>
    <button onclick="increment()">+</button>
  </div>
  <script>
    let count = 0;
    const counterEl = document.getElementById('counter');
    
    function increment() {
      count++;
      counterEl.textContent = count;
    }
    
    function decrement() {
      count--;
      counterEl.textContent = count;
    }
  </script>
</body>
</html>`
}

/**
 * Crea un HTML con información del sistema
 */
export function createSystemInfoHtml(): string {
  return `<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Información del Sistema</title>
  <style>
    body {
      font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
      margin: 0;
      background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
      color: #e0e0e0;
    }
    .container {
      padding: 40px;
      background: rgba(255, 255, 255, 0.05);
      backdrop-filter: blur(10px);
      border-radius: 20px;
      box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
      max-width: 600px;
    }
    h1 {
      font-size: 2em;
      margin-bottom: 30px;
      color: #667eea;
      text-shadow: 0 0 20px rgba(102, 126, 234, 0.5);
    }
    .info-item {
      display: flex;
      justify-content: space-between;
      padding: 15px 0;
      border-bottom: 1px solid rgba(255, 255, 255, 0.1);
    }
    .info-item:last-child {
      border-bottom: none;
    }
    .label {
      color: #888;
      font-weight: 500;
    }
    .value {
      color: #667eea;
      font-weight: bold;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Información del Sistema</h1>
    <div class="info-item">
      <span class="label">Plataforma:</span>
      <span class="value" id="platform">-</span>
    </div>
    <div class="info-item">
      <span class="label">Navegador:</span>
      <span class="value" id="browser">-</span>
    </div>
    <div class="info-item">
      <span class="label">Resolución:</span>
      <span class="value" id="resolution">-</span>
    </div>
    <div class="info-item">
      <span class="label">Ratio de píxeles:</span>
      <span class="value" id="pixelRatio">-</span>
    </div>
    <div class="info-item">
      <span class="label">Idioma:</span>
      <span class="value" id="language">-</span>
    </div>
  </div>
  <script>
    document.getElementById('platform').textContent = navigator.platform || 'Desconocido';
    document.getElementById('browser').textContent = navigator.userAgent.split(' ').pop() || 'Desconocido';
    document.getElementById('resolution').textContent = \`\${window.screen.width}x\${window.screen.height}\`;
    document.getElementById('pixelRatio').textContent = window.devicePixelRatio || 1;
    document.getElementById('language').textContent = navigator.language || 'Desconocido';
  </script>
</body>
</html>`
}

/**
 * Valida las opciones de ventana
 */
export function validateWindowOptions(options: WindowOptions): boolean {
  if (!options.title || options.title.trim() === '') {
    console.error('Error: El título de la ventana es requerido')
    return false
  }

  if (options.width <= 0 || options.height <= 0) {
    console.error('Error: El ancho y alto deben ser positivos')
    return false
  }

  return true
}

/**
 * Valida las opciones de webview
 */
export function validateWebViewOptions(options: WebViewAttributes): boolean {
  if (!options.url && !options.html) {
    console.error('Error: Se debe proporcionar una URL o contenido HTML')
    return false
  }

  if (options.width <= 0 || options.height <= 0) {
    console.error('Error: El ancho y alto deben ser positivos')
    return false
  }

  return true
}

/**
 * Imprime información de configuración para debugging
 */
export function logConfig(type: 'window' | 'webview', config: any): void {
  console.log(`=== Configuración de ${type} ===`)
  console.log(JSON.stringify(config, null, 2))
  console.log('================================')
}

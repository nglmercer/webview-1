//! Módulo de comunicación IPC entre procesos para el eventloop
//!
//! Este módulo implementa un protocolo de comunicación basado en JSON para permitir
//! que el eventloop se ejecute en un proceso separado, evitando bloqueos y permitiendo
//! que otros procesos (como conexiones TCP, WebSocket, HTTP) operen independientemente.

use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{
  mpsc::{self, Receiver, Sender, TryRecvError},
  Arc, Mutex,
};
use std::thread;

/// Mensajes que se pueden enviar desde el proceso principal al proceso del eventloop
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum IpcRequest {
  /// Crear una nueva ventana del navegador
  CreateBrowserWindow {
    window_id: u32,
    options: serde_json::Value,
    is_child: bool,
  },
  /// Cerrar una ventana específica
  CloseWindow { window_id: u32 },
  /// Crear un webview en una ventana
  CreateWebview {
    window_id: u32,
    options: serde_json::Value,
  },
  /// Ejecutar JavaScript en un webview
  EvaluateScript { window_id: u32, script: String },
  /// Cargar una URL en un webview
  LoadUrl { window_id: u32, url: String },
  /// Cargar HTML en un webview
  LoadHtml { window_id: u32, html: String },
  /// Mostrar/Ocultar ventana
  SetWindowVisible { window_id: u32, visible: bool },
  /// Establecer título de ventana
  SetWindowTitle { window_id: u32, title: String },
  /// Solicitar salir de la aplicación
  Exit,
  /// Ping para verificar conexión
  Ping,
}

/// Mensajes que se envían desde el proceso del eventloop al proceso principal
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum IpcResponse {
  /// Respuesta de éxito
  Success {
    request_id: u64,
    data: Option<serde_json::Value>,
  },
  /// Respuesta de error
  Error { request_id: u64, message: String },
  /// Evento de la aplicación (cierre de ventana, etc.)
  ApplicationEvent {
    event_type: String,
    window_id: Option<u32>,
  },
  /// Respuesta a ping
  Pong,
}

/// Wrapper para mensajes con ID de solicitud
#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct IpcMessage<T> {
  pub request_id: u64,
  pub payload: T,
}

/// Contador global para IDs de solicitud
static REQUEST_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Genera un nuevo ID de solicitud único
pub fn generate_request_id() -> u64 {
  REQUEST_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

/// Cliente IPC que se conecta al proceso del eventloop
pub struct IpcClient {
  _stream: TcpStream,
  request_sender: Sender<(u64, IpcRequest)>,
  response_receiver: Receiver<(u64, IpcResponse)>,
}

impl IpcClient {
  /// Conecta al proceso del eventloop en el puerto especificado
  pub fn connect(port: u16) -> io::Result<Self> {
    let stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    stream.set_nodelay(true)?;
    stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

    let (request_sender, request_receiver) = mpsc::channel();
    let (response_sender, response_receiver) = mpsc::channel();

    // Iniciar hilo de lectura
    let mut read_stream = stream.try_clone()?;
    thread::spawn(move || {
      let mut buffer = vec![0u8; 8192];
      loop {
        match read_stream.read(&mut buffer) {
          Ok(0) => break, // Conexión cerrada
          Ok(n) => {
            let data = &buffer[..n];
            if let Ok(response) = deserialize_response(data) {
              let _ = response_sender.send(response);
            }
          }
          Err(_) => break,
        }
      }
    });

    // Iniciar hilo de escritura
    let mut write_stream = stream.try_clone()?;
    thread::spawn(move || {
      while let Ok((request_id, request)) = request_receiver.recv() {
        let message = IpcMessage {
          request_id,
          payload: request,
        };
        if let Ok(data) = serialize_request(&message) {
          let _ = write_stream.write_all(&data);
        }
      }
    });

    Ok(Self {
      _stream: stream,
      request_sender,
      response_receiver,
    })
  }

  /// Envía una solicitud y espera la respuesta
  pub fn send_request(&self, request: IpcRequest) -> io::Result<IpcResponse> {
    let request_id = generate_request_id();
    self
      .request_sender
      .send((request_id, request))
      .map_err(io::Error::other)?;

    // Esperar respuesta con timeout
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(10);

    loop {
      match self.response_receiver.try_recv() {
        Ok((resp_id, response)) => {
          if resp_id == request_id {
            return Ok(response);
          }
        }
        Err(TryRecvError::Empty) => {
          if start.elapsed() > timeout {
            return Err(io::Error::new(
              io::ErrorKind::TimedOut,
              "Timeout waiting for response",
            ));
          }
          thread::sleep(std::time::Duration::from_millis(10));
        }
        Err(TryRecvError::Disconnected) => {
          return Err(io::Error::new(
            io::ErrorKind::ConnectionReset,
            "IPC channel disconnected",
          ));
        }
      }
    }
  }

  /// Envía una solicitud sin esperar respuesta (fire-and-forget)
  pub fn send_request_async(&self, request: IpcRequest) -> io::Result<()> {
    let request_id = generate_request_id();
    self
      .request_sender
      .send((request_id, request))
      .map_err(io::Error::other)?;
    Ok(())
  }

  /// Verifica si hay eventos pendientes
  pub fn try_recv_event(&mut self) -> Option<(u64, IpcResponse)> {
    match self.response_receiver.try_recv() {
      Ok(response) => Some(response),
      Err(TryRecvError::Empty) => None,
      Err(TryRecvError::Disconnected) => None,
    }
  }
}

/// Servidor IPC que ejecuta el eventloop y procesa solicitudes
pub struct IpcServer {
  listener: Arc<TcpListener>,
  event_sender: Sender<IpcEvent>,
  streams: Arc<Mutex<Vec<TcpStream>>>,
}

/// Eventos internos del servidor IPC
#[derive(Debug, Clone)]
pub enum IpcEvent {
  /// Solicitud recibida del cliente
  Request {
    request_id: u64,
    request: IpcRequest,
  },
  /// Cliente desconectado
  ClientDisconnected,
}

impl IpcServer {
  /// Crea un nuevo servidor IPC en un puerto disponible
  pub fn new() -> io::Result<Self> {
    Self::new_with_port(0)
  }

  /// Crea un nuevo servidor IPC en el puerto especificado
  /// Si el puerto es 0, se asigna un puerto disponible automáticamente
  pub fn new_with_port(port: u16) -> io::Result<Self> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    listener.set_nonblocking(true)?;

    let (event_sender, _event_receiver) = mpsc::channel();
    let streams = Arc::new(Mutex::new(Vec::new()));
    let streams_clone = Arc::clone(&streams);
    let event_sender_clone = event_sender.clone();
    let listener = Arc::new(listener);
    let listener_clone = Arc::clone(&listener);

    // Iniciar hilo de aceptación de conexiones y lectura
    thread::spawn(move || {
      let mut buffer = vec![0u8; 8192];

      loop {
        // Aceptar nuevas conexiones
        match listener_clone.accept() {
          Ok((stream, _)) => {
            stream.set_nodelay(true).ok();
            stream.set_nonblocking(true).ok();
            streams_clone.lock().unwrap().push(stream);
          }
          Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
          Err(_) => break,
        }

        // Leer de streams existentes
        let mut streams_guard = streams_clone.lock().unwrap();
        let mut streams_to_remove = Vec::new();

        for (idx, stream) in streams_guard.iter_mut().enumerate() {
          match stream.read(&mut buffer) {
            Ok(0) => {
              // Conexión cerrada
              streams_to_remove.push(idx);
            }
            Ok(n) => {
              let data = &buffer[..n];
              if let Ok(message) = deserialize_request(data) {
                let _ = event_sender_clone.send(IpcEvent::Request {
                  request_id: message.request_id,
                  request: message.payload,
                });
              }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {}
            Err(_) => {
              streams_to_remove.push(idx);
            }
          }
        }

        // Remover streams desconectados (en orden inverso para mantener índices válidos)
        for idx in streams_to_remove.into_iter().rev() {
          streams_guard.remove(idx);
        }

        drop(streams_guard);
        thread::sleep(std::time::Duration::from_millis(10));
      }
    });

    Ok(Self {
      listener,
      event_sender,
      streams,
    })
  }

  /// Retorna el puerto en el que está escuchando el servidor
  pub fn port(&self) -> u16 {
    self.listener.local_addr().unwrap().port()
  }

  /// Retorna el sender de eventos
  pub fn event_sender(&self) -> Sender<IpcEvent> {
    self.event_sender.clone()
  }

  /// Envía una respuesta a todos los clientes conectados
  pub fn send_response(&self, request_id: u64, response: IpcResponse) -> io::Result<()> {
    let data = serialize_response(request_id, response)?;
    let streams = self.streams.lock().unwrap();

    let mut errors = Vec::new();
    for mut stream in streams.iter() {
      if let Err(e) = stream.write_all(&data) {
        errors.push(e);
      }
    }

    if !errors.is_empty() {
      return Err(io::Error::new(
        io::ErrorKind::ConnectionReset,
        format!("Failed to send response to some clients: {:?}", errors),
      ));
    }

    Ok(())
  }

  /// Envía una respuesta a todos los clientes conectados (async)
  pub fn send_response_async(&self, request_id: u64, response: IpcResponse) {
    let data = match serialize_response(request_id, response) {
      Ok(d) => d,
      Err(_) => return,
    };
    let streams = Arc::clone(&self.streams);

    thread::spawn(move || {
      let streams = streams.lock().unwrap();
      for mut stream in streams.iter() {
        let _ = stream.write_all(&data);
      }
    });
  }

  /// Retorna el número de clientes conectados
  pub fn client_count(&self) -> usize {
    self.streams.lock().unwrap().len()
  }
}

/// Serializa una solicitud IPC
fn serialize_request(message: &IpcMessage<IpcRequest>) -> io::Result<Vec<u8>> {
  serde_json::to_vec(message).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Deserializa una solicitud IPC
fn deserialize_request(data: &[u8]) -> io::Result<IpcMessage<IpcRequest>> {
  serde_json::from_slice(data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Deserializa una respuesta IPC
fn deserialize_response(data: &[u8]) -> io::Result<(u64, IpcResponse)> {
  // Las respuestas pueden venir con o sin request_id
  if let Ok(response) = serde_json::from_slice::<IpcResponse>(data) {
    // Respuesta sin request_id (eventos)
    Ok((0, response))
  } else if let Ok(message) = serde_json::from_slice::<IpcMessage<IpcResponse>>(data) {
    // Respuesta con request_id
    Ok((message.request_id, message.payload))
  } else {
    Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "Invalid response format",
    ))
  }
}

/// Serializa una respuesta IPC
pub fn serialize_response(request_id: u64, response: IpcResponse) -> io::Result<Vec<u8>> {
  let message = IpcMessage {
    request_id,
    payload: response,
  };
  serde_json::to_vec(&message).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialization() {
    let request = IpcRequest::Ping;
    let message = IpcMessage {
      request_id: 1,
      payload: request,
    };
    let serialized = serialize_request(&message).unwrap();
    let deserialized = deserialize_request(&serialized).unwrap();
    assert_eq!(deserialized.request_id, 1);
    matches!(deserialized.payload, IpcRequest::Ping);
  }
}

// src/sim_modes/stream.rs

use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use tungstenite::accept;
use tungstenite::Message;
use serde_json;
use rand::Rng;

/// Abre un servidor WebSocket y retransmite el estado de una simulación simple.
pub fn run_streaming_mode() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Crear un servidor que escuche en el puerto 9001.
    // El frontend se conectará a esta dirección.
    let server = TcpListener::bind("127.0.0.1:9001")?;
    println!("📡 Servidor de simulación iniciado en ws://127.0.0.1:9001");
    println!("   Esperando conexión del frontend de Aletheia...");

    // 2. Aceptar la primera conexión entrante.
    // El programa se pausará aquí hasta que el frontend se conecte.
    let (stream, _) = server.accept()?;
    let mut websocket = accept(stream)?;
    println!("✅ Conexión establecida con el frontend. Iniciando stream de datos...");

    let grid_size = 20; // Tamaño de nuestra red de prueba
    let mut rng = rand::thread_rng();

    // 3. Bucle infinito para retransmitir datos.
    loop {
        // Generar un nuevo estado de la red en cada paso.
        let mut grid: Vec<Vec<i8>> = vec![vec![0; grid_size]; grid_size];
        for y in 0..grid_size {
            for x in 0..grid_size {
                // Rellenamos con 0, 1 y -1 aleatoriamente.
                grid[y][x] = rng.gen_range(-1..=1);
            }
        }

        // Convertir nuestra red (datos de Rust) a un string JSON.
        let json_payload = serde_json::to_string(&grid)?;

        // Enviar el JSON como un mensaje de texto a través del WebSocket.
        websocket.write_message(Message::Text(json_payload))?;

        // Esperar un breve momento para simular el paso del tiempo.
        sleep(Duration::from_millis(100));
    }
}
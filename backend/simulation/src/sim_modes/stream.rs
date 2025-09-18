use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use tungstenite::{accept, Message};
use serde_json;
use serde::{Deserialize, Serialize};
use rand::Rng;

// Importamos el motor de simulación y las estructuras de celdas
use crate::sim_modes::simulation_engine::{SimulationEngine, Cell};

// Estructura para serializar los parámetros que el servidor enviará
#[derive(Debug, Serialize)]
struct SimulationUpdate {
    tick: u64,
    cells: Vec<Cell>,
}

// Estructura para deserializar los comandos que el cliente enviará
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum ClientCommand {
    #[serde(rename = "INJECT_STATE")]
    InjectState {
        x: i32,
        y: i32,
        z: i32,
        state: i8,
    },
}

pub fn run_streaming_mode() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:9001")?;
    println!("📡 Servidor de simulación iniciado en ws://127.0.0.1:9001");
    println!("   Esperando conexión del frontend de Aletheia...");

    let (stream, _) = server.accept()?;
    let mut websocket = accept(stream)?;
    println!("✅ Conexión establecida con el frontend. Iniciando stream de datos...");

    let mut engine = SimulationEngine::new();
    let mut rng = rand::thread_rng();

    loop {
        // 1. Manejar mensajes entrantes del cliente
        if let Ok(msg) = websocket.read_message() {
            if msg.is_text() {
                let text = msg.to_text()?;
                if let Ok(command) = serde_json::from_str::<ClientCommand>(&text) {
                    match command {
                        ClientCommand::InjectState { x, y, z, state } => {
                            println!("🚀 Recibido comando de inyección en ({}, {}, {}) con estado {}", x, y, z, state);
                            engine.inject_state(x, y, z, state);
                            engine.debug_grid_state();
                        }
                    }
                }
            }
        }

        // 2. Ejecutar el tick de la simulación
        let updated_cells = engine.tick();

        // 3. Enviar las celdas actualizadas solo si hay cambios
        if !updated_cells.is_empty() {
            // FIX: Enviar en el formato que espera el frontend
            let update = SimulationUpdate {
                tick: engine.step,
                cells: updated_cells,
            };
            let json_payload = serde_json::to_string(&update)?;
            websocket.write_message(Message::Text(json_payload))?;
            println!("📤 Enviadas {} celdas actualizadas", update.cells.len());
        }

        sleep(Duration::from_millis(50));
    }
}
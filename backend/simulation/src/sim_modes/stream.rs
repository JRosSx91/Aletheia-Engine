use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;
use std::io::ErrorKind;
use tungstenite::{accept, Message};
use serde_json;
use serde::{Deserialize, Serialize};

use crate::sim_modes::simulation_engine::{SimulationEngine, Cell};
use crate::sim_modes::lattice_gas::LatticeGasEngine;

#[derive(Debug, Serialize)]
struct SimulationUpdate {
    tick: u64,
    cells: Vec<Cell>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload")]
enum ClientCommand {
    #[serde(rename = "INJECT_STATE")]
    InjectState { x: i32, y: i32, z: i32, state: i8 },
    // Inyección DIRIGIDA: siembra una partícula en una dirección concreta.
    // dir ∈ {"+x","-x","+y","-y","+z","-z"}
    #[serde(rename = "INJECT_DIRECTED")]
    InjectDirected { x: i32, y: i32, z: i32, dir: String },
    #[serde(rename = "SET_RUNNING")]
    SetRunning { running: bool },
    #[serde(rename = "STEP")]
    Step,
    #[serde(rename = "RESET")]
    Reset,
}

/// Contrato que cualquier motor de simulación debe cumplir para ser
/// servido por streaming. Ambos motores (ternario y gas de rejilla)
/// ya tienen esta superficie, así que solo hay que declararlo.
pub trait SimEngine {
    fn inject_state(&mut self, x: i32, y: i32, z: i32, state: i8) -> Option<Cell>;
    fn tick(&mut self) -> Vec<Cell>;
    fn reset(&mut self) -> Vec<Cell>;
    fn running(&self) -> bool;
    fn set_running(&mut self, v: bool);
    fn step(&self) -> u64;
    // Inyección dirigida. Por defecto no hace nada (motores sin dirección,
    // como el ternario, la ignoran). El gas de rejilla la sobreescribe.
    fn inject_directed(&mut self, _x: i32, _y: i32, _z: i32, _dir: &str) -> Option<Cell> {
        None
    }
    // Identidad del experimento, para que el frontend sepa qué explicar.
    fn mode_id(&self) -> &'static str;
    fn mode_name(&self) -> &'static str;
}

// --- Implementación para el motor binario/ternario (premisa 1) ---
impl SimEngine for SimulationEngine {
    fn inject_state(&mut self, x: i32, y: i32, z: i32, s: i8) -> Option<Cell> {
        SimulationEngine::inject_state(self, x, y, z, s)
    }
    fn tick(&mut self) -> Vec<Cell> { SimulationEngine::tick(self) }
    fn reset(&mut self) -> Vec<Cell> { SimulationEngine::reset(self) }
    fn running(&self) -> bool { self.running }
    fn set_running(&mut self, v: bool) { self.running = v; }
    fn step(&self) -> u64 { self.step }
    fn mode_id(&self) -> &'static str { "diffusion_binary" }
    fn mode_name(&self) -> &'static str { "Premisa 01 · Difusión Binaria" }
}

// --- Implementación para el gas de rejilla (experimento 1) ---
impl SimEngine for LatticeGasEngine {
    fn inject_state(&mut self, x: i32, y: i32, z: i32, s: i8) -> Option<Cell> {
        LatticeGasEngine::inject_state(self, x, y, z, s)
    }
    fn tick(&mut self) -> Vec<Cell> { LatticeGasEngine::tick(self) }
    fn reset(&mut self) -> Vec<Cell> { LatticeGasEngine::reset(self) }
    fn running(&self) -> bool { self.running }
    fn set_running(&mut self, v: bool) { self.running = v; }
    fn step(&self) -> u64 { self.step }
    fn inject_directed(&mut self, x: i32, y: i32, z: i32, dir: &str) -> Option<Cell> {
        LatticeGasEngine::inject_directed(self, x, y, z, dir)
    }
    fn mode_id(&self) -> &'static str { "lattice_gas" }
    fn mode_name(&self) -> &'static str { "Experimento · Gas de Rejilla" }
}

/// Punto de entrada del subcomando `stream`: premisa 1 (ternario/binario).
pub fn run_streaming_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("▶ Modo STREAM — motor ternario (premisa 1)");
    serve(SimulationEngine::new())
}

/// Punto de entrada del subcomando `lattice`: experimento del gas de rejilla.
pub fn run_lattice_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("▶ Modo LATTICE — gas de rejilla (masa + momento)");
    serve(LatticeGasEngine::new())
}

/// Fontanería compartida: un único bucle de streaming, genérico sobre el motor.
/// Cualquier experimento futuro que implemente SimEngine se sirve con esto.
fn serve<E: SimEngine>(mut engine: E) -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("127.0.0.1:9001")?;
    println!("📡 Servidor en ws://127.0.0.1:9001");

    loop {
        println!("⏳ Esperando conexión del frontend...");
        let (stream, _) = server.accept()?;

        let mut websocket = match accept(stream) {
            Ok(ws) => ws,
            Err(e) => { eprintln!("❌ Handshake falló: {e}"); continue; }
        };
        println!("✅ Conexión establecida.");

        // Avisa al frontend qué experimento está corriendo, para que
        // muestre la explicación correcta sin que nadie la seleccione a mano.
        let mode_msg = serde_json::json!({
            "type": "MODE",
            "id": engine.mode_id(),
            "name": engine.mode_name(),
        });
        if let Ok(json) = serde_json::to_string(&mode_msg) {
            let _ = websocket.write_message(Message::Text(json));
        }

        if let Err(e) = websocket.get_ref().set_read_timeout(Some(Duration::from_millis(10))) {
            eprintln!("⚠️  read_timeout: {e}");
        }

        'serve: loop {
            // 1. Comando entrante
            match websocket.read_message() {
                Ok(msg) => {
                    if msg.is_close() { break 'serve; }
                    if msg.is_text() {
                        if let Ok(text) = msg.to_text() {
                            if let Ok(cmd) = serde_json::from_str::<ClientCommand>(text) {
                                let immediate: Vec<Cell> = match cmd {
                                    ClientCommand::InjectState { x, y, z, state } => {
                                        engine.inject_state(x, y, z, state).into_iter().collect()
                                    }
                                    ClientCommand::InjectDirected { x, y, z, dir } => {
                                        engine.inject_directed(x, y, z, &dir).into_iter().collect()
                                    }
                                    ClientCommand::SetRunning { running } => {
                                        engine.set_running(running);
                                        println!("⏯️  running = {running}");
                                        Vec::new()
                                    }
                                    ClientCommand::Step => engine.tick(),
                                    ClientCommand::Reset => engine.reset(),
                                };
                                if !immediate.is_empty() {
                                    let payload = SimulationUpdate { tick: engine.step(), cells: immediate };
                                    if let Ok(json) = serde_json::to_string(&payload) {
                                        if websocket.write_message(Message::Text(json)).is_err() {
                                            break 'serve;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(tungstenite::Error::Io(e))
                    if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {}
                Err(e) => {
                    println!("🔌 Conexión perdida ({e}). Esperando nuevo cliente...");
                    break 'serve;
                }
            }

            // 2. Reloj autónomo
            if engine.running() {
                let updated = engine.tick();
                if !updated.is_empty() {
                    let payload = SimulationUpdate { tick: engine.step(), cells: updated };
                    match serde_json::to_string(&payload) {
                        Ok(json) => {
                            if let Err(e) = websocket.write_message(Message::Text(json)) {
                                println!("🔌 Escritura falló ({e}).");
                                break 'serve;
                            }
                        }
                        Err(e) => eprintln!("❌ Serializando: {e}"),
                    }
                }
            }

            sleep(Duration::from_millis(50));
        }
    }
}
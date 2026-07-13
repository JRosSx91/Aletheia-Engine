use std::collections::HashMap;
use rand::Rng;

// Reusamos el MISMO Cell que el motor ternario: es el formato del
// protocolo WebSocket, un único tipo compartido (no dos duplicados).
use crate::sim_modes::simulation_engine::Cell;

const GRID_SIZE: i32 = 50;
const HALF: i32 = GRID_SIZE / 2; // 25

// --- Los 6 canales de dirección de la rejilla cúbica ---
const PX: u8 = 1 << 0; // +x
const NX: u8 = 1 << 1; // -x
const PY: u8 = 1 << 2; // +y
const NY: u8 = 1 << 3; // -y
const PZ: u8 = 1 << 4; // +z
const NZ: u8 = 1 << 5; // -z
const CHANNELS: [u8; 6] = [PX, NX, PY, NY, PZ, NZ];

// A dónde avanza la partícula que viaja por cada canal.
fn channel_dir(bit: u8) -> (i32, i32, i32) {
    match bit {
        PX => (1, 0, 0),
        NX => (-1, 0, 0),
        PY => (0, 1, 0),
        NY => (0, -1, 0),
        PZ => (0, 0, 1),
        NZ => (0, 0, -1),
        _ => (0, 0, 0),
    }
}

// Coordenada toroidal: quien sale por un borde entra por el opuesto.
// El toro es finito, cerrado, y conserva el momento GLOBALMENTE.
fn wrap(v: i32) -> i32 {
    let size = 2 * HALF + 1;
    let shifted = v + HALF;
    (((shifted % size) + size) % size) - HALF
}

fn momentum_of(grid: &HashMap<(i32, i32, i32), u8>) -> (i32, i32, i32) {
    let (mut px, mut py, mut pz) = (0, 0, 0);
    for &cell in grid.values() {
        for bit in CHANNELS {
            if cell & bit != 0 {
                let (dx, dy, dz) = channel_dir(bit);
                px += dx; py += dy; pz += dz;
            }
        }
    }
    (px, py, pz)
}

pub struct LatticeGasEngine {
    // Rejilla DISPERSA: solo celdas ocupadas (u8 != 0). Vacío = ausente.
    pub grid: HashMap<(i32, i32, i32), u8>,
    pub step: u64,
    pub running: bool,
}

impl LatticeGasEngine {
    pub fn new() -> Self {
        println!("🌀 Gas de rejilla 3D (cúbica ±{HALF}, toroidal). Masa y momento conservados.");
        Self { grid: HashMap::new(), step: 0, running: false }
    }

    pub fn total_mass(&self) -> usize {
        self.grid.values().map(|c| c.count_ones() as usize).sum()
    }

    pub fn total_momentum(&self) -> (i32, i32, i32) {
        momentum_of(&self.grid)
    }

    // Inyecta una partícula en una dirección aleatoria en (x,y,z).
    // Firma compatible con el motor ternario (ignora `state`) para reusar stream.rs.
    pub fn inject_state(&mut self, x: i32, y: i32, z: i32, _state: i8) -> Option<Cell> {
        if x.abs() > HALF || y.abs() > HALF || z.abs() > HALF {
            println!("❌ ({x},{y},{z}) fuera de rango");
            return None;
        }
        let mut rng = rand::thread_rng();
        let bit = CHANNELS[rng.gen_range(0..6)];
        *self.grid.entry((x, y, z)).or_insert(0) |= bit; // exclusión implícita
        Some(Cell { x, y, z, state: 1 })
    }

    // Inyección DIRIGIDA por bit de canal.
    pub fn inject_particle(&mut self, x: i32, y: i32, z: i32, bit: u8) -> Option<Cell> {
        if x.abs() > HALF || y.abs() > HALF || z.abs() > HALF {
            return None;
        }
        *self.grid.entry((x, y, z)).or_insert(0) |= bit;
        Some(Cell { x, y, z, state: 1 })
    }

    // Inyección dirigida por nombre de dirección ("+x", "-x", ...).
    pub fn inject_directed(&mut self, x: i32, y: i32, z: i32, dir: &str) -> Option<Cell> {
        let bit = match dir {
            "+x" => PX, "-x" => NX,
            "+y" => PY, "-y" => NY,
            "+z" => PZ, "-z" => NZ,
            _ => {
                println!("❌ Dirección desconocida: {dir}");
                return None;
            }
        };
        println!("🎯 Partícula dirigida en ({x},{y},{z}) → {dir}");
        self.inject_particle(x, y, z, bit)
    }

    pub fn reset(&mut self) -> Vec<Cell> {
        let cleared: Vec<Cell> = self
            .grid
            .keys()
            .map(|&(x, y, z)| Cell { x, y, z, state: 0 })
            .collect();
        self.grid.clear();
        self.step = 0;
        println!("🧹 Reset: {} celdas apagadas", cleared.len());
        cleared
    }

    // Regla de colisión LOCAL. Conserva masa y momento, redistribuye dirección.
    // Mínima: solo el choque frontal de 2 cuerpos se desvía a un eje perpendicular.
    //   {+x,-x} -> {+y,-y} -> {+z,-z} -> {+x,-x}
    fn collide(cell: u8) -> u8 {
        match cell {
            c if c == (PX | NX) => PY | NY,
            c if c == (PY | NY) => PZ | NZ,
            c if c == (PZ | NZ) => PX | NX,
            other => other,
        }
    }

    pub fn tick(&mut self) -> Vec<Cell> {
        let old = self.grid.clone();
        let mom_before = momentum_of(&old);
        let mass_before = self.total_mass();

        // === FASE 1: STREAMING ===
        let mut streamed: HashMap<(i32, i32, i32), u8> = HashMap::new();
        for (&pos, &cell) in &self.grid {
            for bit in CHANNELS {
                if cell & bit != 0 {
                    let (dx, dy, dz) = channel_dir(bit);
                    let target = (wrap(pos.0 + dx), wrap(pos.1 + dy), wrap(pos.2 + dz));
                    *streamed.entry(target).or_insert(0) |= bit;
                }
            }
        }

        // === FASE 2: COLLISION ===
        let mut new_grid: HashMap<(i32, i32, i32), u8> = HashMap::new();
        for (&pos, &cell) in &streamed {
            let after = Self::collide(cell);
            if after != 0 {
                new_grid.insert(pos, after);
            }
        }

        // Verificación de la física (solo en build de debug).
        debug_assert_eq!(mom_before, momentum_of(&new_grid), "¡Momento NO conservado!");
        debug_assert_eq!(mass_before,
            new_grid.values().map(|c| c.count_ones() as usize).sum::<usize>(),
            "¡Masa NO conservada!");

        // Deltas de OCUPACIÓN para el frontend (ocupada <-> vacía).
        let mut updates = Vec::new();
        for &pos in new_grid.keys() {
            if !old.contains_key(&pos) {
                updates.push(Cell { x: pos.0, y: pos.1, z: pos.2, state: 1 });
            }
        }
        for &pos in old.keys() {
            if !new_grid.contains_key(&pos) {
                updates.push(Cell { x: pos.0, y: pos.1, z: pos.2, state: 0 });
            }
        }

        self.grid = new_grid;
        self.step += 1;

        if self.step % 100 == 0 {
            println!("t={} · masa={} · momento={:?}", self.step, self.total_mass(), self.total_momentum());
        }
        updates
    }
}
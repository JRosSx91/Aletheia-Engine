use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::Serialize;

const GRID_RADIUS: i32 = 25;
const GRID_SIDE_LENGTH: usize = (GRID_RADIUS as usize * 2) + 1;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub state: i8, // -1, 0, 1
}

pub struct SimulationEngine {
    pub grid: HashMap<(i32, i32, i32), i8>,
    pub step: u64,
    pub running: bool,
    rng: StdRng,
}

impl SimulationEngine {
    pub fn new() -> Self {
        Self::with_seed(137)
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut grid = HashMap::new();

        for x in -GRID_RADIUS..=GRID_RADIUS {
            for y in -GRID_RADIUS..=GRID_RADIUS {
                for z in -GRID_RADIUS..=GRID_RADIUS {
                    grid.insert((x, y, z), 0);
                }
            }
        }

        println!(
            "🏗️ Grid inicializado con {} celdas ({}³, rango: -{} a +{}, seed: {})",
            grid.len(),
            GRID_SIDE_LENGTH,
            GRID_RADIUS,
            GRID_RADIUS,
            seed
        );

        Self {
            grid,
            step: 0,
            running: false,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Siembra un cuanto en (x,y,z). Si esa celda ya está ocupada, busca
    /// hacia afuera (BFS por anillos) el hueco vacío más cercano y lo ocupa.
    /// Devuelve la celda realmente sembrada, o None si el grid está lleno
    /// o la coordenada de partida está fuera de rango.
    pub fn inject_state(&mut self, x: i32, y: i32, z: i32, state: i8) -> Option<Cell> {
        use std::collections::{HashSet, VecDeque};

        // La coordenada de partida tiene que existir en el grid.
        if !self.grid.contains_key(&(x, y, z)) {
            println!("❌ Coordenada ({x}, {y}, {z}) fuera de rango");
            return None;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((x, y, z));
        visited.insert((x, y, z));

        while let Some(pos) = queue.pop_front() {
            // ¿Es un hueco? Lo ocupamos y terminamos.
            if self.grid.get(&pos) == Some(&0) {
                self.grid.insert(pos, state);
                let (px, py, pz) = pos;
                if (px, py, pz) == (x, y, z) {
                    println!("💉 Inyección en ({px}, {py}, {pz}) -> {state}");
                } else {
                    println!(
                        "💉 ({x}, {y}, {z}) ocupada -> sembrado en hueco más cercano ({px}, {py}, {pz}) -> {state}"
                    );
                }
                return Some(Cell { x: px, y: py, z: pz, state });
            }

            // Ocupada: encolar los 6 vecinos para seguir buscando hacia afuera.
            let (cx, cy, cz) = pos;
            let neighbors = [
                (cx + 1, cy, cz), (cx - 1, cy, cz),
                (cx, cy + 1, cz), (cx, cy - 1, cz),
                (cx, cy, cz + 1), (cx, cy, cz - 1),
            ];
            for n in neighbors {
                // Solo dentro del grid y no visitado.
                if self.grid.contains_key(&n) && visited.insert(n) {
                    queue.push_back(n);
                }
            }
        }

        println!("🚫 Grid lleno: no hay hueco para inyectar");
        None
    }

    /// Vacía el campo entero y devuelve TODAS las celdas que estaban activas
    /// puestas a 0, para que el frontend las apague.
    pub fn reset(&mut self) -> Vec<Cell> {
        let mut cleared = Vec::new();
        for (&(x, y, z), state) in self.grid.iter_mut() {
            if *state != 0 {
                cleared.push(Cell { x, y, z, state: 0 });
                *state = 0;
            }
        }
        self.step = 0;
        println!("🧹 Reset: {} celdas apagadas", cleared.len());
        cleared
    }

    pub fn apply_changes(&mut self, changes: HashMap<(i32, i32, i32), i8>) {
        for (pos, state) in changes {
            if let Some(cell_state) = self.grid.get_mut(&pos) {
                *cell_state = state;
            }
        }
    }

    fn count_active_cells(&self) -> usize {
        self.grid.values().filter(|&&s| s != 0).count()
    }

    pub fn tick(&mut self) -> Vec<Cell> {
    if self.count_active_cells() == 0 {
        self.step += 1;
        return Vec::new();
    }

    let mut changes = HashMap::new();
    let mut updates = Vec::new();
    let grid_snapshot = self.grid.clone();

    let mut active_cells: Vec<_> = grid_snapshot
        .iter()
        .filter(|(_, state)| **state != 0)
        .map(|(&position, &state)| (position, state))
        .collect();

    active_cells.sort_by_key(|(position, _)| *position);

    for (pos, state) in active_cells {
        let (x, y, z) = pos;

        let neighbors = [
            (x + 1, y, z),
            (x - 1, y, z),
            (x, y + 1, z),
            (x, y - 1, z),
            (x, y, z + 1),
            (x, y, z - 1),
        ];

        let empty_neighbors: Vec<_> = neighbors
            .iter()
            .filter(|neighbor| {
                grid_snapshot.get(*neighbor) == Some(&0)
                    && changes.get(*neighbor).is_none()
            })
            .copied()
            .collect();

        if let Some(&chosen) = empty_neighbors.choose(&mut self.rng) {
            if changes.get(&pos).is_none() {
                let (nx, ny, nz) = chosen;

                changes.insert(chosen, state);
                updates.push(Cell {
                    x: nx,
                    y: ny,
                    z: nz,
                    state,
                });

                changes.insert(pos, 0);
                updates.push(Cell {
                    x,
                    y,
                    z,
                    state: 0,
                });
            }
        }
    }

    self.apply_changes(changes);
    self.step += 1;

    updates
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_centered_51_cubed_grid() {
        let engine = SimulationEngine::new();

        assert_eq!(
            engine.grid.len(),
            GRID_SIDE_LENGTH.pow(3)
        );

        assert!(engine.grid.contains_key(&(-25, -25, -25)));
        assert!(engine.grid.contains_key(&(0, 0, 0)));
        assert!(engine.grid.contains_key(&(25, 25, 25)));

        assert!(!engine.grid.contains_key(&(26, 0, 0)));
        assert!(!engine.grid.contains_key(&(-26, 0, 0)));
    }

    #[test]
    fn starts_paused_at_tick_zero() {
        let engine = SimulationEngine::new();

        assert_eq!(engine.step, 0);
        assert!(!engine.running);
    }

    #[test]
    fn injects_a_state_at_an_empty_position() {
        let mut engine = SimulationEngine::new();

        let injected = engine.inject_state(0, 0, 0, 1);

        assert_eq!(
            injected,
            Some(Cell {
                x: 0,
                y: 0,
                z: 0,
                state: 1,
            })
        );

        assert_eq!(engine.grid.get(&(0, 0, 0)), Some(&1));
    }

    #[test]
    fn rejects_an_out_of_bounds_injection() {
        let mut engine = SimulationEngine::new();

        let injected = engine.inject_state(26, 0, 0, 1);

        assert_eq!(injected, None);
    }

    #[test]
    fn reset_clears_all_active_cells_and_resets_tick() {
        let mut engine = SimulationEngine::new();

        engine.inject_state(0, 0, 0, 1);
        engine.inject_state(1, 0, 0, -1);
        engine.step = 42;

        let updates = engine.reset();

        assert_eq!(updates.len(), 2);
        assert_eq!(engine.step, 0);
        assert_eq!(engine.count_active_cells(), 0);
    }

    #[test]
    fn empty_simulation_advances_without_generating_updates() {
        let mut engine = SimulationEngine::new();

        let updates = engine.tick();

        assert!(updates.is_empty());
        assert_eq!(engine.step, 1);
        assert_eq!(engine.count_active_cells(), 0);
    }
}

#[test]
fn identical_seeds_produce_identical_evolution() {
    let mut first = SimulationEngine::with_seed(137);
    let mut second = SimulationEngine::with_seed(137);

    first.inject_state(0, 0, 0, 1);
    first.inject_state(5, 0, 0, -1);

    second.inject_state(0, 0, 0, 1);
    second.inject_state(5, 0, 0, -1);

    for _ in 0..20 {
        let first_updates = first.tick();
        let second_updates = second.tick();

        assert_eq!(first_updates, second_updates);
        assert_eq!(first.grid, second.grid);
        assert_eq!(first.step, second.step);
    }
}

#[test]
fn different_seeds_can_produce_different_evolution() {
    let mut first = SimulationEngine::with_seed(137);
    let mut second = SimulationEngine::with_seed(42);

    first.inject_state(0, 0, 0, 1);
    second.inject_state(0, 0, 0, 1);

    let mut diverged = false;

    for _ in 0..20 {
        first.tick();
        second.tick();

        if first.grid != second.grid {
            diverged = true;
            break;
        }
    }

    assert!(diverged);
}
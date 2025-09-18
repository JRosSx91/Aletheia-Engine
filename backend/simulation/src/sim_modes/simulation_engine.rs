use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize};

const GRID_SIZE: usize = 50;

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
}

impl SimulationEngine {
    pub fn new() -> Self {
        let mut grid = HashMap::new();
        let half_size = GRID_SIZE as i32 / 2;
        
        for x in -half_size..=half_size {
            for y in -half_size..=half_size {
                for z in -half_size..=half_size {
                    grid.insert((x, y, z), 0);
                }
            }
        }
        
        println!("🏗️  Grid inicializado con {} celdas (rango: -{} a +{})", 
                 grid.len(), half_size, half_size);
        
        Self {
            grid,
            step: 0,
        }
    }

    pub fn inject_state(&mut self, x: i32, y: i32, z: i32, state: i8) {
        if let Some(cell_state) = self.grid.get_mut(&(x, y, z)) {
            *cell_state = state;
            println!("💉 Estado inyectado en ({}, {}, {}) -> {}", x, y, z, state);
        } else {
            println!("❌ Error: Coordenada ({}, {}, {}) fuera de rango", x, y, z);
        }
    }

    pub fn apply_changes(&mut self, changes: HashMap<(i32, i32, i32), i8>) {
        for (pos, state) in changes {
            if let Some(cell_state) = self.grid.get_mut(&pos) {
                *cell_state = state;
            }
        }
    }

    // Función auxiliar para contar celdas activas
    fn count_active_cells(&self) -> usize {
        self.grid.values().filter(|&&state| state != 0).count()
    }

    pub fn tick(&mut self) -> Vec<Cell> {
        let active_cells_before = self.count_active_cells();
        println!("\n🔄 TICK {} iniciado - Celdas activas: {}", self.step, active_cells_before);
        
        if active_cells_before == 0 {
            println!("   ⏸️  No hay celdas activas, saltando tick");
            self.step += 1;
            return Vec::new();
        }

        let mut changes = HashMap::new();
        let mut updates = Vec::new();
        let grid_snapshot = self.grid.clone();
        
        // Debug: Mostrar celdas activas
        let active_positions: Vec<_> = grid_snapshot.iter()
            .filter(|(_, &state)| state != 0)
            .collect();
        
        println!("   🎯 Procesando {} celdas activas:", active_positions.len());
        for ((x, y, z), &state) in &active_positions {
            println!("      - ({}, {}, {}) = {}", x, y, z, state);
        }

        for (&pos, &state) in &grid_snapshot {
            if state != 0 {
                let (x, y, z) = pos;
                
                // Vector de vecinos (6 direcciones principales)
                let neighbors: Vec<(i32, i32, i32)> = vec![
                    (x + 1, y, z), (x - 1, y, z),
                    (x, y + 1, z), (x, y - 1, z),
                    (x, y, z + 1), (x, y, z - 1),
                ];
                
                let mut empty_neighbors = Vec::new();
                
                for neighbor_pos in neighbors {
                    // Verificar que el vecino esté dentro del grid
                    if let Some(&neighbor_state) = grid_snapshot.get(&neighbor_pos) {
                        if neighbor_state == 0 && changes.get(&neighbor_pos).is_none() {
                            empty_neighbors.push(neighbor_pos);
                        }
                    }
                }
                
                println!("      🔍 Celda ({}, {}, {}) con estado {} tiene {} vecinos vacíos", 
                         x, y, z, state, empty_neighbors.len());
                
                if !empty_neighbors.is_empty() {
                    let mut rng = thread_rng();
                    if let Some(&chosen_neighbor) = empty_neighbors.choose(&mut rng) {
                        // Asegurar que la celda de origen no sea modificada por otro cambio en el mismo tick
                        if changes.get(&pos).is_none() {
                            let (nx, ny, nz) = chosen_neighbor;
                            
                            // Propagar el estado a la nueva celda
                            changes.insert(chosen_neighbor, state);
                            updates.push(Cell {
                                x: nx,
                                y: ny,
                                z: nz,
                                state,
                            });
                            
                            // Revertir la celda original a 0
                            changes.insert(pos, 0);
                            updates.push(Cell { x, y, z, state: 0 });
                            
                            println!("         ✅ Propagación: ({}, {}, {}) [{}] -> ({}, {}, {}) [{}]", 
                                     x, y, z, state, nx, ny, nz, state);
                        } else {
                            println!("         ⚠️  Celda ({}, {}, {}) ya modificada en este tick", x, y, z);
                        }
                    }
                } else {
                    println!("         🚫 Celda ({}, {}, {}) sin vecinos válidos", x, y, z);
                }
            }
        }
        
        println!("   📝 Aplicando {} cambios...", changes.len());
        self.apply_changes(changes);
        
        let active_cells_after = self.count_active_cells();
        println!("   ✅ TICK {} completado - Celdas activas: {} -> {} | Updates enviados: {}", 
                 self.step, active_cells_before, active_cells_after, updates.len());
        
        self.step += 1;
        updates
    }

    // Función de debug para imprimir el estado del grid
    pub fn debug_grid_state(&self) {
        let active_cells: Vec<_> = self.grid.iter()
            .filter(|(_, &state)| state != 0)
            .collect();
        
        println!("\n🗺️  ESTADO DEL GRID (Step {}):", self.step);
        if active_cells.is_empty() {
            println!("   Todas las celdas están vacías (estado 0)");
        } else {
            for ((x, y, z), &state) in active_cells {
                println!("   ({}, {}, {}) = {}", x, y, z, state);
            }
        }
        println!();
    }
}
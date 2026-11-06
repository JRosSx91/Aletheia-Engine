# CLI Rust y scripts Python

**Idioma:** español. English: [README.md](../README.md) · Español: [README.es.md](../README.es.md).

Crate: `rust-universe` (`backend/simulation`). El binario se invoca así:

```bash
cd backend/simulation
cargo run --bin rust-universe -- <SUBCOMANDO> [flags]
```

Ningún subcomando de exploración numérica (`map`, `library`, `alpha-fixed`, `geometric`, `validate`, `harmonic`, `evolve`) se ejecutó en ALE-01B. Escribirían CSV en el **directorio de trabajo** del proceso.

## Subcomandos

| Comando | Flags (por defecto) | Salida declarada en código | Estado |
|---|---|---|---|
| `map` | `-u/--universes` 1_000_000 | `landscape_data_enhanced.csv` | Implementado sin verificar |
| `library` | `--min-alpha 1 --max-alpha 1000 --samples-per-book 100` | `cosmic_library.csv` | Implementado sin verificar |
| `alpha-fixed` | `--alpha-denominator 137 --samples 10000` | `alpha_fixed_analysis.csv` | Implementado sin verificar |
| `geometric` | `--samples 5000` | `geometric_analysis.csv` | Parcial (geometría no entra en el fitness) |
| `validate` | (ninguno) | stdout de tests internos | Parcial (varias pruebas tautológicas o empíricas) |
| `harmonic` | `--samples 10000` | solo un `println!` | Pendiente (stub) |
| `evolve` | `--seed <archivo> --generations 500` | solo un `println!` | Pendiente (stub) |
| `stream` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |
| `lattice` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |

Ejemplos (no ejecutados en la auditoría de runtime, salvo stream/lattice):

```bash
cargo run --bin rust-universe -- map --universes 1000
cargo run --bin rust-universe -- library --min-alpha 130 --max-alpha 140 --samples-per-book 10
cargo run --bin rust-universe -- alpha-fixed --alpha-denominator 137 --samples 100
cargo run --bin rust-universe -- geometric --samples 50
cargo run --bin rust-universe -- validate
cargo run --bin rust-universe -- harmonic --samples 10
cargo run --bin rust-universe -- evolve --seed genome.json --generations 10
```

`validate` no debe interpretarse como «calibración completa» (el mensaje de éxito del modo es del propio programa).

Feature Cargo `empirical_validation`: sustituye masas hadrónicas teóricas por valores de laboratorio. **No usarla** si se quiere inspeccionar el modelo teórico.

## Scripts Python (`backend/simulation/scripts/`)

Requieren pandas y, según el archivo, plotly o matplotlib. **No verificados en ALE-01B.** Los nombres de CSV **no coinciden** con lo que escribe el Rust actual:

| Script | Espera | El CLI genera |
|---|---|---|
| `find_adam.py` | `landscape_data.csv` (junto al crate) | `landscape_data_enhanced.csv` |
| `plot_landscape.py` | `landscape_data.csv` | idem |
| `plot_interactive.py` | `landscape_data.csv` | idem |
| `analyze_harmonics.py` | `harmonic_results.csv` | `harmonic` no escribe CSV |
| `plot_evolution.py` | `evolution_data.csv` | `evolve` no escribe CSV |
| `analyze_champion.py` | `champion_genome.json`, `our_universe.json` | no los genera el CLI actual |

Hasta alinear nombres, estos scripts **no forman un pipeline reproducible** con el binario de hoy.

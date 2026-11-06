# CLI Rust y scripts Python

**Idioma:** español. English: [README.md](../README.md) · Español: [README.es.md](../README.es.md).

Crate: `rust-universe` (`backend/simulation`). El binario se invoca así:

```bash
cd backend/simulation
cargo run --bin rust-universe -- <SUBCOMANDO> [flags]
```

Los CSV se escriben en el **directorio de trabajo** del proceso. HAR-02 añade `--rng-seed` opcional a los cuatro modos Monte Carlo; no cambia las ecuaciones de fitness.

## Subcomandos

| Comando | Flags (por defecto) | Salida declarada en código | Estado |
|---|---|---|---|
| `map` | `-u/--universes` 1_000_000, `--rng-seed` opcional | `landscape_data_enhanced.csv` | Implementado; reproducible con `--rng-seed` |
| `library` | `--min-alpha 1 --max-alpha 1000 --samples-per-book 100`, `--rng-seed` opcional | `cosmic_library.csv` | Implementado; reproducible con `--rng-seed` |
| `alpha-fixed` | `--alpha-denominator 137 --samples 10000`, `--rng-seed` opcional | `alpha_fixed_analysis.csv` | Implementado; reproducible con `--rng-seed` |
| `geometric` | `--samples 5000`, `--rng-seed` opcional | `geometric_analysis.csv` | Parcial (geometría no entra en el fitness) |
| `validate` | (ninguno) | stdout de tests internos | Falla el nivel 1 (masa del protón); no es un pass científico |
| `harmonic` | `--samples 10000` | solo un `println!` | Pendiente (stub) |
| `evolve` | `--seed <archivo> --generations 500` | solo un `println!` | Pendiente (stub). `--seed` es la ruta del genoma, **no** el RNG |
| `stream` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |
| `lattice` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |

Ejemplos (no ejecutados en la auditoría de runtime, salvo stream/lattice):

```bash
cargo run --bin rust-universe -- map --universes 1000 --rng-seed 137137
cargo run --bin rust-universe -- library --min-alpha 130 --max-alpha 140 --samples-per-book 10 --rng-seed 137137
cargo run --bin rust-universe -- alpha-fixed --alpha-denominator 137 --samples 100 --rng-seed 137137
cargo run --bin rust-universe -- geometric --samples 50 --rng-seed 137137
cargo run --bin rust-universe -- validate
cargo run --bin rust-universe -- harmonic --samples 10
cargo run --bin rust-universe -- evolve --seed genome.json --generations 10
```

`--rng-seed <u64>` usa `rand::rngs::StdRng` (ChaCha12 en `rand` 0.8), un generador por invocación. Sin la flag, el CLI sigue usando `thread_rng()` (no reproducible). No confundir con `evolve --seed`.

Semilla documentada HAR-02: `137137`.

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

# CLI Rust y scripts Python

**Idioma:** español. English: [README.md](../README.md) · Español: [README.es.md](../README.es.md).

Crate: `rust-universe` (`backend/simulation`). El binario se invoca así:

```bash
cd backend/simulation
cargo run --bin rust-universe -- <SUBCOMANDO> [flags]
```

Los CSV de `map` / `library` / `alpha-fixed` / `geometric` se escriben en el **directorio de trabajo** del proceso. `evolve` **exige** `--output-dir` y no usa el CWD por defecto. HAR-02 añade `--rng-seed` opcional a los cuatro modos Monte Carlo; no cambia las ecuaciones de fitness.

## Subcomandos

| Comando | Flags (por defecto) | Salida declarada en código | Estado |
|---|---|---|---|
| `map` | `-u/--universes` 1_000_000, `--rng-seed` opcional, `--observables-csv` opcional | `landscape_data_enhanced.csv`; CSV experimental solo con el flag | Implementado; reproducible con `--rng-seed` |
| `library` | `--min-alpha 1 --max-alpha 1000 --samples-per-book 100`, `--rng-seed` opcional | `cosmic_library.csv` | Implementado; reproducible con `--rng-seed` |
| `alpha-fixed` | `--alpha-denominator 137 --samples 10000`, `--rng-seed` opcional | `alpha_fixed_analysis.csv` | Implementado; reproducible con `--rng-seed` |
| `geometric` | `--samples 5000`, `--rng-seed` opcional | `geometric_analysis.csv` | Parcial (geometría no entra en el fitness) |
| `validate` | (ninguno) | stdout de tests internos | Falla el nivel 1 (masa del protón); no es un pass científico |
| `harmonic` | `--samples 10000` | solo un `println!` | Pendiente (stub) |
| `evolve` | `--population` 100; `--generations` **obligatorio**; `--mutation-sigma` 0.05; `--rng-seed` **obligatorio**; `--output-dir` **obligatorio**; `--force` false | `evo01_individuals.csv`, `evo01_metrics.csv`, `evo01_summary.json` en `--output-dir` | HAR-EVO-02 **cerrado**. Wrapper EVO-01, motor A, H-03 (no Smolin). Plotly: `plot_evo01.py`. Sin `--seed` de genoma |
| `stream` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |
| `lattice` | — | WebSocket `127.0.0.1:9001` | Verificado (ALE-01B / ALE-02A) |

Ejemplos (no ejecutados en la auditoría de runtime, salvo stream/lattice):

```bash
cargo run --bin rust-universe -- map --universes 1000 --rng-seed 137137
cargo run --bin rust-universe -- map --universes 400 --rng-seed 137137 --observables-csv engine_evaluations.csv
cargo run --bin rust-universe -- library --min-alpha 130 --max-alpha 140 --samples-per-book 10 --rng-seed 137137
cargo run --bin rust-universe -- alpha-fixed --alpha-denominator 137 --samples 100 --rng-seed 137137
cargo run --bin rust-universe -- geometric --samples 50 --rng-seed 137137
cargo run --bin rust-universe -- validate
cargo run --bin rust-universe -- harmonic --samples 10
cargo run --release --bin rust-universe -- evolve --population 100 --generations 30 --mutation-sigma 0.05 --rng-seed 137137 --output-dir <dir>
```

`--generations 30` es T=30 transiciones: se escriben las generaciones **0..=30**. Límites: N ∈ [2, 200], T ∈ [0, 100], σ ∈ (0, 0.2], 2×N×(T+1) ≤ 50 000. Sin `--force`, se rechaza si existe **cualquiera** de los tres nombres previstos (el resto no se crea). `--force` publica tras un run completo en staging; **no es atómico** para el trío (una interrupción puede dejar salida mixta). Clap rechaza `--seed`. Debug y `--release` pueden diferir en ULP de `alpha`/`e_si` del CSV de individuos; métricas y JSON coinciden. Hashes canónicos: perfil debug.

`--rng-seed <u64>` en `map` / `library` / `alpha-fixed` / `geometric` usa `rand::rngs::StdRng` (ChaCha12 en `rand` 0.8), un generador por invocación. Sin la flag, esos modos siguen usando `thread_rng()` (no reproducible). En `evolve`, `--rng-seed` es **obligatorio** y alimenta `EvoConfig.base_seed` (flujos SplitMix64 del runner), no un `StdRng` de muestreo `map`.

`--observables-csv <ruta>` **solo existe en `map`**. Escribe un segundo CSV (`EngineEvaluation` + `g_si`/`e_si` del `CosmicLaw` muestreado). No altera columnas ni el filtro de `landscape_data_enhanced.csv`. El bucle histórico es inclusivo: `--universes N` sortea **N+1** muestras (`0..=N`). El CSV experimental tiene una fila por sorteo; el histórico solo las que pasan `fitness > 0.01`.

Semilla documentada HAR-02 / HAR-VIS: `137137`.

`validate` no debe interpretarse como «calibración completa» (el mensaje de éxito del modo es del propio programa).

Feature Cargo `empirical_validation`: sustituye masas hadrónicas teóricas por valores de laboratorio. **No usarla** si se quiere inspeccionar el modelo teórico.

## Scripts Python (`backend/simulation/scripts/`)

Guía: [analysis.md](analysis.md). Entorno aislado + `requirements-analysis.txt`. HAR-03 alinea los lectores al esquema **actual** del CLI.

| Script | CSV | Estado |
|---|---|---|
| `plot_map.py` (alias: `plot_landscape.py`, `plot_interactive.py`) | `landscape_data_enhanced.csv` | Restaurado. La columna `alpha` es 1/α |
| `plot_observables.py` | `engine_evaluations.csv` (optativo) | Exploratorio HAR-VIS. No es validación científica |
| `plot_library.py` | `cosmic_library.csv` | Restaurado. Marca N=137 sin privilegio físico |
| `plot_alpha_fixed.py` | `alpha_fixed_analysis.csv` | Restaurado. Señaliza variantes ignoradas por el motor |
| `plot_geometric.py` | `geometric_analysis.csv` | Restaurado. Advierte que la geometría no puntúa |
| `find_adam.py` | `landscape_data_enhanced.csv` | Restaurado. No rellena constantes ausentes |
| `analyze_harmonics.py` | — | Stub `harmonic`: error explícito |
| `plot_evolution.py` | — | Stub histórico: espera `evolution_data.csv` (no lo genera `evolve`) |
| `plot_evo01.py` | `evo01_*.csv` de `--output-dir` | Figuras HAR-EVO-01; no las escribe el binario |
| `analyze_champion.py` | JSON del usuario | No inventa genomas |

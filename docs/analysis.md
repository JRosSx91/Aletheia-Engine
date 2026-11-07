# Análisis Python de Harmonic-137

Guía breve HAR-03. El CLI Rust: [cli.md](cli.md). Límites del modelo: [models.md](models.md).

Este pipeline **lee CSV ya generados**. No recalcula el fitness ni valida hipótesis físicas.

## Requisitos

- Python 3.14 (probado) u otro 3.10+ con ruedas de pandas.
- Dependencias: `backend/simulation/requirements-analysis.txt` (pandas 3.1.1, matplotlib 3.11.2, plotly 7.1.0).

No instalar en el Python global.

## Instalación

El entorno virtual de referencia está **fuera del repositorio**:

```powershell
# Carpetas hermanas del clon (fuera de Git). Desde la raíz del repositorio:
py -3.14 -m venv ..\Aletheia-Engine-har03-venv
..\Aletheia-Engine-har03-venv\Scripts\python.exe -m pip install -r backend\simulation\requirements-analysis.txt
```

## CSV esperados

Escritos por `rust-universe` en el directorio de trabajo. HAR-02 (semilla `137137`) se conservó en `..\Aletheia-Engine-har02-runs\` (fuera de git).

| Modo | Archivo | Notas de esquema |
|---|---|---|
| `map` | `landscape_data_enhanced.csv` | Columna `alpha` = **1/α** SI, no α |
| `library` | `cosmic_library.csv` | Una fila por denominador N |
| `alpha-fixed` | `alpha_fixed_analysis.csv` | `c_variant`/`hbar_variant`/`epsilon0_variant` no entran en el motor |
| `geometric` | `geometric_analysis.csv` | Curvatura y ratios 5D no entran en el fitness |
| `harmonic` | — | Stub; no hay CSV |
| `evolve` | `evo01_*.csv` / `evo01_summary.json` vía `--output-dir` | HAR-EVO-02; no escribe en el CWD. Plotly: `plot_evo01.py` |
| `map --observables-csv` | p. ej. `engine_evaluations.csv` | **No** sustituye al CSV histórico. Una fila por sorteo (`--universes N` → N+1 filas) |

### CSV experimental (`engine_evaluations.csv`)

Solo se escribe si `map` recibe `--observables-csv`. El flag no existe en `library`, `alpha-fixed` ni `geometric`.

`g_si` y `e_si` se copian del `CosmicLaw` muestreado (m³ kg⁻¹ s⁻² y C). No se reconstruyen a partir de α.

| Grupo | Columnas | Notas |
|---|---|---|
| Trazas | `sample_id`, `rng_seed` | `sample_id` = índice del bucle `0..=N` |
| Entrada | `g_si`, `e_si` | Parámetros muestreados |
| Motor | `alpha_si`, `inverse_alpha_si`, `alpha_s_reference`, `alpha_s_mu_ref_gev`, masas kg, `used_pdg_hadron_fallback` | `alpha_si` es α; `inverse_alpha_si` es 1/α. El CSV map sigue llamando **`alpha` a 1/α** |
| Intermedios opcionales | `hubble_time_s` (s), `jeans_mass_kg`, `coulomb_barrier_j` (J), `chandrasekhar_mass_kg` | Celda vacía = etapa no ejecutada (`None`). Un cero escrito = la etapa corrió y dio 0 (`Some(0.0)`) |
| Heurística | términos, `complexity_level`, `fitness` | `fitness` es un índice interno, no habitabilidad |

`nan` / `inf` en un campo calculado no significan etapa omitida; las etapas omitidas van vacías.

Ejemplo (HAR-VIS, semilla 137137):

```powershell
# cwd = raíz del clon. $labs = directorio padre (carpetas hermanas).
$labs = (Resolve-Path ..).Path
$bin = (Resolve-Path "backend\simulation\target\release\rust-universe.exe").Path
$out = Join-Path $labs "Aletheia-Engine-har-vis01-runs"
New-Item -ItemType Directory -Force -Path $out | Out-Null
Push-Location $out
& $bin map --universes 400 --rng-seed 137137 --observables-csv engine_evaluations.csv
Pop-Location
```

Las figuras Plotly **no validan** el modelo. El HTML usa Plotly desde CDN (`include_plotlyjs="cdn"`); hace falta red la primera vez.

## Comandos

Desde la raíz del clon (`$labs` = directorio padre). Los scripts se invocan por ruta:

```powershell
$labs = (Resolve-Path ..).Path
$py = Join-Path $labs "Aletheia-Engine-har03-venv\Scripts\python.exe"
$in = Join-Path $labs "Aletheia-Engine-har02-runs"
$out = Join-Path $labs "Aletheia-Engine-har03-figures"
$scripts = "backend\simulation\scripts"

& $py "$scripts\plot_map.py" --input "$in\landscape_data_enhanced.csv" --output-dir "$out\map" --rng-seed 137137
& $py "$scripts\plot_library.py" --input "$in\cosmic_library.csv" --output-dir "$out\library" --rng-seed 137137
& $py "$scripts\plot_alpha_fixed.py" --input "$in\alpha_fixed_analysis.csv" --output-dir "$out\alpha_fixed" --rng-seed 137137
& $py "$scripts\plot_geometric.py" --input "$in\geometric_analysis.csv" --output-dir "$out\geometric" --rng-seed 137137
& $py "$scripts\find_adam.py" --input "$in\landscape_data_enhanced.csv" --output-json "$out\adam_summary.json"
$vis = Join-Path $labs "Aletheia-Engine-har-vis01-runs"
& $py "$scripts\plot_observables.py" --input "$vis\engine_evaluations.csv" --output-dir "$vis\figures" --rng-seed 137137
$exp = Join-Path $labs "Aletheia-Engine-har-exp01-runs"
& $py "$scripts\plot_alphas_bifurcation.py" --evaluations "$exp\reproduction\engine_evaluations.csv" --scan "$exp\alpha_s_scan.csv" --output "$exp\exp01_alphas_bifurcation.html"
$exp2 = Join-Path $labs "Aletheia-Engine-har-exp02-runs"
& $py "$scripts\plot_alphas_propagation.py" --scan "$exp2\alpha_s_scan.csv" --output "$exp2\exp02_qcd_propagation.html"
Push-Location $scripts
& $py -m unittest test_harmonic_csv.py
Pop-Location
```

`plot_landscape.py` y `plot_interactive.py` delegan en `plot_map.py`. `analyze_harmonics.py` y `plot_evolution.py` salen con código 2 (stubs).

## Gráficas

HTML (Plotly) y PNG (matplotlib) en el directorio `--output-dir`. Abrir los `.html` en el navegador.

## Limitaciones

- La puntuación es una heurística interna.
- 1/α en `map` no demuestra un valor privilegiado de 137.
- `library` etiqueta el libro 137 solo para localizarlo.
- No hay pipeline para `harmonic` ni `evolve` hasta que dejen de ser stubs.

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
py -3.14 -m venv C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har03-venv
C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har03-venv\Scripts\python.exe -m pip install -r backend\simulation\requirements-analysis.txt
```

## CSV esperados

Escritos por `rust-universe` en el directorio de trabajo. HAR-02 (semilla `137137`) se conservó en `..\Aletheia-Engine-har02-runs\` (fuera de git).

| Modo | Archivo | Notas de esquema |
|---|---|---|
| `map` | `landscape_data_enhanced.csv` | Columna `alpha` = **1/α** SI, no α |
| `library` | `cosmic_library.csv` | Una fila por denominador N |
| `alpha-fixed` | `alpha_fixed_analysis.csv` | `c_variant`/`hbar_variant`/`epsilon0_variant` no entran en el motor |
| `geometric` | `geometric_analysis.csv` | Curvatura y ratios 5D no entran en el fitness |
| `harmonic` / `evolve` | — | Stubs; no hay CSV |

## Comandos

Desde `backend/simulation/scripts`, con el venv activado o invocando su `python.exe`:

```powershell
$py = "C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har03-venv\Scripts\python.exe"
$in = "C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har02-runs"
$out = "C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har03-figures"

& $py plot_map.py --input "$in\landscape_data_enhanced.csv" --output-dir "$out\map" --rng-seed 137137
& $py plot_library.py --input "$in\cosmic_library.csv" --output-dir "$out\library" --rng-seed 137137
& $py plot_alpha_fixed.py --input "$in\alpha_fixed_analysis.csv" --output-dir "$out\alpha_fixed" --rng-seed 137137
& $py plot_geometric.py --input "$in\geometric_analysis.csv" --output-dir "$out\geometric" --rng-seed 137137
& $py find_adam.py --input "$in\landscape_data_enhanced.csv" --output-json "$out\adam_summary.json"
& $py -m unittest test_harmonic_csv.py
```

`plot_landscape.py` y `plot_interactive.py` delegan en `plot_map.py`. `analyze_harmonics.py` y `plot_evolution.py` salen con código 2 (stubs).

## Gráficas

HTML (Plotly) y PNG (matplotlib) en el directorio `--output-dir`. Abrir los `.html` en el navegador.

## Limitaciones

- La puntuación es una heurística interna.
- 1/α en `map` no demuestra un valor privilegiado de 137.
- `library` etiqueta el libro 137 solo para localizarlo.
- No hay pipeline para `harmonic` ni `evolve` hasta que dejen de ser stubs.

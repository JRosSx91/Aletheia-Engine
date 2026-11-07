# HAR-EVO-01 — Primer optimizador experimental de universos (H-03)

**Fase:** HAR-EVO-01A diseño + 01B implementación + 01C QA, **cerrado**.

El **runner experimental** (`sim_modes/evo01.rs`) existe. HAR-EVO-02 cablea el subcomando público **`evolve`** a ese runner. Motor Historical A. Optimizador H-03, no Smolin. Fitness ≠ habitabilidad.

**HEAD de partida:** `547188af211ece68662172a57c30c4500cdf5904` (HAR-EXP-03).

**Clase:** modelo computacional exploratorio. No es selección cosmológica, ni Smolin (H-04), ni habitabilidad.

**Motor:** únicamente `AdvancedPhysicsEngine::new()` (variante **A**, Historical). No se toca `engine.rs`. `map` y `harmonic` no cambian de comportamiento. `evolve` deja de ser stub en 02B.

**Fuentes:** [ROADMAP.md](ROADMAP.md), [HARMONIC_VISION.md](HARMONIC_VISION.md) H-03, [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md), [HAR_EXP_03.md](HAR_EXP_03.md) §9, [DECISIONS.md](DECISIONS.md) D-04, D-10, D-12, D-18.

---

## 0. Qué afirma y qué no

Pregunta de software: *¿un GA mínimo, con el índice interno actual, se mueve de forma distinta a un control sin selección, bajo el mismo presupuesto de evaluaciones y el mismo ruido de mutación indexado?*

No afirma que la naturaleza optimice ese índice. HAR-EXP-03: maximizar el fitness empuja **G hacia el techo del sorteo** y **α_s_ref hacia 0.118** si también baja `e`. Artefacto de umbrales, no evidencia antrópica. `fitness = 1` no es habitabilidad.

La comparación es **reproducible**. No exige que las trayectorias de ambos brazos coincidan: divergen porque la selección elige padres distintos. Lo que debe coincidir es el gen 0 y, a igualdad de `(generación, slot)`, el vector de perturbaciones de mutación.

---

## 1. Genoma (aprobado)

Tres genes. El `CosmicLaw` completo no es el genoma. **Sin cuarta dimensión** (`mass_electron` y `dark_energy_density` fijos).

| Gen | Símbolo | Qué es | Campo activo |
|---|---|---|---|
| Acoplamiento fuerte de referencia | `alpha_s` | α_s(μ_ref = 91.1876 GeV) | `laws.alpha_s` |
| Acoplamiento EM de baja energía | `alpha` | \(\alpha=e^2/(4\pi\varepsilon_0\hbar c)\) SI | `laws.e` vía inversa de `si_alpha` |
| Intensidad gravitacional relativa | `gamma` | \(G/G_N\), \(G_N\) = `G_GRAVITATIONAL` | `laws.g = gamma * G_N` |

`gamma` **no** es el invariante \(\alpha_g=G m_p^2/\hbar c\). Es un cociente de escala respecto a la G de laboratorio del crate. El motor sigue usando G en SI (Jeans, Chandrasekhar). No se le atribuye carácter universal.

### 1.1 Muestreador histórico (`map`) y distribución de α

`sample_mapped_universe` (`map.rs`) sortea **uniformemente en SI**:

| Campo | `gen_range` (semiabierto) |
|---|---|
| `g` | `[6.674e-11, 6.674e-10)` |
| `e` | `[0.5e-19, 2.5e-19)` |
| `alpha_s` | `[0.05, 2.0)` |

α **no** es uniforme: \(\alpha \propto e^2\), de modo que la densidad inducida cumple \(p(\alpha)\,d\alpha \propto d\alpha/\sqrt{\alpha}\) en la imagen del intervalo de `e`. La generación 0 **sortea `e` uniforme** y pone `alpha = si_alpha(e)`. No se sortea α uniforme en \([\alpha_{\min},\alpha_{\max})\).

`g` es uniforme en SI; `gamma = g / G_N` hereda esa uniforme (la caja de γ no es exactamente `[1,10)` porque \(G_N=6.67430\times10^{-11}\) y el `map` usa `6.674e-11`).

### 1.2 Transformaciones (verificables)

\[
e = \sqrt{4\pi\varepsilon_0\hbar c\,\alpha},\qquad
\alpha = \mathrm{si\_alpha}(e),\qquad
g = \gamma\,G_N,\qquad
\gamma = g/G_N.
\]

Caja **equivalente a `map`**, extremos superiores **exclusivos**:

| Gen | Inferior (incluido) | Superior (excluido) | Init |
|---|---|---|---|
| `alpha_s` | 0.05 | 2.0 | uniforme |
| `alpha` | `si_alpha(0.5e-19)` | `si_alpha(2.5e-19)` | vía `e` uniforme |
| `gamma` | `6.674e-11 / G_N` | `6.674e-10 / G_N` | vía `g` uniforme |

Un individuo **nunca** se escribe con un gen `>=` el extremo exclusivo. El clamp de mutación usa `f64::next_down(hi)`, no `hi`. No hay bucle de reparación: una mutación, un clamp o un descarte.

### 1.3 Fondo fijo

`create_reference_universe()` (`validate.rs`). Solo se sobrescriben `alpha_s`, `e`, `g`. Inertes y gates (`mass_electron`, `dark_energy_density`, …) permanecen en valores de referencia. No se llama a `sample_mapped_universe` para hijos.

### 1.4 Espacios lineal vs log

| Operación | Espacio |
|---|---|
| Init (gen 0) | lineal en `e`, `g`, `alpha_s` (como `map`) |
| Mutación | **log-normal** sobre `(alpha, alpha_s, gamma)`: \(\theta'=\theta\exp(\sigma Z)\), \(Z\sim\mathcal{N}(0,1)\), σ=0.05 |
| Diversidad y bordes | cubo [0,1]³ con coordenada **log** \(u=\ln(\theta/\mathrm{lo})/\ln(\mathrm{hi}_\ast/\mathrm{lo})\), `hi_* = next_down(hi)` |

---

## 2. Algoritmo (aprobado)

Sin cruce. Torneo **k=2**. Elitismo **E=0** en la comparación principal (todos los slots mutan). Motor A. Fitness de `evaluate()`.

Configuraciones:

| Nombre | N | T (transiciones) | Evaluaciones/brazo | σ | k | E |
|---|---:|---:|---:|---:|---:|---:|
| Principal | 100 | 30 | 100 × 31 = 3100 | 0.05 | 2 | 0 |
| Ampliada (opcional) | 200 | 100 | 200 × 101 | 0.05 | 2 | 0 |
| Smoke (tests) | 8 | 5 | 8 × 6 | 0.05 | 2 | 0 |

**Generación** = índice de la población viva (0 … T). **Transición** = paso  t → t+1. Hay T transiciones y **T+1** generaciones. Presupuesto: **N × (T+1)** evaluaciones por brazo (E=0: ninguna reutilización).

### 2.1 Semillas (sin hashes de Rust)

Base documental: **`137137`**. Derivación **SplitMix64** (constantes públicas, independientes de `std::hash`):

```text
mix(z):
  z := z xor (z >> 30); z := z * 0xBF58476D1CE4E5B9
  z := z xor (z >> 27); z := z * 0x94D049BB133111EB
  return z xor (z >> 31)

derive(base, stream) = mix(base + stream * 0x9E3779B97F4A7C15)
```

| Stream `s` | Uso | Semilla |
|---|---|---|
| 0 | Init de gen 0 (un solo sorteo, copiado a ambos brazos) | `derive(137137, 0)` |
| 1 | Torneo (solo brazo selección) | `derive(137137, 1)` |
| 2 | Mutación | `derive(137137, 2)` |

Por transiciones, el RNG de mutación del slot `i` en la generación destino `t+1` se siembra con

`derive(derive(137137, 2), (t+1) << 32 | i)`.

Tres normales (α, α_s, γ), mismo vector **Z** en ambos brazos para ese `(t+1, i)`. El control y la selección aplican el mismo ruido a **padres distintos** (salvo gen 0).

El torneo del slot `i` en la transición hacia `t+1` usa

`derive(derive(137137, 1), (t+1) << 32 | i)`

y sortea dos índices `∈ {0,…,N−1}` (con reemplazo). El control **no** consulta este flujo ni el fitness.

No se usa `DefaultHasher` / `siphash` (no estable entre versiones).

### 2.2 Pseudo-código

```text
gen0 ← sample_like_map_active_axes(N, rng_init)   # e, g, α_s uniformes
ambos brazos ← copia profunda de gen0
evaluar cada individuo (universe_id nuevo por brazo; parent_id vacío)

para trans = 1 ..= T:
  para slot = 0 .. N-1:
    Z ← normales(rng_mutación(trans, slot))     # idéntico en ambos brazos
    # selección:
    (a,b) ← torneo_índices(rng_sel(trans, slot))
    padre_s ← ganador(pob_sel[trans-1][a], pob_sel[trans-1][b])
      # no finito no gana; empate → universe_id menor
    hijo_s ← clamp(padre_s ⊙ exp(σ Z))
    # control:
    padre_c ← pob_ctrl[trans-1][slot]           # no mira fitness
    hijo_c ← clamp(padre_c ⊙ exp(σ Z))
  evaluar N hijos por brazo
```

### 2.3 Cero y no finito

| Caso | Fitness usado en torneo | Evalúa motor | `eval_status` / `discard_reason` |
|---|---|---|---|
| `evaluate()` finito, fitness 0 | 0 (puede perder) | sí | `ok` / vacío |
| Genoma no finito tras mutación (antes de clamp útil) | no gana nunca | no | `skipped` / `nonfinite_genome` |
| `evaluate()` NaN/Inf | no gana nunca | sí (resultado descartado para ranking) | `nonfinite_eval` / `nonfinite_eval` |

Un candidato no finito frente a uno finito pierde siempre. Dos no finitos: `universe_id` menor. Sin reintentos.

---

## 3. Control experimental

| Pieza | Selección | Control |
|---|---|---|
| Gen 0 | Idéntico (mismos genes) | Idéntico |
| Padres | torneo k=2 | mismo slot |
| Mutación Z | indexada `(gen, slot)` | **la misma Z** |
| Fitness para elegir padres | sí | **nunca** |
| E | 0 | 0 |
| Evaluaciones | N(T+1) | N(T+1) |
| Motor | A | A |

Si ambos brazos suben igual, deriva o pared. Si solo selección se pega a γ alto, α_s≈0.118 y α baja: artefacto EXP-03.

---

## 4. Genealogía

`run_id` = `evo01-n{N}-t{T}-base{137137}`.

`universe_id` estable: `(arm_code << 56) | (generation << 28) | slot` con `arm_code` 0 = `selection`, 1 = `control`. Único dentro del experimento. No es un hash.

`parent_id` vacío en generación 0. En t≥1 el padre pertenece a t−1 del **mismo** brazo.

Columnas individuales (CSV): `run_id`, `arm`, `generation`, `universe_id`, `parent_id`, `slot`, `alpha`, `alpha_s`, `gamma`, `e_si`, `g_si`, `fitness`, `complexity_level`, `eval_status`, `mutation_effective`, `clamped_genes`, `discard_reason`.

---

## 5. Métricas (por generación y brazo)

| Métrica | Cálculo |
|---|---|
| Mejor fitness | max de fitness con `eval_status=ok` |
| Mediana, Q1, Q3, IQR | de esos fitness (N puntos; no finitos excluidos del ranking, no de la cuenta de filas) |
| Niveles | fracciones 0–5 |
| Diversidad | media de distancias euclídeas en el cubo log [0,1]³ (pares i<j) |
| Distancia a límites | media de \(\min_g \min(u_g,1-u_g)\) |
| Fracción cerca de bordes | individuos con algún \(u_g<0.02\) o \(>0.98\) |
| Fracción mutaciones limitadas | genes cuya preimagen salió de `[lo,hi)` y se clampó / N×3 (gen 0 = 0) |
| Tasa efectiva de mutación | genes con \(\lvert\ln(\theta'/\theta_{\mathrm{padre}})\rvert>10^{-15}\) tras clamp / N×3 |
| Concentración de progenitores | `#(parent_id distintos) / N` (gen 0 = vacío) |

---

## 6. Visualizaciones (01B)

Carpeta hermana `Aletheia-Engine-har-evo01-runs\` (fuera de Git). Cinco HTML Plotly. Hover: `universe_id`, generación, genes, fitness. Sin React.

1. Fitness sel vs ctrl (mejor y mediana).
2. α, α_s, γ (mejor y mediana poblacional).
3. Niveles apilados por generación (ambos brazos).
4. Diversidad y fracción en borde.
5. Genealogía de destacados (p. ej. 12 mayores fitness finales, selección).

---

## 7. Arquitectura 01B (no implementada en el cierre 01A)

Patrón EXP-03: `sim_modes/evo01.rs` + tests + `scripts/plot_evo01.py`. Reusa `CosmicLaw` y `PhysicsEngine::new()`. CLI intacta; el stub `evolve` permanece. Sin framework GA, sin `DimensionlessLaw`.

---

## 8. Criterios de aceptación 01B

1. Determinismo: dos corridas smoke → CSV iguales.
2. Gen 0 idéntico entre brazos (genes bitwise).
3. N filas × (T+1) generaciones × 2 brazos.
4. Padres válidos; gen 0 sin `parent_id`.
5. Genes en caja; ningún valor `>= hi`.
6. `si_alpha(e_si)` coincide con `alpha` (ulps documentados); `g_si = gamma * G_N`.
7. Control: el código de elección de padre no lee fitness.
8. Torneo: no finito no gana; k=2.
9. Mutación log-normal σ=0.05; clamp exclusivo.
10. Diversidad en cubo log.
11. `new()` Historical; hashes VIS-01; `map` y stub `evolve` intactos.
12. Cinco HTML; CSV fuera de Git.

---

## 9. Riesgos

Redescubrir anclas del fitness; saturación en 1.0; pegado a `next_down(hi)` de γ; leer α como invariante más profundo que el cociente de escala γ; cablear CLI y mezclar `--seed` archivo con RNG.

---

## 10. Decisiones (01A)

Aprobadas para este experimento:

1. Genoma `alpha_s`, `alpha`, `gamma=G/G_N` (G1).
2. Motor A.
3. Sin cruces; torneo k=2; E=0 en la comparación principal.
4. σ=0.05 log-normal.
5. Principal N=100, T=30; ampliada opcional N=200, T=100.
6. Caja y init equivalentes a `map` (e y g uniformes).
7. Gen 0 compartido; Z de mutación indexada; flujos init / selección / mutación separados; semillas SplitMix64 desde 137137.
8. CLI pública sin cambios; sin cuarta dimensión genética.

Pendiente explícito: ejecutar o no la corrida ampliada; no bloquea 01B.

---

## 11. Qué no hace 01A

No implementa `evolve`. No cambia ecuaciones, fitness, gates ni muestreo `map`. No PHYS-01, PRIME-01, H-04, `DimensionlessLaw`, React ni bases de datos.

---

## 12. Implementación HAR-EVO-01B (en revisión, sin commit)

Código: `sim_modes/evo01.rs`, `evo01_regression.rs`, `scripts/plot_evo01.py`, loaders en `harmonic_csv.py`. CLI `evolve` **sigue stub**. Motor `new()` Historical. `map` no cambia.

Fondo: `create_reference_universe()`. Init: `e`, `g`, `alpha_s` uniformes como `map`; `alpha=si_alpha(e)`; `gamma=g/G_N`. Mutación log-normal σ=0.05 sobre los tres genes. Clamp `next_down(hi)`. Semillas SplitMix64 desde 137137, flujos 0/1/2. Z de mutación indexada `(generación destino, slot)` idéntica en ambos brazos.

Corrida principal: N=100, T=30, k=2, E=0. Salida fuera de Git: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-evo01-runs\`.

### 12.1 Resultados (índice interno)

Gen 0 idéntico: mejor 0.6435, mediana 0.3094; niveles 0.24 / 0.73 / 0.03 / 0 / 0 / 0.

| Generación | Brazo | Mejor | Mediana | Diversidad | Borde | Niveles 0/1/2/3/4/5 |
|---|---|---:|---:|---:|---:|---|
| 10 | selección | 1.000 | 0.648 | 0.255 | 0.66 | 0 / 0.18 / 0.79 / 0 / 0 / 0.03 |
| 10 | control | 0.639 | 0.323 | 0.513 | 0.14 | 0.20 / 0.79 / 0.01 / 0 / 0 / 0 |
| 30 | selección | **1.000** | 0.661 | 0.117 | 0.63 | 0 / 0.15 / 0.59 / 0 / 0.03 / **0.23** |
| 30 | control | 0.649 | 0.326 | 0.529 | 0.21 | 0.24 / 0.73 / 0.03 / 0 / 0 / 0 |

Al final, selección: 91 % con γ>9; 87 % con |α_s−0.118|<0.02. El mejor (nivel 5, fitness 1) tiene α≈0.00838, α_s≈0.121, γ≈9.90. Control: concentración de padres = 1 (un progenitor por slot); diversidad no colapsa.

Esto **confirma el artefacto EXP-03**: el torneo empuja al techo de G y a la gaussiana de α_s. No es habitabilidad. El control no muestra esa convergencia.

Cero evaluaciones `skipped` / no finitas en esta corrida. Genealogía: gen 0 sin `parent_id`; padres del control = mismo slot de t−1.

Figuras: `evo01_fitness_sel_vs_ctrl.html`, `evo01_genes.html`, `evo01_levels.html`, `evo01_diversity_bounds.html`, `evo01_genealogy.html`.

### 12.2 Tests

96 tests Rust activos (el test de escritura ignorado se sustituyó por directorios temporales con `Drop` y replay bitwise); 22 Python. 16 warnings históricos. Hashes VIS-01 intactos. `harmonic_and_evolve_remain_stubs` verde.

**Reproducibilidad 01C (dos corridas independientes, semilla 137137, N=100, T=30):** SHA-256 idénticos.

| Archivo | SHA-256 |
|---|---|
| `evo01_individuals.csv` | `BE5D4841…F226D748` |
| `evo01_metrics.csv` | `079A3AE9…C5BA10BA` |
| `evo01_summary.json` | `2D0AFC55…48DC77F7` |

El JSON no lleva rutas ni marcas de tiempo. `fitness=1` es saturación de `min(1)` en la heurística, no habitabilidad. Una sola semilla **no** es evidencia estadística general.

Corrida N=200, T=100 no ejecutada (opcional). HAR-EVO-01 **cerrado** como optimizador experimental. CLI `evolve` **cerrada** en HAR-EVO-02C.

---

## 13. Interfaz pública (HAR-EVO-02, cerrado)

**HAR-EVO-02A:** diseño. **02B:** implementación. **02C:** QA. Reutiliza `evo01.rs`; no reimplementa el GA. Motor **A**. H-03, no Smolin. Plotly: `scripts/plot_evo01.py` aparte.

### 13.1 Auditoría

Antes de 02B: `--seed` (archivo de genoma) obligatorio; `--generations` default 500; el stub ignoraba ambos. Esa CLI **nunca funcionó**; no se fingió compatibilidad. `map` sigue escribiendo en el CWD.

Ruta tomada: `pub mod evo01` (helpers de test siguen `cfg(test)`); `evolve.rs` = validar + `write_evo01_run` vía staging; clap **sin** `--universes` y **sin** `--seed`.

### 13.2 Contrato aprobado

```text
rust-universe evolve --population 100 --generations 30 --mutation-sigma 0.05 --rng-seed 137137 --output-dir <dir>
```

`--generations 30` = T=30 **transiciones** ⇒ generaciones **0..=30** (31 poblaciones). Evaluaciones/brazo = N×31.

| Flag | Default | Semántica |
|---|---|---|
| `--population` / `-p` | 100 | N por brazo. No es `--universes` |
| `--generations` / `-g` | **obligatorio** (sin default) | T transiciones, 0..=100. El 500 del stub se retira |
| `--mutation-sigma` | 0.05 | σ log-normal |
| `--rng-seed` | **obligatorio** | u64, `EvoConfig.base_seed`. No hay `thread_rng` |
| `--output-dir` | **obligatorio** | directorio de los tres archivos; no se asume CWD ni una ruta de lab |
| `--force` | false | única vía para reemplazar los tres nombres previstos |

**`--seed` se retira.** La carga de un genoma JSON **no existe**. Clap debe rechazar `--seed` en el parseo (argumento desconocido). Texto de ayuda (`after_help`): usar `--rng-seed`; no hay archivo de genoma. No se acepta `--seed` para fallar después en runtime.

Fijos: motor A; genes α, α_s, γ; k=2; E=0; dos brazos; nombres `evo01_individuals.csv`, `evo01_metrics.csv`, `evo01_summary.json`. Plotly: `plot_evo01.py`, no el binario.

Límites (antes del GA): N ∈ [2, 200]; T ∈ [0, 100]; σ ∈ (0, 0.2]; **2×N×(T+1) ≤ 50 000**. `--output-dir` no es un archivo. Si alguno de los tres CSV/JSON existe y no hay `--force`, abortar **sin** ejecutar. Escritura: staging en un subdirectorio temporal del destino, publicar solo si el runner termina; no borrar archivos ajenos. Si la publicación de tres `rename` no es atómica en Windows, el staging evita truncar el resultado previo antes de un run válido.

### 13.3 Tests (02B)

Parseo del ejemplo; rechazo de `evolve` sin `--generations` / `--rng-seed` / `--output-dir`; rechazo de `--seed` y de `--universes`; límites N/T/σ (T=0 permitido) y fórmula de presupuesto; ruta archivo vs directorio; overwrite sin/con `--force`; CLI vs runner (bytes); `map` intacto; `harmonic` sigue stub; `evolve` ya no es stub; `new()` Historical.

### 13.4 Riesgos

Mezclar `--seed` histórico con RNG; `--output-dir .` dentro del repo; publicación de tres archivos no atómica en Windows; leer la CLI como selección cósmica.

### 13.5 Notas de implementación y QA (02C)

Validación **antes** del GA. Escritura: subdirectorio `.evo01-staging-<pid>` dentro de `--output-dir`; si el runner falla, se borra el staging y **no** se tocan los tres nombres previstos. El mensaje de éxito (`wrote …`) solo se imprime **después** de `publish_staged`. Con `--force`, tras un run válido se sustituye archivo a archivo (`rename`, o `copy`+`remove` si el `rename` cruza volúmenes). Eso **no** es atómico para el trío: una interrupción a mitad de publicación puede dejar un CSV nuevo y otro antiguo (salida parcial o mixta). El staging evita truncar resultados previos **antes** de tener una corrida completa. No se borran archivos ajenos. Un fallo a mitad de publicación puede dejar el staging; no hay transacción de tres archivos.

El binario escribe CSV/JSON. Las figuras Plotly requieren `scripts/plot_evo01.py` (no `plot_evolution.py`).

**Reproducibilidad por perfil** (N=100, T=30, σ=0.05, semilla 137137, Windows):

- Dos debug independientes: mismos tres SHA-256 que HAR-EVO-01C.
- Dos release independientes: mismos entre sí. Métricas y JSON = 01C. Individuos distintos del debug.
- 9/6200 filas difieren solo en `alpha` (9) y `e_si` (4). Idénticos: `alpha_s`, `gamma`, `g_si`, fitness, nivel, padres, IDs, orden, genealogía.
- |Δ|_máx = 5.20×10⁻¹⁸ en α (relativo 3.74×10⁻¹⁶), ~1 ULP; a veces 1–2 dígitos finales de `{:.16e}`, no siempre un único carácter ASCII.
- No cambia decisiones evolutivas. No se altera el formato histórico para igualar perfiles.

Hashes canónicos **debug**: individuos `BE5D4841…F226D748`, métricas `079A3AE9…C5BA10BA`, resumen `2D0AFC55…48DC77F7`. Release individuos: `81121EF4…6D0F9B5B`.

`evolve` es un **optimizador experimental H-03** sobre el índice interno. No es selección cosmológica ni H-04. Fitness ≠ habitabilidad.

Ejemplo (T=30 ⇒ generaciones 0..=30):

```bash
cd backend/simulation
cargo run --release --bin rust-universe -- evolve --population 100 --generations 30 --mutation-sigma 0.05 --rng-seed 137137 --output-dir <dir>
```

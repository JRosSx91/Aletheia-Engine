# HAR-EXP-02 — Propagación de la discontinuidad de `running_alpha_s`

Informe científico-computacional. HEAD de partida: `3fba1217174cdfd2540607af488f0c50f1a9cafc`.

Las cifras son **salidas del motor**. El 1-loop **no** es una predicción cuantitativa fiable en acoplamiento fuerte, y el corte no es una transición QCD.

## Barrido

Universo de referencia; solo varía `alpha_s`. Malla 0.05–2.0, refinada en 0.12–0.16 y 0.1388–0.1398. **936** puntos. Runner de tests `write_reference_alpha_s_scan` (no CLI). Columnas extra leídas de `evaluate()` existente.

CSV: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp02-runs\alpha_s_scan.csv`

## 1. Tabla junto al umbral

`denom > 0.1` ⇔ α_s ≲ **0.1392266**. PDG = 0. Gate nucleónico = 1. Fitness no saturado (`min(1)` no actúa).

| α_s_ref | rama | denom | α_s(1 GeV) | m_p (kg) | Δm (kg) | nucleosynthesis | stellar | heavy | fitness | nivel |
|---:|---|---:|---:|---:|---:|---:|---:|---|---:|---:|
| 0.139220 | 1-loop | 0.100043 | 1.3916 | 6.952e-28 | 2.538e-30 | 0.3586 | 0 | — | 0.2397 | 1 |
| 0.139223 | 1-loop | 0.100021 | 1.3919 | 6.951e-28 | 2.538e-30 | 0.3584 | 0 | — | 0.2396 | 1 |
| 0.139227 | α_s_ref | 0.100000 | 0.13923 | 9.717e-28 | 2.259e-30 | 0.9615 | 0 | — | 0.3904 | 1 |
| 0.139230 | α_s_ref | 0.099978 | 0.13923 | 9.717e-28 | 2.259e-30 | 0.9615 | 0 | — | 0.3904 | 1 |

Los porcentajes siguientes **no** son derivadas continuas dX/dα_s. Son el cambio relativo entre **dos puntos adyacentes de la malla** a ambos lados del `if` (α_s = 0.139223 con denom>0.1 y α_s = 0.139227 con denom≤0.1):

α_s(1 GeV) **−90 %**; m_p **+40 %**; Δm **−11 %**; `nucleosynthesis_term` **+168 %**; `fitness` **+63 %**.

`nucleosynthesis_term` y `fitness` son **índices heurísticos** (adimensionales 0–1), no observables de laboratorio. m_p, m_n y Δm son **salidas teóricas del modelo** (kg). No se mezclan en un mismo eje de la figura.

## 2. Gráfica

`C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp02-runs\exp02_qcd_propagation.html`

Cuatro paneles Plotly (α_s(1 GeV), m_p, Δm, índices 0–1). Zoom por defecto α_s ∈ [0.10, 0.18]. Figuras HAR-VIS-01 intactas.

## 3. Propagación

| Resultado | Comportamiento en este barrido | Causa en el código |
|---|---|---|
| α_s(1 GeV) | **Discontinuidad** | `if denominator > 0.1` |
| m_p, m_n, m_π teóricas | **Discontinuidad** | constituyentes + binding usan `running_alpha_s(1)` |
| Δm teórico | **Discontinuidad** | d−u quirales; mismo α_s_had |
| nucleosynthesis_term | **Discontinuidad (amplificada)** | gaussiana de Δm vs 2.305e-30 kg |
| fitness | **Discontinuidad (transmitida y amplificada)** | 0.15·cosmo(1) + 0.25·BBN; no hay `min(1)` aquí |
| cosmology_term | **Sin cambio** | no usa α_s ni masas hadrónicas |
| stellar_term | **Evaluado = 0**, continuo en el corte | Jeans/lifetime fuera de ventana; `stellar_score > 0.4` no se cumple |
| heavy_element_term | **No evaluado** | gate estelar |
| black_hole_term | **No evaluado** | no se entra en la etapa |
| PDG / `.max(0)` masas | **No intervienen** | PDG=0; m_p mín. 5.61e-28 kg |
| saturación fitness | **No** | `fitness_was_capped=0` |

El gaussiano `alpha_s_score` de `heavy_element_details` (óptimo 0.118) **no se ejecuta** en este universo de referencia.

## 4. Condiciones adicionales

- **Gate BBN `nucleosynthesis_score > 0.3`:** 24 puntos con α_s ≳ 1.497 tienen nivel 0 y `stellar_term=None`. El fitness **no** salta ahí porque el término estelar ya era 0. Es un umbral sobre una función continua de Δm, distinto del `if` del running.
- No hay un segundo salto comparable al del denom=0.1. Los siguientes cambios de fitness son ~0.004, suaves, en α_s ~ 1.

## 5. Tests

- Rust: **64 passed** (línea base 63 + `alpha_s_cutoff_propagates_to_masses_not_cosmology`). 16 warnings históricos.
- Python: **16 passed**.
- Release OK.
- HAR-EXP-01: sample 230/297 estables; SHA-256 de VIS-01 reproducidos.
- `git diff --check` OK.

## 6. Robustez numérica

El motor **no** es continuo en α_s. El recorte 0.1 convierte una divergencia 1-loop en un salto finito de ~40 % en m_p y ~63 % en fitness en el universo de referencia. IEEE no explica el salto. La heurística **amplifica** el Δm (gaussiana de BBN) y **oculta** etapas posteriores (stellar=0). No se sigue una QCD de laboratorio.

## 7. Siguiente experimento propuesto

**HAR-EXP-03 (sin cambiar ecuaciones):** repetir el barrido con un `CosmicLaw` que sí pase `stellar_score > 0.4`, para ver si el gaussiano de `alpha_s` en la etapa pesada añade otra no linealidad, y si el salto de m_p mueve Jeans/Chandrasekhar lo bastante como para abrir o cerrar gates. Alternativa: no tocar fitness y solo **documentar / acotar** el dominio α_s donde el 1-loop se usa.

Sin `DimensionlessLaw`, `evolve` ni primalidad. No corregir `running_alpha_s` hasta revisión.

# HAR-EXP-01 — Bifurcación α_s / Δm

Informe científico-computacional. HEAD de partida: `e0102e3dfa53f195d0b34b62db01228ad76583a6`.

Las cifras siguientes son **salidas del motor**. No se interpretan aquí como QCD de laboratorio ni como cosmología.

## Observación

En `obs_alphas_vs_npdiff.html` (HAR-VIS-01) hay dos nubes en el plano (α_s de referencia, Δm teórico):

- una rama densa, aproximadamente creciente, para α_s ≳ 0.139;
- una rama más corta, también creciente pero desplazada hacia Δm mayor, para α_s ≲ 0.135.

## 1. Reproducción

Comando original: `map --universes 400 --rng-seed 137137` con `--observables-csv`.

| Comprobación | Resultado |
|---|---|
| Muestras | 401 (`0..=400`) |
| SHA-256 `engine_evaluations.csv` | `2D19E32D…D0C715`, idéntico a HAR-VIS-01 |
| SHA-256 `landscape_data_enhanced.csv` | `FC966CE1…9F9268`, idéntico a HAR-VIS-01 |
| CSV histórico VIS-01 | no sobrescrito |
| Figuras HAR-VIS-01 | no modificadas |

Copias fuera de Git:

- original: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp01-runs\original_vis01_copy\`
- reproducción: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp01-runs\reproduction\`

## 2. Muestras representativas

Clasificación **no** por umbral de fitness. Cada fila se etiqueta con el denominador de `running_alpha_s` a μ=1 GeV, n_f=3, μ_ref=91.1876 GeV:

`denom = 1 + (9 α_s / 2π) ln(1/91.1876)`  
`used_running = denom > 0.1` ⇔ α_s ≲ 0.1392266

Conteos: **22** muestras con `denom > 0.1`; **379** con `denom ≤ 0.1`; **0** con fallback PDG.

| Papel | sample_id | α_s_ref | α_si | m_p teór. (kg) | m_n teór. (kg) | Δm teór. (kg) | PDG | fitness | nivel | g_si | e_si | denom | rama código |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| running, cerca del corte | 230 | 0.135216 | 0.017501 | 7.654e-28 | 7.678e-28 | 2.468e-30 | 0 | 0.302 | 1 | 1.83e-10 | 2.48e-19 | 0.126 | denom>0.1 |
| running | 351 | 0.133919 | 0.001967 | 7.823e-28 | 7.847e-28 | 2.451e-30 | 0 | 0.318 | 1 | 6.02e-10 | 8.32e-20 | 0.134 | denom>0.1 |
| running | 144 | 0.128070 | 0.010651 | 8.382e-28 | 8.406e-28 | 2.394e-30 | 0 | 0.365 | 1 | 1.46e-10 | 1.94e-19 | 0.172 | denom>0.1 |
| running, media | 315 | 0.085457 | 0.007725 | 9.603e-28 | 9.626e-28 | 2.271e-30 | 0 | 0.395 | 1 | 4.92e-10 | 1.65e-19 | 0.448 | denom>0.1 |
| running, media | 189 | 0.097890 | 0.017518 | 9.436e-28 | 9.459e-28 | 2.288e-30 | 0 | 0.649 | 2 | 6.59e-10 | 2.48e-19 | 0.367 | denom>0.1 |
| cutoff, cerca del corte | 297 | 0.139369 | 0.012317 | 9.717e-28 | 9.740e-28 | 2.259e-30 | 0 | 0.390 | 1 | 5.19e-10 | 2.08e-19 | 0.099 | denom≤0.1 |
| cutoff | 234 | 0.140370 | 0.014109 | 9.715e-28 | 9.737e-28 | 2.260e-30 | 0 | 0.390 | 1 | 2.13e-10 | 2.23e-19 | 0.093 | denom≤0.1 |
| cutoff | 384 | 0.143435 | 0.016763 | 9.708e-28 | 9.731e-28 | 2.260e-30 | 0 | 0.391 | 1 | 3.27e-10 | 2.43e-19 | 0.073 | denom≤0.1 |
| cutoff, α_s alta | 310 | 1.984309 | 0.001136 | 5.643e-28 | 5.670e-28 | 2.670e-30 | 0 | 0.170 | 0 | 2.92e-10 | 6.32e-20 | −11.83 | denom≤0.1 |
| cutoff, α_s alta | 49 | 1.997135 | 0.004603 | 5.615e-28 | 5.641e-28 | 2.673e-30 | 0 | 0.169 | 0 | 5.50e-10 | 1.27e-19 | −11.91 | denom≤0.1 |

La población muestreada **no** llena el intervalo (0.13522, 0.13937). La unión visible en el scatter es un hueco de muestreo más el salto del `if`, no un tercer régimen.

Fitness en 230 (0.30) vs 297 (0.39) y 189 (0.65) muestra que las ramas no coinciden con un corte de puntuación.

## 3. Recorrido causal hasta Δm

Código no modificado. Cadena:

1. `CosmicLaw.alpha_s` → `AdvancedPhysicsEngine.alpha_s_reference`.
2. `constituent_quark_mass` y `realistic_qcd_binding` llaman `running_alpha_s(1.0)`.
3. En `running_alpha_s`: si `μ ≤ 1.5 Λ_QCD` (0.3255 GeV) se devuelve α_s_ref. **No se dispara** a 1 GeV.
4. n_f=3, β_0=9, `denominator = 1 + (β_0 α_s_ref / 2π) ln(μ/μ_ref)`.
5. **Si `denominator > 0.1`:** α_s(1 GeV)=α_s_ref/denom. **Si no:** α_s(1 GeV)=α_s_ref.
6. Masas constituyentes quirales: u=0.310(1+0.1 α_s_had) GeV, d=0.31125(1+0.1 α_s_had) GeV. Las masas desnudas del sorteo **no** entran en u/d.
7. `get_theoretical_hadron_masses`: m_p=2u+d−B, m_n=u+2d−B. `realistic_qcd_binding` ignora el tipo de hadrón, así que B_p=B_n y **Δm = d−u**.
8. Tras `.max(0.0)` (ninguna de las 401 masas teóricas es 0).
9. `get_validated_hadron_masses` / PDG: `hadron_masses_need_pdg` es falso en las 401 filas. El Δm exportado es el teórico.

Fórmula equivalente (observación numérica, residual relativo máximo 1.85×10⁻¹³):

Δm = 0.00125 × (1 + 0.1 α_s_had) × (GEV_TO_JOULE / c²)

con α_s_had = α_s_ref/denom si denom>0.1, si no α_s_ref.

Otras piezas por tramos en el mismo archivo (μ≤1.5Λ, n_f 1.3/4.2, `.max` de masas) **no** explican esta figura a μ=1 GeV.

## 4. Barrido controlado

Runner interno: `write_reference_alpha_s_scan` en tests de `harmonic_regression.rs`. No está en la CLI. Fija `create_reference_universe()` y varía solo `alpha_s` (malla 0.05–0.25, refinada 0.12–0.16, más 0.25–2.0).

CSV: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp01-runs\alpha_s_scan.csv`

| α_s_ref | denom | fórmula 1-loop | α_s(1 GeV) | Δm teór. (kg) | PDG |
|---:|---:|---|---:|---:|---:|
| 0.1392 | 0.10017 | sí | 1.390 | 2.538e-30 | 0 |
| 0.1394 | 0.09888 | no | 0.1394 | 2.259e-30 | 0 |

Salto Δm ≈ 2.8×10⁻³¹ kg (~11 %) entre puntos adyacentes de la malla. Fuera de la zona (α_s ∈ (0.5, 2)): una sola rama continua y creciente, PDG=0.

Las 22+379 muestras del map caen sobre esas dos curvas del barrido, no sobre una tercera.

## 5. Gráfica

`C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp01-runs\exp01_alphas_bifurcation.html`

Script: `backend/simulation/scripts/plot_alphas_bifurcation.py` (no reescribe las siete figuras HAR-VIS-01). Zoom por defecto α_s ∈ [0.10, 0.18]. Etiquetas: ramas del `if`, no nombres físicos.

## 6. Hipótesis A–D

| | Veredicto | Evidencia |
|---|---|---|
| **A.** Cambio de rama en el código hadrónico | **Compatible y suficiente** | El `if denominator > 0.1` reproduce el corte α_s≈0.13923, los conteos 22/379 y el salto del barrido 1-parámetro. |
| **B.** Otro parámetro oculto en el eje x | **Rechazada para Δm** | Residual ≤2×10⁻¹³ frente a α_s_had solo. corr(Δm,g)≈0.074, corr(Δm,e)≈0.010. El barrido con G,e,masas fijas conserva las dos ramas. |
| **C.** Artefacto numérico / aproximación | **El corte es algorítmico, no un error IEEE** | El salto coincide con el umbral 0.1, no con NaN/inf. El 1-loop divergiría al acercarse a denom=0; el código **sustituye** α_s_ref cuando denom≤0.1. Eso es una saturación explícita, no un redondeo accidental. |
| **D.** Fallback PDG | **Rechazada** | `used_pdg_hadron_fallback=0` en 401 muestras y en todo el barrido. |

A y C describen el mismo `if`: A nombra la rama de control; C nombra el efecto (discontinuidad de α_s_had, de ~α_s/0.1 a α_s).

**Implicación física no demostrada:** nada de esto establece un polo de Landau de QCD ni una transición de fase. Es la receta 1-loop más un umbral 0.1 escrito en Rust.

La fórmula a un lazo **no es una predicción cuantitativa fiable en el régimen de acoplamiento fuerte**. Cuando denom se acerca a 0.1, α_s(1 GeV) calculada llega a ~1.4 en el barrido; ese valor es un síntoma de que el 1-loop está fuera de su dominio, no una medida de QCD. El `if denominator > 0.1` recorta esa divergencia de forma ad hoc. Hay que distinguir esa saturación algorítmica de un error de redondeo IEEE y de cualquier transición física no demostrada.

## 7. Limitaciones del modelo

- El running 1-loop del motor no es una predicción cuantitativa fiable en acoplamiento fuerte (α_s_had ≳ O(1), denom ≲ 0.1). El umbral 0.1 es un recorte de implementación, no un resultado de QCD.
- α_s_had no se exporta en el CSV de 30 columnas; hay que recomputar denom.
- Binding QCD realista no distingue protón/neutrón; Δm no ve esa receta.
- Masas u/d desnudas del muestreador no entran en las masas constituyentes quirales.
- Umbral 0.1 y Λ=0.217 GeV son constantes de implementación.
- El map no muestrea el intervalo justo bajo el corte; el hueco visual mezcla muestreo y `if`.

## 8. Tests y reproducibilidad

- Suite Rust: 63 passed. Siguen las 16 advertencias previas; no hay nuevas.
- Suite Python: `test_harmonic_csv.py` (incluye la figura EXP-01).
- `cargo build --release` (16 advertencias históricas).
- `git diff --check` sin errores de espacios.
- Tests añadidos: corte `denom>0.1` vs motor; barrido determinista = `evaluate`; sample 230/297 estables; Δm exportado = `evaluate`; fitness de referencia invariante.

Alcance del runner de barrido: solo tests; variable opcional `HAR_EXP01_SCAN` para volcar el CSV. No hay flag CLI nuevo.

## 9. Siguiente experimento propuesto

**HAR-EXP-02 (diagnóstico, sin cambiar ecuaciones):** exportar en un CSV de diagnóstico (no el histórico) `running_denominator`, `used_running_formula` y `running_alpha_s_1gev` por muestra, y repetir el scatter m_p vs α_s. Predicción falsable: el mismo corte α_s≈0.13923 debe aparecer en m_p y m_n, y no debe aparecer un segundo kink por `.max(0)` ni por `μ≤1.5Λ` en esta población.

No implementar `DimensionlessLaw`, `evolve`, ni primalidad en ese paso.

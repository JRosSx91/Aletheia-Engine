# Registro de investigación

Resúmenes. Los números completos están en los informes citados. Semilla documental: `137137`.

Estados de revisión: **cerrado** (commit local), **histórico** (pre-Harmonic o auditoría), **propuesto**.

---

## HAR-01 — Límites del motor original

- **Pregunta.** Qué calcula realmente el motor frente a lo que narran `validate` y los comentarios.
- **Método.** Lectura de `engine.rs` / `validate.rs`; snapshot de masas teóricas.
- **Datos.** Universo de referencia (`alpha_s = 0.1181`, α SI CODATA).
- **Resultado.** Masas u/d constituyentes quirales fijas (~310 MeV); binding QCD simplificado; `validate` falla el nivel atómico; fallback PDG y feature `empirical_validation` no son predicción independiente. Funciones MIT bag no se llaman.
- **Limitaciones.** No hubo barrido sistemático de α_s.
- **Informe.** [models.md](models.md); tests `theoretical_hadron_masses_match_har01_snapshot_not_pdg`.
- **Revisión.** Cerrado en la línea HAR-02/04 (software).

---

## HAR-02 — Reproducibilidad

- **Pregunta.** ¿El muestreo y los CSV son deterministas?
- **Método.** `--rng-seed` (`StdRng`); tests de replay; distinción `evolve --seed` (archivo) vs RNG.
- **Datos.** Semilla `137137`.
- **Resultado.** Misma semilla → mismos universos y scores. Sin flag, `thread_rng()`. Columna map `alpha` = **1/α**, no α.
- **Limitaciones.** No valida física; solo software.
- **Informe.** [cli.md](cli.md). Commit `b718e80`.
- **Revisión.** Cerrado.

---

## HAR-03 — Python / visualización restaurada

- **Pregunta.** ¿Se pueden leer y graficar los CSV actuales?
- **Método.** `harmonic_csv.py` + Plotly/matplotlib; venv `Aletheia-Engine-har03-venv` fuera de Git.
- **Resultado.** Loaders por modo; stubs `harmonic`/`evolve` no inventan archivos.
- **Limitaciones.** Las figuras no son evidencia de 137 privilegiado.
- **Informe.** [analysis.md](analysis.md). Commit `e30b86d`.
- **Revisión.** Cerrado.

---

## HAR-04A / 04B — Visión y dimensiones

- **Pregunta.** Qué se quería preguntar (H-01…H-07) y qué parámetros son dimensionales vs adimensionales.
- **Resultado.** Visión clasificada (establecida / especulativa / exploratoria / analogía). Inventario: varios campos de `CosmicLaw` no entran al fitness.
- **Informes.** [HARMONIC_VISION.md](HARMONIC_VISION.md), [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md). Commits `a1473ab`, `ba124aa`.
- **Revisión.** Cerrado como documentación. VISION §4 **no** actualiza HAR-04C ni EXP-01/02.

---

## HAR-04C — Iso-Alpha A/B/C y observables

- **Pregunta.** ¿Qué es invariante al reescalar MKSA y qué mueve α_s?
- **Método.** Motores A/B (λ=4 en e, ε₀, ħ, c) y C (α_s ×1.05). Separar `ModelObservables` y `HeuristicBreakdown`.
- **Resultado.** A/B: misma α, Coulomb comparable fuera de gates. C: misma α, masas hadrónicas y fitness pueden moverse. `evaluate()` bitwise-idéntico al fitness histórico. Predicado PDG unificado.
- **Limitaciones.** Iso-B no es un universo físicamente distinto (unidades). C no es QCD NNLO.
- **Informe.** Código `harmonic_regression.rs`; [models.md](models.md). Commits `bd16cf0` … `7e747a1`.
- **Revisión.** Cerrado (software).

---

## HAR-VIS-01 — Población de 401 universos

- **Pregunta.** ¿Qué aspecto tiene un map reproducible con observables exportados?
- **Método.** `map --universes 400 --rng-seed 137137 --observables-csv` (bucle `0..=N` → 401 filas). Plotly en `plot_observables.py`.
- **Datos.** `Aletheia-Engine-har-vis01-runs\` (fuera de Git). SHA-256 `engine_evaluations.csv` `2D19E32D…D0C715`; histórico `FC966CE1…9F9268`.
- **Resultado.** 30 columnas; PDG = 0 en esa población; figura `obs_alphas_vs_npdiff.html` con dos nubes.
- **Limitaciones.** Exploratorio; no es un landscape físico.
- **Informe.** [analysis.md](analysis.md). Commit `e0102e3`.
- **Revisión.** Cerrado.

---

## HAR-EXP-01 — Bifurcación α_s / Δm

- **Pregunta.** Qué parte del motor genera las dos ramas en α_s vs Δm.
- **Método.** Clasificar por `denom` de `running_alpha_s(1 GeV)`; barrido 1-parámetro; overlay Plotly.
- **Datos.** VIS-01 + `Aletheia-Engine-har-exp01-runs\`.
- **Resultado.** 22 muestras `denom>0.1` (α_s ≲ 0.1392266), 379 en el recorte (devuelve α_s_ref). PDG 0. Hipótesis D (PDG) y B (eje oculto) rechazadas. A/C: el `if denominator > 0.1`.
- **Limitaciones.** 1-loop no es cuantitativo en acoplamiento fuerte; hueco de muestreo (0.135, 0.139).
- **Informe.** [HAR_EXP_01.md](HAR_EXP_01.md). Commit `3fba121`.
- **Revisión.** Cerrado.

---

## HAR-EXP-02 — Propagación del salto

- **Pregunta.** Cómo llega el corte a masas, heurística y fitness.
- **Método.** Mismo runner; 936 puntos; `evaluate()`; cuatro paneles Plotly.
- **Datos.** `Aletheia-Engine-har-exp02-runs\alpha_s_scan.csv`, `exp02_qcd_propagation.html`.
- **Resultado.** Entre puntos de malla 0.139223 y 0.139227: α_s(1 GeV) −90 %, m_p +40 %, Δm −11 %, BBN +168 %, fitness +63 % (**no** son derivadas continuas). Cosmología plana. Stellar evaluado = 0. Heavy/BH no evaluados. PDG 0. Fitness no saturado. Gate BBN a α_s ≳ 1.497 cambia el nivel, no el fitness.
- **Limitaciones.** Universo de referencia no abre la etapa pesada; porcentajes ligados a esos dos nodos de malla.
- **Informe.** [HAR_EXP_02.md](HAR_EXP_02.md). Commit `e550654`.
- **Revisión.** Cerrado (`e550654`); CSV/HTML fuera del repo.

---

## HAR-VISION-02 — Premisas P1–P4

- **Pregunta.** Recuperar las cuatro premisas fundacionales sin convertirlas en física demostrada.
- **Método.** Documentación en [HARMONIC_VISION.md](HARMONIC_VISION.md) §2A.
- **Resultado.** P1–P4 escritas (pregunta, motivación, propuesta histórica, interpretación, límites, vínculo H-01…H-07, futuros). No hay código.
- **Limitaciones.** No hay bibliografía añadida; no hay código ni experimento numérico.
- **Revisión.** Hito documental HAR-VISION-02B.

---

## HAR-MODEL-01 — Comparación A/B/D del running

- **Pregunta.** Sensibilidad del motor al `if denom > 0.1` frente a un blend C¹ (B) y a no correr α_s (D).
- **Método.** Un sorteo VIS-01 (`137137`, 401 ids); `RunningKind`; CSV por variante; Plotly pareado.
- **Datos.** `Aletheia-Engine-har-model01b-runs\` (fuera de Git).
- **Resultado.** A = histórico (hashes VIS-01). B ε=0.02 cambia **2/401** (234, 297), sin cambio de nivel. D cambia **22/401** (rama running EXP-01); 219 y 351 pasan de nivel 1 a 5 porque `stellar_term` pasa de 0.0 a 1.0 (Jeans), no por un error de CSV. Nivel máximo del motor = 5.
- **Limitaciones.** B no es QCD. D no es un modelo físico mejor. D4–D14 siguen.
- **Decisión.** Conservar A como default ([DECISIONS.md](DECISIONS.md) D-18). Informe [HAR_MODEL_01.md](HAR_MODEL_01.md).
- **Revisión.** Cerrado en HAR-MODEL-01C.

---

## HAR-EXP-03 — Gates estelares

- **Pregunta.** Qué abre `stellar>0.4` y las etapas pesada/BH en el motor A.
- **Método.** Censo VIS-01 401 ids; contraste D en 219/351; barridos 1-parámetro (G SI, e, α_s) sobre las leyes 189 y 219.
- **Datos.** `Aletheia-Engine-har-exp03-runs\` (fuera de Git).
- **Resultado.** A: niveles **94 / 299 / 8**. Ocho passers estelares (G ≳ 6×10⁻¹⁰ SI). Ninguno pasa pesados ni evalúa BH. 219/351: A nivel 1 con `stellar_term=0.0` (τ&lt;1 Myr); D nivel 5. Barrido de 219: α_s_ref ∈ [0.103, 0.115] da nivel 5 **bajo A** (no es un hecho de la población original).
- **Limitaciones.** No habitabilidad. G en SI no es invariante. Fitness favorece techos de muestreo.
- **Informe.** [HAR_EXP_03.md](HAR_EXP_03.md).
- **Revisión.** Cerrado en HAR-EXP-03B.

---

## HAR-EVO-01 — Optimizador experimental (H-03)

- **Pregunta.** ¿Un GA mínimo con el índice interno se aparta de un control sin selección bajo el mismo ruido de mutación?
- **Método.** Genoma `alpha`, `alpha_s`, `gamma=G/G_N`; init como `map` (`e` y `g` uniformes); torneo k=2; E=0; σ=0.05 log-normal; Z indexada `(generación, slot)`; motor A. N=100, T=30, semilla 137137. Control: mismo slot, sin leer fitness.
- **Datos.** `Aletheia-Engine-har-evo01-runs\` (fuera de Git). SHA-256 individuos `BE5D4841…F226D748`.
- **Resultado.** Gen 0 idéntico (mejor 0.6435, mediana 0.3094). Selección gen 30: mejor 1.0 (saturación heurística), mediana 0.661, 23 % nivel 5, 91 % γ>9, 87 % α_s cerca de 0.118, diversidad 0.51→0.12. Control: mejor 0.649, mediana 0.326, padres 1.0/slot. Confirma el artefacto EXP-03, no cosmología.
- **Limitaciones.** Una semilla. `gamma` no es α_g. No es Smolin ni habitabilidad.
- **Informe.** [HAR_EVO_01.md](HAR_EVO_01.md).
- **Revisión.** Cerrado en HAR-EVO-01C.

---

## HAR-EVO-02 — CLI `evolve`

- **Pregunta.** ¿Se puede exponer el runner 01 sin cambiar el GA ni fingir compatibilidad con el stub (`--seed`, 500 generaciones)?
- **Método.** Wrapper: validar límites → staging → `write_evo01_run` → publicar. `--generations` / `--rng-seed` / `--output-dir` obligatorios. `--force` para colisiones. Motor A.
- **Datos.** Debug: SHA 01C. Release: mismos métricas/JSON; individuos `81121EF4…6D0F9B5B` (9 filas, ULP de α/`e_si`).
- **Resultado.** CLI ejecutable. Dos debug y dos release independientes coinciden **dentro del perfil**. Genealogía y fitness idénticos entre perfiles. Plotly sigue en Python.
- **Limitaciones.** `--force` no atómico a tres archivos. Fitness ≠ habitabilidad. No es Smolin.
- **Informe.** [HAR_EVO_01.md](HAR_EVO_01.md) §13, [cli.md](cli.md).
- **Revisión.** Cerrado en HAR-EVO-02C.

---

## Propuestos (no ejecutados)

Orden: [ROADMAP.md](ROADMAP.md) §6.

| Id | Pregunta breve |
|---|---|
| HAR-PHYS-01 | ¿Se acota o cambia `running_alpha_s`? Opcional; no bloquea lo anterior |
| HAR-PRIME-01 | Primos vs compuestos cuando α = 1/N sea coherente |

# Visión fundacional de Harmonic-137

**Idioma:** español.  
**Alcance:** intención histórica y marco metodológico.  
**No sustituye:** [ROADMAP.md](ROADMAP.md) (estado y prioridades), [DECISIONS.md](DECISIONS.md), [RESEARCH_LOG.md](RESEARCH_LOG.md), [models.md](models.md) (ecuaciones del código), [THEORY_SUMMARY.md](THEORY_SUMMARY.md), [cli.md](cli.md), [analysis.md](analysis.md).

Este texto documenta **qué se quería preguntar**. Lo que el software **hace hoy** está en `models.md` y el [ROADMAP.md](ROADMAP.md). Un `fitness` alto, un CSV reproducible o una gráfica no demuestran las hipótesis siguientes.

Clasificación usada en cada ficha:

| Clase | Significado |
|---|---|
| Física establecida | Relación o dato con respaldo independiente (p. ej. definición SI de α). |
| Hipótesis física especulativa | Afirmación sobre la naturaleza que el laboratorio no puede cerrar solo. |
| Modelo computacional exploratorio | Procedimiento numérico para ordenar, muestrear o optimizar. |
| Analogía filosófica | Imagen o pregunta reflexiva; no es un protocolo empírico. |

---

## 1. Laboratorio conceptual

Harmonic-137 nació como laboratorio **conceptual y computacional**: explorar si ciertas regiones del espacio de leyes favorecerían estructuras complejas, y si un proceso evolutivo **hipotético** podría hacer converger poblaciones de universos hacia esas regiones.

No es una teoría publicada ni un ajuste a observaciones. Es un programa de preguntas, con un motor numérico incompleto (`backend/simulation`) y dos autómatas 3D (`stream`, `lattice`) que ilustran otra analogía (H-07).

### 1.1 Metáfora de la biblioteca

La imagen central es una **biblioteca conceptual de cinco dimensiones** cuyos «libros» contienen **historias de cuatro dimensiones** (espacio-tiempo ordinario más el relato de su complejidad).

- Cada libro se identifica, en primera aproximación, por un entero \(N\) y la parametrización experimental \(\alpha = 1/N\).
- El **libro 137** es una etiqueta de trabajo para *acercarse* a nuestro universo. El inverso real de la constante de estructura fina no es 137: \(\alpha^{-1} \approx 137{,}035999\ldots\) (CODATA; el código de referencia usa \(7{,}2973525693\times 10^{-3}\) en `validate.rs`).
- La «quinta dimensión» **indexa libros** (el eje \(N\) o un eje de leyes). **No** es una dimensión física demostrada, ni un resultado de `geometric.rs`.

Módulos actuales de la metáfora:

| Intención | Módulo | Hecho de implementación |
|---|---|---|
| Recorrer el paisaje | `sim_modes/map.rs` | Monte Carlo de `CosmicLaw`; columna CSV `alpha` = **1/α SI**, no \(N\). |
| Estantería por \(N\) | `sim_modes/library.rs` | Un registro por denominador; géneros literarios. El motor **no** retiene α = 1/N (recalcula α con \(c,\hbar,\varepsilon_0\) SI). |
| Iso-α (H-01) | `sim_modes/alpha_fixed.rs` | Deriva \(e\) a partir de variantes de \(c,\hbar,\varepsilon_0\); esas variantes **no** entran en `AdvancedPhysicsEngine`. |
| Geometría extra | `sim_modes/geometric.rs` | Muestrea curvatura y `dimensional_ratios[4]`; el fitness **no** los usa. |
| Puntuación | `physics/engine.rs` → `calculate_enhanced_fitness` | Heurística 0–1. No es probabilidad de habitabilidad. |
| Evolución (H-03, H-04) | `sim_modes/evolve.rs` | Stub. |
| Búsqueda «armónica» | `sim_modes/harmonic.rs` | Stub. |
| Autómatas (H-07) | `simulation_engine.rs`, `lattice_gas.rs`, `stream.rs` | Metáfora de información en rejilla; no son agujeros negros. |

### 1.2 Lo que el laboratorio no afirma

- Que 137 sea un número privilegiado por la naturaleza.
- Que una biblioteca 5D exista como espacio físico.
- Que maximizar `fitness` sea selección natural cósmica.
- Que las capas de complejidad (H-05) formen una cadena causal necesaria.
- Que el universo «quiera» conocerse (H-06).

---

## 2. Hipótesis históricas

### H-01 — Iso-Alpha

**Clase:** mezcla de física establecida (definición de α) y modelo computacional exploratorio. La tesis «α prima sobre las constantes dimensionales» es hipótesis especulativa.

**Pregunta original.** ¿Pueden configuraciones distintas de parámetros producir la **misma** α y, aun así, física distinta?

**Motivación.** α es adimensional. Si dos mundos coinciden en α pero difieren en otras relaciones adimensionales (masas relativas, \(\alpha_s\), \(\alpha_w\), densidades en unidades de Planck, etc.), α no agota la física. Si las diferencias son solo reescalados de constantes **dimensionales** (\(c\), \(\hbar\), \(\varepsilon_0\), \(e\)) que dejan invariantes todos los adimensionales, no hay mundos distintos: hay un cambio de unidades.

**Observables / computables en un modelo.**

- α SI: \(\alpha = e^2 / (4\pi\varepsilon_0\hbar c)\) (`AdvancedPhysicsEngine::new`).
- Otras cantidades adimensionales derivadas del vector `CosmicLaw` (p. ej. \(\alpha_s\), cocientes de masas).
- Puntuación interna `fitness` **solo** como índice de software, no como observable.

**Qué se podría comprobar dentro del modelo.** Si, **fijada α de verdad en el motor** y variando parámetros adimensionales independientes, cambian las salidas del modelo (masas hadrónicas *teóricas*, filtros de BBN, etc.).

**Qué exige evidencia externa.** Que esas salidas correspondan a física real, no a la heurística actual.

**Supuestos.** Que α es una coordenada útil; que el resto del vector es físicamente interpretable.

**Sesgos.** Confundir un rescalado de unidades con un universo nuevo. El código actual deriva \(e\) con \(c,\hbar,\varepsilon_0\) aleatorios y luego **vuelve a calcular α con las constantes SI globales**: no realiza el experimento iso-α. Las gráficas HAR-03 de `c_variant` no demuestran «primacía de alpha».

**Estado en el código.** `alpha-fixed` y `library` siguen expresando la intención en el CLI: derivan \(e\) y el motor **recalcula** α SI. Tests HAR-02 documentan esa inconsistencia. **Estado actual:** HAR-04B inventarió dimensionales; HAR-04C añadió Iso-Alpha A/B/C **en tests** (`new` vs `new_with_em`, variación de α_s). Eso no inyecta α objetivo en el CLI. La cola vigente está en [ROADMAP.md](ROADMAP.md), no en la lista HAR-04A de la §4.

**Experimentos futuros (texto HAR-04A, conservado).** HAR-04B: separar constantes dimensionales de adimensionales. Solo entonces repetir iso-α con α inyectada en el motor. *Hecho parcial:* inventario 04B e Iso-Alpha de test 04C; α objetivo en el CLI sigue pendiente.

---

### H-02 — Primalidad

**Clase:** hipótesis especulativa + modelo exploratorio. 137 primo es un **hecho aritmético**, no un privilegio físico.

**Pregunta original.** ¿Hay diferencia sistemática entre universos etiquetados con \(N\) primo y \(N\) compuesto, bajo la parametrización \(\alpha = 1/N\)?

**Motivación.** Curiosidad numerológica histórica (137) convertida en pregunta de control: si el modelo es ciego a la primalidad, las estadísticas de primos y compuestos no deben diferir más que el ruido y los sesgos de muestreo.

**Observables / computables.** \(N\), primalidad de \(N\), distribuciones de `fitness` / `story_quality` por clase {primo, compuesto}, **después** de un modelo físico coherente.

**Qué se podría comprobar en el modelo.** Tests de homogeneidad (p. ej. que la diferencia de medias primo vs compuesto sea compatible con azar), con tamaños de muestra y semillas fijos (HAR-02).

**Qué exige evidencia externa.** Cualquier afirmación de que los primos importan en las leyes. El laboratorio no puede producirla.

**Supuestos.** Que \(N\) entero es una malla razonable. Es una malla **arbitraria**: α real no es 1/137.

**Sesgos.** Mirar el libro 137 porque ya se conoce el resultado terrestre (sesgo de selección). Confundir \(\alpha^{-1}\approx 137{,}036\) con \(N=137\). HAR-03 marca N=137 en amarillo **solo para localizarlo**; con semilla `137137` no fue el máximo de `story_quality`.

**Estado en el código.** `library.rs` recorre enteros consecutivos. No hay test de primalidad. `harmonic.rs` es stub.

**Experimentos futuros.** HAR-PRIME-01 **solo** cuando exista un modelo que retenga α = 1/N de forma coherente. Incluir controles: permutar etiquetas primo/compuesto; comparar con \(N\) no entero.

---

### H-03 — Evolución y convergencia

**Clase:** modelo computacional exploratorio (algoritmo evolutivo). No es implementación de un proceso físico.

**Pregunta original.** ¿Puede una población de vectores `CosmicLaw` aleatorios, con selección y mutaciones pequeñas, acercarse a configuraciones «parecidas a las nuestras»?

**Motivación.** Intención original de `evolve`: optimizar una puntuación de viabilidad a lo largo de generaciones. El CLI conserva `--seed` (archivo de genoma) y `--generations`.

**Observables / computables.** Trayectoria de la mejor puntuación; distancia (a definir) al universo de referencia `create_reference_universe`; diversidad poblacional; comparación con un control **sin** selección (deriva aleatoria).

**Qué se podría comprobar en el modelo.** Convergencia del **optimizador** hacia el máximo de *esa* función. Eso no prueba que la naturaleza optimice lo mismo.

**Qué exige evidencia externa.** Que la función objetivo represente complejidad real.

**Supuestos.** Representación de «genoma» = `CosmicLaw`; mutación; selección por `fitness`.

**Sesgos.** Circularidad: si el fitness se construye para premiar valores terrestres (Δm nucleón, α_s ≈ 0,118, etc.), la población «descubrirá» la Tierra porque se la escondió en la función. HAR-02/03 no deben leerse como convergencia evolutiva.

**Estado en el código.** `evolve.rs` es un `println!`. `plot_evolution.py` rechaza CSV inexistentes. No hay `evolution_data.csv`.

**Experimentos futuros.** HAR-EVO-01 (diseño de herencia/mutación/selección y de controles). HAR-EVO-02 (convergencia vs azar). Distinguir siempre optimización de `fitness` y cosmología.

---

### H-04 — Selección cosmológica (Smolin, como hipótesis)

**Clase:** hipótesis física especulativa (Smolin: *fecund universes* / selección natural cósmica). El código, si existiera, sería un **modelo exploratorio**, no esa teoría.

**Pregunta original.** ¿Universos que forman más agujeros negros dejan más descendencia, con parámetros heredados y mutados, de modo que las leyes observadas serían típicas de linajes fértiles?

**Motivación.** Narrativa de la landing y del nivel «Reproductivo» / `black_hole_formation_potential` en el fitness (peso 0,15). Metáfora de portales en `library.rs` (`reproducibility` reutiliza el score de agujeros negros).

**Observables / computables.** En un modelo: un proxy de «tasa de agujeros negros» y un grafo de descendencia. Hoy el proxy es Chandrasekhar vs 25 M☉, no un conteo de colapsos.

**Qué se podría comprobar en el modelo.** Si un esquema de herencia acoplado a ese proxy aumenta la abundancia de linajes de alto score **dentro del simulador**.

**Qué exige evidencia externa.** Relación entre parámetros fundamentales y tasa real de agujeros negros; mecanismo de nacimiento de universos. No hay tal evidencia en este repo.

**Supuestos.** Que los agujeros negros «producen» universos; que las constantes se heredan con ruido. Son supuestos de Smolin, no resultados de Harmonic-137.

**Sesgos.** Identificar `fitness` alto con fecundidad cósmica. El término de agujeros negros es una pieza de la heurística, no un operador de reproducción.

**Estado en el código.** Contribución heurística en `engine.rs`; `evolve` stub; sin población ni árbol genealógico.

**Experimentos futuros.** HAR-EVO-01 debe decidir explícitamente: (a) optimizador de puntuación, (b) modelo tipo Smolin con descendencia por colapso. No mezclar los informes.

---

### H-05 — Capas de información

**Clase:** analogía filosófica / esquema descriptivo. Los niveles 0–5 del código son **etiquetas literarias** (`helpers.rs`: Estéril … Reproductivo), no una teoría de emergencias.

**Pregunta original.** ¿Puede organizarse la complejidad en capas —física, química, biología, cognición/cultura— y preguntar bajo qué leyes aparece cada una?

**Las cuatro capas propuestas (intención):**

1. **Física:** leyes, partículas, campos, interacciones.
2. **Química:** nucleosíntesis, elementos, moléculas.
3. **Biología:** autorreplicación, herencia, evolución.
4. **Cognición y cultura:** lenguaje, escritura, conocimiento, tecnología.

**Motivación.** Ordenar preguntas. El fitness apila términos (cosmología → BBN → estrellas → elementos pesados → agujeros negros) de forma **parecida**, no equivalente: no hay química molecular ni biología ni cultura en el motor.

**Observables / computables.** Umbrales internos 0,3 / 0,4 / 0,5 / 0,6 / 0,7 / 0,8 (`models.md`). No hay observables de vida ni de lenguaje.

**Qué se podría comprobar en el modelo.** Si los filtros se disparan en cascada para un `CosmicLaw` dado. Eso es coherencia del *software*.

**Qué exige evidencia externa.** Que cada capa emerja de la anterior en el universo real. El laboratorio no lo demuestra. **La aparición de una capa no implica la siguiente.**

**Supuestos.** Jerarquía útil para narrar. Falso si se toma como ley de emergencias.

**Sesgos.** Leer «Nuclear» o «Reproductivo» como nucleosíntesis o reproducción biológica reales.

**Estado en el código.** `complexity_level` y géneros de `library.rs`. Sin módulos de química, genomas ni cultura.

**Experimentos futuros.** No añadir capas de biología/cognición hasta que la capa 1 (H-01, HAR-04C) sea coherente. Si se modelan, cada salto de capa debe ser un criterio independiente, no un bonus del mismo `fitness`.

---

### H-06 — Información y autocomprensión

**Clase:** analogía filosófica.

**Pregunta original.** ¿Puede la evolución de sistemas que procesan información hacer que *partes* del universo construyan modelos de su propia realidad (ciencia, este laboratorio incluido)?

**Motivación.** Cerrar el arco: si hay leyes que permiten observadores, esos observadores pueden cartografiar las leyes. Es una reflexión sobre el lugar del conocimiento, no un término de `CosmicLaw`.

**Observables / computables.** Ninguno en el motor actual. El grafo NestJS (`backend/knowledge`) es un ejemplo de almacenamiento de conceptos; **no** está acoplado a la simulación.

**Qué se podría comprobar en el modelo.** Como máximo, metáforas (agentes que escriben descripciones). Eso no es autocomprensión cósmica.

**Qué exige evidencia externa.** Ciencias cognitivas, historia de la ciencia. Fuera de alcance.

**Supuestos.** Ninguno físico obligatorio.

**Sesgos.** Atribuir propósito, teleología o conciencia *al universo*. Este documento **no** lo hace. Los observadores, si existen, son subsistemas.

**Estado en el código.** Ausente como hipótesis científica. Textos de UI/landing no cuentan como modelo.

**Experimentos futuros.** Ninguno en HAR-04. Si algún día hay agentes, tratarlos como H-05 capa 4, con criterios explícitos y sin lenguaje de «el cosmos se piensa».

---

### H-07 — Representación mediante autómatas

**Clase:** modelo computacional exploratorio / analogía. No es simulación validada de gravedad, agujeros negros ni partogénesis cósmica.

**Pregunta original.** ¿Puede una rejilla 3D con inyección de bits ilustrar, de forma controlada, la **propagación de información** y, en la metáfora, una «descendencia» local?

**Motivación.** Complementar el escáner de constantes con un mundo discreto observable en el visor (`stream`, `lattice`). Semilla por defecto 137 en `SimulationEngine` es una **etiqueta**, igual que el libro 137.

**Observables / computables.** Conservación de masa/momento en lattice (HPP reducido); reproducibilidad de ticks con `StdRng` (tests en `simulation_engine.rs`). En difusión binaria: paseo aleatorio, sin aniquilación.

**Qué se podría comprobar en el modelo.** Invariantes del autómata; que la UI y el WebSocket respetan el contrato JSON.

**Qué exige evidencia externa.** Cualquier vínculo con relatividad, horizontos o nacimiento de universos.

**Supuestos.** Vecindad de von Neumann o canales ±x,±y,±z; torus en lattice.

**Sesgos.** Interpretar un HEAD-ON que conserva momento como «física de agujeros negros». `models.md` ya clasifica lattice como aproximación educativa.

**Estado en el código.** `stream` / `lattice` verificados a nivel de laboratorio 3D (ALE-01B / ALE-02A). Desconectados del fitness Harmonic-137.

**Experimentos futuros.** No acoplar el grid al `fitness` sin un puente teórico escrito. HAR-04 no incluye ese acoplamiento.

---

## 3. Correspondencia intención ↔ código (resumen)

| Hipótesis | ¿Hay código que *pretenda* abordarla? | ¿La aborda de forma coherente hoy? |
|---|---|---|
| H-01 Iso-Alpha | `alpha-fixed`, `library` | No: α del motor ≠ α objetivo. |
| H-02 Primalidad | Recorrido entero en `library` | No hay análisis primo/compuesto. |
| H-03 Evolución | CLI `evolve` | Stub. |
| H-04 Smolin | Término BH del fitness; narrativa | No hay reproducción ni herencia. |
| H-05 Capas | `complexity_level` | Etiquetas; sin química/biología/cultura. |
| H-06 Autocomprensión | — | Solo analogía. |
| H-07 Autómatas | `stream`, `lattice` | Autómatas; no cosmología de BH. |

Reproducibilidad **del software** (HAR-02: `--rng-seed`; HAR-03: CSV/Python) no es validación de H-01…H-07.

---

## 4. Cola escrita en HAR-04A (histórica)

Esta lista es el plan **en el momento de HAR-04A** (`a1473ab`). No es el backlog vigente.

1. **HAR-04B — Constantes y adimensionales.** *Estado actual: hecho* (`ba124aa`, [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md)).
2. **HAR-04C — Consistencia y rediseño controlado.** *Estado actual: hecho en software* (trazas α, Iso-Alpha A/B/C, `EngineEvaluation`; commits `bd16cf0`–`7e747a1`). No se sustituyeron ecuaciones hadrónicas. α objetivo en el CLI y rediseño de PDG/fitness siguen abiertos como trabajo posterior.
3. **HAR-EVO-01 — Herencia, mutación, selección.** *Pendiente.* El backlog vigente lo plantea como **primer optimizador experimental**, distinto de H-04 (Smolin).
4. **HAR-EVO-02 — Convergencia y azar.** *Pendiente.*
5. **HAR-PRIME-01 — Primalidad.** *Pendiente*; no bloquea toda experimentación (HAR-PHYS-01 no es requisito absoluto).

Cola vigente, P1–P4 y política Git: [ROADMAP.md](ROADMAP.md). Fuera de esa cola: no acoplar autómatas 3D al escáner; no modelar biología ni cognición; no reescribir la landing como si las hipótesis estuvieran demostradas.

---

## 5. Criterios para implementaciones futuras

- Documentar la clase (tabla del encabezado) de cada cambio.
- No presentar heurísticas como leyes.
- No rellenar con PDG lo que se pretende predecir.
- Distinguir \(N\), \(\alpha = 1/N\) de malla, y \(\alpha^{-1}\) CODATA.
- Distinguir biblioteca 5D (metáfora) y dimensiones extra físicas.
- Distinguir optimización numérica y selección cósmica.
- Conservar tests de regresión cuando el comportamiento histórico deba permanecer como línea base.

Fin de HAR-04A. Sin cambios de código ni de modelo.

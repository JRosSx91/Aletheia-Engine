# Registro de decisiones

Las **firmes** rigen el trabajo hasta que el propietario las revoque. Las **propuestas** no autorizan código.

Protocolo de sesión: [ROADMAP.md](ROADMAP.md) §10.

---

## Firmes

| Id | Decisión |
|---|---|
| D-01 | El propietario es arquitecto y revisor. Cursor ejecuta cambios; no redefine la física ni el alcance. |
| D-02 | Ni commit ni push sin autorización explícita del propietario/revisor. Sin force-push a `main` ni rebase destructivo de historial publicado. |
| D-03 | Preservar resultados históricos reproducibles (semilla `137137`, hashes VIS-01, tests HAR-02). Las corridas viven fuera de Git. |
| D-04 | `fitness` es un índice interno 0–1, **no** habitabilidad ni probabilidad antrópica. |
| D-05 | Separar `ModelObservables` (kg, s, α, …) de `HeuristicBreakdown` (términos, nivel, fitness). No mezclar unidades en un eje. |
| D-06 | No presentar hipótesis especulativas, 137, o un salto numérico como física de laboratorio demostrada. |
| D-07 | Hitos pequeños y verificables: tests, hashes, informe, revisión. |
| D-08 | Priorizar volver a **experimentar y visualizar** sobre reescribir ecuaciones. |
| D-09 | Autómatas 3D (`stream`/`lattice`) separados del motor cosmológico hasta que exista un puente teórico escrito. |
| D-10 | `evolve` es **objetivo estratégico** como **optimizador experimental** (H-03), no selección cosmológica. H-04 (Smolin) aparte. *(La cláusula «CLI stub» quedó sustituida por D-22 / HAR-EVO-02.)* |
| D-15 | Tras EVO-02: siguiente opcional PHYS-01, luego PRIME-01; revisable. HAR-PHYS-01 no bloquea experimentar. |
| D-16 | Fuentes: ROADMAP (estado), VISION (preguntas), RESEARCH_LOG (hechos), DECISIONS (acuerdos), informes HAR (detalle). Al cerrar un hito se tocan solo los docs afectados. |
| D-17 | P1–P4 son **preguntas documentadas**, no requisitos de código ni resultados físicos. H-01…H-07 se conservan. |
| D-11 | No implementar `DimensionlessLaw`, primalidad, ni acoplamiento grid–fitness de contrabando. |
| D-12 | No modificar CLI pública, pesos de fitness, RNG ni CSV históricos salvo hito que lo pida. |
| D-13 | Distinguir tests de software y validación científica en cada informe. |
| D-14 | El running 1-loop **no** es predicción cuantitativa en acoplamiento fuerte. El umbral `denom>0.1` es recorte de implementación. |
| D-18 | Tras HAR-MODEL-01: **A (Historical) es el único default** de `new()` y de la CLI. B (smooth ε) y D (α_s_had=α_s_ref) son variantes internas de comparación. B **no** está validada como QCD de laboratorio. |
| D-19 | HAR-EVO-01 (diseño): genoma `alpha_s`, `alpha`, `gamma=G/G_N`; motor A; sin cruce; torneo k=2; E=0; σ=0.05; caja e init como `map`; gen 0 compartido; mutación indexada `(generación, slot)`. No es Smolin. *(«CLI intacta» = alcance de 01A; sustituido por D-22.)* |
| D-20 | HAR-EVO-01C: runner `evo01.rs` con control sin selección. No se afirma selección cosmológica. *(«`evolve` stub» sustituido por D-22.)* |
| D-21 | Ramas: `main` estable y pública; `develop` integración habitual; `feature/har-*` funcionalidad; `experiment/har-*` experimentos. Promoción a `main` deliberada. Sin force-push. |
| D-22 | Tras HAR-EVO-02: el runner está en la CLI; `evolve` es ejecutable (motor A, H-03). `harmonic` sigue stub. |

---

## Propuestas (no aprobadas)

| Id | Propuesta | Estado |
|---|---|---|
| P-01 | HAR-EXP-03 | Hecho (gates estelares; A default) |
| P-02 | HAR-MODEL-01 (límites del 1-loop / comparación running) | Hecho (A default; B/D experimentales) |
| P-03 | HAR-PHYS-01 / parchear `running_alpha_s` | En cola (5); no es puerta de EXP-03 |
| P-04 | HAR-VISION-02 | Documentado (preguntas P1–P4); no implementado en código |
| P-05 | Push a `origin` | **Sustituida:** la prohibición temporal de la recuperación (cuando `origin/main` era solo el lab 3D) cede a **publicación controlada** (autorización expresa, fast-forward, D-02, D-21). HAR-GIT-01B. |
| P-06 | HAR-EVO-01 parametrización | Hecho (D-19, D-20, D-22). CLI `evolve` operativa |

---

## Cómo se toma una decisión nueva

1. El propietario la formula en el chat o en un hito.
2. Se ejecuta el alcance pedido, no el siguiente.
3. Se informa; se espera revisión.
4. Si hay commit autorizado, un mensaje, un paquete homogéneo.

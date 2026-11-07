# Registro de decisiones

Las **firmes** rigen el trabajo hasta que el propietario las revoque. Las **propuestas** no autorizan código.

Protocolo de sesión: [ROADMAP.md](ROADMAP.md) §10.

---

## Firmes

| Id | Decisión |
|---|---|
| D-01 | El propietario es arquitecto y revisor. Cursor ejecuta cambios; no redefine la física ni el alcance. |
| D-02 | Ni commit ni push sin autorización explícita en el mensaje. Sin force-push a `main`. |
| D-03 | Preservar resultados históricos reproducibles (semilla `137137`, hashes VIS-01, tests HAR-02). Las corridas viven fuera de Git. |
| D-04 | `fitness` es un índice interno 0–1, **no** habitabilidad ni probabilidad antrópica. |
| D-05 | Separar `ModelObservables` (kg, s, α, …) de `HeuristicBreakdown` (términos, nivel, fitness). No mezclar unidades en un eje. |
| D-06 | No presentar hipótesis especulativas, 137, o un salto numérico como física de laboratorio demostrada. |
| D-07 | Hitos pequeños y verificables: tests, hashes, informe, revisión. |
| D-08 | Priorizar volver a **experimentar y visualizar** sobre reescribir ecuaciones. |
| D-09 | Autómatas 3D (`stream`/`lattice`) separados del motor cosmológico hasta que exista un puente teórico escrito. |
| D-10 | `evolve` es **objetivo estratégico**. HAR-EVO-01 será un **optimizador experimental** (H-03), no selección cosmológica. H-04 (Smolin) se mantiene aparte. El CLI sigue stub hasta ese hito. |
| D-15 | Prioridades vigentes: EVO-01 → PHYS-01 → PRIME-01, revisables tras cada experimento. HAR-PHYS-01 no bloquea experimentar. |
| D-16 | Fuentes: ROADMAP (estado), VISION (preguntas), RESEARCH_LOG (hechos), DECISIONS (acuerdos), informes HAR (detalle). Al cerrar un hito se tocan solo los docs afectados. |
| D-17 | P1–P4 son **preguntas documentadas**, no requisitos de código ni resultados físicos. H-01…H-07 se conservan. |
| D-11 | No implementar `DimensionlessLaw`, primalidad, ni acoplamiento grid–fitness de contrabando. |
| D-12 | No modificar CLI pública, pesos de fitness, RNG ni CSV históricos salvo hito que lo pida. |
| D-13 | Distinguir tests de software y validación científica en cada informe. |
| D-14 | El running 1-loop **no** es predicción cuantitativa en acoplamiento fuerte. El umbral `denom>0.1` es recorte de implementación. |
| D-18 | Tras HAR-MODEL-01: **A (Historical) es el único default** de `new()` y de la CLI. B (smooth ε) y D (α_s_had=α_s_ref) son variantes internas de comparación. B **no** está validada como QCD de laboratorio. |
| D-19 | HAR-EVO-01 (diseño): genoma `alpha_s`, `alpha`, `gamma=G/G_N`; motor A; sin cruce; torneo k=2; E=0 en la comparación principal; σ=0.05; caja e init como `map` (`e` y `g` uniformes); gen 0 compartido; mutación indexada `(generación, slot)`; CLI pública intacta. No es Smolin. |

---

## Propuestas (no aprobadas)

| Id | Propuesta | Estado |
|---|---|---|
| P-01 | HAR-EXP-03 | Hecho (gates estelares; A default) |
| P-02 | HAR-MODEL-01 (límites del 1-loop / comparación running) | Hecho (A default; B/D experimentales) |
| P-03 | HAR-PHYS-01 / parchear `running_alpha_s` | En cola (5); no es puerta de EXP-03 |
| P-04 | HAR-VISION-02 | Documentado (preguntas P1–P4); no implementado en código |
| P-05 | Push a `origin` | Prohibido de momento; `origin/main` ≠ `main` local |
| P-06 | HAR-EVO-01 parametrización | **Aprobado en diseño** (D-19). Código = hito 01B aparte |

---

## Cómo se toma una decisión nueva

1. El propietario la formula en el chat o en un hito.
2. Se ejecuta el alcance pedido, no el siguiente.
3. Se informa; se espera revisión.
4. Si hay commit autorizado, un mensaje, un paquete homogéneo.

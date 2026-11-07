# Roadmap maestro — Aletheia Engine / Harmonic-137

Fuente de verdad operativa. Complementa, no sustituye: [HARMONIC_VISION.md](HARMONIC_VISION.md) (intención histórica), [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md) (auditoría dimensional), [models.md](models.md) (ecuaciones del código), [DECISIONS.md](DECISIONS.md), [RESEARCH_LOG.md](RESEARCH_LOG.md).

Un test verde es **regresión de software**, no validación científica. No hay fechas de entrega.

**Línea base de experimentos:** `e5506541a2c5e22a309617cadd0b10af1b686b7a` (HAR-EXP-02). Este paquete documental es HAR-ROADMAP-01B. `origin/main` = `d58c16e` (lab 3D, sin Harmonic). Comprobar `git rev-parse HEAD`.

## Fuentes de verdad

| Documento | Función |
|---|---|
| Este [ROADMAP.md](ROADMAP.md) | Estado, arquitectura, prioridades, Git |
| [HARMONIC_VISION.md](HARMONIC_VISION.md) | Preguntas e hipótesis (H-01…H-07, P1–P4) |
| [RESEARCH_LOG.md](RESEARCH_LOG.md) | Experimentos y resultados comprobados |
| [DECISIONS.md](DECISIONS.md) | Acuerdos arquitectónicos y científicos |
| Informes `HAR_EXP_*.md`, [models.md](models.md), [cli.md](cli.md), [analysis.md](analysis.md) | Evidencia y detalle |

Al **cerrar un hito relevante** se actualizan solo los documentos que ese hito cambia (p. ej. un experimento → RESEARCH_LOG + informe; un cambio de política → DECISIONS). No se reescribe el resto por sistema.

---

## 1. Visión y objetivos

Laboratorio personal de **cómputo científico** y **exploración de hipótesis** sobre el espacio de leyes (`CosmicLaw`), con un visor 3D de autómatas y un grafo NestJS desconectado.

Objetivos vigentes:

1. Hacer **explícito y reproducible** lo que el motor calcula (α SI, running de α_s, masas teóricas vs PDG, heurística).
2. Separar **observables del modelo** de **índices de ranking**.
3. Conservar líneas base históricas (CSV, hashes, tests) antes de cambiar ecuaciones.
4. Avanzar hacia un `evolve` **estratégico**: primero un optimizador experimental (H-03); Smolin (H-04) aparte. Hoy el CLI es stub.
5. No presentar 137, fitness alto o una gráfica Plotly como física demostrada.

---

## 2. Estado actual (comprobado)

| Pieza | Estado | Notas |
|---|---|---|
| CLI `map` / `library` / `alpha-fixed` / `geometric` | Implementado + tests de esquema/RNG | `--rng-seed`; `map --observables-csv` |
| `validate` | Implementado; falla nivel atómico | No es pass científico |
| `harmonic`, `evolve` | **Stub** | Solo `println!` |
| Motor `AdvancedPhysicsEngine` | Implementado + tests de regresión | Running 1-loop con corte `denom>0.1`; no es QCD de laboratorio |
| Iso-Alpha A/B/C (HAR-04C) | Tests de software | A/B: reescalado MKSA; C: α_s distinto, misma α |
| HAR-VIS-01 | Experimental | 401 muestras, semilla `137137` |
| HAR-EXP-01 / 02 | Experimental, **cerrados en Git local** | Discontinuidad de `running_alpha_s` y su propagación |
| Autómatas `stream` / `lattice` + frontend | Verificados ALE-01B / ALE-02A | Sin puente al `CosmicLaw` |
| `backend/knowledge` | Implementado, runtime poco ejercido | Grafo SQLite; no cableado al visor |

Notas de estado actual añadidas en [README.md](../README.md) y [HARMONIC_VISION.md](HARMONIC_VISION.md) §4 (la cola HAR-04A se conserva como histórica).

---

## 3. Arquitectura resumida

```
frontend (Vite/R3F)  --WS 9001-->  rust-universe stream|lattice
rust-universe map|…  --> CSV -->  scripts Python/Plotly (venv fuera de Git)
backend/knowledge    --> SQLite     (independiente)
```

El fitness **no usa** `spatial_curvature`, `dimensional_ratios`, `temporal_evolution_rate`, `alpha_w`, `dark_matter_coupling`. Detalle: [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md), [models.md](models.md).

Resultados de corridas: carpetas hermanas `Aletheia-Engine-har*-runs` **fuera de Git**.

---

## 4. Hitos completados (commits locales)

| Hito | Commit | Qué quedó hecho |
|---|---|---|
| Monorepo inicial | `09087d8` | Árbol |
| «Calibration pass» | `53a24c8` | Mensaje histórico; **no** es validación PDG independiente |
| Lab 3D | `61f6540`, `d58c16e` | Visor + FluidCore. **`origin/main` = `d58c16e`** |
| HAR-02 RNG/CSV | `b718e80` | `--rng-seed`, tests deterministas |
| HAR-03 Python | `e30b86d` | Loaders y figuras; venv fuera del repo |
| HAR-04A visión | `a1473ab` | [HARMONIC_VISION.md](HARMONIC_VISION.md) |
| HAR-04B dimensiones | `ba124aa` | [HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md) |
| HAR-04C1–C5 | `bd16cf0` … `7e747a1` | Traza α, Iso-Alpha, `EngineEvaluation` vs heurística |
| HAR-VIS-01 | `e0102e3` | CSV de 30 columnas + Plotly |
| HAR-EXP-01 | `3fba121` | Corte `denominator > 0.1`, α_s ≈ 0.1392266 |
| HAR-EXP-02 | `e550654` | Propagación a masas, BBN y fitness |
| HAR-ROADMAP-01/01B | este commit documental | Memoria persistente; cola reordenada |

HAR-01 (límites del motor) no tiene un SHA propio; está en [RESEARCH_LOG.md](RESEARCH_LOG.md) y [models.md](models.md). ALE-01/02 (3D) son anteriores a Harmonic.

---

## 5. En curso

Nada en curso. Siguiente: **HAR-EVO-01** (diseño; no implementar hasta autorización). A sigue siendo el default.

---

## 6. Backlog priorizado (aprobado)

Orden **orientativo**; se puede revisar tras cada experimento. Criterios = software y trazabilidad, no descubrimiento físico. HAR-PHYS-01 **no** bloquea EXP-03 ni EVO-01.

1. **HAR-VISION-02** — Documentar P1–P4. *Hecho (documental); no implementado en código.*
2. **HAR-MODEL-01** — Inventario del modelo QCD y comparación pareada A/B/D. *Hecho* ([HAR_MODEL_01.md](HAR_MODEL_01.md)). A es el default; B/D experimentales; B no validada como QCD. PHYS-01 sigue aparte.
3. **HAR-EXP-03** — Gates estelares y niveles. *Hecho* ([HAR_EXP_03.md](HAR_EXP_03.md)). VIS-01 A: niveles 94/299/8; 8 con stellar>0.4; 0 pesados/BH. A no se sustituyó.
4. **HAR-EVO-01** — Primer **optimizador experimental** de poblaciones de `CosmicLaw` (H-03): genoma, mutación, selección, control sin selección. **No** afirma selección cosmológica. H-04 (Smolin) queda separado y no se implementa aquí. Criterio: diseño escrito; stub sustituido solo si el hito lo pide; fitness sigue siendo índice interno.
5. **HAR-PHYS-01** — Decisión explícita sobre aproximaciones físicas (p. ej. `running_alpha_s` / umbral 0.1). Opcional. No es requisito absoluto para seguir experimentando. Criterio: conservar línea base EXP-01/02 o versionar un motor nuevo sin reescribir hashes VIS-01 en silencio.
6. **HAR-PRIME-01** — Primos vs compuestos cuando el modelo retenga α = 1/N de forma coherente. Hoy `library` no lo hace.

### Fuera de alcance próximo

`DimensionlessLaw` como tipo Rust; acoplar autómatas 3D (H-07) al fitness; biología/cognición (H-05/H-06) como módulos; reescritura de la landing como teoría demostrada; force-push.

---

## 7. Líneas estratégicas (documentadas; no implementadas)

Detalle: [HARMONIC_VISION.md](HARMONIC_VISION.md) §2A.

**P1 — Primacía de α.** Relación H-01. Invariantes adimensionales ≠ cambio de unidades MKSA.

**P2 — Variación temporal de constantes adimensionales *dentro* de un universo** (`temporal_evolution_rate` se muestrea y no entra al fitness). **No** es H-03.

**P3 — Sensibilidad gravitacional y cosmología** (G, estructuras, expansión/colapso en el modelo). Subir G **no** implica un Big Crunch.

**P4 — Consistencia matemática de simetrías y teorías gauge.** La cancelación de anomalías **no** es una suma sencilla de cargas.

**H-03 — Evolución de poblaciones** de vectores `CosmicLaw` (optimizador). Distinto de P2.

**H-04 — Selección cosmológica de Smolin**, separada del optimizador genérico de HAR-EVO-01.

**H-05 / H-06 — Emergencia de complejidad y reflexión filosófica.** Analogía; no hay módulos de vida ni de autocomprensión cósmica.

**H-07 — Autómatas 3D** como línea **independiente** del escáner de constantes.

---

## 8. Dependencias y riesgos

| Dependencia / riesgo | Efecto |
|---|---|
| Running 1-loop + umbral 0.1 | Salto ~40 % m_p y ~63 % fitness en el universo de referencia (puntos de malla, no derivada) |
| Fallback PDG | Enmascara masas teóricas si se activa; 0/401 en VIS-01 |
| Fitness ≠ habitabilidad | Lectura antrópica indebida |
| `evolve` stub | No hay linajes |
| `origin/main` atrasado | Push no autorizado; no forzar |
| 16 warnings `rustc` históricos | No «arreglarlos» de paso en hitos de física |

---

## 9. Próximo hito recomendado

**HAR-EVO-01** — primer optimizador experimental de poblaciones `CosmicLaw` (H-03). Diseño antes que código; fitness = índice interno; control sin selección. **No** implementar hasta autorización. No PHYS-01 / PRIME-01 / Smolin aquí.

---

## 10. Política Git y revisión

- Propietario: arquitecto y revisor. Cursor: ejecutor.
- Commit y push **solo** con autorización explícita. Sin `--force` sobre `main`.
- Un hito, un commit cuando el alcance es homogéneo.
- Resultados de corridas fuera del árbol.
- Conservar hashes VIS-01 (`engine_evaluations` `2D19E32D…`, histórico `FC966CE1…`) salvo hito que los reemplace a propósito.

### Continuidad (sesiones futuras)

**Antes:** leer este archivo, [DECISIONS.md](DECISIONS.md) y el informe del hito activo; `git rev-parse HEAD`, rama, `git status`, remoto; no saltar al siguiente hito.

**Después:** listar archivos, correr tests de referencia, anotar hallazgos, pedir revisión; no commit/push sin orden.

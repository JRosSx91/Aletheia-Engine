# Modelos, reglas y límites científicos

**Idioma:** español. English: [README.md](../README.md) · Español: [README.es.md](../README.es.md).

Este documento describe **lo que implementa el código**, no un artículo de física. Las fórmulas se citan desde `backend/simulation/src/`.

Leyenda de tipos:

| Tipo | Significado |
|---|---|
| Física establecida | Expresión estándar usada como pieza (p. ej. definición de α) |
| Aproximación | Forma reducida o parámetros fijos tomados de tablas |
| Heurística | Puntuación 0–1 diseñada para ordenar muestras |
| Hipótesis | Motivación del experimento; no demostrada por el programa |

La **reproducibilidad del software** (misma semilla → mismos ticks en el grid binario) no equivale a **validación de una hipótesis física**.

Si `get_validated_hadron_masses` cae a masas de laboratorio, un test que compare esas masas con PDG **no es una predicción independiente**.

## Vector `CosmicLaw`

Definido en `backend/simulation/src/core/models.rs`. Campos muestreados en los modos Monte Carlo:

`g`, `e`, `alpha_s`, `alpha_w`, masas de quarks y leptones, `spatial_curvature`, `dimensional_ratios[4]`, `temporal_evolution_rate`, `dark_energy_density`, `dark_matter_coupling`.

**El fitness no usa** `spatial_curvature`, `dimensional_ratios`, `temporal_evolution_rate`, `alpha_w` ni `dark_matter_coupling`. Se muestrean y a veces se escriben en CSV, pero no entran en `calculate_enhanced_fitness`.

## Constante de estructura fina

En `AdvancedPhysicsEngine::new`:

```text
α = e² / (4 π ε₀ ħ c)
```

Tipo: **física establecida** (definición en SI). El CLI `alpha-fixed` **invierte** esa relación: fija α = 1/N y deriva `e` a partir de C, ħ y ε₀ aleatorios. Tipo: **experimento numérico / hipótesis** («primacía de alpha» en los comentarios de `alpha_fixed.rs`).

## Running de α_s

`running_alpha_s` usa un esquema a un lazo con `β₀ = (11·3 − 2 n_f)/3`, escala de referencia 91.1876 GeV y Λ_QCD = 0.217 GeV. Tipo: **aproximación** (no es un cálculo de red ni NNLO de PDG).

## Masas hadrónicas

`constituent_quark_mass` aplica masas quirales **fijas** (~310 MeV para up/down) más correcciones ad hoc con α_s. Tipo: **aproximación / heurística**.

`get_theoretical_hadron_masses` combina esas masas con un «binding» QCD simplificado. Si el resultado es no físico, o si se activa `--features empirical_validation`, se sustituyen por masas empíricas de protón, neutrón y pion. Tipo: **no es predicción independiente** cuando gana el fallback.

Hay funciones de bolsa MIT (`calculate_bag_radius`, etc.) **sin llamar** desde el fitness.

## Función de fitness

`calculate_enhanced_fitness` en `physics/engine.rs`:

1. Exige m_n > m_p y m_n > m_p + m_e; si no, fitness 0.
2. Suma ponderada (tope 1.0):

| Término | Peso | Notas |
|---|---|---|
| Viabilidad cosmológica | 0.15 | Tiempos de caída libre / Hubble con densidades **fijas** (`1e-26`, etc.) |
| Nucleosíntesis | 0.25 | Gamow simplificado + diferencia de masa vs 2.305×10⁻³⁰ kg |
| Formación estelar | 0.25 | Masa de Jeans a T=20 K, ρ=10⁻¹⁸, vida de secuencia principal ∝ M⁻²·⁵ |
| Elementos pesados | 0.20 | Barrera Coulomb + offset Hoyle 379 keV, factor empírico `× 1e18` |
| Agujeros negros | 0.15 | Chandrasekhar vs estrella de 25 M☉ |
| Bonus nivel 5 | 0.05 | Si scores altos |

Niveles 0–5 («Estéril» … «Reproductivo» en `utils/helpers.rs`). Tipo: **heurística de selección**. Los umbrales 0.3, 0.4, 0.5, 0.6, 0.7, 0.8 son del código, no de un paper.

## Validación (`validate`)

Compara el universo de referencia (constantes PDG/CODATA en `create_reference_universe`) con valores de laboratorio. El test de edad del universo compara `4.35e17` consigo mismo si la viabilidad es alta. El de masa del protón usa `get_validated_hadron_masses` (posible fallback empírico). Tipo: **comprobación de software**, no un *pass* científico.

## Autómata binario (`SimulationEngine`)

Cubo `[-25, 25]³` (51³ = 132 651 celdas). Estado `i8` ∈ {−1, 0, 1}. Cada tick, cada celda ≠ 0 elige un vecino von Neumann vacío y **salta**. Semilla RNG por defecto 137 (`StdRng`).

Tipo: **autómata / random walk**. No hay aniquilación +1/−1. No hay momento. Los tests unitarios cubren semilla, inyección y reset.

## Gas de rejilla (`LatticeGasEngine`)

Seis bits de dirección. Streaming + wrap toroidal. Colisión solo de pares opuestos:

```text
{+x, −x} → {+y, −y} → {+z, −z} → {+x, −x}
```

Masa = número de bits; momento = suma de vectores unitarios. `debug_assert` en builds debug. Tipo: **aproximación educativa** (HPP reducido). En ALE-01B, un HEAD-ON imprimió `masa=2 · momento=(0,0,0)` en t=100 y t=200; no es una prueba formal de todos los ticks.

## Hipótesis que motivan el laboratorio (código y landing)

Documentadas como **intención**, no como resultado:

- Biblioteca de universos indexada por el denominador de α (CLI `library`, texto de la landing).
- Selección de constantes vía agujeros negros / reproducción cósmica (`evolve` es stub; la landing lo narra).
- Información que «sube de nivel» (texto de UI, no un modelo acoplado).
- Campo ternario con conservación de carga (landing: *aún sobre el papel*).

Nada de esto está demostrado por una simulación validada contra observación.

# HAR-EXP-03 — Gates estelares y niveles superiores

Informe científico-computacional. HEAD de partida: `c498a4afffdc5acb1f55d2ca59cf67843d628d93`. Modelo **A** (Historical) en todo el censo y los barridos. D es solo contraste en 219/351 y en el funnel VIS-01. **No** es habitabilidad.

Corridas fuera de Git: `C:\Users\igomez\Desktop\labs\labs\Aletheia-Engine-har-exp03-runs\`

---

## 1. Mapa de gates

Fuente: `backend/simulation/src/physics/engine.rs` `evaluate` (467–529). Tres estados: **no evaluado** (`None` / celda vacía), **evaluado = 0**, **superado** (umbral de nivel).

| Etapa | Se entra si | Score | Superar (nivel) | Magnitudes |
|---|---|---|---|---|
| Nucleón | siempre se calcula | `m_n>m_p` y `m_n>m_p+m_e` sobre masas **validadas** | si falla: fitness 0, resto `None`, nivel 0 | `engine.rs:471–472` |
| Cosmología | nucleón ok | 1 si t_ff y t_H ≥ 1e8 yr; si no 0 | `cosmic_viability ≥ 0.1` para seguir | `g`, ρ=1e-26, `dark_energy_density`. `268–284` |
| BBN | cosmología superada | `pp>1e-50` × `dp>1e-47` × gaussiana Δm vs 2.305e-30 kg | `> 0.3` → nivel 1 | Δm, m_p, **α SI** (Gamow). `309–327`, `288–307` |
| Estelar | BBN > 0.3 | 0 si τ&lt;1 Myr; 1 si 1 Myr≤τ≤100 Gyr; √(100 Gyr/τ) si τ&gt;100 Gyr | `> 0.4` → nivel 2 | M_Jeans ∝ c_s³/(G^{3/2}ρ^{1/2}), c_s∝1/√m_p, τ∝(M/M_☉)^{−2.5}. `330–386` |
| Pesados | estelar > 0.4 | `min(1, rate×α_s_score)`; α_s_score gaussiana **α_s_ref** (óptimo 0.118, σ=5 %) | `> 0.5` → nivel 3 | Coulomb ∝ e²; **no** usa `running_alpha_s`. `398–418` |
| BH | pesados > 0.5 | 1 si 2.5 M_Ch &lt; 25 M_☉; si no (25 M_☉ / TOV)^{1.5} | `> 0.6` → nivel 4; `>0.8` **y** pesados `>0.7` → **nivel 5** | M_Ch ∝ (ħc/G)^{3/2}/m_p². `425–441` |

Niveles 0–5. En VIS-01 el score estelar es **binario** 0 o 1 (nadie cae en la cola √). `stellar_term=0.0` ≠ `None`.

Jeans/τ diagnósticos (`jeans_diagnostic_kg`, `stellar_lifetime_*`) se calculan siempre con métodos públicos; `jeans_mass_kg` del `evaluate` sigue vacío si la etapa no se abre.

---

## 2. Casos 219 y 351 (A vs D)

Mismos `CosmicLaw` (emparejados por `sample_id`). No se retocaron parámetros.

| | 219 A | 219 D | 351 A | 351 D |
|---|---:|---:|---:|---:|
| G (SI) | 6.167e-10 | idem | 6.020e-10 | idem |
| e (C) | 1.043e-19 | idem | 8.318e-20 | idem |
| α SI | 0.00309 | idem | 0.00197 | idem |
| α_s_ref | 0.1247 | idem | 0.1339 | idem |
| α_s(1 GeV) | 0.643 (1-loop) | 0.125 | 0.997 | 0.134 |
| m_p (kg) | 8.60e-28 | 9.75e-28 | 7.82e-28 | 9.73e-28 |
| M_Jeans (kg) | 8.48e31 | 7.03e31 | 1.01e32 | 7.31e31 |
| τ (yr) | **8.42e5** (below_1myr) | **1.35e6** (in_window) | **5.39e5** | **1.22e6** |
| BBN | 0.920 (ok) | 0.956 | 0.671 (ok) | 0.960 |
| stellar | **0.0 evaluado** | **1.0** | **0.0** | **1.0** |
| pesados / BH | None / None | 1 / 1 | None / None | 1 / 1 |
| nivel / fitness | 1 / 0.380 | **5** / 1.0 | 1 / 0.318 | **5** / 1.0 |

Motivo: el 1-loop baja m_p → sube c_s → sube M_Jeans → τ∝M^{−2.5} cae bajo 1 Myr. D usa α_s_ref, m_p mayor, τ entra en ventana, y con e pequeño + α_s_ref≈0.12 la gaussiana de pesados y el TOV puntúan 1. **No** es un error de CSV.

---

## 3. Distribución VIS-01 (401, semilla 137137)

No se amplió la población: A ya tiene 8 universos con `stellar>0.4`. Conteos **completos** (incluye ceros).

| | A (principal) | D (contraste) |
|---|---:|---:|
| nucleón ok | 401 | 401 |
| cosmología = 1 | 401 | 401 |
| BBN evaluado | 401 | 401 |
| BBN > 0.3 (nivel ≥1) | 307 | 307 |
| BBN ≤ 0.3 (nivel 0) | 94 | 94 |
| stellar evaluado | 307 | 307 |
| stellar = 0 | 299 | 297 |
| stellar > 0.4 | **8** | 10 |
| pesados evaluado | 8 | 10 |
| pesados > 0.5 | **0** | 2 |
| BH evaluado | **0** | 2 |
| nivel 0 / 1 / 2 / 3 / 4 / 5 | 94 / 299 / 8 / 0 / 0 / 0 | 94 / 297 / 8 / 0 / 0 / 2 |

Passers estelares A (`sample_id`): **175, 189, 200, 227, 236, 242, 298, 373**. Todos G ≳ 6.0e-10 (techo del sorteo). Pesados A: 7 veces 0 y 189 = 1.44e-8 (e grande, α_s=0.0979 lejos de 0.118).

---

## 4. Sensibilidad (A, un eje)

Universos base: **189** (A ya abre stellar) y **219** (A no abre stellar).

**G (SI).** Jeans ∝ G^{−3/2} a m_p y ρ=1e-18 fijas. Subir G acorta M_Jeans y alarga τ. **No** es una variación adimensional: se deja el resto en SI ([HARMONIC_DIMENSIONS.md](HARMONIC_DIMENSIONS.md), P3). 189: stellar>0.4 para G ∈ [5.94e-10, 6.67e-10]. 219: hace falta G ≥ **6.47e-10** (nativo 6.17e-10, por eso A falla).

**α_s_ref.** En 189 (G alto) stellar sigue abierto en casi toda la malla 0.05–0.50; **ningún** punto pasa pesados>0.5 (α_s=0.0979 + e grande). En el **barrido 1-parámetro** de la ley 219 (no en la población VIS-01 original), α_s_ref ∈ **[0.103, 0.115]** abre stellar (m_p via running) **y** la gaussiana de pesados → nivel 5 **bajo A**. El nativo de 219 es 0.1247: en VIS-01-A sigue en nivel 1.

**α / e.** Coulomb ∝ e². En 189, bajar e hasta α≈0.0011 sube pesados a **0.20**, aún &lt; 0.5: el cuello es la gaussiana de α_s_ref, no solo Coulomb.

Cada CSV de `scan_*` clona el `CosmicLaw` base y escribe **un** campo (`alpha_s`, `e` o `g`). Fitness y nivel salen de `evaluate()` con `RunningKind::Historical`.

**Genoma vs fitness.** Activos en fórmula: `g`, `e` (α SI), `alpha_s`. Umbral: `mass_electron`, `dark_energy_density` (y `g` en el corte cosmológico). Ignorados: `alpha_w`, masas u/d (sustituidas por quirales), quarks pesados, μ/τ, `spatial_curvature`, `dimensional_ratios`, `temporal_evolution_rate`, `dark_matter_coupling`. Un ranking de fitness empujará G al techo del sorteo (~6.67×10⁻¹⁰ SI) y α_s_ref hacia 0.118: **límites artificiales del modelo**, no evidencia antrópica.

---

## 5. Figuras

En `Aletheia-Engine-har-exp03-runs\`:

- `exp03_levels.html` — niveles A vs D
- `exp03_gate_funnel.html` — evaluado vs superado
- `exp03_params_vs_level.html` — G vs α_s_ref, color nivel
- `exp03_jeans_lifetime.html` — M_Jeans y τ vs G (189 y 219)
- `exp03_219_351_AD.html` — A vs D
- `exp03_189_heavy.html` — pesados vs α_s y vs α (ejes separados)

---

## 6. Resultados inesperados

1. VIS-01 **A ya tiene 8** stellar>0.4; EXP-02 no los veía porque el universo de referencia tiene G de laboratorio.
2. Esos 8 **no** abren pesados: e demasiado grande y/o α_s_ref lejos de 0.118.
3. Nivel 5 en la población VIS-01 solo aparece en D (219, 351). El intervalo α_s ≈ 0.103–0.115 es del **barrido** de la ley 219, no de esas muestras originales.
4. El score estelar en esta población es 0 o 1, nunca la cola de 100 Gyr.
5. Nucleón y cosmología no recortan VIS-01; el nivel 0 es solo BBN≤0.3.

---

## 7. Limitaciones

Índices internos. 1-loop no es QCD. G SI ≠ invariante. ρ Jeans y T=20 K fijos. Pesados usan PDG proton en reduced_mass y α_s_ref. No se afirma habitabilidad.

---

## 8. Tests

Censo A 94/299/8; passers 175…373; 219/351 A nivel 1 `below_1myr` vs D nivel 5 `in_window`; 189 A nivel 2; `new()` = Historical; barridos deterministas.

---

## 9. Recomendación para evolución de universos

HAR-EVO-01, si se autoriza, debe **conservar A**. Un optimizador que maximice el fitness empujará G al techo del sorteo y α_s_ref hacia 0.118 **solo si** también baja e lo bastante para Coulomb; eso es un artefacto de umbrales, no selección cosmológica. Control sin selección obligatorio. No implementar `evolve` aquí.

Sin PHYS-01, PRIME-01 ni cambio de gates.

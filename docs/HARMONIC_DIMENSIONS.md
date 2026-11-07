# HAR-04B — Auditoría dimensional y espacio de parámetros

**Alcance:** diagnóstico y diseño. El motor **no** se modifica.
**Visión:** [HARMONIC_VISION.md](HARMONIC_VISION.md) (H-01 Iso-Alpha).
**Código actual:** [models.md](models.md).

Convención SI del crate: MKSA. Coulomb \(\propto e^2/(4\pi\varepsilon_0 r)\). No se usa Heaviside-Lorentz ni unidades de Planck en el muestreo.

La constante de estructura fina que se identifica con \(\approx 1/137\) es el **acoplamiento QED a baja energía** (límite Thomson / espectroscopía atómica), no \(\alpha(M_Z)\). El motor **no** hace running de \(\alpha_{\mathrm{EM}}\); solo un running simplificado de \(\alpha_s\).

HAR-04B.1 corrige clasificaciones dimensionales del texto (Λ_QCD, ρ_Λ, Iso-Alpha B). No cambia el motor. Las siete hipótesis de [HARMONIC_VISION.md](HARMONIC_VISION.md) permanecen intactas.

---

## 1. Inventario

### 1.1 Constantes SI globales (`physics/constants.rs` y duplicados en `engine.rs`)

Fijas. Nunca viven en `CosmicLaw`. Toda α del motor se calcula con **estas** \(c,\hbar,\varepsilon_0\).

| Nombre | Valor | Dim. SI | Uso real |
|---|---|---|---|
| `C` | 299792458 m/s | L T⁻¹ | α, conversiones masa–energía, Chandrasekhar, bolsa MIT (muerta) |
| `H_BAR` | 1.054571817×10⁻³⁴ J·s | M L² T⁻¹ | α, Chandrasekhar, bolsa MIT |
| `EPSILON_0` | 8.8541878128×10⁻¹² F/m | M⁻¹ L⁻³ T⁴ I² | α y barrera Coulomb en `heavy_element_creation` |
| `K_B` | 1.380649×10⁻²³ J/K | M L² T⁻² Θ⁻¹ | T_BBN=10⁹ K, T_Jeans=20 K, T_core=1.5×10⁸ K (todas fijas) |
| `G_GRAVITATIONAL` | 6.67430×10⁻¹¹ m³ kg⁻¹ s⁻² | L³ M⁻¹ T⁻² | Solo universo de referencia (`validate`) |
| `ELEMENTARY_CHARGE` | 1.602176634×10⁻¹⁹ C | I T | Solo referencia |
| `ELECTRON_MASS_EXACT` | 9.1093837015×10⁻³¹ kg | M | Referencia; duplicada en `engine.rs` |
| `PROTON/NEUTRON/PION` empíricas | PDG en kg | M | Fallback `get_validated_hadron_masses`; Hoyle usa protón empírico **siempre** |
| `LAMBDA_QCD_GEV` | 0.217 GeV | energía (M L² T⁻²) | Binding QCD y corte IR del running. **No** es adimensional: 0.217 es el valor numérico; GeV es la unidad; la dimensión es energía. En SI: `0.217 * GEV_TO_JOULE` joules. |
| `M_SOLAR` | 1.98847×10³⁰ kg | M | Vida MS. BH usa **otra** masa solar 1.989×10³⁰ |
| `MEV_TO_JOULE`, `GEV_TO_JOULE`, `KG_TO_GEV`, `MEV_TO_KG` | de `C` SI | — | Conversiones. `JOULE_TO_EV`, `MPC_TO_METERS`, `KEV_TO_JOULE` **no se usan** |

`engine.rs` redefine `MEV_TO_JOULE`, `GEV_TO_JOULE`, `KG_TO_GEV`, masas PDG y `LAMBDA_QCD_GEV` en lugar de importarlas. Mismo valor numérico, dos fuentes.

### 1.2 Vector `CosmicLaw` (`core/models.rs`)

Consumo comprobado en llamadas, no en nombres.

| Campo | Dim. | ¿Adim.? | Muestreo | ¿Entra en el motor? | ¿En fitness? |
|---|---|---|---|---|---|
| `g` | L³ M⁻¹ T⁻² | no | Sí (×1–10 G_N típ.) | cosmología, Jeans, Chandrasekhar | Sí |
| `e` | I T (coulomb) | no | Sí, o derivado en library/alpha-fixed | α SI; barrera Coulomb (`laws.e`, no `engine.alpha`) | Sí (vía α y Coulomb) |
| `alpha_s` | 1 | sí (acoplamiento) | 0.05–2 (map) | `alpha_s_reference` = α_s(μ_ref), μ_ref=`reference_scale`=91.1876 GeV (M_Z); running 1-lazo; gaussiana 0.118 sobre el valor *sin* correr | Sí |
| `alpha_w` | 1 | sí | Sí | **Ninguna** | No |
| `mass_up_quark` | kg (map/library); `rng.gen()` 0–1 en map para *otras* masas | — | Sí | Pasado a `constituent_quark_mass("up")` **y se ignora** (quiral 310 MeV) | No (salvo fallback hadrónico indirecto nulo) |
| `mass_down_quark` | kg | — | Sí | Igual: quiral 310.00125 MeV | No de forma efectiva |
| `mass_electron` | kg | no | Sí | Solo el gate `m_p + m_e < m_n` | Gate 0/sigue |
| `mass_strange/charm/top/bottom`, `mass_muon`, `mass_tauon` | kg o 0–1 (map) | — | Sí | **Ninguna** (charm usaría bare mass si se llamara con flavor `"charm"`; el fitness no lo llama) | No |
| `spatial_curvature` | etiqueta (no es k [L⁻²]) | no física | Sí | **Ninguna** | No (sí CSV) |
| `dimensional_ratios[4]` | 1 | sí | Sí | **Ninguna** | No |
| `temporal_evolution_rate` | 1 | sí | Sí | **Ninguna** | No (sí CSV map) |
| `dark_energy_density` | kg m⁻³ (densidad de **masa**) | no | 0–2e-29 (map) | \(t_H\sim(G\rho_\Lambda)^{-1/2}\) en `cosmological_viability` | Sí (umbral binario) |
| `dark_matter_coupling` | 1 | sí | Sí | **Ninguna** | No |

Coeficientes **arbitrarios** (no CODATA) que sí mueven números: densidades 1e-26 y 1e-18 kg m⁻³, T=20 K / 1e9 K / 1.5e8 K, 1e8 yr, Δm objetivo 2.305e-30 kg, cortes de sección 1e-50 / 1e-47, factores 1e-45 / 1e-42 / 1e18, α_s óptima 0.118 ± 5 % (ancla numérica frente a `laws.alpha_s`, no al running), umbrales de nivel 0.3–0.8, pesos 0.15/0.25/0.25/0.20/0.15/0.05, masas quirales 310 MeV/\(c^2\), Λ_QCD×2 binding, r_Coulomb=10 fm, Hoyle 379 keV, TOV=2.5 M_Ch, estrella 25 M☉, μ_e=2, n_f saltos 1.3/4.2 GeV, μ_ref=M_Z=91.1876 GeV.

`dark_energy_density` entra en \(t_H = (3/(8\pi G \rho_\Lambda))^{1/2}\). Esa fórmula es dimensionalmente coherente **solo** si \(\rho_\Lambda\) es densidad de masa [kg m⁻³], no densidad de energía [J m⁻³]. El código no multiplica por \(c^2\). Densidad de energía asociada (si se necesitara): \(\varepsilon_\Lambda = \rho_\Lambda c^2\).

**Normalización adimensional (no usar \(\rho_\Lambda/m_p^4\)).** En SI, \(\rho_\Lambda/m_p^4\) tiene dimensiones L⁻³ M⁻³. Combinación válida, con \(\lambda_C=\hbar/(m_p c)\):

\[
\eta_\Lambda = \frac{\rho_\Lambda\,\hbar^3}{m_p^4 c^3} = \frac{\varepsilon_\Lambda\,\hbar^3}{m_p^4 c^5} = \frac{\rho_\Lambda\,\lambda_C^3}{m_p}.
\]

\(\Omega_\Lambda=\rho_\Lambda/\rho_c\) es adimensional, pero \(\rho_c=3H^2/(8\pi G)\) introduce una escala \(H\) extra. Si \(H\) se deriva del mismo \(\rho_\Lambda\), \(\Omega_\Lambda\) se vuelve tautológico. Para `DimensionlessLaw` preferir \(\eta_\Lambda\).

Λ_QCD y \(\alpha_s(\mu_{\mathrm{ref}})\) no son intercambiables: la primera es una **escala de energía** fija del crate; la segunda es un acoplamiento adimensional anclado en \(\mu_{\mathrm{ref}}=91.1876\,\mathrm{GeV}\). El running 1-lazo **no** reconstruye Λ_QCD a partir de \(\alpha_s\).

Funciones MIT bag (`calculate_bag_radius` … `vacuum`) **nunca se llaman** desde el fitness.

### 1.3 CLI → muestreo

| Modo | Extra | Efecto real |
|---|---|---|
| `map --universes --rng-seed` | `0..=N` sorteos | `e` independiente → α SI; CSV `alpha` = 1/α |
| `library --min-alpha --max-alpha --samples-per-book` | α_obj=1/N; c,ħ,ε₀ × U(0.5,2) solo para derivar `e` | Esas tres **no se guardan**; el motor usa SI |
| `alpha-fixed --alpha-denominator --samples` | c,ħ,ε₀ × U(0.1,10) → `e`; se escriben en CSV | Igual: motor ignora variantes |
| `geometric --samples` | rangos más anchos de curvatura/ratios | Fitness idéntico en estructura a map |
| `validate` | universo PDG | Tests vs laboratorio; protón teórico ~500 MeV |

---

## 2. Grafo de dependencias reales

```
CLI (N, samples, seed)
  → sample CosmicLaw
  → AdvancedPhysicsEngine::new
        α := e² / (4π ε₀_SI ħ_SI c_SI)     // pierde c_var, ħ_var, ε₀_var
        α_s_ref := CosmicLaw.alpha_s   // α_s(μ_ref), μ_ref = 91.1876 GeV
  → hadrones teóricos: quirales(α_s) − binding(α_s, Λ_QCD)
        o fallback PDG si no físicos
  → gate m_n > m_p + m_e
  → cosmological_viability(g, ρ_Λ; ρ_ff fija)     → 0 o 1
  → BBN(α, Δm vs 2.305e-30 kg, cortes de σ)      → [0,1]
  → estrellas(g, m_p, T,ρ fijos)                 → 0, 1 o √(cap/τ)
  → heavy(e, ε₀_SI, α_s vs 0.118, Hoyle fijo)    → [0,1] saturado
  → BH(g, m_p, ħ, c)                             → [0,1]
  → fitness = min(1, suma ponderada) + niveles
  → CSV (subconjunto; a veces 1/α o geometría inerte)
```

**Previsto originalmente (visión H-01):** α_obj → (c,ħ,ε₀,e) coherentes **dentro** del motor → observables distintos a igual α.

**Real:** α_obj solo escala `e`; el motor reconstruye otra α con SI. Iso-α no ocurre.

### 2.1 Recálculos con SI globales

- `engine.alpha` y columna map `alpha`.
- Chandrasekhar: \(\hbar c / G\).
- Coulomb en CNO/Hoyle: \(e^2/(4\pi\varepsilon_0 r)\) con \(\varepsilon_0\) SI y `laws.e`.
- Conversiones kg↔GeV con `C` SI.

### 2.2 Circularidad empírica

- Fallback PDG en masas hadrónicas si el teórico es «no físico».
- Feature `empirical_validation`: PDG siempre.
- `validate` «Proton Mass» compara `get_validated_hadron_masses` con PDG: si hay fallback, tautología. Hoy **no** hay fallback en el universo de referencia (teórico ~500 MeV) y el test **falla**.
- Test de edad: 4.35×10¹⁷ vs sí mismo si viability>0.5.
- Δm objetivo, Hoyle 379 keV, α_s=0.118, masas quirales: anclas terrestres dentro de la heurística.

### 2.3 Conversiones / unidades dudosas

- `nuclear_cross_section`: `2.0 * alpha * sqrt(μ E_th / 2)` no es la forma SI del factor de Gamow (falta \(c\), Z, y las dimensiones no cierran como energía).
- `map` asigna charm…tau con `rng.gen()` ∈ (0,1), no kg.
- Dos masas solares distintas.
- `main_sequence_lifetime(jeans_mass)` trata una masa de nube como masa estelar.
- `GEV_TO_JOULE` en `realistic_qcd_binding` convierte GeV (energía) a J; coherente como energía de ligadura, luego se divide por \(c^2\) a masa.

---

## 3. Cómo se calcula y propaga α

Definición **establecida** (SI, baja energía):

\[
\alpha = \frac{e^2}{4\pi\varepsilon_0\hbar c} \approx \frac{1}{137.036}.
\]

Escala: acoplamiento QED en el umbral atómico / Thomson, no \(\alpha(M_Z)\approx 1/128\).

| Camino | Fórmula | ¿Es α de baja energía? |
|---|---|---|
| Motor | `laws.e² / (4π ε₀_SI ħ_SI c_SI)` | Sí, con \(e\) del universo y **vacío SI terrestre** |
| `alpha-fixed` / `library` (intención) | `e = sqrt(α_obj · 4π ε₀_var ħ_var c_var)` | α_obj solo si el motor usara las mismas var |
| CSV map | `1 / α_motor` | Recíproco, 6 decimales |

Lo físicamente relevante no es el valor numérico aislado de \(c\), \(\hbar\) o \(\varepsilon_0\), sino qué **relaciones adimensionales independientes** se mantienen fijas. Un cambio de un solo parámetro dimensional, con el resto de números SI inmóviles, suele alterar α, \(\alpha_g=G m_p^2/(\hbar c)\) u otras razones: eso es ambiguo hasta declarar el conjunto invariante. Un reescalado **compensado** que deje invariantes todas esas razones es un cambio de representación (unidades o convención EM), no un vacío distinto.

Hoy el motor clava \(c,\hbar,\varepsilon_0\) SI y trata cada `e` distinto como α distinta. Eso mezcla universos numéricos que, con una convención EM explícita, podrían ser la misma física.

`heavy_element_creation` usa \(e^2/\varepsilon_0\) y **no** `self.alpha`. Si en el futuro se mutara \(c\) o \(\hbar\) en CosmicLaw sin la misma convención, Coulomb y α se desincronizarían.

---

## 4. Parámetros que no afectan a ningún resultado del motor

No entran en `calculate_enhanced_fitness` ni en observables del engine usados por él:

`alpha_w`, `mass_strange_quark`, `mass_charm_quark`, `mass_muon`, `mass_top_quark`, `mass_bottom_quark`, `mass_tauon`, `spatial_curvature`, `dimensional_ratios`, `temporal_evolution_rate`, `dark_matter_coupling`.

`mass_up_quark` / `mass_down_quark` se pasan y **se descartan** para u/d (masas quirales fijas). Solo `flavor == "charm"` leería la masa desnuda.

`c_variant`, `hbar_variant`, `epsilon0_variant`: CSV y derivación de `e`; el engine no los ve.

Bag MIT: código muerto.

`JOULE_TO_EV`, `MPC_TO_METERS`, `KEV_TO_JOULE`, `reduced_mass` en Hoyle: no usados.

Geometría 5D: muestreada, narrada, **inerte**.

---

## 5. Fitness (sin modificarlo)

| Término | Peso | ¿Calculado? | ¿Heurístico / ancla terrestre? | Sensible a CosmicLaw |
|---|---|---|---|---|
| Gate Δm nucleón + \(m_e\) | corte | masas hadrónicas + `mass_electron` | umbral de estabilidad β | \(m_e\); u/d **no**; α_s sí (quiral/binding) |
| Cosmología | 0.15 | \(t_{ff}(G,\rho_{\mathrm{fix}})\), \(t_H(G,\rho_\Lambda)\) | ρ_ff=1e-26; 10⁸ yr; **salida 0 o 1** | `g`, `dark_energy_density` (binario) |
| BBN | 0.25 | Δm vs 2.305e-30 kg; σ Gamow | cortes 1e-50/1e-47; Δm PDG | α (fórmula dudosa), Δm(α_s), no \(m_e\) en Gamow |
| Estrellas | 0.25 | Jeans + τ∝M⁻²·⁵ | T,ρ ISM fijos; Jeans como M_* | `g`, m_p(α_s) |
| Pesados | 0.20 | Coulomb(e,ε₀)+Hoyle+gauss α_s | 379 keV, 10 fm, ×1e18, cap 1 | `e`, `alpha_s`; **satura** a 1 a menudo |
| BH | 0.15 | M_Ch(G,m_p,ħ,c) vs 25 M☉ | 2.5 TOV, μ_e=2 | `g`, m_p |
| Bonus niv. 5 | 0.05 | si BH>0.8 y heavy>0.7 | umbrales | pocos universos HAR-02 |

**Saturaciones que ocultan diferencias:** cosmología binaria; BBN pp/dp binarios; stellar 0/1/√; heavy `.min(1.0)`; fitness `.min(1.0)`.

No es estabilidad física medida, complejidad química ni tasa de agujeros negros de Smolin. Es un **índice interno de software** (ranking heurístico). No atribuye validez científica a H-03, H-04 ni H-05.

HAR-02: campos ignorados no cambian el score (`unused_cosmic_law_fields_do_not_change_fitness`).

---

## 6. Diseño conceptual Iso-Alpha (no implementar)

**Invariantes a preservar en equivalencia (caso B):** cantidades adimensionales que el *modelo* use: α (baja energía), α_s(μ_ref=M_Z), \(m_e/m_p\), \(m_n/m_p\), \(\alpha_g=G m_p^2/(\hbar c)\), \(\eta_\Lambda=\rho_\Lambda\hbar^3/(m_p^4 c^3)\), \(kT/(m_p c^2)\) en cada umbral fijo del código.

**Caso A — Referencia.** `create_reference_universe()`: e, G, α_s(μ_ref=91.1876 GeV), masas PDG, ρ_Λ=1.2e-29 kg m⁻³ (densidad de masa). Observables de control: `engine.alpha`, masas teóricas, cada término del índice interno, nivel. El índice no se interpreta como fertilidad.

**Caso B — Representación equivalente (test de software).** Transformación **controlada**, no un universo distinto:

\[
\varepsilon_0'=\lambda\varepsilon_0,\quad e'=e\sqrt{\lambda},\quad c'=c,\quad \hbar'=\hbar.
\]

Conserva α y el acoplamiento de Coulomb \(e^2/(4\pi\varepsilon_0 r)\) en la convención MKSA del crate. El test compara **dos escrituras de la misma física** y comprueba que el software no las trata como vacíos distintos. **Hoy el motor no puede** representar B: \(\varepsilon_0\) no está en `CosmicLaw`. Si solo se cambia `e` a \(e\sqrt{\lambda}\), el motor verá otra α: fallo de consistencia, no descubrimiento. Criterio de éxito futuro: A y B coinciden en los invariantes adimensionales (y en el índice interno *si* este se expresa en forma adimensional).

**Caso C — Misma α, otra física.** Fijar α en el motor y variar al menos otra razón independiente (p. ej. α_s(μ_ref) o \(\alpha_g\) o \(m_e/m_p\)). Deben poder cambiar: running QCD, binding, Δm relativo, Jeans/Chandrasekhar en unidades de \(m_p\). No debe cambiar: α. No declarar «más fértil» por un parámetro dimensional aislado ni por un movimiento del índice interno.

**Falsación del diseño, no de la naturaleza:** si A≠B en observables adimensionales del modelo, la arquitectura de unidades es incorrecta. Si A=C para todo cambio de α_s(μ_ref), el modelo es ciego (como u/d quirales fijas).

**Controles:** semilla HAR-02; no usar fallback PDG en A/B/C; informar α_motor, α_obj, \(\alpha_g=G m_p^2/(\hbar c)\), \(\eta_\Lambda\).

---

## 7. Arquitectura mínima propuesta

No reescribir el crate. Tres capas **al lado** del histórico, detrás de un flag o módulo nuevo, con tests HAR-02 como línea base.

1. **`ReferenceSI`** (ya existe de facto): `c`, `ħ`, `ε₀`, `k_B` de `constants.rs`. Convención de unidades, no leyes del universo simulado.

2. **`DimensionlessLaw`** (nuevo, pequeño): α (baja energía, escala explícita), α_s(μ_ref=M_Z), \(m_e/m_p\), \(\alpha_g = G m_p^2 /(\hbar c)\), \(\eta_\Lambda=\rho_\Lambda\hbar^3/(m_p^4 c^3)\) (con \(\rho_\Lambda\) densidad de masa SI). Opcional: \(\Delta m/m_p\) si se modela el sector hadrónico. Omitir al inicio lo inerte (`alpha_w`, 5D, DM coupling) o marcarlo `unused`. No incluir \(\rho_\Lambda/m_p^4\).

3. **`DerivedSI`:** \(e\), \(G\), masas en kg **calculadas** desde (1)+(2) para no romper fórmulas actuales.

4. **`Observables`:** α, masas hadrónicas teóricas (sin fallback silencioso), Δm, t_H, M_Ch, … valores con unidad, sin recortar a [0,1].

5. **`HeuristicScore`:** `calculate_enhanced_fitness` **intocado** como métrica histórica; nueva puntuación, si llega, en otro nombre.

6. **`Validation`:** comparar Observables con PDG **sin** copiar PDG al observable. El fallo del protón ~500 MeV se conserva como regresión.

Compatibilidad Rust: `From<DimensionlessLaw> for CosmicLaw` para alimentar el engine histórico. `evolve` futuro mutaría `DimensionlessLaw`, no \(c\).

---

## 8. Riesgos

**Científicos:** optimizar el índice interno = reencontrar anclas terrestres; Iso-α mal implementado = no reconocer representaciones equivalentes; α de baja energía vs α(M_Z); quirales fijas anulan H-01 en el sector hadrónico; Smolin ≠ ranking heurístico. El índice no valida hipótesis.

**Técnicos:** duplicar constantes engine/constants; flag `empirical_validation`; CSV `alpha`=1/α; `rng.gen()` en masas; Gamow mal dimensionado; saturaciones; romper HAR-02 si se cambia el orden RNG o las fórmulas.

---

## 9. Plan HAR-04C (incrementos, cada uno con test)

No adelantar EVO ni primalidad.

| Paso | Qué | Verificación | Qué no |
|---|---|---|---|
| C1 | Inventario en código: `#[allow]` o lista `UNUSED_COSMIC_LAW_FIELDS` + test que mutarlos no cambia fitness (ya existe; extender a masas pesadas) | `cargo test` 29+ | No borrar campos aún |
| C2 | Documentar/trazar las tres α: objetivo, derivada, motor. Función pura `si_alpha(e)` | igualdad con `engine.alpha` | No cambiar muestreo |
| C3 | **Opción A (mínima):** pasar ε₀,ħ,c al engine **solo** bajo test Iso-α B (transformación \(\varepsilon_0'=\lambda\varepsilon_0\), \(e'=e\sqrt{\lambda}\)), default SI = histórico | A y B: misma α y mismo Coulomb; tests HAR-02 con default | No activar en CLI map; B no es física nueva |
| C4 | Dejar de ignorar u/d **o** dejar de muestrearlos: decisión explícita + snapshot masas 500.509 MeV | HAR-02 snapshot sigue o se versiona | No PDG silencioso |
| C5 | Separar Observables de Heuristic en API (struct de retorno), fitness idéntico | bitwise/float fitness HAR-02 | No nuevos pesos |
| C6 | Informe Iso-α A/B/C con semilla fija, **lectura** no CLI nuevo obligatorio | tabla A=B, A≠C en α_s | No «CONFIRMADO» |

Criterio de parada C: el motor **puede** expresar B sin mentir sobre α; C mueve un adimensional real (α_s). Entonces H-01 es investigable. H-03 espera HAR-EVO-01 sobre `DimensionlessLaw`.

---

## 10. Hipótesis tras HAR-04C

Las definiciones de H-01 … H-07 en [HARMONIC_VISION.md](HARMONIC_VISION.md) **no se reformulan**. Solo se indica qué podrá *ejercitarse en software* después de C3–C5.

| Hipótesis | ¿Ejercitable en software si C3–C5 salen? | Sigue fuera del modelo |
|---|---|---|
| H-01 Iso-Alpha | Sí: A/B consistencia de representación; A/C otra razón adim. (α_s o α_g) | Física QED real, running α_EM |
| H-02 Primalidad | No hasta modelo que retenga α=1/N **y** observables adim. | Privilegio aritmético en la naturaleza |
| H-03 Evolución | Diseño posible sobre `DimensionlessLaw` | Que la naturaleza optimice el índice interno |
| H-04 Smolin | No: hace falta herencia por colapso, no un score | Evidencia de universos hijo |
| H-05 Capas | No: no hay química/biología | Emergencia real |
| H-06 Autocomprensión | Analogía | — |
| H-07 Autómatas | Ya son autómatas | Acoplar al escáner |

Fin HAR-04B / HAR-04B.1. Sin cambios de comportamiento del motor.

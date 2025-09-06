use crate::core::models::CosmicLaw;
use crate::physics::constants::*;
use std::f64::consts::PI;

const KEV_TO_JOULE: f64 = 1.602176634e-16;
const MEV_TO_JOULE: f64 = 1.602176634e-13;
const GEV_TO_JOULE: f64 = 1.602176634e-10;

const KG_TO_GEV: f64 = C * C / GEV_TO_JOULE;
const GEV_TO_KG: f64 = GEV_TO_JOULE / (C * C);
const MEV_TO_KG: f64 = MEV_TO_JOULE / (C * C);

const PROTON_MASS_EMPIRICAL: f64 = 1.67262192369e-27;
const NEUTRON_MASS_EMPIRICAL: f64 = 1.67492749804e-27;
const ELECTRON_MASS_EXACT: f64 = 9.1093837015e-31;
const PION_MASS_CHARGED: f64 = 2.48835417e-28; 

const LAMBDA_QCD_GEV: f64 = 0.217; 

pub struct AdvancedPhysicsEngine {
    pub laws: CosmicLaw,
    pub alpha: f64,
    pub alpha_s_reference: f64,
    pub reference_scale: f64, 
    // cosmic_timeline: Vec<f64>, // Diferentes épocas cosmológicas
}

impl AdvancedPhysicsEngine {
    pub fn new(laws: CosmicLaw) -> Self {
        let alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
        // let cosmic_timeline = vec![...]; //

        Self {
            laws: laws.clone(),
            alpha,
            alpha_s_reference: laws.alpha_s,
            reference_scale: 91.1876,  
            // cosmic_timeline,
        }
    }

    pub fn running_alpha_s(&self, mu_gev: f64) -> f64 {
        let lambda_qcd_gev = LAMBDA_QCD_GEV; // 0.217 GeV
        
        // Avoid singularity at Λ_QCD
        if mu_gev <= lambda_qcd_gev * 1.5 {
            return self.alpha_s_reference;
        }
        
        // β₀ para QCD con Nf=3 quarks activos (u,d,s) a escalas ~ 1-2 GeV
        let nf = if mu_gev < 1.3 { 3.0 } else if mu_gev < 4.2 { 4.0 } else { 5.0 };
        let beta_0 = (11.0 * 3.0 - 2.0 * nf) / 3.0; // (11Nc - 2Nf)/3
        
        // Running a 1-loop: más estable
        let log_ratio = (mu_gev / self.reference_scale).ln();
        let denominator = 1.0 + (beta_0 * self.alpha_s_reference / (2.0 * PI)) * log_ratio;
        
        if denominator > 0.1 {
            self.alpha_s_reference / denominator
        } else {
            self.alpha_s_reference // Fallback
        }
    }
    
    /// Calculate constituent quark mass including QCD running and condensates
    pub fn constituent_quark_mass(&self, bare_mass_kg: f64, flavor: &str) -> f64 {
        let bare_mass_gev = bare_mass_kg * KG_TO_GEV;
        
        // Typical hadronic scale where constituent masses are defined
        let mu_hadronic = 1.0; // GeV
        let alpha_s_had = self.running_alpha_s(mu_hadronic);

        let up_down_asymmetry = 0.5; // ¡Este es el dial que vamos a girar!
        
        match flavor {
            "up" => {
            let chiral_mass_gev = 0.310; // ~310 MeV
            let running_correction = 1.0 + 0.1 * alpha_s_had;
            (chiral_mass_gev * running_correction) * GEV_TO_KG
        },
        "down" => {
            let chiral_mass_gev = 0.310 + (0.0025 * up_down_asymmetry); 
            let running_correction = 1.0 + 0.1 * alpha_s_had;
            (chiral_mass_gev * running_correction) * GEV_TO_KG
        },
            "strange" => {
                // Quark extraño: caso intermedio
                let base_mass_gev = 0.48; // ~500 MeV
                let condensate_contribution = 0.1;
                ((base_mass_gev + condensate_contribution) * (1.0 + 0.05 * alpha_s_had)) * GEV_TO_KG
            },
            "charm" => {
                // Quark charm: principalmente masa desnuda + pequeña corrección
                let mass_gev = bare_mass_gev.max(1.27); // masa MS a 2 GeV
                (mass_gev * (1.0 + 0.02 * alpha_s_had)) * GEV_TO_KG
            },
            _ => {
                // Quarks pesados: usar masa desnuda con correcciones mínimas
                bare_mass_kg * (1.0 + 0.01 * alpha_s_had)
            }
        }
    }

    pub fn get_empirical_hadron_masses(&self) -> (f64, f64, f64) {
        (PROTON_MASS_EMPIRICAL, NEUTRON_MASS_EMPIRICAL, PION_MASS_CHARGED)
    }
    /// Calculate realistic QCD binding energy for hadron formation
    pub fn realistic_qcd_binding(&self, _hadron_type: &str) -> f64 {
        // Modelo simplificado basado en la escala de confinamiento.
        // La energía de enlace total de los hadrones ligeros es del orden de unos pocos cientos de MeV.
        // E_enlace ≈ Λ_QCD + correcciones de espín.
        let mu_hadronic = 1.0; // GeV
        let alpha_s_had = self.running_alpha_s(mu_hadronic);

        // La energía de enlace es la energía que se "pierde" para mantener a los quarks confinados.
        // Es una combinación de energía cinética y potencial. Una aproximación robusta es que es
        // proporcional a la propia escala de energía de la QCD.
        let base_binding_gev = LAMBDA_QCD_GEV * 2.0; // ~434 MeV

        // Pequeña corrección dependiente de alpha_s para dar variabilidad
        let binding_gev = base_binding_gev * (1.0 + (alpha_s_had - 0.3) * 0.5);

        // Convertir a kg*c^2 (Joules)
        binding_gev * GEV_TO_JOULE
    }

    /// Calculate effective "bag" radius for hadron (MIT bag model inspired)
    fn calculate_bag_radius(&self, hadron_type: &str) -> f64 {
        let lambda_qcd_j = LAMBDA_QCD_GEV * GEV_TO_JOULE;
        let base_radius = H_BAR * C / lambda_qcd_j;
        
        match hadron_type {
            "proton" | "neutron" => base_radius * 0.9,      // ~0.9 fm
            "pion_charged" | "pion_neutral" => base_radius * 0.6, // Mesones más pequeños
            _ => base_radius,
        }
    }
    
    /// Kinetic energy of confined quarks (uncertainty principle)
    fn calculate_kinetic_energy(&self, quark_masses: &[f64], bag_radius: f64) -> f64 {
        let mut total_kinetic = 0.0;
        
        for &mass in quark_masses {
            // Relativistic kinetic energy: T ≈ ħc/R for massless, T ≈ mc² for heavy
            let momentum_scale = H_BAR * C / bag_radius;
            let rest_energy = mass * C.powi(2);
            
            total_kinetic += ((momentum_scale * C).powi(2) + rest_energy.powi(2)).sqrt();
        }
        
        total_kinetic
    }
    
    /// Coulomb-like one-gluon exchange energy
    fn calculate_coulomb_like_energy(&self, quark_masses: &[f64], alpha_s: f64, bag_radius: f64) -> f64 {
         if quark_masses.len() < 2 { 
            return 0.0; 
        }
        
        // Factor de color para SU(3): C_F = 4/3 para quarks
        let color_factor = 4.0 / 3.0;
        let n_quarks = quark_masses.len() as f64;
        
        // Energía atractiva de intercambio de gluones
        -color_factor * alpha_s * H_BAR * C * n_quarks * (n_quarks - 1.0) / (2.0 * bag_radius)
    }

    fn calculate_confinement_energy(&self, bag_radius: f64, alpha_s: f64) -> f64 {
        // Tensión de cuerda: σ ≈ 1 GeV/fm = 0.16 N
        let string_tension = 0.16; // N
        
        // Para hadrones: E_conf ≈ σ * L donde L ~ radio del bag
        let confinement_length = bag_radius;
        
        // Escaleo con α_s (más débil que lineal)
        let alpha_s_scaling = (alpha_s / 0.3).powf(0.3);
        
        string_tension * confinement_length * alpha_s_scaling
    }
    
    
    /// Linear confinement potential energy
  fn calculate_vacuum_contributions(&self, bag_radius: f64) -> f64 {
        // Constante del bag: B^(1/4) ≈ 200 MeV
        let bag_constant_gev4 = (0.2_f64).powi(4); // (GeV)^4
        let bag_constant_si = bag_constant_gev4 * (GEV_TO_JOULE / (H_BAR * C)).powi(3); // J/m³
        
        // Volumen del bag (esférico)
        let bag_volume = (4.0 / 3.0) * PI * bag_radius.powi(3);
        
        bag_constant_si * bag_volume
    }
    
    pub fn get_theoretical_hadron_masses(&self) -> (f64, f64, f64) {
        let up_const = self.constituent_quark_mass(self.laws.mass_up_quark, "up");
        let down_const = self.constituent_quark_mass(self.laws.mass_down_quark, "down");
        
        // CORRECCIÓN CLAVE: La energía de enlace se RESTA de la suma de las masas.
        let proton_binding_j = self.realistic_qcd_binding("proton");
        let neutron_binding_j = self.realistic_qcd_binding("neutron");
        let pion_binding_j = self.realistic_qcd_binding("pion_charged");

        // Convertimos la energía de enlace a masa (E/c^2)
        let proton_binding_mass = proton_binding_j / C.powi(2);
        let neutron_binding_mass = neutron_binding_j / C.powi(2);
        let pion_binding_mass = pion_binding_j / C.powi(2);

        let mass_proton = 2.0 * up_const + down_const - proton_binding_mass;
        let mass_neutron = up_const + 2.0 * down_const - neutron_binding_mass;
        let mass_pion = up_const + down_const - pion_binding_mass;
        
        (mass_proton.max(0.0), mass_neutron.max(0.0), mass_pion.max(0.0))
    }

    pub fn get_validated_hadron_masses(&self) -> (f64, f64, f64) {
        if cfg!(feature = "empirical_validation") {
            self.get_empirical_hadron_masses()
        } else {
            let theoretical = self.get_theoretical_hadron_masses();
            // Sanity check: si las masas teóricas son muy irrealistas, usar empíricas
            let (m_p, m_n, _) = theoretical;
            if m_p <= 0.0 || m_n <= m_p || m_p > 2e-26 || m_n > 2e-26 {
                self.get_empirical_hadron_masses()
            } else {
                theoretical
            }
        }
    }

    pub fn cosmological_viability(&self) -> f64 {
    // Esta función debe filtrar solo los casos más extremos.
    
    // 1. Evitar un Big Crunch instantáneo. La gravedad no puede ser tan alta.
    // Usamos una aproximación del tiempo de caída libre.
    let density_approx = 1e-26; // Densidad aproximada tras la inflación
    let freefall_time = (1.0 / (self.laws.g * density_approx)).sqrt();
    
    // Si el universo colapsa en menos de 100 millones de años, es inviable.
    if freefall_time < (1e8 * 31557600.0) {
        return 0.0;
    }

    // 2. Evitar una expansión desbocada. La energía oscura no puede ser muy alta.
    // Usamos el tiempo de Hubble como escala.
    let hubble_time_approx = (3.0 / (8.0 * PI * self.laws.g * self.laws.dark_energy_density)).sqrt();
    
    // Si la escala de tiempo de expansión es menor que 100 millones de años, no se formarán galaxias.
    if hubble_time_approx < (1e8 * 31557600.0) {
        return 0.0;
    }
    
    // Si el universo sobrevive a estos filtros extremos, lo consideramos cosmológicamente viable.
    1.0
}
    pub fn nuclear_cross_section(&self, reaction_type: &str) -> f64 {
        let (m_proton, _, _) = self.get_validated_hadron_masses();
        let thermal_energy = K_B * 1e9; // BBN temperature ~1 GK

        match reaction_type {
            "p_p_fusion" => {
                let reduced_mass = m_proton / 2.0;
                let gamow_energy = 2.0 * self.alpha * (reduced_mass * thermal_energy / 2.0).sqrt();
                let tunnel_probability = (-gamow_energy / thermal_energy).exp();
                tunnel_probability * 1e-45
            }
            "d_p_fusion" => {
                let deuteron_mass = m_proton * 2.0; // Aproximación
                let reduced_mass = m_proton * deuteron_mass / (m_proton + deuteron_mass);
                let gamow_energy = 2.0 * self.alpha * (reduced_mass * thermal_energy / 2.0).sqrt();
                (-gamow_energy / thermal_energy).exp() * 1e-42
            }
            _ => 0.0,
        }
    }

    pub fn primordial_nucleosynthesis_success(&self) -> f64 {
        let (m_proton, m_neutron, _) = self.get_validated_hadron_masses();
        let mass_diff = m_neutron - m_proton;

        // Diferencia correcta para BBN exitosa: Δm = 1.29332 MeV
        let target_diff = 2.305e-30; // kg
        if mass_diff <= 0.0 {
            return 0.0;
        }

        let pp_cross_section = self.nuclear_cross_section("p_p_fusion");
        let dp_cross_section = self.nuclear_cross_section("d_p_fusion");

        let pp_viability = if pp_cross_section > 1e-50 { 1.0 } else { 0.0 };
        let dp_viability = if dp_cross_section > 1e-47 { 1.0 } else { 0.0 };

        let mass_diff_score = (-((mass_diff - target_diff) / target_diff).powi(2) / 0.01).exp();

        pp_viability * dp_viability * mass_diff_score
    }


    pub fn calculate_jeans_mass(&self) -> f64 {
        let temperature = 20.0; // K - nubes moleculares frías
        let (m_proton, _, _) = self.get_validated_hadron_masses();
        let sound_speed = (K_B * temperature / m_proton).sqrt();
        let density: f64 = 1e-18; // kg/m³ - densidad típica de nubes moleculares

        // Masa de Jeans: M_J = (π^5/6)^(1/2) * (c_s^3)/(G^(3/2) * ρ^(1/2))
        (PI.powi(5) / 6.0).sqrt() * sound_speed.powi(3) / (self.laws.g.powf(1.5) * density.sqrt())
    }

    pub fn main_sequence_lifetime(&self, stellar_mass: f64) -> f64 {
        if stellar_mass <= 0.0 {
            return 0.0;
        }
        
        let mass_ratio = stellar_mass / M_SOLAR;
        
        let solar_lifetime_years = 10e9; 
    
    let lifetime = solar_lifetime_years * mass_ratio.powf(-2.5);
    
    // Convertir de años a segundos para el resto de la lógica del engine
    lifetime * 31557600.0
    }

    pub fn chandrasekhar_mass(&self) -> f64 {
        let (m_proton, _, _) = self.get_validated_hadron_masses();
        if m_proton <= 0.0 || self.laws.g <= 0.0 {
            return 0.0;
        }

        // Masa de Chandrasekhar: M_Ch ≈ 1.4 M_☉
        let mu_e: f64 = 2.0; // Peso molecular promedio por electrón (para He/C/O)
        let fundamental_scale = (H_BAR * C / self.laws.g).powf(1.5) / m_proton.powi(2);
        
        fundamental_scale / (mu_e.powi(2) * (2.0_f64).sqrt())
    }

    pub fn stellar_formation_epoch(&self) -> f64 {
        let jeans_mass = self.calculate_jeans_mass();
        if jeans_mass <= 0.0 {
            return 0.0;
        }
        
        let stellar_lifetime = self.main_sequence_lifetime(jeans_mass);

        // Las estrellas deben vivir lo suficiente para procesos importantes
        let minimum_lifetime = 1e6 * 365.25 * 24.0 * 3600.0; // 1 Myr
        let maximum_lifetime = 1e11 * 365.25 * 24.0 * 3600.0; // 100 Gyr
        
        if stellar_lifetime < minimum_lifetime {
            return 0.0;
        }
        
        if stellar_lifetime > maximum_lifetime {
            return (maximum_lifetime / stellar_lifetime).sqrt();
        }

        1.0
    }
    pub fn heavy_element_creation(&self) -> f64 {
    // La temperatura del núcleo de una gigante roja es de unos 100-200 millones K
    let core_temp = 1.5e8; // 150 MK
    let thermal_energy_joules = K_B * core_temp;

    // La tasa de la reacción triple-alfa es sensible a la energía de la resonancia de Hoyle.
    // E_res = 379 keV por encima de la masa de 3 partículas alfa (helio-4).
    let hoyle_resonance_offset_joules = 379e3 * 1.60218e-19; // 379 keV en Joules

    // La energía de Gamow describe la probabilidad de túnel a través de la barrera de Coulomb.
    // Para la fusión He+Be, que es el paso intermedio, es el factor dominante.
    let reduced_mass = (4.0 * 8.0) / (4.0 + 8.0) * PROTON_MASS_EMPIRICAL; // Masa reducida de He-4 y Be-8
    let coulomb_barrier_energy = (2.0 * 4.0 * self.laws.e.powi(2)) / (4.0 * PI * EPSILON_0 * 1e-14); // r ~ 10 fm
    
    // El factor de Gamow. Una aproximación.
    let gamow_factor = (coulomb_barrier_energy / thermal_energy_joules).sqrt();
    let tunnel_probability = (-3.0 * gamow_factor / 2.0).exp();

    // La tasa de reacción depende fuertemente de qué tan cerca está la energía térmica de la resonancia.
    // Usamos una distribución de Maxwell-Boltzmann para la energía de las partículas.
    let resonance_probability = (hoyle_resonance_offset_joules / thermal_energy_joules) * (-hoyle_resonance_offset_joules / thermal_energy_joules).exp();

    // El score final es una combinación de la probabilidad de túnel y la de resonancia.
    // Normalizamos para que un universo como el nuestro dé un score alto.
    let reaction_rate_score = (tunnel_probability * resonance_probability) * 1e18; // Factor de escala empírico

    // La fuerza fuerte también debe estar en un rango muy preciso para que la resonancia exista.
    let alpha_s_optimal = 0.118;
    let alpha_s_score = (-((self.laws.alpha_s - alpha_s_optimal).powi(2) / (alpha_s_optimal * 0.05).powi(2))).exp();
    
    (reaction_rate_score * alpha_s_score).min(1.0)
}

    pub fn black_hole_formation_potential(&self) -> f64 {
        let chandrasekhar = self.chandrasekhar_mass();
        if chandrasekhar <= 0.0 {
            return 0.0;
        }
        
        // Límite de Tolman-Oppenheimer-Volkoff ≈ 2-3 M_Ch
        let tov_limit = chandrasekhar * 2.5;
        
        // Estrellas masivas típicas que pueden formar agujeros negros
        let solar_mass = 1.989e30;
        let typical_massive_star = 25.0 * solar_mass;

        if tov_limit < typical_massive_star {
            1.0 // Los agujeros negros se forman fácilmente
        } else {
            (typical_massive_star / tov_limit).powf(1.5)
        }
    }
}

// CORRECCIÓN 3: Ajustar calculate_enhanced_fitness para usar la nueva tupla
pub fn calculate_enhanced_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = AdvancedPhysicsEngine::new(laws.clone());
    
    let (mass_proton, mass_neutron, _mass_pion) = engine.get_validated_hadron_masses();

    // Verificación fundamental: protón debe ser más ligero que neutrón
    if mass_proton >= mass_neutron || mass_proton + laws.mass_electron >= mass_neutron {
        return (0.0, 0);
    }

    let mut fitness = 0.0;
    let mut complexity_level = 0;

    // NIVEL 0: Viabilidad Cosmológica (peso: 0.15)
    let cosmic_viability = engine.cosmological_viability();
    if cosmic_viability < 0.1 {
        return (0.0, 0);
    }
    fitness += 0.15 * cosmic_viability;

    // NIVEL 1: Nucleosíntesis Primordial (peso: 0.25)
    let nucleosynthesis_score = engine.primordial_nucleosynthesis_success();
    fitness += 0.25 * nucleosynthesis_score;

    if nucleosynthesis_score > 0.3 {
        complexity_level = 1;

        // NIVEL 2: Formación Estelar (peso: 0.25)
        let stellar_score = engine.stellar_formation_epoch();
        fitness += 0.25 * stellar_score;

        if stellar_score > 0.4 {
            complexity_level = 2;

            // NIVEL 3: Elementos Pesados (peso: 0.20)
            let heavy_elements_score = engine.heavy_element_creation();
            fitness += 0.20 * heavy_elements_score;

            if heavy_elements_score > 0.5 {
                complexity_level = 3;

                // NIVEL 4: Agujeros Negros (peso: 0.15)
                let black_hole_score = engine.black_hole_formation_potential();
                fitness += 0.15 * black_hole_score;

                if black_hole_score > 0.6 {
                    complexity_level = 4;

                    // NIVEL 5: Potencial Reproductivo (bonus)
                    if black_hole_score > 0.8 && heavy_elements_score > 0.7 {
                        complexity_level = 5;
                        fitness += 0.05; // Bonus pequeño
                    }
                }
            }
        }
    }

    (fitness.min(1.0), complexity_level)
}
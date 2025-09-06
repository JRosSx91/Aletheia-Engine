use crate::core::models::CosmicLaw;
use crate::physics::constants::*;
use std::f64::consts::PI;

const KG_TO_GEV: f64 = C * C / (1.602176634e-19 * 1e9); // c²/(1 GeV en Joules)
const GEV_TO_KG: f64 = 1.602176634e-19 * 1e9 / (C * C); // Inversa

// CORRECCIÓN 2: Masas hadrónicas empíricas para calibración
const PROTON_MASS_EMPIRICAL: f64 = 1.6726219e-27; // kg
const NEUTRON_MASS_EMPIRICAL: f64 = 1.67492749804e-27; // kg

pub struct AdvancedPhysicsEngine {
    pub laws: CosmicLaw,
    pub alpha: f64,
    // cosmic_timeline: Vec<f64>, // Diferentes épocas cosmológicas
}

impl AdvancedPhysicsEngine {
    pub fn new(laws: CosmicLaw) -> Self {
        let alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
        // let cosmic_timeline = vec![...]; //

        Self {
            laws,
            alpha,
            // cosmic_timeline,
        }
    }

    pub fn running_alpha_s(&self, mu_gev: f64) -> f64 {
        let lambda_qcd_gev = LAMBDA_QCD; // 0.217 GeV
        
        // Avoid singularity at Λ_QCD
        if mu_gev <= lambda_qcd_gev * 1.1 {
            return self.laws.alpha_s; // Use input value in non-perturbative regime
        }
        
        // 1-loop beta function: β₀ = (11Nc - 2Nf)/3 = (33 - 12)/3 = 7 for Nf=3
        let beta_0 = 7.0;
        
        // 1-loop running: α_s(μ) = α_s(μ₀) / [1 + (β₀ α_s(μ₀) / 2π) ln(μ/μ₀)]
        let reference_mu = 2.0; // GeV, typical hadronic scale
        let alpha_s_ref = self.laws.alpha_s;
        
        let denominator = 1.0 + (beta_0 * alpha_s_ref / (2.0 * PI)) * 
                               (mu_gev / reference_mu).ln();
        
        if denominator > 0.0 {
            alpha_s_ref / denominator
        } else {
            alpha_s_ref // Fallback for unphysical parameters
        }
    }
    
    /// Calculate constituent quark mass including QCD running and condensates
    pub fn constituent_quark_mass(&self, bare_mass_kg: f64, flavor: &str) -> f64 {
        let bare_mass_gev = bare_mass_kg * KG_TO_GEV;
        
        // Typical hadronic scale where constituent masses are defined
        let mu_hadronic = 1.0; // GeV
        let alpha_s_had = self.running_alpha_s(mu_hadronic);
        
        // Anomalous dimension for quark mass (1-loop): γ_m = 6 C_F = 6 * 4/3 = 8
        let gamma_m = 8.0;
        let mass_running_factor = 1.0 + (gamma_m * alpha_s_had) / (4.0 * PI);
        
        // QCD condensate contribution (non-perturbative)
        let condensate_contribution_gev = match flavor {
            "up" | "down" => {
                // Light quarks get most of their mass from chiral symmetry breaking
                let chiral_condensate = 0.25; // GeV, typical <qq̄> scale
                let current_mass = bare_mass_gev * mass_running_factor;
                current_mass + chiral_condensate + 0.05 // Ajuste empírico
            },
            "strange" => {
                // Strange quark: intermediate case
                let current_mass = bare_mass_gev * mass_running_factor;
                current_mass + 0.15 * (alpha_s_had / 0.3).powf(0.3)
            },
            _ => {
                // Heavy quarks (charm, bottom, top): mostly bare mass
                bare_mass_gev * mass_running_factor
            }
        };
        
        condensate_contribution_gev * GEV_TO_KG
    }

    pub fn get_empirical_hadron_masses(&self) -> (f64, f64, f64) {
             let mass_proton = PROTON_MASS_EMPIRICAL;
        let mass_neutron = NEUTRON_MASS_EMPIRICAL;
        let mass_pion = 139.6e6 * GEV_TO_KG / 1e6; // 139.6 MeV en kg
        (mass_proton, mass_neutron, mass_pion)
    }
    
    /// Calculate realistic QCD binding energy for hadron formation
    pub fn realistic_qcd_binding(&self, quark_masses: &[f64], hadron_type: &str) -> f64 {
        if quark_masses.is_empty() {
            return 0.0;
        }        
        let mu_hadronic = 1.0; // GeV, confinement scale
        let alpha_s_conf = self.running_alpha_s(mu_hadronic);
      
        // 1. Kinetic energy (quark motion in bag)
        let bag_radius = self.calculate_bag_radius(hadron_type);
        let kinetic_energy = self.calculate_kinetic_energy(quark_masses, bag_radius);
        
        // 2. One-gluon exchange (perturbative)
        let coulomb_energy = self.calculate_coulomb_like_energy(quark_masses, alpha_s_conf, bag_radius);
        
        // 3. Confinement potential (non-perturbative)
        let confinement_energy = self.calculate_confinement_energy(bag_radius, alpha_s_conf);
        
        // 4. Gluon self-energy and vacuum contributions
        let gluon_energy = self.calculate_gluon_contributions(alpha_s_conf, bag_radius);

        let total_binding = kinetic_energy + coulomb_energy + confinement_energy + gluon_energy;
        
        total_binding.max(0.0)
    }

    /// Calculate effective "bag" radius for hadron (MIT bag model inspired)
    fn calculate_bag_radius(&self, hadron_type: &str) -> f64 {
        let base_radius = H_BAR * C / (LAMBDA_QCD * 1.783e-36 * C.powi(2)); // ~ 1 fm
        
        match hadron_type {
            "proton" | "neutron" => base_radius,
            "pion_charged" | "pion_neutral" => base_radius * 0.7, // Mesons smaller
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
            
            if momentum_scale * C > rest_energy {
                // Relativistic regime
                total_kinetic += momentum_scale * C;
            } else {
                // Non-relativistic: T = p²/2m with p ~ ħ/R
                total_kinetic += momentum_scale.powi(2) / (2.0 * mass);
            }
        }
        
        total_kinetic
    }
    
    /// Coulomb-like one-gluon exchange energy
    fn calculate_coulomb_like_energy(&self, quark_masses: &[f64], alpha_s: f64, bag_radius: f64) -> f64 {
         if quark_masses.len() < 2 { return 0.0; }
        let color_factor = 4.0 / 3.0;
        let num_pairs = (quark_masses.len() * (quark_masses.len() - 1)) / 2;
        - (num_pairs as f64) * color_factor * alpha_s * H_BAR * C / bag_radius
    }
    
    /// Linear confinement potential energy
     fn calculate_confinement_energy(&self, bag_radius: f64, alpha_s: f64) -> f64 {
        let string_tension_gev_fm = 1.0; // ~1 GeV/fm
        let string_tension_si = string_tension_gev_fm * (1.602e-19 * 1e9) / 1e-15; // N
        string_tension_si * bag_radius * (alpha_s / 0.3).powf(0.5)
    }
    
    /// Gluon vacuum energy and self-interactions
    fn calculate_gluon_contributions(&self, alpha_s: f64, bag_radius: f64) -> f64 {
        let bag_constant_gev_fm3 = 0.2; // ~200 MeV/fm³
        let bag_constant_si = bag_constant_gev_fm3 * (1.602e-19 * 1e9) / (1e-15_f64).powi(3); // J/m³
        let bag_volume = (4.0 / 3.0) * PI * bag_radius.powi(3);
        let vacuum_energy = bag_constant_si * bag_volume;
        let gluon_kinetic = 8.0 * H_BAR * C / bag_radius;
        let self_interaction = alpha_s.powi(2) * gluon_kinetic / (4.0 * PI);
        vacuum_energy + gluon_kinetic + self_interaction
    }
    
    /// Get improved realistic hadron masses using sophisticated QCD binding
    pub fn get_theoretical_hadron_masses(&self) -> (f64, f64, f64) {
        let up_constituent = self.constituent_quark_mass(self.laws.mass_up_quark, "up");
        let down_constituent = self.constituent_quark_mass(self.laws.mass_down_quark, "down");
        
        let proton_binding = self.realistic_qcd_binding(&[up_constituent, up_constituent, down_constituent], "proton");
        let neutron_binding = self.realistic_qcd_binding(&[up_constituent, down_constituent, down_constituent], "neutron");
        let pion_binding = self.realistic_qcd_binding(&[up_constituent, down_constituent], "pion_charged");
        
        let mass_proton = 2.0 * up_constituent + down_constituent - proton_binding;
        let mass_neutron = up_constituent + 2.0 * down_constituent - neutron_binding;
        let mass_pion = up_constituent + down_constituent - pion_binding;
        
        (mass_proton, mass_neutron, mass_pion)
    }

     pub fn get_validated_hadron_masses(&self) -> (f64, f64, f64) {
        if cfg!(feature = "empirical_validation") {
            self.get_empirical_hadron_masses()
        } else {
            self.get_theoretical_hadron_masses()
        }
    }

    pub fn cosmological_viability(&self) -> f64 {
        let matter_density = 2.47e-27; // kg/m³ (densidad de materia observada, Ω_m * ρ_crit)
        let total_density = matter_density + self.laws.dark_energy_density;
        
        let h_squared = 8.0 * PI * self.laws.g * total_density / 3.0;
        if h_squared <= 0.0 { return 0.0; }
        
        let hubble_parameter = h_squared.sqrt();
        
        // Esta es una aproximación. Una mejor sería integrar la ecuación de Friedmann.
        // Pero para un universo con materia y energía oscura, 1/H es un buen orden de magnitud.
        let age_universe = 1.0 / hubble_parameter;
        
        let age_observed = HUBBLE_TIME; // 4.35e17 s
        
        // Permitimos un margen de error de un factor de 3 (más joven o más viejo).
        let age_ratio = age_universe / age_observed;
        let age_score = if age_ratio > 0.33 && age_ratio < 3.0 {
            1.0 - (age_ratio.ln()).abs() / 3.0_f64.ln()
        } else {
            0.0
        };
        age_score.max(0.0)
    }

    pub fn nuclear_cross_section(&self, reaction_type: &str) -> f64 {
        let (m_proton, _, _) = self.get_validated_hadron_masses(); // Usar tupla de 3
        let thermal_energy = K_B * 1e9; // BBN temperature ~1 GK

        match reaction_type {
            "p_p_fusion" => {
                let gamow_energy = 2.0 * PI * self.alpha * (m_proton * thermal_energy / 2.0).sqrt();
                let tunnel_probability = (-gamow_energy / thermal_energy).exp();
                tunnel_probability * 1e-45 // Typical nuclear cross section
            }
            "d_p_fusion" => {
                // Deuterium burning - much easier than p-p
                let reduced_mass = m_proton * 2.0 * m_proton / (m_proton + 2.0 * m_proton);
                let gamow_energy =
                    2.0 * PI * self.alpha * (reduced_mass * thermal_energy / 2.0).sqrt();
                (-gamow_energy / thermal_energy).exp() * 1e-42
            }
            _ => 0.0,
        }
    }

    pub fn primordial_nucleosynthesis_success(&self) -> f64 {
        let (m_proton, m_neutron, _) = self.get_validated_hadron_masses(); // Usar tupla de 3
        let mass_diff = m_neutron - m_proton;

        // Must have correct mass difference for BBN
        let target_diff = 2.305e-30; // ~1.29 MeV
        if mass_diff <= 0.0 {
            return 0.0;
        }

        let pp_cross_section = self.nuclear_cross_section("p_p_fusion");
        let dp_cross_section = self.nuclear_cross_section("d_p_fusion");

        // BBN success requires both reactions to be viable
        let pp_viability = if pp_cross_section > 1e-50 { 1.0 } else { 0.0 };
        let dp_viability = if dp_cross_section > 1e-47 { 1.0 } else { 0.0 };

        let mass_diff_score = (-((mass_diff - target_diff) / target_diff).powi(2) / 0.02).exp();

        pp_viability * dp_viability * mass_diff_score
    }

    pub fn calculate_jeans_mass(&self) -> f64 {
        let temperature = 100.0; // Typical molecular cloud temperature in K
        let (m_proton, _m_neutron, _m_pion) = self.get_validated_hadron_masses(); // Usar tupla de 3
        let sound_speed = (K_B * temperature / m_proton).sqrt();
        let density = 1e-18; // Typical molecular cloud density kg/m³

        (PI * sound_speed.powi(3) / (self.laws.g * density).sqrt()).powf(1.5)
    }

    pub fn main_sequence_lifetime(&self, stellar_mass: f64) -> f64 {
        let luminosity = (stellar_mass / M_SOLAR).powf(3.5);
        let fuel_mass = 0.1 * stellar_mass; // ~10% of mass available for fusion
        let energy_per_kg = 6.4e14; // J/kg for H->He fusion

        (fuel_mass * energy_per_kg) / (luminosity * 3.828e26) // Solar luminosity
    }

    pub fn chandrasekhar_mass(&self) -> f64 {
        let (m_proton, _, _) = self.get_validated_hadron_masses(); // Usar tupla de 3
        if m_proton <= 0.0 || self.laws.g <= 0.0 {
            return 0.0;
        }

        let mu_e = 2.0; // Mean molecular weight per electron
        (H_BAR * C / self.laws.g).powf(1.5) / (m_proton * mu_e).powi(2)
    }

    pub fn stellar_formation_epoch(&self) -> f64 {
        let jeans_mass = self.calculate_jeans_mass();
        let stellar_lifetime = self.main_sequence_lifetime(jeans_mass);

        // Stars must live long enough to synthesize heavy elements
        let minimum_lifetime = 1e6 * 365.25 * 24.0 * 3600.0; // 1 Myr in seconds
        if stellar_lifetime < minimum_lifetime {
            return 0.0;
        }

        // But not so long that they never evolve
        let maximum_lifetime: f64 = 1e11 * 365.25 * 24.0 * 3600.0; // 100 Gyr
        if stellar_lifetime > maximum_lifetime {
            return (maximum_lifetime / stellar_lifetime).sqrt();
        }

        1.0
    }

    pub fn heavy_element_creation(&self) -> f64 {
        let stellar_core_temp = 1e8; // 100 million K - carbon burning temperature
        let thermal_energy = K_B * stellar_core_temp;

        // Carbon formation via triple-alpha process
        let (he4_mass, _neutron_mass, _pion_mass) = self.get_validated_hadron_masses();
        let _he4_approx = 4.0 * he4_mass; // Approximation for He-4
        let alpha_particle_energy = 3.0 * thermal_energy;

        // Hoyle resonance - requires fine-tuning of nuclear forces
        let resonance_energy = 7.65e6 * 1.602e-19; // 7.65 MeV in Joules
        let energy_match =
            (-((alpha_particle_energy - resonance_energy) / resonance_energy).powi(2) / 0.01).exp();

        // Also needs strong force to be in right range
        let alpha_s_optimal = 0.118;
        let alpha_s_score =
            (-((self.laws.alpha_s - alpha_s_optimal) / alpha_s_optimal).powi(2) / 0.001).exp();

        energy_match * alpha_s_score
    }

    pub fn black_hole_formation_potential(&self) -> f64 {
        let chandrasekhar = self.chandrasekhar_mass();
        let tov_limit = chandrasekhar * 2.16; // Approximate TOV limit

        // Need massive stars to form black holes
        let typical_massive_star = 25.0 * M_SOLAR;

        if tov_limit < typical_massive_star {
            1.0 // Black holes form readily
        } else {
            (typical_massive_star / tov_limit).powf(2.0)
        }
    }
}

// CORRECCIÓN 3: Ajustar calculate_enhanced_fitness para usar la nueva tupla
pub fn calculate_enhanced_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = AdvancedPhysicsEngine::new(laws.clone());
    
    // CAMBIO: Ahora get_improved_hadron_masses retorna (proton, neutron, pion)
    let (mass_proton, mass_neutron, _mass_pion) = engine.get_validated_hadron_masses();

    // Puerta de entrada: Viabilidad atómica básica
    if mass_proton >= mass_neutron || mass_proton + laws.mass_electron >= mass_neutron {
        return (0.0, 0);
    }

    let mut fitness = 0.0;
    let mut complexity_level = 0;

    // NIVEL 0: Viabilidad Cosmológica (peso: 0.1)
    let cosmic_viability = engine.cosmological_viability();
    if cosmic_viability < 0.1 {
        return (0.0, 0);
    }
    fitness += 0.1 * cosmic_viability;

    // [resto del código permanece igual...]
    
    // NIVEL 1: Nucleosíntesis Primordial (peso: 0.2)
    let nucleosynthesis_score = engine.primordial_nucleosynthesis_success();
    fitness += 0.2 * nucleosynthesis_score;

    if nucleosynthesis_score > 0.3 {
        complexity_level = 1;

        // NIVEL 2: Formación Estelar (peso: 0.3)
        let stellar_score = engine.stellar_formation_epoch();
        fitness += 0.3 * stellar_score;

        if stellar_score > 0.4 {
            complexity_level = 2;

            // NIVEL 3: Creación de Elementos Pesados (peso: 0.2)
            let heavy_elements_score = engine.heavy_element_creation();
            fitness += 0.2 * heavy_elements_score;

            if heavy_elements_score > 0.5 {
                complexity_level = 3;

                // NIVEL 4: Potencial de Agujeros Negros (peso: 0.2)
                let black_hole_score = engine.black_hole_formation_potential();
                fitness += 0.2 * black_hole_score;

                if black_hole_score > 0.6 {
                    complexity_level = 4;

                    // NIVEL 5: Reproducción Cósmica (bonus)
                    if black_hole_score > 0.8 && heavy_elements_score > 0.7 {
                        complexity_level = 5;
                        fitness += 0.1; // Bonus for reproductive potential
                    }
                }
            }
        }
    }

    (fitness.min(1.2), complexity_level)
}

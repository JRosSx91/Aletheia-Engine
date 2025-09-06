// --- Constantes Fundamentales (CODATA 2018) ---
pub const C: f64 = 299792458.0; // Velocidad de la luz (m/s)
pub const H_BAR: f64 = 1.054571817e-34; // Constante de Planck reducida (J·s)
pub const EPSILON_0: f64 = 8.8541878128e-12; // Permitividad del vacío (F/m)
pub const K_B: f64 = 1.380649e-23; // Constante de Boltzmann (J/K)
pub const G_GRAVITATIONAL: f64 = 6.67430e-11; // Constante gravitacional (m³·kg⁻¹·s⁻²)
pub const ELEMENTARY_CHARGE: f64 = 1.602176634e-19; // Carga elemental (C)

// --- Masas de Partículas (CODATA 2018 / PDG 2022) ---
pub const ELECTRON_MASS_EXACT: f64 = 9.1093837015e-31; // kg
pub const PROTON_MASS_EMPIRICAL: f64 = 1.67262192369e-27; // kg
pub const NEUTRON_MASS_EMPIRICAL: f64 = 1.67492749804e-27; // kg
pub const PION_MASS_CHARGED: f64 = 2.48835417e-28; // kg (139.57 MeV)

// --- Constantes de Modelo y Conversión ---
pub const LAMBDA_QCD_GEV: f64 = 0.217; // Escala de QCD (GeV)
pub const M_SOLAR: f64 = 1.98847e30; // Masa solar (kg)
pub const MPC_TO_METERS: f64 = 3.08567758e22; // Megaparsec a metros

// --- Factores de Conversión de Energía ---
const JOULE_TO_EV: f64 = 6.242e18;
pub const MEV_TO_JOULE: f64 = 1.602176634e-13;
pub const GEV_TO_JOULE: f64 = 1.602176634e-10;
pub const KG_TO_GEV: f64 = C * C / GEV_TO_JOULE;
pub const MEV_TO_KG: f64 = MEV_TO_JOULE / (C * C);
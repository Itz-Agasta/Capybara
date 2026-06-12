use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub enabled: bool,
    pub stellarium_url: String,
}

#[derive(Debug, Clone)]
pub struct ObserverConfig {
    pub longitude: f64,
    pub latitude: f64,
    pub elevation_km: f64,
}

#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub fov_estimate_deg: f64,
}

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub simulation: SimulationConfig,
    pub observer: ObserverConfig,
    pub solver: SolverConfig,
}

impl EngineConfig {
    pub fn from_env() -> Self {
        todo!("Read env vars: CCE_SIMULATION, CCE_STELLARIUM_HOST, CCE_STELLARIUM_PORT, etc.")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SolveResult {
    pub ra: f64,
    pub dec: f64,
    pub roll: f64,
    pub fov: f64,
    pub stars_matched: u32,
    pub confidence: f64,
    pub solve_time_ms: f64,
    pub backend: String,
}

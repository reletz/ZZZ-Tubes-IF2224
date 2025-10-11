use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct DfaConfig {
    pub start_state: String,
    pub states: HashMap<String, State>,
}

#[derive(Deserialize, Debug)]
pub struct State {
    #[serde(default)]
    pub is_final: bool,

    pub token_type: Option<String>,

    #[serde(default)]
    pub transitions: Vec<Transition>,
}

#[derive(Deserialize, Debug)]
pub struct Transition {
    pub input: String,
    pub next_state: String,
}

/// Parsing file konfigurasi DFA.
///
/// # Arguments
///
/// * `file_path` - Path `dfa.json`.
///
/// # Returns
///
/// * `Result<DfaConfig, Box<dyn std::error::Error>>` - 
///    Returns `DfaConfig` jika berhasil,
///    atau error kalau file gaada/JSON ga valid.
pub fn load_dfa_config(file_path: &str) -> Result<DfaConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(Path::new(file_path))?;
    let config: DfaConfig = serde_json::from_str(&content)?;

    Ok(config)
}

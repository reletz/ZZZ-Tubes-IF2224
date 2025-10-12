use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct DfaConfig {
    pub start_state: String,
    pub states: HashMap<String, State>,

    #[serde(default)]
    pub reserved_words: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct State {
    #[serde(default)]
    pub is_final: bool,

    pub token_type: Option<String>,

    #[serde(default)]
    pub transitions: Vec<Transition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transition {
    pub input: String,
    pub next_state: String,
}

impl DfaConfig {
    /// Simulasi DFA
    /// 
    /// # Arguments
    /// * `chars` - Peekable iterator over characters
    /// * `char_categorizer` - Yang golongin letter, digit
    /// * `transition_matcher` - 
    /// 
    /// # Returns
    /// * `Option<(String, String)>` - Final state and matched lexeme
    pub fn dfa<I, F, M>(&self, 
        chars: &mut std::iter::Peekable<I>, 
        _char_categorizer: F, 
        transition_matcher: M
    ) -> Option<(String, String)>
    where
        I: Iterator<Item = char> + Clone,
        F: Fn(char) -> String,
        M: Fn(&str, char) -> bool,
    {
        let mut current_state_name = self.start_state.clone();
        let mut current_lexeme = String::new();
        let mut last_final_state: Option<String> = None;
        let mut lexeme_at_last_final = String::new();
        let mut temp_chars = chars.clone();

        loop {
            let next_char = match temp_chars.peek() {
                Some(c) => *c,
                None => break,
            };

            let current_state = self.states.get(&current_state_name)
                .expect("State DFA tidak valid ditemukan.");
            
            let mut found_transition = false;

            // Transisinya cocok ga
            for transition in &current_state.transitions {
                if transition_matcher(&transition.input, next_char) {
                    current_lexeme.push(temp_chars.next().unwrap());
                    current_state_name = transition.next_state.clone();
                    found_transition = true;

                    // Cek state barunya final apa engga
                    if let Some(new_state) = self.states.get(&current_state_name) {
                        if new_state.is_final {
                            last_final_state = Some(current_state_name.clone());
                            lexeme_at_last_final = current_lexeme.clone();
                        }
                    }
                    break;
                }
            }

            if !found_transition {
                break;
            }
        }

        last_final_state.map(|state| (state, lexeme_at_last_final))
    }
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

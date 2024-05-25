use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct SwitchboardConfig {
    pub max_sessions_per_agent: usize,
    pub max_agents: Option<usize>,
}
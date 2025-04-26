use anyhow::{Result as AnyResult, anyhow};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn from_string(stage: &str) -> AnyResult<Self> {
        match stage {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow!("Unknown stage")),
        }
    }
}

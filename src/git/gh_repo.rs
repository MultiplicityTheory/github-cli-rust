use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhRepo {
    pub owner: String,
    pub name: String,
    pub hostname: String,
}

impl GhRepo {
    pub fn new(owner: &str, name: &str, hostname: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
            hostname: hostname.to_string(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }
}

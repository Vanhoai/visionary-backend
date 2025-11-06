use shared::models::failure::Failure;

pub enum Role {
    Admin,
    Normal,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Role::Admin => "ADMIN",
            Role::Normal => "NORMAL",
        }
    }

    pub fn from_str(role_str: &str) -> Result<Self, Failure> {
        match role_str.to_uppercase().as_str() {
            "ADMIN" => Ok(Role::Admin),
            "NORMAL" => Ok(Role::Normal),
            _ => Err(Failure::ValidationError(format!("Unknown role: {}", role_str))),
        }
    }
}

use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

/// A universally unique identifier(UUID) for domain identifier
///
/// This wrapper ensures type safety, preventing different entity IDs
/// form being accidentally swapped. By default, it uses **UUID v7**
/// (random) for new instances.
///
/// # Examples
///
/// '''
/// let id = Id::generate();
/// '''
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(Uuid);

#[derive(Debug, Error)]
pub enum IdError {
    #[error("Invalid value: {0}")]
    InvalidValue(String),
}

impl Id {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Id {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map_err(|e| IdError::InvalidValue(e.to_string()))
            .map(Id)
    }
}

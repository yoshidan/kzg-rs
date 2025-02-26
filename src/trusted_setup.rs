use crate::{enums::KzgError, NUM_G1_POINTS, NUM_ROOTS_OF_UNITY};

use alloc::sync::Arc;
use core::{
    hash::{Hash, Hasher},
};


pub fn get_kzg_settings() -> KzgSettings {
    KzgSettings {
    }
}

static DEFAULT_KZG_SETTINGS: KzgSettings = KzgSettings {
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KzgSettings {
}

#[derive(Debug, Clone, Default, Eq)]
pub enum EnvKzgSettings {
    #[default]
    Default,
    Custom(Arc<KzgSettings>),
}

impl PartialEq for EnvKzgSettings {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Default, Self::Default) => true,
            (Self::Custom(a), Self::Custom(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Hash for EnvKzgSettings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Default => {}
            Self::Custom(settings) => Arc::as_ptr(settings).hash(state),
        }
    }
}

impl EnvKzgSettings {
    pub fn get(&self) -> &KzgSettings {
        match self {
            Self::Default => {
                &DEFAULT_KZG_SETTINGS
            }
            Self::Custom(settings) => settings,
        }
    }
}

impl KzgSettings {
    pub fn load_trusted_setup_file() -> Result<Self, KzgError> {
        Ok(get_kzg_settings())
    }
}

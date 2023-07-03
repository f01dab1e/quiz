use stdx::Result;

use crate::toml;

type Symbol = Box<str>;

/// Field
#[derive(Debug)]
pub struct Field<T> {
    value: T,
    kind: FieldKind,
}

impl<T> Field<T> {
    fn set(&mut self, value: T) {
        self.value = value;
        self.kind = FieldKind::User;
    }

    fn inherit(name: &str, value: T) -> Self {
        Self { value, kind: FieldKind::Inherits(name.into()) }
    }

    /// Returns the kind of the field.
    pub fn kind(&self) -> &FieldKind {
        &self.kind
    }

    /// Returns a reference to the value of the field.
    pub fn value(&self) -> &T {
        &self.value
    }
}

/// Represents the kind of a field.
#[derive(Debug, Clone)]
pub enum FieldKind {
    /// Represents a user-defined field.
    User,
    /// Represents a field that inherits from a symbol (name).
    Inherits(Symbol),
}

impl std::fmt::Display for FieldKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => f.write_str("user-defined"),
            Self::Inherits(name) => f.write_str(name),
        }
    }
}

/// `toml::Config::from_home_dir`
#[derive(Debug)]
pub struct Config {
    /// `toml::Config`
    pub theme: Field<String>,
}

impl Default for Config {
    fn default() -> Self {
        let profile: &str = "default";
        Self { theme: Field::inherit(profile, "GitHub".into()) }
    }
}

impl Config {
    /// `toml::Config::from_home_dir`
    pub fn from_home_dir() -> Result<Self> {
        toml::Config::from_home_dir().map(Self::from_toml)
    }

    pub(crate) fn from_toml(toml: toml::Config) -> Self {
        let mut config = Self::default();

        if let Some(theme) = toml.theme {
            config.theme.set(theme);
        }

        config
    }
}

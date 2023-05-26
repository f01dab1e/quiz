use crate::{toml, Result};

pub(crate) type Symbol = Box<str>;

pub(crate) struct Field<T> {
    pub(crate) value: T,
    pub(crate) kind: FieldKind,
}

impl<T> std::ops::Deref for Field<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Field<T> {
    fn set(&mut self, value: T) {
        self.value = value;
        self.kind = FieldKind::User;
    }

    fn inherit(name: &str, value: T) -> Self {
        Self { value, kind: FieldKind::Inherits(name.into()) }
    }
}

pub(crate) enum FieldKind {
    User,
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

pub(crate) struct Config {
    pub(crate) theme: Field<String>,
}

impl Default for Config {
    fn default() -> Self {
        let profile: &str = "default";
        Self { theme: Field::inherit(profile, "GitHub".into()) }
    }
}

impl Config {
    pub(crate) fn from_home_dir() -> Result<Self> {
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

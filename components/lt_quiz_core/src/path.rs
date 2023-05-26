use std::path::PathBuf;

pub fn app() -> PathBuf {
    let config = home::home_dir().unwrap_or_default().join(concat!(".", env!("CARGO_PKG_NAME")));
    let _ = std::fs::create_dir(&config);
    config
}

pub fn config() -> PathBuf {
    app().join("config.toml")
}

pub fn db() -> PathBuf {
    app().join("db")
}

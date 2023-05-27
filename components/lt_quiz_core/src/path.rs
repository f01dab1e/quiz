use std::path::PathBuf;

/// Returns the path to the application directory.
///
/// This function retrieves the home directory using the `home::home_dir`
/// function, and appends the application name (obtained from the
/// `CARGO_PKG_NAME` environment variable) to it. If the home directory cannot
/// be determined, a default path is used. The function creates the directory if
/// it does not exist and returns the path.
pub fn app() -> PathBuf {
    let config = home::home_dir().unwrap_or_default().join(concat!(".", env!("CARGO_PKG_NAME")));
    let _ = std::fs::create_dir(&config);
    config
}

/// Returns the path to the configuration file.
///
/// This function calls the `app` function to get the application directory and
/// appends "config.toml" to it. It returns the resulting `PathBuf`.
pub fn config() -> PathBuf {
    app().join("config.toml")
}

/// Returns the path to the database directory.
///
/// This function calls the `app` function to get the application directory and
/// appends "db" to it. It returns the resulting `PathBuf`.
pub fn db() -> PathBuf {
    app().join("db")
}

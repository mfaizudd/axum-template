use std::path::Path;

use anyhow::Result;
use config::{Config, Environment, File};
use directories::ProjectDirs;
use secrecy::Secret;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
}

#[derive(Deserialize)]
pub struct OauthSettings {
    pub issuer: String,
    pub audience: String,
    pub jwks_url: String,
    pub userinfo_url: String,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub database: String,
}

#[derive(Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub oauth: OauthSettings,
}

pub fn get_config(environment: &str) -> Result<Settings> {
    // TODO: Change config directory
    let base_path = ProjectDirs::from("com", "example", "app")
        .and_then(|dirs| {
            let config_dir = dirs.config_dir();
            if config_dir.exists() {
                config_dir.to_str().map(|path| path.to_owned())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "configuration".to_owned());

    let base_path = Path::new(&base_path);
    let base_file = base_path.join("base.yml");
    let environment_file = base_path.join(format!("{environment}.yml"));

    // TODO: Change environment prefix
    let settings = Config::builder()
        .add_source(File::from(base_file))
        .add_source(File::from(environment_file).required(false))
        .add_source(Environment::with_prefix("APP").separator("_"))
        .build()?
        .try_deserialize::<Settings>()?;
    Ok(settings)
}

pub fn get_config_from_file(file: &str) -> Result<Settings> {
    let settings = Config::builder()
        .add_source(File::with_name(file))
        .build()?
        .try_deserialize::<Settings>()?;
    Ok(settings)
}

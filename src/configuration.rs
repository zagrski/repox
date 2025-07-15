use anyhow::Context;
use anyhow::Result;
use config::Config;
use config::Environment;
use config::File;
use config::FileFormat;
use serde::Deserialize;
use std::env as std_env;
use std::fs as std_fs;
use std::str as std_str;

use crate::assets::GlobalAssets;

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub debug: bool,
}

pub fn init() -> Result<Configuration> {
    let exe_path = std_env::current_exe().context("Failed to get executable path")?;
    let exe_directory = exe_path
        .parent()
        .context("Executable has no parent directory")?;
    let embedded_configuration =
        GlobalAssets::get("repox.yaml").context("Failed to get embedded configuration file")?;
    let embedded_configuration_yaml = std_str::from_utf8(embedded_configuration.data.as_ref())
        .context("Failed to convert embedded configuration to string")?;
    let embedded_configuration_source =
        File::from_str(embedded_configuration_yaml, FileFormat::Yaml);
    let local_configuration_path = exe_directory.join("repox.yaml");
    if !local_configuration_path.exists() {
        std_fs::write(
            &local_configuration_path,
            embedded_configuration.data.as_ref(),
        )
        .context("Failed to write local configuration file")?;
    }
    let local_configuration_source = File::from(local_configuration_path)
        .required(true)
        .format(FileFormat::Yaml);
    let environment_source = Environment::with_prefix("REPOX").separator("__");
    let configuration = Config::builder()
        .add_source(embedded_configuration_source)
        .add_source(local_configuration_source)
        .add_source(environment_source)
        .build()
        .context("Failed to build configuration")?
        .try_deserialize::<Configuration>()
        .context("Failed to deserialize configuration")?;
    Ok(configuration)
}

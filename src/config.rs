use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;
use toml::value::Datetime;

pub(crate) const TOML_TEMPLATE: &'static str = "\
    [general]\n\
    # ttf_font_path = '/path/to/font.ttf'\n\
    \n\
    # [countdown]\n\
    # term_start = <YYYY-MM-DD>\n\
    # term_last_lecture = <YYYY-MM-DD>\n\
    # first_paper = <YYYY-MM-DD>\n\
    # last_paper_end_time = <YYYY-MM-DD>T<HH:MM:SS>\n\
    \n\
    # [overlay]\n\
    # text = ''\n\
";

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) general: ConfigGeneral,
    pub(crate) countdown: Option<ConfigCountdown>,
    pub(crate) overlay: Option<ConfigOverlay>,
}

impl Config {
    pub(crate) fn read_from_dir(parent_dir: &str) -> anyhow::Result<Self> {
        let config_toml_path: PathBuf = [parent_dir, "Working", "config.toml"].iter().collect();
        if !config_toml_path.exists() {
            std::fs::write(&config_toml_path, TOML_TEMPLATE)?;
        }
        let toml_str = std::fs::read_to_string(&config_toml_path).with_context(|| {
            format!(
                "Failed to read configuration from {}",
                config_toml_path.to_string_lossy().to_string()
            )
        })?;
        Ok(toml::from_str(&toml_str).with_context(|| {
            format!(
                "Please fix the TOML file at {}. (Is this the first time this program is run?)",
                config_toml_path.to_string_lossy().to_string()
            )
        })?)
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigGeneral {
    pub(crate) ttf_font_path: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigCountdown {
    pub(crate) term_start: Datetime,
    pub(crate) term_last_lecture: Datetime,
    pub(crate) first_paper: Datetime,
    pub(crate) last_paper_end_time: Datetime,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigOverlay {
    pub(crate) text: String,
}

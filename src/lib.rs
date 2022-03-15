use std::convert::TryFrom;
use std::path::PathBuf;

mod error;
mod parser;
mod utils;

use parser::json::parse_json;
use parser::yaml::parse_yaml;
use utils::{get_config_path, get_current_configuration_environment};

#[derive(Debug)]
pub struct Config {
    pub(crate) filetype: FileType,
    pub file_content: String,
}

#[derive(Debug)]
pub enum FileType {
    Json,
    Yaml,
}

impl TryFrom<PathBuf> for FileType {
    // TODO: Replace with custom error type
    type Error = String;

    fn try_from(p: PathBuf) -> Result<Self, Self::Error> {
        if let Some(s) = p.extension() {
            // TODO: Handle error
            let s = s.to_str().unwrap();
            match s {
                "json" => Ok(FileType::Json),
                "yaml" | "yml" => Ok(FileType::Yaml),
                _ => Err("Unsupported FileType".to_string()),
            }
        } else {
            Err("No file detected".to_string())
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let filename = get_current_configuration_environment();
        let config_path = get_config_path();
        let res = std::fs::read_dir(config_path)
            .unwrap()
            .filter(|d| {
                d.as_ref()
                    .unwrap()
                    .file_name()
                    .into_string()
                    .unwrap()
                    .split(".")
                    .next()
                    .unwrap()
                    == filename
            })
            .map(|v| v.unwrap().path())
            .collect::<Vec<PathBuf>>();
        let config_file_path = res.iter().next().unwrap();
        Config {
            filetype: FileType::try_from(config_file_path.to_owned()).unwrap(),
            file_content: std::fs::read_to_string(config_file_path).unwrap(),
        }
    }

    pub fn get<'a, T: for<'de> serde::Deserialize<'de>>(&self, s: &'a str) -> T {
        // TODO: Add regex to validate string path `s`
        match self.filetype {
            FileType::Json => parse_json(&self.file_content, s),
            FileType::Yaml => parse_yaml(&self.file_content, s),
        }
    }
}

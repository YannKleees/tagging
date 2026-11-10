use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{EntityType, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectorKind {
    Email,
    Url,
    Ip,
    Phone,
    Iban,
    CreditCard,
    NationalId,
    Date,
    Address,
    Gazetteer,
}

impl DetectorKind {
    pub const ALL: [Self; 10] = [
        Self::Email,
        Self::Url,
        Self::Ip,
        Self::Phone,
        Self::Iban,
        Self::CreditCard,
        Self::NationalId,
        Self::Date,
        Self::Address,
        Self::Gazetteer,
    ];
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GazetteerFile {
    pub path: PathBuf,
    #[serde(rename = "type")]
    pub kind: EntityType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Wordlist {
    #[serde(rename = "type")]
    pub kind: EntityType,
    pub words: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub detectors: Vec<DetectorKind>,
    pub threshold: f64,
    pub builtin_gazetteers: bool,
    pub gazetteers: Vec<GazetteerFile>,
    pub wordlists: Vec<Wordlist>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            detectors: DetectorKind::ALL.to_vec(),
            threshold: 0.5,
            builtin_gazetteers: true,
            gazetteers: Vec::new(),
            wordlists: Vec::new(),
        }
    }
}

impl Config {
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Gazetteer paths in the file are resolved relative to the config file's directory.
    pub fn from_path(path: &Path) -> Result<Self> {
        let json = read_to_string(path)?;
        let mut config = Self::from_json(&json)?;
        let dir = path.parent().unwrap_or(Path::new(""));
        for g in &mut config.gazetteers {
            if g.path.is_relative() {
                g.path = dir.join(&g.path);
            }
        }
        Ok(config)
    }
}

pub(crate) fn read_to_string(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_path_buf(),
        source,
    })
}

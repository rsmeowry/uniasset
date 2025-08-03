use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde::Deserialize;
use walkdir::WalkDir;

/// A scope that contains info useful for the project
///
/// For example, it maps file IDs to file paths
#[derive(Debug, Clone)]
pub struct ProjectScope {
    base_dir: PathBuf,
    pub files: HashMap<String, PathBuf>,
    scan_config: ScanConfig,
}

impl ProjectScope {
    pub fn init<P>(source: P, cfg: ScanConfig) -> anyhow::Result<Self> where P: Into<PathBuf> {
        let mut slf = Self {
            base_dir: source.into(),
            files: HashMap::default(),
            scan_config: cfg
        };
        let enabled_scan: Vec<&str> = slf.scan_config.extension_filter.split(",").collect();

        for entry in WalkDir::new(&slf.base_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_str().unwrap();
            let parts: Vec<&str> = path.split(".").collect();
            if *parts.last().unwrap() == "meta" && enabled_scan.contains(&parts[parts.len() - 2]) {
                let mut buf = String::new();
                File::open(&entry.path())?.read_to_string(&mut buf)?;
                let meta = serde_yaml_ng::from_str::<MetaFile>(&buf)?;
                if meta.file_format_version != 2 {
                    anyhow::bail!("Invalid file format version encountered: '{}'. Only version 2 is supported at the moment", meta.file_format_version)
                }
                slf.files.insert(meta.guid.into(), entry.path().to_path_buf());
            }
        }

        Ok(slf)
    }
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// File extensions to only search for, comma separated
    pub extension_filter: String
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            extension_filter: "png,jpg,hdr,asset".into()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetaFile {
    pub file_format_version: u8,
    pub guid: String
}
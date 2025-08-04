use crate::data::UnityAssetReference;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Add;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::{Error, Res};

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
    pub fn init<P>(source: P, cfg: ScanConfig) -> Res<Self> where P: Into<PathBuf> {
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
                    return Err(Error::InvalidFormat(meta.file_format_version));
                }
                slf.files.insert(meta.guid.into(), entry.path().to_path_buf());
            }
        }

        Ok(slf)
    }

    pub fn find_asset_by_guid<A, S>(&self, asset_id: S) -> Res<A> where S: AsRef<str>, A: From<UnityAssetReference> {
        let asset_id_str = asset_id.as_ref();
        if !self.files.contains_key(asset_id_str) {
            return Err(Error::GuidNotFound(asset_id_str.to_string()));
        }

        Ok(UnityAssetReference {
            file_id: 0,
            guid: String::from(asset_id_str),
            ty: 0,
        }.into())
    }

    pub fn find_asset_by_name<A, S>(&self, asset_name: S) -> Res<A> where S: AsRef<str>, A: From<UnityAssetReference> {
        let asset_name_str = asset_name.as_ref();
        let found = self.files
            .iter()
            .find(|(_, val)| val.file_name().unwrap().to_str().unwrap().contains(&asset_name_str));

        if let Some(pair) = found {
            Ok(UnityAssetReference {
                file_id: 0,
                guid: pair.0.clone(),
                ty: 0,
            }.into())
        } else {
            Err(Error::NameNotFound(asset_name_str.to_string()))
        }
    }

    pub fn load_scriptable_object<T, P>(&self, path: P) -> Res<T> where T: Serialize + DeserializeOwned, P: AsRef<Path> {
        let file = File::open(path)?;
        serde_yaml_ng::from_reader::<File, MonoBehaviourContainer<T>>(file)
            .map(|v| v.mono).map_err(crate::Error::from)
    }

    pub fn save_scriptable_object<T, P>(&self, asset_base: T, path: P) -> Res<()> where T: Serialize + DeserializeOwned, P: AsRef<Path> {
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let header = buf.split("\n").take(3).collect::<Vec<&str>>().join("\n");

        let mut base_value = serde_yaml_ng::from_str::<Value>(&buf)?;

        let Value::Mapping(asset_value) = serde_yaml_ng::to_value(asset_base)? else { return Err(Error::GenericInvalidFormat) };

        let Value::Mapping(mono) = base_value.get_mut("MonoBehaviour").unwrap() else { return Err(Error::GenericInvalidFormat) };

        for key in asset_value.keys() {
            mono.insert(key.clone(), asset_value.get(key.as_str().unwrap()).unwrap().clone());
        }

        let v = serde_yaml_ng::to_string(&base_value)?;

        let final_str = header.add("\n").add(&v);

        let mut final_write = File::create(&path)?;

        final_write.write_all(&final_str.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct MonoBehaviourContainer<T> {
    #[serde(rename = "MonoBehaviour")]
    pub mono: T
}

#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// File extensions to only search for, comma separated
    pub extension_filter: String
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            extension_filter: "png,jpg,hdr,asset,mat".into()
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetaFile {
    pub file_format_version: u8,
    pub guid: String
}
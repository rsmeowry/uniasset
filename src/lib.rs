use thiserror::Error;

/// Contains all the necessary data formats
pub mod data;
mod scope;

pub use scope::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to perform an IO operation: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to deserialize YAML data: {0}")]
    YamlError(#[from] serde_yaml_ng::Error),
    #[error("Invalid file format encountered in file {0}. File format {1} is not supported")]
    InvalidFormat(String, u8),
    #[error("Couldn't find any assets with this GUID {0}")]
    GuidNotFound(String),
    #[error("Couldn't find any assets with this name {0}")]
    NameNotFound(String),
    #[error("Failed to deserialize asset file")]
    GenericInvalidFormat,
}

pub(crate) type Res<T> = Result<T, Error>;

#[cfg(test)]
mod tests {
    use crate::data::{UnityColor, UnityEnum, UnityMaterial, UnitySprite, UnityTexture2D};
    use crate::scope::{ProjectScope, ScanConfig};
    use serde::{Deserialize, Serialize};
    use crate::unity_enum;

    #[test]
    fn test_open_project() {
        let init = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default());
        assert!(init.is_ok())
    }

    #[test]
    fn test_load_texture() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let tex: UnityTexture2D = project.find_asset_by_guid("9fc5e61e92565b649a2a621367653c24")?;
        assert!(tex.as_path(&project).to_str().is_some());
        Ok(())
    }

    #[test]
    fn test_load_scriptable_object() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let obj: crate::Res<DebugScriptable> = project.load_scriptable_object(r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#);

        assert!(obj.is_ok());

        Ok(())
    }

    #[test]
    fn test_save_scriptable_object() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let mut obj: DebugScriptable = project.load_scriptable_object(r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#)?;

        obj.long_string_val = "This is an another new value!".into();
        obj.debug_enum_val = UnityEnum::from(DebugEnum::Val2);
        obj.texture_val = project.find_asset_by_name("coconut_parts.png")?;
        obj.string_list_val.push("Str".to_string());
        obj.material_val = project.find_asset_by_name("FresnelGlass.mat")?;

        project.save_scriptable_object(obj, r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#)?;

        Ok(())
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DebugScriptable {
        pub texture_val: UnityTexture2D,
        pub float_val: f32,
        pub long_string_val: String,
        pub sprite_val: UnitySprite,
        pub color_val: UnityColor,
        pub debug_enum_val: UnityEnum<DebugEnum>,
        pub string_list_val: Vec<String>,
        pub material_val: UnityMaterial
    }

    unity_enum! {
        DebugEnum {
            Val1 = 0,
            Val2 = 1,
            Val3 = 2
        }
    }

}
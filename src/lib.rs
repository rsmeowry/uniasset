pub mod data;
mod scope;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{UnityColor, UnityEnum, UnitySprite, UnityTexture2D};
    use crate::scope::{ProjectScope, ScanConfig};
    use serde::{Deserialize, Serialize};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_open_project() {
        let init = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default());
        assert!(init.is_ok())
    }

    #[test]
    fn test_load_texture() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let tex: UnityTexture2D = project.find_asset("9fc5e61e92565b649a2a621367653c24")?;
        assert!(tex.as_path(&project).to_str().is_some());
        Ok(())
    }

    #[test]
    fn test_load_scriptable_object() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let obj: anyhow::Result<DebugScriptable> = project.load_scriptable_object(r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#);

        assert!(obj.is_ok());

        Ok(())
    }

    #[test]
    fn test_save_scriptable_object() -> anyhow::Result<()> {
        let project = ProjectScope::init(r#"C:\Users\rm\Projects\Jabki\Assets"#, ScanConfig::default())?;
        let mut obj: DebugScriptable = project.load_scriptable_object(r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#)?;

        obj.long_string_val = "This is an another new value!".into();
        obj.debug_enum_val = UnityEnum::from(DebugEnum::Val2);

        project.save_scriptable_object(obj, r#"C:\Users\rm\Projects\Jabki\Assets\Data\New Debug Scriptable Object.asset"#)?;

        Ok(())
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DebugScriptable {
        pub texture_val: UnityTexture2D,
        pub float_val: f32,
        pub long_string_val: String,
        pub sprite_val: UnitySprite,
        pub color_val: UnityColor,
        pub debug_enum_val: UnityEnum<DebugEnum>,
        pub string_list_val: Vec<String>
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[repr(u8)]
    pub enum DebugEnum {
        Val1,
        Val2,
        Val3
    }

    impl Into<u8> for DebugEnum {
        fn into(self) -> u8 {
            self as u8
        }
    }

    impl From<u8> for DebugEnum {
        fn from(value: u8) -> Self {
            match value {
                0 => Self::Val1,
                1 => Self::Val2,
                _ => Self::Val3,
            }
        }
    }
}

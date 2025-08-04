use std::path::PathBuf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::scope::ProjectScope;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityAssetReference {
    #[serde(rename = "fileID")]
    pub file_id: u32,
    pub guid: String,
    #[serde(rename = "type")]
    pub ty: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnityColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Debug, Clone)]
pub struct UnityMaterial {
    guid: String
}

#[derive(Debug, Clone)]
pub struct UnitySprite {
    guid: String
}

#[derive(Debug, Clone)]
pub struct UnityTexture {
    guid: String
}

#[derive(Debug, Clone)]
pub struct UnityTexture2D {
    guid: String
}

#[derive(Debug, Clone)]
pub struct UnityEnum<E> where E: Into<u8> + From<u8> {
    e: E
}

impl<E> From<E> for UnityEnum<E> where E: Into<u8> + From<u8> {
    fn from(value: E) -> Self {
        UnityEnum {
            e: value
        }
    }
}

impl<E> UnityEnum<E> where E: Into<u8> + From<u8> {
    pub fn get_val(&self) -> &E {
        &self.e
    }
}


impl<E> Serialize for UnityEnum<E> where E: Into<u8> + From<u8> + Clone {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_u8(self.e.clone().into())
    }
}

impl<'de, E> Deserialize<'de> for UnityEnum<E> where E: Into<u8> + From<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        u8::deserialize(deserializer).map(|v| Self { e: E::from(v) } )
    }
}

macro_rules! impl_bases_for_assets {
    ($($asset_ty:ty : $asset_id:literal : $asset_idx:literal);*) => {
        $(
            impl From<UnityAssetReference> for $asset_ty {
                fn from(value: UnityAssetReference) -> Self {
                    Self {
                        guid: value.guid
                    }
                }
            }

            impl Into<UnityAssetReference> for &$asset_ty {
                fn into(self) -> UnityAssetReference {
                    UnityAssetReference {
                        file_id: $asset_id,
                        guid: self.guid.clone(),
                        ty: $asset_idx,
                    }
                }
            }


            impl Serialize for $asset_ty {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer
                {
                    let asset_ref: UnityAssetReference = self.into();
                    asset_ref.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for $asset_ty {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>
                {
                    UnityAssetReference::deserialize(deserializer).map(Self::from)
                }
            }

            impl $asset_ty {
                pub fn as_path<'a>(&self, scope: &'a ProjectScope) -> PathBuf {
                    let mut meta_file_path = scope.files.get(&self.guid).unwrap().clone();
                    meta_file_path.set_extension("");
                    meta_file_path
                }
            }
        )*
    };
}

impl_bases_for_assets! {
    UnityTexture   : 27_00000  : 3;
    UnityTexture2D : 28_00000  : 3;
    UnityMaterial  : 21_00000  : 2;
    UnitySprite    : 213_00000 : 3
}
# uniasset

This library allows you to process Unity scriptable objects with Rust.

## Requirements

This library relies heavily on serde, so you are expected to have it.
No other external libraries are necessary apart from it.

## Usage

### Loading a project

To load a project, you will have to instantiate a `ProjectScope`

```rust
let scope = ProjectScope::init(r#"<PATH TO ASSETS FOLDER>"#, ScanConfig::default());
assert!(scope.is_ok());
```
This will create a new ProjectScope that you can use later.

### Representing a ScriptableObject

If you have this scriptable object in your Unity project:

```csharp
    public class DebugScriptableObject: ScriptableObject
    {
        public float floatVal;
        public Texture textureVal;
        public string longStringVal;
        public Sprite spriteVal;
        public Color colorVal;
        public List<string> stringListVal;
        public Material materialVal;
    }
```

You will have to define it in Rust like this:

```rust
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct DebugScriptable {
        pub float_val: f32,
        pub texture_val: UnityTexture2D,
        pub long_string_val: String,
        pub sprite_val: UnitySprite,
        pub color_val: UnityColor,
        pub string_list_val: Vec<String>,
        pub material_val: UnityMaterial
    }
```

Note the Serialize and Deserialize traits, as well as the important `#[serde(rename_all = "camelCase")]`, without
it the struct will not serialize correctly.

uniasset also provides several types that are necessary for representing certain Unity types, such as UnityTexture for Textures, UnitySprite for sprites, UnityColor for colors, etc. You can find all of them in the docs

### Manipulating a ScriptableObject

To load a ScriptableObject, use `ProjectScope::load_scriptable_object(<PATH TO ASSET>)`. After that you can
change its values as you please and to save, call `ProjectScope::save_scriptable_object(asset, <PATH TO ASSET>)`

### Loading assets

To load certain assets (such as textures, sprites and others) you can use `ProjectScope::find_asset_by_name` and `ProjectScope::find_asset_by_guid`

### Working with enums

Enums are serialized in a specific way in UnityYAML, to support this, uniasset introduces a wrapper type called `UnityEnum`. Use it to store
enums in your scriptable objects.

If this is your enum in Unity:

```csharp
    public enum DebugEnum
    {
        Val1,
        Val2,
        Val3
    }
```

Here's how it will look in Rust

```rust
    unity_enum! {
        DebugEnum {
            Val1 = 0,
            Val2 = 1,
            Val3 = 2
        }
    }
```

This will automatically derive `From` and `Into` traits for your enum. Then you can use it in your scriptable object struct by wrapping it
like this: `UnityEnum<DebugEnum>`

## Roadmap

 * [ ] Macros to simplify struct creation
 * [ ] Support for more Unity asset types
 * [ ] Creating new asset files, not just serializing old ones
use anyhow::Context;
use serde::{Deserialize, Serialize};

// region: rename_all
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum SortBy {
    Name,
    Age,
    Date,
}

#[test]
pub fn serialize_rename_all() -> anyhow::Result<()> {
    let sort = SortBy::Name;
    let serialized = serde_json::to_string(&sort).context("Failed to serialize")?;
    assert_eq!(serialized, r#""name""#);
    Ok(())
}

#[test]
pub fn deserialize_rename_all() -> anyhow::Result<()> {
    let _deserialized: SortBy =
        serde_json::from_str(r#""name""#).context("Failed to deserialize")?;
    Ok(())
}
// endregion

// region: Inner rename_all
#[derive(Serialize, Deserialize)]
struct Outer(SortBy);

#[test]
pub fn serialize_inner_rename_all() -> anyhow::Result<()> {
    let outer = Outer(SortBy::Name);
    let serialized = serde_json::to_string(&outer).context("Failed to serialize")?;
    assert_eq!(serialized, r#""name""#);
    Ok(())
}

#[test]
pub fn deserialize_inner_rename_all() -> anyhow::Result<()> {
    let _deserialized: Outer =
        serde_json::from_str(r#""name""#).context("Failed to deserialize")?;
    Ok(())
}
// endregion

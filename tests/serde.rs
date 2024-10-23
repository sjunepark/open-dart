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
pub fn serialize_rename_all() {
    let sort = SortBy::Name;
    let serialized = serde_json::to_string(&sort).expect("Failed to serialize");
    assert_eq!(serialized, r#""name""#);
}

#[test]
pub fn deserialize_rename_all() {
    let _deserialized: SortBy = serde_json::from_str(r#""name""#).expect("Failed to deserialize");
}
// endregion

// region: Inner rename_all
#[derive(Serialize, Deserialize)]
struct Outer(SortBy);

#[test]
pub fn serialize_inner_rename_all() {
    let outer = Outer(SortBy::Name);
    let serialized = serde_json::to_string(&outer).expect("Failed to serialize");
    assert_eq!(serialized, r#""name""#);
}

#[test]
pub fn deserialize_inner_rename_all() {
    let _deserialized: Outer = serde_json::from_str(r#""name""#).expect("Failed to deserialize");
}
// endregion
